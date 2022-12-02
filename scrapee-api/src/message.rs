use tokio::sync::broadcast::{channel, Receiver, Sender};

use crate::job::JobStatus;

#[derive(Debug, Clone)]
pub enum Message {
    JobCreated { job_id: i32 },
    JobUpdated { job_id: i32, job_status: JobStatus },
}

pub struct MessageCenter {
    tx: Sender<Message>,
}

impl MessageCenter {
    pub fn new() -> Self {
        let (tx, rx) = channel(64);

        start_message_logger(rx);

        Self { tx }
    }

    pub fn tx(&self) -> Sender<Message> {
        self.tx.clone()
    }
}

fn start_message_logger(mut rx: Receiver<Message>) {
    tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Ok(msg) => log::info!("message center received: {:?}", msg),
                Err(err) => {
                    log::info!("message center error: {}", err);
                    break;
                }
            }
        }
    });
}
