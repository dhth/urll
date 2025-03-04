use super::common::Pane;
use crate::domain::Page;

pub(crate) enum Message {
    TerminalResize(u16, u16),
    GoToNextListItem,
    GoToPreviousListItem,
    GoToFirstListItem,
    GoToLastListItem,
    YankUrlToClipboard,
    YankUrlsToClipboard,
    OpenUrlInBrowser,
    GoToPane(Pane),
    GoBack,
    UrlChosen,
    ResultsFetched {
        current: String,
        chosen: String,
        page_result: Result<Page, String>,
    },
    ContentYanked(anyhow::Result<()>),
    UrlOpened(anyhow::Result<()>),
    GoBackOrQuit,
    QuitImmediately,
}
