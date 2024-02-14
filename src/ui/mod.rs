use ratatui::{
    prelude::Frame,
};

use crate::app::App;

mod login;
mod main;
pub mod tui;

pub fn render(app: &mut App, f: &mut Frame) {
    main::render_main(app, f);
    if !app.is_authenticated() {
        login::render_login_register(app, f);
    }
}
