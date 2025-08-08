use super::common::{MIN_TERMINAL_HEIGHT, MIN_TERMINAL_WIDTH, Pane, TerminalDimensions};
use crate::domain::{Page, PageDetails};
use ratatui::widgets::ListState;
use std::collections::HashMap;
use std::collections::VecDeque;
const USER_MESSAGE_DEFAULT_FRAMES: u16 = 4;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug)]
pub enum MessageKind {
    Info,
    Error,
}

pub struct UserMessage {
    pub frames_left: u16,
    pub value: String,
    pub kind: MessageKind,
}

impl UserMessage {
    pub(super) fn info(message: &str) -> Self {
        UserMessage {
            frames_left: USER_MESSAGE_DEFAULT_FRAMES,
            value: message.to_string(),
            kind: MessageKind::Info,
        }
    }
    pub(super) fn error(message: &str) -> Self {
        UserMessage {
            frames_left: USER_MESSAGE_DEFAULT_FRAMES,
            value: message.to_string(),
            kind: MessageKind::Error,
        }
    }

    pub(super) fn with_frames_left(mut self, frames_left: u16) -> Self {
        self.frames_left = frames_left;
        self
    }
}

#[derive(Debug)]
pub(crate) struct Results {
    pub(crate) items: Vec<String>,
    pub(crate) state: ListState,
}

impl Default for Results {
    fn default() -> Self {
        let items: Vec<String> = Vec::new();
        let state = ListState::default().with_selected(Some(0));

        Self { items, state }
    }
}

impl From<&Vec<String>> for Results {
    fn from(value: &Vec<String>) -> Self {
        let items = value.iter().map(|line| line.to_string()).collect();
        let state = ListState::default().with_selected(Some(0));

        Self { items, state }
    }
}

impl From<&Page> for Results {
    fn from(page: &Page) -> Self {
        let items = page.page_urls.iter().map(|line| line.to_string()).collect();
        let state = ListState::default().with_selected(Some(0));

        Self { items, state }
    }
}

pub(crate) struct Model {
    pub active_pane: Pane,
    pub page_details: PageDetails,
    pub results: Result<Results, String>,
    pub results_cache: HashMap<String, Page>,
    pub history: VecDeque<String>,
    pub debug: bool,
    pub event_counter: u64,
    pub last_active_pane: Option<Pane>,
    pub render_counter: u64,
    pub running_state: RunningState,
    pub terminal_dimensions: TerminalDimensions,
    pub terminal_too_small: bool,
    pub user_message: Option<UserMessage>,
}

impl Model {
    pub(crate) fn new(page: Page, terminal_dimensions: TerminalDimensions, debug: bool) -> Self {
        let terminal_too_small = terminal_dimensions.width < MIN_TERMINAL_WIDTH
            || terminal_dimensions.height < MIN_TERMINAL_HEIGHT;
        let results = Ok(Results::from(&page.page_urls));
        let mut results_cache = HashMap::new();
        let page_details = page.details.clone();
        results_cache.insert(page.details.url.clone(), page);

        Self {
            active_pane: Pane::ResultsList,
            page_details,
            results,
            results_cache,
            history: VecDeque::new(),
            debug,
            event_counter: 0,
            last_active_pane: None,
            render_counter: 0,
            running_state: RunningState::Running,
            terminal_dimensions,
            terminal_too_small,
            user_message: None,
        }
    }
    pub(super) fn go_back_or_quit(&mut self) {
        let active_pane = Some(self.active_pane);
        match self.active_pane {
            Pane::ResultsList => self.running_state = RunningState::Done,
            Pane::Help => self.active_pane = self.last_active_pane.unwrap_or(Pane::ResultsList),
        }

        self.last_active_pane = active_pane;
    }

    pub(super) fn select_next_list_item(&mut self) {
        match self.active_pane {
            Pane::ResultsList => {
                if let Ok(r) = &mut self.results {
                    r.state.select_next();
                }
            }
            Pane::Help => {}
        }
    }

    pub(super) fn select_previous_list_item(&mut self) {
        if self.active_pane == Pane::ResultsList
            && let Ok(r) = &mut self.results
        {
            r.state.select_previous();
        }
    }

    pub(super) fn select_first_list_item(&mut self) {
        if self.active_pane == Pane::ResultsList
            && let Ok(r) = &mut self.results
        {
            r.state.select_first();
        }
    }
    pub(super) fn select_last_list_item(&mut self) {
        if self.active_pane == Pane::ResultsList
            && let Ok(r) = &mut self.results
        {
            r.state.select_last();
        }
    }

    pub fn get_selected_url(&self) -> Option<(String, usize)> {
        match &self.results {
            Ok(r) => {
                let index = r.state.selected()?;
                r.items.get(index).map(|si| (si.clone(), index))
            }
            Err(_) => None,
        }
    }
}
