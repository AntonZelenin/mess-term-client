use ratatui::prelude::Color;

// todo it should be secure
pub const AUTH_SERVER_API_URL: &str = "http://localhost:8000/api/v1/";
pub const APP_SERVER_API_URL: &str = "ws://localhost:8800";

pub struct Theme {
    pub fg: Color,
    pub bg: Color,
    pub active: Color,
    pub inactive: Color,
    pub error: Color,
}

pub const DEFAULT_THEME: Theme = Theme {
    fg: Color::Rgb(147, 183, 190),
    bg: Color::Rgb(40, 42, 54),
    active: Color::Rgb(212, 245, 245),
    inactive: Color::Rgb(140, 154, 158),
    error: Color::Rgb(165, 117, 72),
};