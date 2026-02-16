use std::{net::SocketAddr, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Parser)]
#[command(author, version, about = "Server process")]
struct Cli {
    /// Filesystem path to the SQLite database file.
    #[arg(long, value_name = "PATH")]
    sqlite_path: PathBuf,
    /// gRPC listen address.
    #[arg(long, value_name = "ADDR", default_value = "127.0.0.1:50051")]
    listen_addr: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    info!(
        sqlite_path = %cli.sqlite_path.display(),
        listen_addr = %cli.listen_addr,
        "server started"
    );

    server::run_server(&cli.sqlite_path, cli.listen_addr).await
}
