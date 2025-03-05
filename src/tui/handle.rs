use super::command::Command;
use super::message::Message;
use crate::service::fetch_urls;
use arboard::Clipboard;
use tokio::sync::mpsc::Sender;

pub(super) async fn handle_command(command: Command, event_tx: Sender<Message>) {
    match command {
        Command::GetResults { current, chosen } => {
            tokio::spawn(async move {
                let page = fetch_urls(&chosen).await.map_err(|e| e.to_string());
                let _ = event_tx.try_send(Message::ResultsFetched {
                    previous: current,
                    page_result: page,
                });
            });
        }
        Command::YankContentToClipboard(content) => {
            tokio::task::spawn_blocking(move || {
                let result = copy_content_to_clipboard(&content);
                let _ = event_tx.try_send(Message::ContentYanked(result));
            });
        }
        Command::OpenInBrowser(url) => {
            tokio::task::spawn_blocking(move || {
                let result = open_url_in_browser(&url);

                let _ = event_tx.try_send(Message::UrlOpened(result));
            });
        }
    }
}

fn copy_content_to_clipboard(content: &str) -> anyhow::Result<()> {
    let mut clipboard = Clipboard::new()?;

    clipboard.set_text(content)?;

    Ok(())
}

fn open_url_in_browser(url: &str) -> anyhow::Result<()> {
    open::that(url)?;

    Ok(())
}
