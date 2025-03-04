use ratatui::style::Color;

pub const SECTION_TITLE_FG_COLOR: Color = Color::from_u32(0x282828);
pub const PRIMARY_COLOR: Color = Color::from_u32(0xfe8019);
pub const PRIMARY_BORDER_COLOR: Color = Color::from_u32(0x665c54);
pub const URL_COLOR: Color = Color::from_u32(0xfabd2f);
pub const SECONDARY_COLOR: Color = Color::from_u32(0xb8bb26);
pub const HELP_COLOR: Color = Color::from_u32(0xfabd2f);
pub const INFO_MESSAGE_COLOR: Color = Color::from_u32(0x83a598);
pub const ERROR_COLOR: Color = Color::from_u32(0xfb4934);

pub const TITLE: &str = " urll ";
pub const MIN_TERMINAL_WIDTH: u16 = 64;
pub const MIN_TERMINAL_HEIGHT: u16 = 30;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Pane {
    ResultsList,
    Help,
}

impl std::fmt::Display for Pane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pane::ResultsList => write!(f, "rl"),
            Pane::Help => write!(f, "h"),
        }
    }
}

pub(super) struct TerminalDimensions {
    pub(super) width: u16,
    pub(super) height: u16,
}
