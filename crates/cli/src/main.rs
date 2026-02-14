use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, value_name = "PATH")]
    sqlite_db: PathBuf,
    #[arg(long, value_name = "URL")]
    server_url: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut terminal = ratatui::init();
    let draw_result = terminal.draw(|frame| {
        let message = format!(
            "sqlite db: {}\nserver url: {}",
            args.sqlite_db.display(),
            args.server_url
        );
        frame.render_widget(ratatui::widgets::Paragraph::new(message), frame.area());
    });
    ratatui::restore();
    draw_result?;

    println!("sqlite db: {}", args.sqlite_db.display());
    println!("server url: {}", args.server_url);

    Ok(())
}
