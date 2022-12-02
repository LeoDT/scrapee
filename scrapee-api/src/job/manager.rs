use std::sync::Arc;

use futures::stream::StreamExt;
use tokio::sync::mpsc;

use crate::{
    app_state::AppContext,
    collector::{site::make_site_by_id, CollectedItem, Collector},
    dao::DaoProvider,
    error::ScrapeeResult,
    message::Message,
};

use super::{Job, JobMessage, JobStatus};

pub struct JobManager {
    queue: Vec<Job>,
    app_context: AppContext,

    runner_concurrency: usize,
    item_receive_concurrency: usize,
}

impl JobManager {
    pub fn new(app_context: AppContext) -> Self {
        Self {
            queue: Vec::new(),
            app_context,
            runner_concurrency: 5,
            item_receive_concurrency: 5,
        }
    }

    pub async fn load_all_waiting_job(&mut self) -> ScrapeeResult<()> {
        let jobs = self.dao().find_jobs().await?;

        for job in jobs.iter().filter(|j| j.status == JobStatus::Waiting) {
            self.queue.push(Job::try_from(job.clone())?);
        }

        Ok(())
    }

    async fn run_job(&self, job_id: i32) {
        let job = self.dao().get_job_by_id(job_id).await.unwrap();
        let job = Job::try_from(job).unwrap();

        match job.message {
            JobMessage::Collect { site_id } => {
                match make_site_by_id(site_id, self.dao()).await {
                    Ok(site) => {
                        let tx = self.app_context.message_center.tx();
                        let dao = self.dao();
                        let site = Arc::new(site);
                        let (item_tx, mut item_rx) = mpsc::channel::<(i32, CollectedItem)>(
                            self.item_receive_concurrency * 20,
                        );

                        let collector = Collector::new(site, item_tx);

                        tokio::spawn(async move {
                            collector.collect().await;
                        });

                        while let Some((page_id, new_item)) = item_rx.recv().await {
                            let _ = dao
                                .add_page_content(page_id, new_item.url, new_item.content, false)
                                .await;

                            log::info!("added new page_content for page {}", page_id);
                        }
                    }
                    _ => (), // TODO
                }
            }
            _ => (),
        }

        ()
    }

    pub fn run(&self) {
        let tx = self.app_context.message_center.tx();
        let rx = tx.subscribe();
        let concurrency = self.runner_concurrency;

        tokio::spawn(async move {
            tokio_stream::wrappers::BroadcastStream::new(rx)
                .for_each_concurrent(concurrency, |msg| {
                    let tx = tx.clone();

                    async move {
                        match msg {
                            Ok(msg) => match msg {
                                Message::JobCreated { job_id } => {
                                    log::info!("job manager received message: {:?}", msg);

                                    tx.send(Message::JobUpdated {
                                        job_id,
                                        job_status: JobStatus::Running,
                                    });

                                    self.run_job(job_id).await;

                                    self.dao().finish_job(job_id).await;

                                    tx.send(Message::JobUpdated {
                                        job_id,
                                        job_status: JobStatus::Success,
                                    });
                                }
                                _ => (),
                            },
                            Err(_) => (),
                        }
                    }
                })
                .await
        });
    }
}

impl DaoProvider for JobManager {
    fn dao_app_context(&self) -> AppContext {
        self.app_context.clone()
    }
}
