use std::sync::Arc;

use async_trait::async_trait;
use russh::{server::Handle, ChannelId};

#[async_trait]
pub trait App {
    async fn new_instance(
        &mut self,
        session_id: usize,
        channel_id: ChannelId,
        handle: Handle,
    ) -> Arc<dyn AppInstance>;
    async fn close_instance(&mut self, session_id: usize);
    async fn update(&mut self);
}

pub trait AppInstance {}
