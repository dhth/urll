use super::common::*;
use super::message::Message;
use super::model::*;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

pub fn get_event_handling_msg(model: &Model, event: Event) -> Option<Message> {
    match event {
        Event::Key(key_event) => match model.terminal_too_small {
            true => match key_event.kind {
                KeyEventKind::Press => match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => Some(Message::GoBackOrQuit),
                    _ => None,
                },
                _ => None,
            },
            false => match key_event.kind {
                KeyEventKind::Press => match model.active_pane {
                    Pane::ResultsList => match key_event.code {
                        KeyCode::Enter => match model.results {
                            Ok(_) => Some(Message::UrlChosen),
                            Err(_) => None,
                        },
                        KeyCode::Backspace => Some(Message::GoBack),
                        KeyCode::Char('j') | KeyCode::Down => Some(Message::GoToNextListItem),
                        KeyCode::Char('k') | KeyCode::Up => Some(Message::GoToPreviousListItem),
                        KeyCode::Char('g') => Some(Message::GoToFirstListItem),
                        KeyCode::Char('G') => Some(Message::GoToLastListItem),
                        KeyCode::Char('y') => Some(Message::YankUrlToClipboard),
                        KeyCode::Char('Y') => Some(Message::YankUrlsToClipboard),
                        KeyCode::Char('o') => Some(Message::OpenUrlInBrowser),
                        KeyCode::Char('?') => Some(Message::GoToPane(Pane::Help)),
                        KeyCode::Esc | KeyCode::Char('q') => Some(Message::GoBackOrQuit),
                        KeyCode::Char('c') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                Some(Message::QuitImmediately)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    },
                    Pane::Help => match key_event.code {
                        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?') => {
                            Some(Message::GoBackOrQuit)
                        }
                        KeyCode::Char('c') => {
                            if key_event.modifiers == KeyModifiers::CONTROL {
                                Some(Message::QuitImmediately)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    },
                },
                _ => None,
            },
        },
        Event::Resize(w, h) => Some(Message::TerminalResize(w, h)),
        _ => None,
    }
}
