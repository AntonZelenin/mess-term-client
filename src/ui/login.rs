use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Line, Style, Stylize};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Tabs};
use crate::app::App;
use crate::constants::THEME;
use crate::helpers;
use crate::window::login::{LoginActiveInput, LoginTabs};

pub fn render_login_register(app: &mut App, f: &mut Frame) {
    let tabs = Tabs::new(
        ["Увійти", "Зареєструватися"]
            .iter()
            .map(|t| {
                Line::from(*t)
            })
            .collect()
    )
        .block(Block::default())
        .select(app.login_window.active_tab as usize)
        .style(Style::default())
        .highlight_style(Style::default().bold().black().bg(THEME.active));

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
        .style(Style::default().fg(THEME.fg).bg(THEME.bg));

    let username_value = helpers::input_to_string(&app.login_window.username_input);
    let username_input = Paragraph::new(username_value.as_str())
        .style(match app.login_window.active_input_field {
            LoginActiveInput::Username => Style::default().fg(THEME.active),
            LoginActiveInput::Password => Style::default().fg(THEME.inactive),
            _ => unreachable!(),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Ім'я користувача")
        );
    let password_value = helpers::input_to_string(&app.login_window.password_input);
    let password_input = Paragraph::new(password_value.as_str())
        .style(match app.login_window.active_input_field {
            LoginActiveInput::Username => Style::default().fg(THEME.inactive),
            LoginActiveInput::Password => Style::default().fg(THEME.active),
            _ => unreachable!(),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Пароль")
        );
    let error_message = Paragraph::new(app.login_window.login_error_message.as_str())
        .style(Style::default().fg(THEME.error))
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

    let active_input_area = match app.login_window.active_input_field {
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
        .style(Style::default().fg(THEME.fg).bg(THEME.bg));

    let username_input_value = helpers::input_to_string(&app.login_window.register_username_input);
    let username_input = Paragraph::new(username_input_value.as_str())
        .style(match app.login_window.active_input_field {
            LoginActiveInput::RegisterUsername => Style::default().fg(THEME.active),
            _ => Style::default().fg(THEME.inactive),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Ім'я користувача")
        );
    let password_input_value = helpers::input_to_string(&app.login_window.register_password_input);
    let password_input = Paragraph::new(password_input_value.as_str())
        .style(match app.login_window.active_input_field {
            LoginActiveInput::RegisterPassword => Style::default().fg(THEME.active),
            _ => Style::default().fg(THEME.inactive),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Пароль")
        );
    let password_confirmation_input_value = helpers::input_to_string(&app.login_window.register_password_confirmation_input);
    let password_confirmation_input = Paragraph::new(password_confirmation_input_value.as_str())
        .style(match app.login_window.active_input_field {
            LoginActiveInput::RegisterPasswordConfirmation => Style::default().fg(THEME.active),
            _ => Style::default().fg(THEME.inactive),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Підтвердіть пароль")
        );
    let error_message = Paragraph::new(app.login_window.register_error_message.as_str())
        .style(Style::default().fg(THEME.error))
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

    let active_input_area = match app.login_window.active_input_field {
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

fn create_login_area(r: Rect) -> Rect {
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

fn create_register_area(r: Rect) -> Rect {
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
