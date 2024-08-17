use russh::{keys::key::KeyPair, server::Server, MethodSet};

struct Server;

impl russh::server::Server for Server {
    type Handler = ;

    fn new_client(&mut self, peer_addr: Option<std::net::SocketAddr>) -> Self::Handler {
        todo!()
    }
}


/// Start an SSH server. 
pub fn server_init() {

    // Create a reasonable default configuration. 
    let config = russh::server::Config {
        // Set an amusing server id
        server_id: russh::SshId::Standard("ssh-smooth-wordings".to_owned()),
        // Set an amusing banner. 
        auth_banner: Some("All are welcome."),
        // No authentication is required for our use case. 
        methods: MethodSet::NONE,
        // TODO: load a key from a file
        keys: vec![KeyPair::generate_ed25519().unwrap()],
        ..Default::default()
    };

    let mut server = Server;

    server.run_on_address(config, ("0.0.0.0", "22"));

}