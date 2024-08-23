use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use log::debug;
use russh::{server::Handle, ChannelId, CryptoVec};
use tokio::sync::Mutex;

use crate::app::{App, AppInstance};

pub struct TestApp {
    app_instances: Arc<Mutex<HashMap<usize, Arc<TestAppInstance>>>>,
    open_sessions: u8,
}

impl TestApp {
    pub fn new() -> Self {
        TestApp {
            app_instances: Arc::new(Mutex::new(HashMap::new())),
            open_sessions: 0,
        }
    }
}

#[async_trait]
impl App for TestApp {
    async fn new_instance(
        &mut self,
        session_id: usize,
        channel_id: ChannelId,
        handle: russh::server::Handle,
    ) -> std::sync::Arc<dyn crate::app::AppInstance> {
        self.open_sessions = self.open_sessions + 1;
        let instance = Arc::new(TestAppInstance {
            channel_id: channel_id,
            handle: handle,
        });
        self.app_instances
            .lock()
            .await
            .insert(session_id, instance.clone());
        instance
    }

    async fn update(&mut self) {
        debug!("running update loop");
        let mut failed_instances: Vec<usize> = Vec::new();
        {
            for (session_id, instance) in self.app_instances.lock().await.iter() {
                match instance.update() {
                    Ok(()) => continue,
                    Err(_) => failed_instances.push(*session_id),
                };
            }
        }
        for session_id in failed_instances.iter() {
            self.close_instance(*session_id).await
        }
    }

    async fn close_instance(&mut self, session_id: usize) {
        self.app_instances.lock().await.remove(&session_id);
    }
}

struct TestAppInstance {
    channel_id: ChannelId,
    handle: Handle,
}

impl AppInstance for TestAppInstance {}

impl TestAppInstance {
    fn update(&self) -> Result<(), CryptoVec> {
        debug!("updating app instance!");
        futures::executor::block_on(async {
            self.handle
                .data(self.channel_id, String::from("data").into())
                .await
        })
    }
}
