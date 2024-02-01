use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::app::App;
use crate::constants::DEFAULT_THEME;
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
    let auth_area = create_login_area(f.size());
    let input_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
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
    let error_message = Paragraph::new(app.auth_window.error_message.as_str())
        .style(Style::default().fg(DEFAULT_THEME.error))
        .block(
            Block::default()
        )
        .alignment(Alignment::Center);

    f.render_widget(Clear, auth_area);
    f.render_widget(block, auth_area);
    f.render_widget(username_input, input_area[1]);
    f.render_widget(password_input, input_area[2]);
    f.render_widget(error_message, input_area[4]);

    let active_input_area = match app.auth_window.active_input {
        AuthActiveInput::Username => input_area[1],
        AuthActiveInput::Password => input_area[2],
    };
    f.set_cursor(
        active_input_area.x + app.auth_window.get_cursor_position() as u16 + 1,
        // Move one line down, from the border to the input line
        active_input_area.y + 1,
    )
}

fn render_main(app: &mut App, f: &mut Frame) {
    let (main_area, footer_area) = create_main_and_footer(f);
    let (chats_area, messages_area) = create_chats_and_messages_areas(main_area);
    let (search_area, chats_area) = create_search_and_chats_area(chats_area);

    render_chats_area(app, f, chats_area, search_area);
    render_message_area(app, f, messages_area);
    render_footer(app, f, footer_area);
}

fn render_chats_area(app: &mut App, f: &mut Frame, chats_area: Rect, search_area: Rect) {
    let search_input = Paragraph::new(app.search_input.input.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Search")
        );

    f.render_widget(search_input, search_area);
    f.render_stateful_widget(
        build_chats(&app.chats.items, get_main_color(app)),
        chats_area,
        &mut app.chats.state,
    );
}

fn render_message_area(app: &App, f: &mut Frame, messages_area: Rect) {
    let main_color = get_main_color(app);
    if app.chats.state.selected().is_none() {
        f.render_widget(
            get_chat_hints(main_color),
            messages_area,
        );
    } else {
        f.render_widget(
            build_messages(&app.chats, main_color),
            messages_area,
        );
    }
}

fn render_footer(app: &App, f: &mut Frame, footer_area: Rect) {
    f.render_widget(
        get_app_hints(app),
        footer_area,
    );
}

pub fn create_login_area(r: Rect) -> Rect {
    let percent_x = 60;
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(11),
            Constraint::Min(1),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn create_search_and_chats_area(chats_area: Rect) -> (Rect, Rect) {
    let chats_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(chats_area);
    (chats_layout[0], chats_layout[1])
}

fn create_chats_and_messages_areas(main_area: Rect) -> (Rect, Rect) {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(75),
        ])
        .split(main_area);

    (main_layout[0], main_layout[1])
}

fn create_main_and_footer(f: &mut Frame) -> (Rect, Rect) {
    let terminal_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(f.size());

    (terminal_layout[0], terminal_layout[1])
}

fn get_main_color(app: &App) -> Color {
    if app.is_authenticated() {
        Color::Yellow
    } else {
        Color::DarkGray
    }
}

fn build_chats(chats: &[StatefulChat], fg_color: Color) -> List {
    let items: Vec<ListItem> = chats
        .iter()
        // .flat_map(|s| vec![ListItem::new(*s), ListItem::new("")])
        .map(|s| ListItem::new(s.name.as_str()))
        .collect();

    List::new(items)
        .block(Block::default().title("Chats").borders(Borders::ALL).border_type(BorderType::Rounded))
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
