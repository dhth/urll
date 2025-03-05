use super::command::Command;
use super::common::*;
use super::message::Message;
use super::model::*;

pub fn update(model: &mut Model, msg: Message) -> Vec<Command> {
    let mut cmds = Vec::new();

    match msg {
        Message::UrlChosen => {
            if let Some((url, _)) = model.get_selected_url() {
                if url == model.page_details.url {
                    model.user_message = Some(UserMessage::error(
                        "selected URL is the same as the current one",
                    ));
                } else if let Some(page) = model.results_cache.get(&url) {
                    model.history.push_back(model.page_details.url.clone());
                    model.page_details.url = url;
                    model.page_details = page.details.clone();
                    model.results = Ok(Results::from(page));
                } else {
                    cmds.push(Command::GetResults {
                        current: model.page_details.url.clone(),
                        chosen: url,
                    });
                }
            }
        }
        Message::ResultsFetched {
            previous,
            page_result,
        } => match page_result {
            Ok(page) => {
                if page.page_urls.is_empty() {
                    model.user_message = Some(UserMessage::info("no urls on the selected page"));
                } else {
                    model.results = Ok(Results::from(&page.page_urls));
                    model.page_details = page.details.clone();
                    model.results_cache.insert(page.details.url.clone(), page);
                    model.history.push_back(previous.clone());
                    model.user_message = None;
                }
            }
            Err(e) => {
                model.results = Err(e);
                model.history.push_back(previous.clone());
            }
        },
        Message::GoToNextListItem => model.select_next_list_item(),
        Message::GoToPreviousListItem => model.select_previous_list_item(),
        Message::GoToFirstListItem => model.select_first_list_item(),
        Message::GoToLastListItem => model.select_last_list_item(),
        Message::TerminalResize(width, height) => {
            model.terminal_dimensions = TerminalDimensions { width, height };
            model.terminal_too_small =
                !(width >= MIN_TERMINAL_WIDTH && height >= MIN_TERMINAL_HEIGHT);
        }
        Message::YankUrlToClipboard => {
            if let Some((url, _)) = model.get_selected_url() {
                cmds.push(Command::YankContentToClipboard(url));
            }
        }
        Message::YankUrlsToClipboard => {
            if let Ok(results) = &model.results {
                let urls = results.items.join("\n");
                cmds.push(Command::YankContentToClipboard(urls));
            }
        }
        Message::ContentYanked(result) => {
            if let Err(error) = result {
                model.user_message = Some(
                    UserMessage::error(&format!("couldn't yank url to clipboard: {}", error))
                        .with_frames_left(2),
                );
            } else {
                model.user_message = Some(UserMessage::info("yanked!").with_frames_left(2));
            }
        }
        Message::OpenUrlInBrowser => {
            if let Some((url, _)) = model.get_selected_url() {
                cmds.push(Command::OpenInBrowser(url));
            }
        }
        Message::UrlOpened(result) => {
            if let Err(error) = result {
                model.user_message = Some(
                    UserMessage::error(&format!("couldn't open url: {}", error))
                        .with_frames_left(2),
                )
            }
        }
        Message::GoToPane(pane) => {
            model.last_active_pane = Some(model.active_pane);
            model.active_pane = pane;
        }
        Message::GoBack => {
            if let Some(last_url) = model.history.pop_back() {
                if let Some(page) = model.results_cache.get(&last_url) {
                    model.page_details.url = last_url;
                    model.page_details = page.details.clone();
                    model.results = Ok(Results::from(page));
                    model.user_message = None;
                } else {
                    model.user_message = Some(UserMessage::error("something went wrong"));
                }
            } else {
                model.user_message = Some(UserMessage::error("at the start of navigation history"));
            }
        }
        Message::GoBackOrQuit => model.go_back_or_quit(),
        Message::QuitImmediately => model.running_state = RunningState::Done,
    }

    if let Some(message) = &mut model.user_message {
        let clear = if message.frames_left == 0 {
            true
        } else {
            message.frames_left -= 1;
            false
        };

        if clear {
            model.user_message = None;
        }
    }

    cmds
}
