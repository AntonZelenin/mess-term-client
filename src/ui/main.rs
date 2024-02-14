use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Line, Modifier, Span, Style, Stylize};
use ratatui::widgets::{Block, Borders, BorderType, List, ListDirection, ListItem, Padding, Paragraph, Wrap};
use crate::schemas::Message;
use crate::app::App;
use crate::chat::Chat;
use crate::constants::THEME;
use crate::helpers;
use crate::window::main::ActiveInputEntity;

pub fn render_main(app: &mut App, f: &mut Frame) {
    let (main_area, footer_area) = create_main_and_footer(f);
    let (chats_area, messages_area) = create_chats_and_messages_areas(main_area);
    let (search_area, chats_area) = create_search_and_chats_area(chats_area);

    render_chats_area(app, f, chats_area, search_area);
    render_message_area(app, f, messages_area);
    render_footer(app, f, footer_area);
}

fn render_chats_area(app: &mut App, f: &mut Frame, chats_area: Rect, search_area: Rect) {
    let is_active = app.main_window.get_active_input_entity() == ActiveInputEntity::SearchChats;
    let fg_color = if is_active {
        THEME.fg
    } else {
        THEME.inactive
    };
    let search_input_value = helpers::input_to_string(&app.main_window.get_search_input());
    let search_input = Paragraph::new(search_input_value.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title("Пошук")
                .style(Style::default().fg(fg_color))
        );

    let chats = app.main_window.chat_manager.get_active_chats_mut();
    f.render_widget(search_input, search_area);
    f.render_stateful_widget(
        build_chats(
            &chats.items,
            chats_area,
            is_active,
        ),
        chats_area,
        &mut chats.state,
    );
}

fn render_message_area(app: &App, f: &mut Frame, messages_area: Rect) {
    let is_active = app.main_window.get_active_input_entity() == ActiveInputEntity::EnterMessage;
    let fg_color = if is_active {
        THEME.fg
    } else {
        THEME.inactive
    };
    let (message_list_area, message_input_area) = create_message_area(messages_area);

    if let Some(loaded_chat) = app.main_window.chat_manager.get_loaded_chat() {
        let message_input_value = helpers::input_to_string(&app.main_window.get_message());
        let message_paragraph = Paragraph::new(message_input_value.as_str())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .style(Style::default().fg(fg_color))
            );

        let messages = if loaded_chat.id.is_none() {
            vec![]
        } else {
            app.main_window.chat_manager.get_messages(loaded_chat.id.unwrap()).clone()
        };
        f.render_widget(
            build_messages(messages, fg_color),
            message_list_area,
        );
        f.render_widget(message_paragraph, message_input_area);
    } else {
        f.render_widget(
            get_chat_hints(fg_color),
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

fn build_chats(chats: &[Chat], chats_area: Rect, is_active: bool) -> List {
    let items: Vec<ListItem> = chats
        .iter()
        .map(|chat| {
            // let name = chat.name.clone();
            // let created_at = chat.last_message.as_ref().map(|message| message.created_at.to_string()).unwrap_or_else(|| "".to_string());
            // let total_width = chats_area.width as usize;
            // -2 because 1 cell goes for the border at each side
            // let space_count = total_width - name.len() - created_at.len() - 2;
            // let mut formatted_string = format!("{name:<0$}{created_at}", space_count + name.len(), name = name, created_at = created_at);

            let name = chat.name.clone();
            let unread_count = if chat.number_of_unread_messages > 0 {
                format!("(+{})", chat.number_of_unread_messages)
            } else {
                "".to_string()
            };
            let total_width = chats_area.width as usize;
            // -2 because 1 cell goes for the border at each side
            let space_count = total_width - name.len() - unread_count.len() - 2;
            let formatted_string = format!("{name:<0$}{unread_count}", space_count + name.len(), name = name, unread_count = unread_count);
            let message_dt = Line::from(vec![
                Span::from(formatted_string),
            ]);

            ListItem::new(message_dt)
        })
        .collect();

    let fg_color = if is_active {
        THEME.fg
    } else {
        Color::DarkGray
    };
    List::new(items)
        .block(Block::default().title("Чати").borders(Borders::ALL).border_type(BorderType::Plain))
        .style(Style::default().fg(fg_color))
        .highlight_style(Style::default().bg(fg_color).bold().black())
        .direction(ListDirection::TopToBottom)
}

fn get_app_hints<'a>(app: &App) -> Paragraph<'a> {
    let paragraph = match app.is_authenticated() {
        true => {
            Paragraph::new("Натисніть `Ctrl-C` щоб закрити застосунок")
        }
        false => {
            Paragraph::new("`Enter` - відправити, `Tab` - наступне поле вводу, `Стрілки праворуч/ліворуч` - переключитись між вкладками")
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

fn build_messages<'a>(messages: Vec<Message>, fg_color: Color) -> List<'a> {
    let mut items: Vec<ListItem> = vec![];
    let mut sender_username = None;
    for message in messages.iter() {
        if sender_username.is_none() || sender_username.clone().unwrap() != message.sender_username {
            sender_username = Some(message.sender_username.clone());
            items.push(ListItem::new(""));
            items.push(
                ListItem::new(format!(
                    "{}: {}",
                    sender_username.clone().unwrap(),
                    message.text.clone(),
                ))
                    .style(Style::default())
            );
        } else {
            let spaces_count = sender_username.as_ref().map_or(0, |name| name.len() + 2);
            let spaces = " ".repeat(spaces_count);
            let formatted_message = format!("{}{}", spaces, message.text.clone());

            items.push(ListItem::new(formatted_message));
        }
    }
    items.reverse();

    List::new(items)
        .block(
            Block::default().title("Повідомлення").borders(Borders::ALL))
        .style(Style::default().fg(fg_color))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .direction(ListDirection::BottomToTop)
}

fn get_chat_hints<'a>(fg_color: Color) -> Paragraph<'a> {
    Paragraph::new("Використовуйте стрілки вгору/вниз щоб вибрати чат. Натисніть `Enter`, щоб відкрити чат. Почніть вводити текст щоб знайти користувача та натисніть `Enter`")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .padding(Padding::new(5, 5, 15, 0)),
        )
        .style(Style::default().fg(fg_color))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}
