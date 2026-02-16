use std::{
    net::SocketAddr,
    str::FromStr,
    sync::atomic::{AtomicU64, Ordering},
};

use anyhow::{Context, Result};
use client::Client;
use sqlx::sqlite::SqliteConnectOptions;
use tokio::{
    net::TcpListener,
    sync::oneshot,
    task::JoinHandle,
    time::{Duration, sleep},
};

static NEXT_DB_ID: AtomicU64 = AtomicU64::new(1);

pub struct TestHarness {
    endpoint: String,
    shutdown_tx: Option<oneshot::Sender<()>>,
    server_task: JoinHandle<Result<()>>,
}

impl TestHarness {
    pub async fn start() -> Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .context("failed to bind test server listener")?;
        let addr = listener
            .local_addr()
            .context("failed to read test server addr")?;
        let endpoint = format!("http://{addr}");

        let db_id = NEXT_DB_ID.fetch_add(1, Ordering::Relaxed);
        let sqlite_url = format!("sqlite:file:integration-test-{db_id}?mode=memory&cache=shared");
        let connect_options = SqliteConnectOptions::from_str(&sqlite_url)
            .context("failed to build in-memory sqlite connect options")?;

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let server_task = tokio::spawn(async move {
            server::run_server_with_listener_and_options(connect_options, listener, async move {
                let _ = shutdown_rx.await;
            })
            .await
        });

        wait_for_server(addr).await?;

        Ok(Self {
            endpoint,
            shutdown_tx: Some(shutdown_tx),
            server_task,
        })
    }

    pub async fn connect_client(&self) -> Result<Client> {
        Client::connect(self.endpoint.clone())
            .await
            .context("failed to connect client")
    }

    pub async fn shutdown(mut self) -> Result<()> {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _ = shutdown_tx.send(());
        }

        self.server_task
            .await
            .context("server task join failure")??;

        Ok(())
    }
}

async fn wait_for_server(addr: SocketAddr) -> Result<()> {
    let endpoint = format!("http://{addr}");

    for _ in 0..40 {
        if Client::connect(endpoint.clone()).await.is_ok() {
            return Ok(());
        }

        sleep(Duration::from_millis(25)).await;
    }

    anyhow::bail!("server did not start listening in time")
}
