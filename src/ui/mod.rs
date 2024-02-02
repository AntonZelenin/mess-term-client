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
use crate::input::entities::login::{LoginActiveInput, LoginTabs};
use crate::ui::chat::StatefulChat;

pub mod tui;
pub mod chat;

pub fn render(app: &mut App, f: &mut Frame) {
    render_main(app, f);
    if !app.is_authenticated() {
        render_login_register(app, f);
    }
}

fn render_login_register(app: &mut App, f: &mut Frame) {
    let tabs = Tabs::new(
        ["Login", "Register"]
            .iter()
            .map(|t| {
                Line::from(*t)
            })
            .collect()
    )
        .block(Block::default())
        .select(app.login_window.active_tab as usize)
        .style(Style::default())
        .highlight_style(Style::default().bold().black().bg(DEFAULT_THEME.active));

    match app.login_window.active_tab {
        LoginTabs::Login => {
            render_login(app, tabs, f);
        }
        LoginTabs::Register => {
            render_register(app, tabs, f);
        }
    }
}

fn render_login(app: &mut App, tabs: Tabs, f: &mut Frame) {
    // todo make separate error field for all fields, including register
    let login_area = create_login_area(f.size());
    let input_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .horizontal_margin(1)
        .split(login_area);

    let login_block = Block::default()
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(DEFAULT_THEME.fg).bg(DEFAULT_THEME.bg));

    let username_input = Paragraph::new(app.login_window.username_input.as_str())
        .style(match app.login_window.active_input {
            LoginActiveInput::Username => Style::default().fg(DEFAULT_THEME.active),
            LoginActiveInput::Password => Style::default().fg(DEFAULT_THEME.inactive),
            _ => unreachable!(),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Username")
        );
    let password_input = Paragraph::new(app.login_window.password_input.as_str())
        .style(match app.login_window.active_input {
            LoginActiveInput::Username => Style::default().fg(DEFAULT_THEME.inactive),
            LoginActiveInput::Password => Style::default().fg(DEFAULT_THEME.active),
            _ => unreachable!(),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Password")
        );
    let error_message = Paragraph::new(app.login_window.login_error_message.as_str())
        .style(Style::default().fg(DEFAULT_THEME.error))
        .block(
            Block::default()
        )
        .alignment(Alignment::Center);

    f.render_widget(Clear, login_area);
    f.render_widget(login_block, login_area);
    f.render_widget(tabs, input_area[1]);
    f.render_widget(username_input, input_area[3]);
    f.render_widget(password_input, input_area[4]);
    f.render_widget(error_message, input_area[6]);

    let active_input_area = match app.login_window.active_input {
        LoginActiveInput::Username => input_area[3],
        LoginActiveInput::Password => input_area[4],
        _ => unreachable!(),
    };
    f.set_cursor(
        active_input_area.x + app.login_window.get_cursor_position() as u16 + 1,
        // Move one line down, from the border to the input line
        active_input_area.y + 1,
    )
}

fn render_register(app: &mut App, tabs: Tabs, f: &mut Frame) {
    let register_area = create_register_area(f.size());
    let input_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            // the sum should be the same as in create_register_area middle popup value
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .horizontal_margin(1)
        .split(register_area);

    let block = Block::default()
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(DEFAULT_THEME.fg).bg(DEFAULT_THEME.bg));

    let username_input = Paragraph::new(app.login_window.register_username_input.as_str())
        .style(match app.login_window.active_input {
            LoginActiveInput::RegisterUsername => Style::default().fg(DEFAULT_THEME.active),
            _ => Style::default().fg(DEFAULT_THEME.inactive),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Username")
        );
    let password_input = Paragraph::new(app.login_window.register_password_input.as_str())
        .style(match app.login_window.active_input {
            LoginActiveInput::RegisterPassword => Style::default().fg(DEFAULT_THEME.active),
            _ => Style::default().fg(DEFAULT_THEME.inactive),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Password")
        );
    let password_confirmation_input = Paragraph::new(app.login_window.register_password_confirmation_input.as_str())
        .style(match app.login_window.active_input {
            LoginActiveInput::RegisterPasswordConfirmation => Style::default().fg(DEFAULT_THEME.active),
            _ => Style::default().fg(DEFAULT_THEME.inactive),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Confirm password")
        );
    let error_message = Paragraph::new(app.login_window.register_error_message.as_str())
        .style(Style::default().fg(DEFAULT_THEME.error))
        .block(
            Block::default()
        )
        .alignment(Alignment::Center);

    f.render_widget(Clear, register_area);
    f.render_widget(block, register_area);
    f.render_widget(tabs, input_area[1]);
    f.render_widget(username_input, input_area[3]);
    f.render_widget(password_input, input_area[5]);
    f.render_widget(password_confirmation_input, input_area[6]);
    f.render_widget(error_message, input_area[8]);

    let active_input_area = match app.login_window.active_input {
        LoginActiveInput::RegisterUsername => input_area[3],
        LoginActiveInput::RegisterPassword => input_area[5],
        LoginActiveInput::RegisterPasswordConfirmation => input_area[6],
        _ => unreachable!(),
    };
    f.set_cursor(
        active_input_area.x + app.login_window.get_cursor_position() as u16 + 1,
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
            Constraint::Length(12),
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

pub fn create_register_area(r: Rect) -> Rect {
    let percent_x = 60;
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(16),
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
            Paragraph::new("`Enter` - submit data, `Tab` - switch input, `Left/Right arrow` - switch tabs.")
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
