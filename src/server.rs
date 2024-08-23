use std::sync::Arc;

use async_trait::async_trait;
use log::{info, warn};
use russh::{
    keys::PublicKeyBase64,
    server::{Config, Msg, Server as _, Session},
    Channel, MethodSet,
};
use tokio::sync::Mutex;

use crate::app::App;

#[derive(Clone)]
pub struct AppServer {
    app: Arc<Mutex<dyn App + Send + 'static>>,
    config: Arc<Config>,
    session_id: usize,
}

impl AppServer {
    /// Start an SSH server.
    pub fn new(private_key: russh::keys::key::KeyPair, app: impl App + Send + 'static) -> Self {
        let public_key_base64 = private_key.public_key_base64();

        info!("starting server with public key {public_key_base64}");

        // Create a reasonable default configuration.
        let config = Config {
            // Set an amusing server id
            //server_id: russh::SshId::Standard("ssh-smooth-wordings".to_owned()),
            // Set an amusing banner.
            auth_banner: Some("All are welcome."),
            // No authentication is required for our use case.
            methods: MethodSet::NONE,
            // TODO: load a key from a file
            keys: vec![private_key],
            ..Default::default()
        };
        let config = Arc::new(config);

        Self {
            config: config,
            app: Arc::new(Mutex::new(app)),
            session_id: 0,
        }
    }

    pub async fn run(&mut self) {
        let app = self.app.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                app.lock().await.update().await;
            }
        });

        self.run_on_address(self.config.clone(), ("127.0.0.1", 2222))
            .await
            .unwrap();
    }
}

impl russh::server::Server for AppServer {
    type Handler = Self;

    fn new_client(&mut self, peer_addr: Option<std::net::SocketAddr>) -> Self {
        match peer_addr {
            Some(peer_addr) => info!("received connection from peer {peer_addr}"),
            None => warn!("recieved connection with no peer address"),
        }
        let new_session = self.clone();
        self.session_id += 1;
        new_session
    }
}

#[async_trait]
impl russh::server::Handler for AppServer {
    type Error = russh::Error;

    async fn auth_none(&mut self, _: &str) -> Result<russh::server::Auth, russh::Error> {
        Ok(russh::server::Auth::Accept)
    }

    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        session: &mut Session,
    ) -> Result<bool, russh::Error> {
        self.app
            .lock()
            .await
            .new_instance(self.session_id, channel.id(), session.handle())
            .await;
        Ok(true)
    }
}
