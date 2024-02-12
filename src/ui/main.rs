use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Line, Modifier, Span, Style, Stylize};
use ratatui::widgets::{Block, Borders, BorderType, List, ListDirection, ListItem, Paragraph};
use crate::schemas::Message;
use crate::app::App;
use crate::chat::Chat;
use crate::constants::THEME;
use crate::{helpers, ui};

pub fn render_main(app: &mut App, f: &mut Frame) {
    let (main_area, footer_area) = create_main_and_footer(f);
    let (chats_area, messages_area) = create_chats_and_messages_areas(main_area);
    let (search_area, chats_area) = create_search_and_chats_area(chats_area);

    render_chats_area(app, f, chats_area, search_area);
    render_message_area(app, f, messages_area);
    render_footer(app, f, footer_area);
}

fn render_chats_area(app: &mut App, f: &mut Frame, chats_area: Rect, search_area: Rect) {
    let search_input_value = helpers::input_to_string(&app.main_window.get_search_input());
    let search_input = Paragraph::new(search_input_value.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title("Search")
        );

    let main_color = ui::get_main_color(app);
    let chats = app.main_window.chat_manager.get_active_chats_mut();
    f.render_widget(search_input, search_area);
    f.render_stateful_widget(
        build_chats(
            &chats.items,
            main_color,
            chats_area,
        ),
        chats_area,
        &mut chats.state,
    );
}

fn render_message_area(app: &App, f: &mut Frame, messages_area: Rect) {
    let (message_list_area, message_input_area) = create_message_area(messages_area);

    if let Some(loaded_chat) = app.main_window.chat_manager.get_loaded_chat() {
        let message_input_value = helpers::input_to_string(&app.main_window.get_message());
        let message_paragraph = Paragraph::new(message_input_value.as_str())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .style(Style::default().fg(THEME.fg))
            );

        let messages = if loaded_chat.id.is_none() {
            None
        } else {
            Some(app.main_window.chat_manager.get_messages(loaded_chat.id.unwrap()))
        };
        f.render_widget(
            build_messages(messages, THEME.fg),
            message_list_area,
        );
        f.render_widget(message_paragraph, message_input_area);
    } else {
        f.render_widget(
            get_chat_hints(THEME.fg),
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
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(75),
        ])
        .split(main_area);

    (main_layout[0], main_layout[1])
}

fn create_message_area(main_area: Rect) -> (Rect, Rect) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(3),
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

fn build_chats(chats: &[Chat], fg_color: Color, chats_area: Rect) -> List {
    let items: Vec<ListItem> = chats
        .iter()
        .map(|s| {
            let name = s.name.clone();
            let sent_at = s.last_message.as_ref().map(|message| message.sent_at.to_string()).unwrap_or_else(|| "".to_string());
            let total_width = chats_area.width as usize;
            // -2 because 1 cell goes for the border at each side
            let space_count = total_width - name.len() - sent_at.len() - 2;
            let formatted_string = format!("{name:<0$}yesterday", space_count + name.len(), name = name);
            let message_dt = Line::from(vec![
                Span::from(formatted_string),
                " ".into(),
                sent_at.into(),
            ]);

            ListItem::new(message_dt)
        })
        .collect();

    List::new(items)
        .block(Block::default().title("Chats").borders(Borders::ALL).border_type(BorderType::Plain))
        .style(Style::default().fg(fg_color))
        .highlight_style(Style::default().bg(THEME.fg).bold().black())
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
        .style(Style::default().fg(THEME.fg))
        .alignment(Alignment::Center)
}

fn build_messages(messages: Option<&Vec<Message>>, fg_color: Color) -> List {
    let items: Vec<ListItem> = if messages.is_some() {
        messages
            .unwrap()
            .iter()
            .map(|s| ListItem::new(s.text.clone()))
            .collect()
    } else {
        vec![]
    };

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
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        )
        .style(Style::default().fg(fg_color))
        .alignment(Alignment::Center)
}
