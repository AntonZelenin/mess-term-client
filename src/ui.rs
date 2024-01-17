use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    let body_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(1),
            ]
                .as_ref(),
        )
        .split(f.size());

    let main = &body_layout[0];
    let footer = &body_layout[1];

    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(75),
        ])
        .split(*main);

    let chats = &main_layout[0];
    let messages = &main_layout[1];

    f.render_stateful_widget(
        build_chats(&app),
        *chats,
        &mut app.chats.state,
    );
    f.render_widget(
        get_chat_hint(),
        *messages,
    );
    f.render_widget(
        get_app_hints(),
        *footer,
    );
}

fn build_chats<'a>(app: &App<'a>) -> List<'a> {
    let items: Vec<ListItem> = app
        .chats
        .items
        .iter()
        // .flat_map(|s| vec![ListItem::new(*s), ListItem::new("")])
        .map(|s| ListItem::new(*s))
        .collect();

    List::new(items)
        .block(Block::default().title("Chats").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom)
}

fn get_app_hints<'a>() -> Paragraph<'a> {
    Paragraph::new("Press `Esc`, `Ctrl-C` or `q` to stop running.")
        .block(
            Block::default()
                .title_alignment(Alignment::Center),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
}

fn get_chat_hint<'a>() -> Paragraph<'a> {
    Paragraph::new("Select a chat to start messaging.")
        .block(
            Block::default()
                .title("Messages")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
}
