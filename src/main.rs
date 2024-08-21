use std::fs;

use clap::Parser;
use russh::keys::decode_secret_key;
use tokio;

mod server;
mod app;
mod example;

/// An art project of smooth wordings
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the key file to load
    #[arg(short, long)]
    key_file: String,
}

fn main() {
    let args = Args::parse();

    let key_contents = fs::read_to_string(args.key_file).expect("unable to read key file");

    let private_key = decode_secret_key(&key_contents, Option::None).expect("unable to parse private key");

    let app = example::TestApp;

    let mut app_server = server::AppServer::new(private_key, app);

    simple_logger::init_with_level(log::Level::Info).expect("unable to initialize logging");
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("unable to start tokio")
        .block_on(async {
            app_server.run().await;
        });
}
