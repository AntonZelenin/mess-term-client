use ratatui::widgets::ListState;

#[derive(Debug, Default, Clone)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}
