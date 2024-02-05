use ratatui::{
    prelude::Frame,
    style::Color,
};
use crate::constants;

use crate::app::App;

pub mod chat;
mod login;
mod main;
pub mod tui;

pub fn render(app: &mut App, f: &mut Frame) {
    main::render_main(app, f);
    if !app.is_authenticated() {
        login::render_login_register(app, f);
    }
}


fn get_main_color(app: &App) -> Color {
    if app.is_authenticated() {
        constants::THEME.fg
    } else {
        constants::THEME.inactive
    }
}
