use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, BorderType, List, ListDirection, ListItem, Paragraph};
use crate::app::App;
use crate::constants::DEFAULT_THEME;
use crate::helpers::list::StatefulList;
use crate::ui;
use crate::ui::chat::StatefulChat;

pub fn render_main(app: &mut App, f: &mut Frame) {
    let (main_area, footer_area) = create_main_and_footer(f);
    let (chats_area, messages_area) = create_chats_and_messages_areas(main_area);
    let (search_area, chats_area) = create_search_and_chats_area(chats_area);

    render_chats_area(app, f, chats_area, search_area);
    render_message_area(app, f, messages_area);
    render_footer(app, f, footer_area);
}

fn render_chats_area(app: &mut App, f: &mut Frame, chats_area: Rect, search_area: Rect) {
    let search_input = app.main_window.get_search_input();
    let search_input = Paragraph::new(search_input.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Search")
        );

    let main_color = ui::get_main_color(app);
    let chats = app.main_window.get_chats_mut();
    f.render_widget(search_input, search_area);
    f.render_stateful_widget(
        build_chats(
            &chats.items,
            main_color,
        ),
        chats_area,
        &mut chats.state,
    );
}

fn render_message_area(app: &App, f: &mut Frame, messages_area: Rect) {
    let main_color = ui::get_main_color(app);
    let chats = app.main_window.get_chats();
    if chats.state.selected().is_none() {
        f.render_widget(
            get_chat_hints(main_color),
            messages_area,
        );
    } else {
        f.render_widget(
            build_messages(&chats, main_color),
            messages_area,
        );
    }
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

fn render_footer(app: &App, f: &mut Frame, footer_area: Rect) {
    f.render_widget(
        get_app_hints(app),
        footer_area,
    );
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