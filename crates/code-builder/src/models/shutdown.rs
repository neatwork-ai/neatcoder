use std::sync::Arc;
use tokio::{sync::Mutex, task::JoinHandle};

pub struct ShutdownSignal {
    shutdown: Arc<Mutex<bool>>,
}

impl ShutdownSignal {
    pub fn new() -> Self {
        Self {
            shutdown: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn wait_for_signal(&self) -> JoinHandle<Arc<Mutex<bool>>> {
        let shutdown_clone = Arc::clone(&self.shutdown);

        tokio::spawn(async move {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to listen to SIGINT");
            *shutdown_clone.lock().await = true;
            shutdown_clone
        })
    }
}
