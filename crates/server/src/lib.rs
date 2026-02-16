use std::{future::Future, net::SocketAddr, path::Path};

use anyhow::{Context, Result};
use common::hello::{
    HelloReply, HelloRequest,
    greeter_server::{Greeter, GreeterServer},
};
use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;

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

async fn build_pool(connect_options: SqliteConnectOptions) -> Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await
        .context("failed to connect to sqlite db")?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("failed to run database migrations")?;

    Ok(pool)
}

pub async fn run_server(sqlite_path: impl AsRef<Path>, listen_addr: SocketAddr) -> Result<()> {
    let connect_options = SqliteConnectOptions::new()
        .filename(sqlite_path.as_ref())
        .create_if_missing(true);

    let _pool = build_pool(connect_options).await?;

    tonic::transport::Server::builder()
        .add_service(GreeterServer::new(HelloService))
        .serve(listen_addr)
        .await
        .with_context(|| format!("grpc server failed on {listen_addr}"))?;

    Ok(())
}

pub async fn run_server_with_listener<F>(
    sqlite_path: impl AsRef<Path>,
    listener: TcpListener,
    shutdown_signal: F,
) -> Result<()>
where
    F: Future<Output = ()> + Send + 'static,
{
    let connect_options = SqliteConnectOptions::new()
        .filename(sqlite_path.as_ref())
        .create_if_missing(true);

    run_server_with_listener_and_options(connect_options, listener, shutdown_signal).await
}

pub async fn run_server_with_listener_and_options<F>(
    connect_options: SqliteConnectOptions,
    listener: TcpListener,
    shutdown_signal: F,
) -> Result<()>
where
    F: Future<Output = ()> + Send + 'static,
{
    let _pool = build_pool(connect_options).await?;
    let local_addr = listener.local_addr().context("failed to read local addr")?;

    tonic::transport::Server::builder()
        .add_service(GreeterServer::new(HelloService))
        .serve_with_incoming_shutdown(TcpListenerStream::new(listener), shutdown_signal)
        .await
        .with_context(|| format!("grpc server failed on {local_addr}"))?;

    Ok(())
}
