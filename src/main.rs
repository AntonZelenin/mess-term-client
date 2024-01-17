use anyhow::Result;
use crate::app::App;
use crate::event::Event;
use crate::update::update;

mod app;
mod event;
mod helpers;
mod tui;
mod ui;
mod update;
mod chat;

fn main() -> Result<()> {
    let mut app = App::new();
    app.add_chats(
        vec![
            "Danil\nDid you learn Thai today?".to_string(),
            "Anya\nI've finished your video".to_string(),
        ]
    );
    let mut tui = tui::build_tui();

    tui.enter()?;

    while !app.should_quit {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    tui.exit()?;
    Ok(())
}
