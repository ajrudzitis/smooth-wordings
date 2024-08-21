use crate::app::App;

pub struct TestApp;

impl App for TestApp {
    fn new_instance(
        &self,
        session: russh::server::Session,
    ) -> std::sync::Arc<dyn crate::app::AppInstance> {
        todo!()
    }
}
