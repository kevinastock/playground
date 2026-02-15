use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, value_name = "URL", default_value = "http://127.0.0.1:50051")]
    server_url: String,
    #[arg(long, value_name = "NAME", default_value = "world")]
    name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut client = client::Client::connect(args.server_url.clone()).await?;
    let greeting = client.say_hello(args.name).await?;

    println!("server url: {}", args.server_url);
    println!("rpc response: {greeting}");

    Ok(())
}
