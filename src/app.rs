use std::sync::Arc;

use russh::server::Session;

pub trait App {
    fn new_instance(&self, session: Session) -> Arc<dyn AppInstance>;
}

pub trait AppInstance {}
