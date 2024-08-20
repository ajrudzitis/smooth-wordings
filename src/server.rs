use std::sync::Arc;

use async_trait::async_trait;
use russh::{keys::key::KeyPair, server::{Msg, Server as _, Session}, Channel, MethodSet};

#[derive(Clone)]
struct Server;

impl russh::server::Server for Server {
    type Handler = Self;

    fn new_client(&mut self, _peer_addr: Option<std::net::SocketAddr>) -> Self::Handler {
        self.clone()
    }
}

#[async_trait]
impl russh::server::Handler for Server {
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
pub async fn server_init() {

    // Create a reasonable default configuration. 
    let config = russh::server::Config {
        // Set an amusing server id
        //server_id: russh::SshId::Standard("ssh-smooth-wordings".to_owned()),
        // Set an amusing banner. 
        auth_banner: Some("All are welcome."),
        // No authentication is required for our use case. 
        methods: MethodSet::NONE,
        // TODO: load a key from a file
        keys: vec![KeyPair::generate_ed25519().unwrap()],
        ..Default::default()
    };
    let config = Arc::new(config);

    let mut s = Server;

    s.run_on_address(config, ("127.0.0.1", 2222)).await.unwrap();
}