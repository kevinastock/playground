use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Parser)]
#[command(author, version, about = "Server process")]
struct Cli {
    /// Filesystem path to the SQLite database file.
    #[arg(long, value_name = "PATH")]
    sqlite_path: PathBuf,
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

    info!(sqlite_path = %cli.sqlite_path.display(), "server started");

    Ok(())
}
