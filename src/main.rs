mod args;
mod domain;
mod service;
mod tui;

use args::Args;
use clap::Parser;
use service::fetch_urls;
use tui::run_tui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Args::parse();

    let page = fetch_urls(&args.url).await?;
    if page.page_urls.is_empty() {
        return Ok(());
    }

    match args.tui {
        true => run_tui(page).await?,
        false => {
            println!("{}", page.page_urls.join("\n"));
        }
    }

    Ok(())
}
