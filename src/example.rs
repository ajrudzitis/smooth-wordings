use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use log::debug;
use russh::{server::Handle, ChannelId};
use tokio::sync::Mutex;

use crate::app::{App, AppInstance};

pub struct TestApp {
    app_instances: Arc<Mutex<HashMap<ChannelId, Arc<TestAppInstance>>>>,
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
        channel_id: ChannelId,
        handle: russh::server::Handle,
    ) -> std::sync::Arc<dyn crate::app::AppInstance> {
        self.open_sessions = self.open_sessions + 1;
        let instance = Arc::new(TestAppInstance {
            channel_id: channel_id,
            handle: handle,
        });
        self.app_instances.lock().await.insert(channel_id, instance.clone());
        instance
    }

    async fn update(&self) {
        debug!("running update loop");
        for (_channel, instance) in self.app_instances.lock().await.iter() {
            instance.update();
        }
    }

    async fn close_instance(&mut self, channel_id: ChannelId) {
        self.app_instances.lock().await.remove(&channel_id);
    }
}

struct TestAppInstance {
    channel_id: ChannelId,
    handle: Handle,
}

impl AppInstance for TestAppInstance {}

impl TestAppInstance {
    fn update(&self) {
        debug!("updating app instance!");
        futures::executor::block_on(async {
            let _ = self
                .handle
                .data(self.channel_id, String::from("data").into())
                .await;
        })
    }
}
