use anyhow::Result;
use crate::app::App;
use crate::chat::Chat;
use crate::event::Event;
use crate::ui::tui;
use crate::update::update;

mod app;
mod chat;
mod constants;
mod event;
mod helpers;
mod session;
mod ui;
mod update;

fn main() -> Result<()> {
    let mut app = App::new();
    let mut chats = vec![
        // Chat::new(1, "Danil".to_string()),
        // Chat::new(4, "Olya".to_string()),
        // Chat::new(2, "Anya".to_string()),
        // Chat::new(3, "Masha".to_string()),
    ];
    // chats[0].add_messages(vec![
    //     "How are you?".to_string(),
    //     "Hello".to_string(),
    // ]);
    // chats[1].add_messages(vec![
    //     "Kelpi is beating Plyushik, omg!!!".to_string(),
    // ]);
    // chats[2].add_messages(vec![
    //     "Hey, Anton. I've finished editing your video!".to_string(),
    // ]);
    // chats[3].add_messages(vec![
    //     "Hello, when the next photo session will happen??".to_string(),
    // ]);

    app.add_chats(chats);
    let mut tui = tui::build_tui();

    tui.enter()?;

    while !app.should_quit() {
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
