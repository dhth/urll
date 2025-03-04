#[derive(Clone, Debug)]
pub(super) enum Command {
    GetResults { current: String, chosen: String },
    YankContentToClipboard(String),
    OpenInBrowser(String),
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::GetResults { .. } => write!(f, "get services"),
            Command::YankContentToClipboard(_) => write!(f, "yank url"),
            Command::OpenInBrowser(_) => write!(f, "open url"),
        }
    }
}
