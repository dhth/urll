use super::common::*;
use super::model::{MessageKind, Model, Results};
use crate::domain::PageDetails;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List, ListDirection, ListItem, Padding, Paragraph, Wrap},
};

const HELP_CONTENTS: &str = include_str!("static/help.txt");

pub fn view(model: &mut Model, frame: &mut Frame) {
    if model.terminal_too_small {
        render_terminal_too_small_view(&model.terminal_dimensions, frame);
        return;
    }

    match model.active_pane {
        Pane::Help => render_help_view(model, frame),
        _ => render_list_view(model, frame),
    }
}

fn render_terminal_too_small_view(dimensions: &TerminalDimensions, frame: &mut Frame) {
    let message = format!(
        r#"
Terminal size too small:
  Width = {} Height = {}

Minimum dimensions needed:
  Width = {} Height = {}

Press (q/<ctrl+c>/<esc> to exit)
"#,
        dimensions.width, dimensions.height, MIN_TERMINAL_WIDTH, MIN_TERMINAL_HEIGHT
    );

    let p = Paragraph::new(message)
        .block(Block::bordered())
        .style(Style::new().fg(PRIMARY_COLOR))
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Center);

    frame.render_widget(p, frame.area());
}

fn render_help_view(model: &mut Model, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![Constraint::Min(21), Constraint::Length(1)])
        .split(frame.area());

    let lines: Vec<Line<'_>> = HELP_CONTENTS.lines().map(Line::from).collect();

    let p = Paragraph::new(lines)
        .block(
            Block::bordered()
                .border_style(Style::default().fg(PRIMARY_BORDER_COLOR))
                .title_style(
                    Style::new()
                        .bold()
                        .bg(HELP_COLOR)
                        .fg(SECTION_TITLE_FG_COLOR),
                )
                .title(" help ")
                .padding(Padding::new(2, 0, 1, 0)),
        )
        .style(Style::new().white())
        .alignment(Alignment::Left);

    frame.render_widget(p, layout[0]);
    render_status_bar(model, frame, layout[1]);
}

fn render_status_bar(model: &Model, frame: &mut Frame, rect: Rect) {
    let mut status_bar_lines = vec![Span::styled(
        TITLE,
        Style::new()
            .bold()
            .bg(PRIMARY_COLOR)
            .fg(SECTION_TITLE_FG_COLOR),
    )];

    status_bar_lines.push(Span::from(format!(" [{}]", model.page_details.url)).fg(URL_COLOR));

    if model.debug {
        status_bar_lines.push(Span::from(
            model
                .last_active_pane
                .map(|p| format!(" [{}]", p))
                .unwrap_or(" -".to_string()),
        ));
        status_bar_lines.push(Span::from(format!(" -> [{}]", model.active_pane)));
        status_bar_lines.push(Span::from(format!(
            " [render counter: {}]",
            model.render_counter
        )));
        status_bar_lines.push(Span::from(format!(
            " [event counter: {}]",
            model.event_counter
        )));

        status_bar_lines.push(Span::from(format!(
            " [dimensions: {}x{}] ",
            model.terminal_dimensions.width, model.terminal_dimensions.height
        )));
    }

    if let Some(msg) = &model.user_message {
        let span = match msg.kind {
            MessageKind::Info => Span::styled(
                format!(" {}", msg.value),
                Style::new().fg(INFO_MESSAGE_COLOR),
            ),
            MessageKind::Error => {
                Span::styled(format!(" {}", msg.value), Style::new().fg(ERROR_COLOR))
            }
        };

        status_bar_lines.push(span);
    }

    let status_bar_text = Line::from(status_bar_lines);

    let status_bar = Paragraph::new(status_bar_text).block(Block::default());

    frame.render_widget(&status_bar, rect);
}

fn render_results_list_and_details(
    details: &PageDetails,
    results: &mut Results,
    frame: &mut Frame,
    rect: Rect,
) {
    let title = " results ";
    let items: Vec<ListItem> = results
        .items
        .iter()
        .map(|s| ListItem::new(s.clone()))
        .collect();

    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(rect);

    let details_str = match (&details.title, &details.description) {
        (None, None) => "No details found".to_string(),
        (None, Some(d)) => d.clone(),
        (Some(t), None) => format!("Title: {}", t),
        (Some(t), Some(d)) => format!("Title: {}\n\n{}", d, t),
    };

    let details_paragraph = Paragraph::new(details_str)
        .block(
            Block::bordered()
                .border_style(Style::default().fg(PRIMARY_BORDER_COLOR))
                .title_style(
                    Style::new()
                        .bold()
                        .bg(SECONDARY_COLOR)
                        .fg(SECTION_TITLE_FG_COLOR),
                )
                .title(" page details ")
                .padding(Padding::new(1, 1, 1, 1)),
        )
        .wrap(Wrap { trim: false })
        .style(Style::new().white().on_black())
        .alignment(Alignment::Left);

    if items.is_empty() {
        let list = Paragraph::new("no urls")
            .block(
                Block::bordered()
                    .border_style(Style::default().fg(PRIMARY_BORDER_COLOR))
                    .title_style(
                        Style::new()
                            .bold()
                            .bg(SECONDARY_COLOR)
                            .fg(SECTION_TITLE_FG_COLOR),
                    )
                    .title(title)
                    .padding(Padding::new(1, 0, 1, 1)),
            )
            .style(Style::new().white().on_black())
            .alignment(Alignment::Left);
        frame.render_widget(&list, layout[0]);
        frame.render_widget(&details_paragraph, layout[1]);

        return;
    }

    let list = List::new(items)
        .block(
            Block::bordered()
                .border_style(Style::default().fg(PRIMARY_BORDER_COLOR))
                .padding(Padding::new(0, 0, 1, 1))
                .title_style(
                    Style::new()
                        .bold()
                        .bg(SECONDARY_COLOR)
                        .fg(SECTION_TITLE_FG_COLOR),
                )
                .title(title),
        )
        .style(Style::new().white())
        .highlight_symbol("> ")
        .repeat_highlight_symbol(true)
        .highlight_style(Style::new().fg(PRIMARY_COLOR))
        .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(list, layout[0], &mut results.state);
    frame.render_widget(&details_paragraph, layout[1]);
}

fn render_results_error(error: &str, frame: &mut Frame, rect: Rect) {
    let title = " error ";
    let details = Paragraph::new(error)
        .block(
            Block::bordered()
                .border_style(Style::default().fg(ERROR_COLOR))
                .title_style(
                    Style::new()
                        .bold()
                        .bg(ERROR_COLOR)
                        .fg(SECTION_TITLE_FG_COLOR),
                )
                .title(title)
                .padding(Padding::new(1, 0, 1, 1)),
        )
        .style(Style::new().white().on_black())
        .alignment(Alignment::Left);

    frame.render_widget(&details, rect);
}

fn render_results(model: &mut Model, frame: &mut Frame, rect: Rect) {
    match &mut model.results {
        Ok(r) => render_results_list_and_details(&model.page_details, r, frame, rect),
        Err(e) => render_results_error(e, frame, rect),
    }
}

fn render_list_view(model: &mut Model, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(vec![Constraint::Min(10), Constraint::Length(1)])
        .split(frame.area());

    render_results(model, frame, layout[0]);
    render_status_bar(model, frame, layout[1]);
}
