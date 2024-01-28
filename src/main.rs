use anyhow::Result;
use crate::app::App;
use crate::event::Event;
use crate::ui::tui;
use crate::input::process_input;

mod api;
mod app;
mod chat;
mod constants;
mod event;
mod helpers;
mod auth;
mod ui;
mod input;
mod contact;

fn main() -> Result<()> {
    let mut app = App::new(api::Client::new(auth::load_auth_tokens()));
    let mut tui = tui::build_tui();

    tui.enter()?;

    while !app.should_quit() {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => process_input(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    tui.exit()?;
    Ok(())
}
