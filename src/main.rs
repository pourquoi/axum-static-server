use std::net::ToSocketAddrs;
use std::path::PathBuf;
use axum::Router;
use clap::Parser;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, default_value = "127.0.0.1", env)]
    host: String,
    #[clap(long, default_value = "3000", env)]
    port: u32,
    #[clap(long, default_value = "./")]
    dir: PathBuf
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let service = ServeDir::new(args.dir.clone())
        .not_found_service(ServeFile::new(args.dir.join("index.html")));

    let app = Router::new()
        .fallback_service(service)
        .layer(TraceLayer::new_for_http());

    let addr = format!("{}:{}", args.host, args.port).to_socket_addrs().unwrap().next().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
