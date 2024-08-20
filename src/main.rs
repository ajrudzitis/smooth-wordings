
mod server;

use tokio;

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("unable to start tokio")
        .block_on(async {
            server::server_init().await;
        });
}
