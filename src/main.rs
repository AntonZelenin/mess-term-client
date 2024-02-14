use std::num::NonZeroU32;
use std::sync::Arc;
use anyhow::Result;
use governor::{Quota, RateLimiter};
use tokio::time::sleep;
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
mod schemas;
mod storage;
mod ui;
mod window;

#[tokio::main]
async fn main() -> Result<()> {
    let message_rate_limiter = Arc::new(RateLimiter::direct(Quota::per_second(NonZeroU32::new(10).unwrap())));
    let events_rate_limiter = Arc::new(RateLimiter::direct(Quota::per_second(NonZeroU32::new(60).unwrap())));

    let mut app = App::new(api::Client::new(storage::load_auth_tokens()).await).await;
    let mut tui = tui::build_tui();

    tui.enter()?;

    while !app.should_quit() {
        tui.draw(&mut app)?;

        tokio::select! {
            _ = app.receive_message(), if message_rate_limiter.check().is_ok() => {},
            event = tui.events.next(), if events_rate_limiter.check().is_ok() => {
                match event {
                    Event::Tick => {
                        app.tick();
                    },
                    Event::Key(key_event) => process(&mut app, key_event).await,
                    Event::Mouse(_) => {},
                    Event::Resize(_, _) => {},
                }
            },
            else => {
                sleep(std::time::Duration::from_millis(10)).await;
            }
        }
    }

    tui.exit()?;
    Ok(())
}
