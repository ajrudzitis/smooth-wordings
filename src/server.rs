use std::sync::Arc;

use async_trait::async_trait;
use log::{info, warn};
use russh::{keys::PublicKeyBase64, server::{Config, Msg, Server as _, Session}, Channel, MethodSet};

use crate::app::App;

/// Server implements an SSH server interface to the App.
/// It doesn't do much besides log information and log the address that is connectiong. 
struct Server {
    app: Arc<dyn App>,
}

impl russh::server::Server for Server {
    type Handler = crate::server::Handler;

    fn new_client(&mut self, peer_addr: Option<std::net::SocketAddr>) -> Self::Handler {
        match peer_addr {
            Some(peer_addr) => info!("received connection from peer {peer_addr}"),
            None => warn!("recieved connection with no peer address")
        }
        return Handler{ app: self.app.clone() };
    }
}

struct Handler {
    app: Arc<dyn App>,
}

#[async_trait]
impl russh::server::Handler for Handler {
    type Error = russh::Error;

    async fn auth_none(
        &mut self,
        _: &str,
    ) -> Result<russh::server::Auth, russh::Error> {
        Ok(russh::server::Auth::Accept)
    }

    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        _session: &mut Session,
    ) -> Result<bool, russh::Error> {
        {
            let _ = channel.data("hello".as_bytes()).await;
        }
        Ok(true)
    }
}

/// Start an SSH server. 
pub async fn server_init<T: App>(private_key: russh::keys::key::KeyPair, app: T) {

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

    let mut s = Server{app: Arc::new(app)};

    s.run_on_address(config, ("127.0.0.1", 2222)).await.unwrap();
}