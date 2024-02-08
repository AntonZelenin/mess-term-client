use anyhow::Result;
use crate::app::App;
use crate::event::Event;
use crate::ui::tui;
use crate::window::process;

mod api;
mod auth;
mod app;
mod chat;
mod constants;
mod event;
mod helpers;
mod ui;
mod window;
pub mod schemas;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::new(api::Client::new(auth::load_auth_tokens()).await).await;
    let mut tui = tui::build_tui();

    tui.enter()?;

    while !app.should_quit() {
        tui.draw(&mut app)?;

        tokio::select! {
            _ = app.receive_message() => {},
            event = tui.events.next() => {
                match event {
                    Event::Tick => {
                        app.tick();
                    },
                    Event::Key(key_event) => process(&mut app, key_event).await,
                    Event::Mouse(_) => {},
                    Event::Resize(_, _) => {},
                }
            },
        }
    }

    tui.exit()?;
    Ok(())
}
