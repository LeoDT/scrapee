use std::{
    collections::{HashMap, HashSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use futures::stream::StreamExt;
use tokio::sync::{mpsc, Barrier};
use tokio::time::sleep;

use libxml::parser::Parser;
use libxml::xpath;
use url::Url;

use crate::error;
use crate::site::{Page, Site};

pub struct Collector {
    site: Arc<Site>,
    delay: Duration,
    crawling_concurrency: usize,
    processing_concurrency: usize,
}

#[derive(Debug, Clone)]
struct CollectedItem {
    pub url: String,
    pub content: HashMap<String, Vec<String>>,
}

impl Collector {
    pub fn new(site: Arc<Site>) -> Self {
        Self {
            site,
            delay: Duration::from_millis(200),
            crawling_concurrency: 3,
            processing_concurrency: 100,
        }
    }

    pub async fn collect(&self) {
        let mut visited_urls = HashSet::<String>::new();
        let crawling_concurrency = self.crawling_concurrency;
        let crawling_queue_capacity = crawling_concurrency * 100;
        let processing_concurrency = self.processing_concurrency;
        let processing_queue_capacity = processing_concurrency * 10;
        let active_crawlers = Arc::new(AtomicUsize::new(0));

        let (urls_to_visit_tx, urls_to_visit_rx) = mpsc::channel::<String>(crawling_queue_capacity);
        let (items_tx, items_rx) = mpsc::channel::<CollectedItem>(processing_queue_capacity);
        let (new_urls_tx, mut new_urls_rx) =
            mpsc::channel::<(String, Vec<String>)>(crawling_queue_capacity);
        let barrier = Arc::new(Barrier::new(3));

        let start_url = self.site.get_start_urls();

        for url in start_url {
            visited_urls.insert(url.clone());
            let _ = urls_to_visit_tx.send(url).await;
        }

        self.launch_processors(
            self.site.clone(),
            processing_concurrency,
            items_rx,
            barrier.clone(),
        );

        self.lauch_scrapers(
            self.site.clone(),
            crawling_concurrency,
            urls_to_visit_rx,
            new_urls_tx.clone(),
            items_tx,
            active_crawlers.clone(),
            self.delay,
            barrier.clone(),
        );

        loop {
            if let Some((visited_url, new_urls)) = new_urls_rx.try_recv().ok() {
                visited_urls.insert(visited_url);

                for url in new_urls {
                    if !visited_urls.contains(&url) {
                        visited_urls.insert(url.clone());
                        log::debug!("queueing: {}", url);

                        let _ = urls_to_visit_tx.send(url).await;
                    }
                }
            }

            if new_urls_tx.capacity() == crawling_queue_capacity // new_urls channel is empty
		&& urls_to_visit_tx.capacity() == crawling_queue_capacity // urls_to_visit channel is empty
		&& active_crawlers.load(Ordering::SeqCst) == 0
            {
                // no more work, we leave
                break;
            }

            sleep(Duration::from_millis(5)).await;
        }

        log::info!("collector: control loop exited");

        // we drop the transmitter in order to close the stream
        drop(urls_to_visit_tx);

        // and then we wait for the streams to complete
        barrier.wait().await;
    }

    fn launch_processors(
        &self,
        site: Arc<Site>,
        concurrency: usize,
        items: mpsc::Receiver<CollectedItem>,
        barrier: Arc<Barrier>,
    ) {
        tokio::spawn(async move {
            tokio_stream::wrappers::ReceiverStream::new(items)
                .for_each_concurrent(concurrency, |item| {
		    let site = site.clone();

		    async move {
			log::debug!("got item: {:?} {}", item, site.name);

			if let Some(page) = site.get_page_for_url(item.url.clone()) {
			    
			} else {
			    log::warn!("not page found for url {}", item.url);
			}
		    }
                })
                .await;

            barrier.wait().await;
        });
    }

    fn lauch_scrapers(
        &self,
        site: Arc<Site>,
        concurrency: usize,
        urls_to_vist: mpsc::Receiver<String>,
        new_urls: mpsc::Sender<(String, Vec<String>)>,
        items_tx: mpsc::Sender<CollectedItem>,
        active_crawlers: Arc<AtomicUsize>,
        delay: Duration,
        barrier: Arc<Barrier>,
    ) {
        tokio::spawn(async move {
            tokio_stream::wrappers::ReceiverStream::new(urls_to_vist)
                .for_each_concurrent(concurrency, |queued_url| {
                    async {
                        active_crawlers.fetch_add(1, Ordering::SeqCst);

                        let parsed_queued_url = url::Url::parse(queued_url.as_str()).unwrap();
                        let mut urls = Vec::new();

                        let res = crawl(CrawlArgs::HttpGet {
                            url: queued_url.clone(),
                        })
                        .await
                        .map_err(|err| {
                            log::error!("crawl error: {}", err);
                            err
                        })
                        .ok();

                        if let Some(text) = res {
                            let page = site.get_page_for_url(queued_url.clone()).unwrap();
                            let (output, links) = scrape(text, parsed_queued_url, page.clone());

                            let _ = items_tx
                                .send(CollectedItem {
                                    url: queued_url.clone(),
                                    content: output,
                                })
                                .await;

                            urls = links;
                        }

                        let _ = new_urls.send((queued_url, urls)).await;
                        sleep(delay).await;

                        active_crawlers.fetch_sub(1, Ordering::SeqCst);
                    }
                })
                .await;

            drop(items_tx);
            barrier.wait().await;
        });
    }
}

pub enum CrawlArgs {
    HttpGet { url: String },
    BrowserGet { url: String },
}

pub async fn crawl(args: CrawlArgs) -> Result<String, error::Error> {
    match args {
        CrawlArgs::HttpGet { url } => {
            log::debug!("http get: {}", url);

            Ok(reqwest::get(url).await?.text().await?)
        }
        CrawlArgs::BrowserGet { url } => {
            log::debug!("browser get: {}", url);

            Ok("test".to_string())
        }
    }
}

pub fn scrape(
    text: String,
    base_url: Url,
    page: Arc<Page>,
) -> (HashMap<String, Vec<String>>, Vec<String>) {
    let parser = Parser::default_html();
    let doc = parser.parse_string(text).unwrap();
    let ctx = xpath::Context::new(&doc).unwrap();

    let mut content: HashMap<String, Vec<String>> = HashMap::new();
    let mut links = Vec::new();

    for field in page.fields.iter() {
        let selected = ctx.evaluate(&field.xpath).unwrap();

        let mut result = Vec::new();
        for node in &selected.get_nodes_as_vec() {
            // get attribute value for attr and full html for others
            let mut scraped = if let Some(t) = node.get_type() {
                match t {
                    libxml::tree::NodeType::AttributeNode => node.get_content(),
                    _ => doc.node_to_string(node),
                }
            } else {
                node.get_content()
            };

            if field.try_follow {
                // normalize urls
                match base_url.join(scraped.as_str()) {
                    Ok(u) => {
                        scraped = u.to_string();
                        links.push(u.to_string());
                    }
                    Err(_) => log::warn!("got scraped try_follow content not an url: {}", scraped),
                }
            }

            result.push(scraped);
        }

        content.insert(field.name.clone(), result);
    }

    (content, links)
}

struct PageContent {
    url: String,
    raw: HashMap<String, Vec<String>>,
    page: Arc<Page>,
}

impl PageContent {
    pub fn new(url: String, raw: HashMap<String, Vec<String>>, page: Arc<Page>) -> Self {
	Self {
	    url: url.to_owned(),
	    raw: raw.to_owned(),
	    page,
	}
    }
}
