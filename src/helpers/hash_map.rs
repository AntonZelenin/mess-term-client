use std::collections::HashMap;
use std::hash::Hash;
use ratatui::widgets::ListState;

pub struct StatefulHashMap<K, V> {
    pub state: ListState,
    pub items: HashMap<K, V>,
}

impl<K, V> StatefulHashMap<K, V>
    where K: Eq + PartialEq + Hash
{
    pub fn new() -> StatefulHashMap<K, V> {
        StatefulHashMap {
            state: ListState::default(),
            items: HashMap::new(),
        }
    }

    pub fn with_items(items: HashMap<K, V>) -> StatefulHashMap<K, V> {
        StatefulHashMap {
            state: ListState::default(),
            items,
        }
    }

    pub fn push(&mut self, key: K, value: V) {
        self.items.insert(key, value);
    }

    pub fn next(&mut self) {
        if self.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<K, V> Default for StatefulHashMap<K, V>
    where K: Eq + PartialEq + Hash
{
    fn default() -> Self {
        StatefulHashMap::new()
    }
}
