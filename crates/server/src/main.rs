use std::{net::SocketAddr, path::PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use common::hello::{
    HelloReply, HelloRequest,
    greeter_server::{Greeter, GreeterServer},
};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Default)]
struct HelloService;

#[tonic::async_trait]
impl Greeter for HelloService {
    async fn say_hello(
        &self,
        request: tonic::Request<HelloRequest>,
    ) -> Result<tonic::Response<HelloReply>, tonic::Status> {
        let name = request.into_inner().name;
        let message = format!("Hello, {name}!");

        Ok(tonic::Response::new(HelloReply { message }))
    }
}

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

    let connect_options = SqliteConnectOptions::new()
        .filename(&cli.sqlite_path)
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await
        .with_context(|| {
            format!(
                "failed to connect to sqlite db at {}",
                cli.sqlite_path.display()
            )
        })?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("failed to run database migrations")?;

    info!(
        sqlite_path = %cli.sqlite_path.display(),
        listen_addr = %cli.listen_addr,
        "server started"
    );

    tonic::transport::Server::builder()
        .add_service(GreeterServer::new(HelloService))
        .serve(cli.listen_addr)
        .await
        .with_context(|| format!("grpc server failed on {}", cli.listen_addr))?;

    Ok(())
}
