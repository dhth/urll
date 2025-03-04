use super::common::*;
use super::event::get_event_handling_msg;
use super::handle::handle_command;
use super::message::Message;
use super::model::{Model, RunningState};
use super::update::update;
use super::view::view;
use crate::domain::Page;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io::Error as IOError;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

const EVENT_POLL_DURATION_MS: u64 = 16;

pub async fn run_tui(page: Page) -> anyhow::Result<()> {
    let mut tui = AppTui::new(page)?;
    tui.run().await?;

    Ok(())
}

struct AppTui {
    pub(super) terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    pub(super) event_tx: Sender<Message>,
    pub(super) event_rx: Receiver<Message>,
    pub(super) model: Model,
}

impl AppTui {
    pub fn new(page: Page) -> anyhow::Result<Self> {
        let terminal = ratatui::try_init()?;
        let (event_tx, event_rx) = mpsc::channel(10);

        let (width, height) = ratatui::crossterm::terminal::size()?;

        let terminal_dimensions = TerminalDimensions { width, height };

        let debug = std::env::var("URLL_DEBUG").unwrap_or_default().trim() == "1";

        let model = Model::new(page, terminal_dimensions, debug);

        Ok(Self {
            terminal,
            event_tx,
            event_rx,
            model,
        })
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        let _ = self.terminal.clear();

        // first render
        self.model.render_counter += 1;
        self.terminal.draw(|f| view(&mut self.model, f))?;

        loop {
            tokio::select! {
                Some(message) = self.event_rx.recv() => {
                    let cmds = update(&mut self.model, message);

                    if self.model.running_state == RunningState::Done {
                        self.exit()?;
                        return Ok(());
                    }

                        self.model.render_counter += 1;
                        self.terminal.draw(|f| view(&mut self.model, f))?;

                    for cmd in cmds {
                        handle_command(cmd, self.event_tx.clone()).await;
                    }
                }

                Ok(ready) = tokio::task::spawn_blocking(|| ratatui::crossterm::event::poll(Duration::from_millis(EVENT_POLL_DURATION_MS))) => {
                    match ready {
                        Ok(true) => {
                            let event = ratatui::crossterm::event::read()?;
                            self.model.event_counter += 1;
                            if let Some(handling_msg) = get_event_handling_msg(&self.model, event) {
                                self.event_tx.try_send(handling_msg)?;
                            }
                        }
                        Ok(false) => continue,
                        Err(e) => {
                                return Err(anyhow::anyhow!(e));
                        }
                    }
                }
            }
        }
    }

    fn exit(&mut self) -> Result<(), IOError> {
        ratatui::try_restore()
    }
}
