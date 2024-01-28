use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::app::App;
use crate::constants::DEFAULT_THEME;
use crate::helpers;
use crate::helpers::list::StatefulList;
use crate::ui::chat::StatefulChat;
use crate::ui::states::AuthActiveInput;

pub mod tui;
pub mod states;
pub mod chat;

pub fn render(app: &mut App, f: &mut Frame) {
    render_main(app, f);
    if !app.is_authenticated() {
        render_auth(app, f);
    }
}

fn render_auth(app: &mut App, f: &mut Frame) {
    let auth_area = helpers::centered_rect(60, 70, f.size());
    let input_are = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(2),
        ])
        .horizontal_margin(1)
        .split(auth_area);

    let block = Block::default()
        .title("Login")
        // .borders(Borders::NONE)
        // .border_type(BorderType::Rounded)
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(DEFAULT_THEME.fg).bg(DEFAULT_THEME.bg));

    let username_input = Paragraph::new(app.auth_window.username_input.as_str())
        .style(match app.auth_window.active_input {
            AuthActiveInput::Username => Style::default().fg(DEFAULT_THEME.active),
            AuthActiveInput::Password => Style::default().fg(DEFAULT_THEME.inactive),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Username")
        );
    let password_input = Paragraph::new(app.auth_window.password_input.as_str())
        .style(match app.auth_window.active_input {
            AuthActiveInput::Username => Style::default().fg(DEFAULT_THEME.inactive),
            AuthActiveInput::Password => Style::default().fg(DEFAULT_THEME.active),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Password")
        );

    f.render_widget(Clear, auth_area);
    f.render_widget(block, auth_area);
    f.render_widget(username_input, input_are[1]);
    f.render_widget(password_input, input_are[2]);

    let active_input_area = match app.auth_window.active_input {
        AuthActiveInput::Username => input_are[1],
        AuthActiveInput::Password => input_are[2],
    };
    f.set_cursor(
        active_input_area.x + app.auth_window.get_cursor_position() as u16 + 1,
        // Move one line down, from the border to the input line
        active_input_area.y + 1,
    )
}

fn render_main(app: &mut App, f: &mut Frame) {
    let body_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(f.size());

    let main = &body_layout[0];
    let footer_layout = &body_layout[1];

    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(75),
        ])
        .split(*main);

    let chats_layout = &main_layout[0];
    let messages_layout = &main_layout[1];

    let main_color = get_main_color(app);

    f.render_stateful_widget(
        build_chats(&app.chats.items, main_color.clone()),
        *chats_layout,
        &mut app.chats.state,
    );
    if app.chats.state.selected().is_none() {
        f.render_widget(
            get_chat_hints(main_color.clone()),
            *messages_layout,
        );
    } else {
        f.render_widget(
            build_messages(&app.chats, main_color.clone()),
            *messages_layout,
        );
    }
    f.render_widget(
        get_app_hints(app),
        *footer_layout,
    );
}

fn get_main_color(app: &App) -> Color {
    if app.is_authenticated() {
        Color::Yellow
    } else {
        Color::DarkGray
    }
}

fn build_chats(chats: &Vec<StatefulChat>, fg_color: Color) -> List {
    let items: Vec<ListItem> = chats
        .iter()
        // .flat_map(|s| vec![ListItem::new(*s), ListItem::new("")])
        .map(|s| ListItem::new(s.name.as_str()))
        .collect();

    List::new(items)
        .block(Block::default().title("Chats").borders(Borders::ALL))
        .style(Style::default().fg(fg_color))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom)
}

fn get_app_hints<'a>(app: &App) -> Paragraph<'a> {
    let paragraph = match app.is_authenticated() {
        true => {
            Paragraph::new("Press `Ctrl-C` to quit.")
        }
        false => {
            Paragraph::new("`Enter` - submit data, `Tab` - switch input")
        }
    };

    paragraph
        .block(
            Block::default()
                .title_alignment(Alignment::Center),
        )
        .style(Style::default().fg(DEFAULT_THEME.active))
        .alignment(Alignment::Center)
}

fn build_messages(chats: &StatefulList<StatefulChat>, fg_color: Color) -> List {
    let chat = &chats.items[chats.state.selected().unwrap()];
    let messages = chat.messages.clone();

    let items: Vec<ListItem> = messages
        .items
        .iter()
        // .flat_map(|s| vec![ListItem::new(*s), ListItem::new("")])
        .map(|s| ListItem::new(s.as_string()))
        .collect();

    List::new(items)
        .block(Block::default().title("Messages").borders(Borders::ALL))
        .style(Style::default().fg(fg_color))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">")
        .direction(ListDirection::BottomToTop)
}

fn get_chat_hints<'a>(fg_color: Color) -> Paragraph<'a> {
    Paragraph::new("Select a chat to start messaging.")
        .block(
            Block::default()
                .title("Messages")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(fg_color))
        .alignment(Alignment::Center)
}
