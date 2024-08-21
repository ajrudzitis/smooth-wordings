use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::app::{App, AppInstance};

pub struct TestApp {
    app_instances: Arc<Mutex<Vec<TestAppInstance>>>,
    open_sessions: u8,
}

impl TestApp {
    pub fn new() -> Self {
        TestApp {
            app_instances: Arc::new(Mutex::new(Vec::new())),
            open_sessions: 0,
        }
    }
}

#[async_trait]
impl App for TestApp {
    fn new_instance(
        &mut self,
        session: russh::server::Session,
    ) -> std::sync::Arc<dyn crate::app::AppInstance> {
        self.open_sessions = self.open_sessions + 1;
        return Arc::new(TestAppInstance {});
    }

    async fn update(&self) {
        for client in self.app_instances.lock().await.iter() {
            client.update();
        }
    }
}

struct TestAppInstance {}

impl AppInstance for TestAppInstance {}

impl TestAppInstance {
    fn update(&self) {
        todo!()
    }
}
