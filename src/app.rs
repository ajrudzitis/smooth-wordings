use std::sync::Arc;

use async_trait::async_trait;
use russh::server::Session;

#[async_trait]
pub trait App {
    fn new_instance(&mut self, session: Session) -> Arc<dyn AppInstance>;
    async fn update(&self);
}

pub trait AppInstance {}
