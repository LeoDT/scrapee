use std::{env, sync::Arc};

use tokio;

use scrapee_api::collector::Collector;

use scrapee_api::collector::site::{make_url_pattern, Site, Field, Page};

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let pages = vec![Page {
        name: "index".to_string(),
        url: Some("https://bbs.saraba1st.com/2b/forum-75-1.html".to_string()),
        url_pattern: make_url_pattern("https://bbs.saraba1st.com/2b/forum-75-{{page}}.html".to_string()).unwrap(),
        fields: vec![
            Field {
                name: "排名".to_string(),
                xpath: r#"/html/body/div[8]/div[4]/div/div/div[1]/div[1]/h1/span/strong[3]/text()"#
                    .to_string(),
                try_follow: false,
                group_to: None,
            },
            Field {
                name: "版规".to_string(),
                xpath: r#"/html/body/div[8]/div[4]/div/div/div[1]/div[2]/div[2]/div/text()"#
                    .to_string(),
                try_follow: false,
                group_to: None,
            },
	    Field {
		name: "链接".to_string(),
		xpath: r#"//table[@id="threadlisttableid"]/tbody[contains(@id, "normalthread")]/tr/th/a[2]/@href"#.to_string(),
		try_follow: true,
		group_to: Some("帖子".to_string())
	    },
	    Field {
		name: "标题".to_string(),
		xpath: r#"//table[@id="threadlisttableid"]/tbody[contains(@id, "normalthread")]/tr/th/a[2]/text()"#.to_string(),
		try_follow: false,
		group_to: Some("帖子".to_string())
	    }
        ],
    }, Page {
	name: "detail".to_string(),
	url: None,
	url_pattern: make_url_pattern("https://bbs.saraba1st.com/2b/thread-{{id}}-{{page}}-1.html".to_string()).unwrap(),
	fields: vec![
	    Field {
		name: "内容".to_string(),
		xpath: r#"(//td[contains(@id, "postmessage")])[1]"#.to_string(),
		try_follow: false,
		group_to: None
	    }
	]
    }];

    let site = Site {
        name: "Saraba1st".to_string(),
        save_context: false,
        pages: pages.into_iter().map(|p| Arc::new(p)).collect(),
    };

    let site = Arc::new(site);

    let collector = Collector::new(site);

    collector.collect().await;

    ()
}
