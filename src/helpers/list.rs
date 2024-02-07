use std::collections::HashMap;
use ratatui::widgets::ListState;
use crate::helpers::traits::InternalID;

#[derive(Debug, Clone)]
pub struct StatefulOrderedList<T>
    where T: InternalID + Ord + Clone
{
    pub state: ListState,
    pub items: Vec<T>,
    pub selected_item_id: Option<String>,
    item_indices: HashMap<String, usize>,
}

impl<T> StatefulOrderedList<T>
    where T: InternalID + Ord + Clone
{
    pub fn get(&self, item_id: &String) -> &T {
        &self.item_indices.get(item_id).map(|i| &self.items[*i]).expect("Item not found")
    }

    pub fn get_mut(&mut self, item_id: &String) -> &mut T {
        self.item_indices.get_mut(item_id).map(|i| &mut self.items[*i]).expect("Item not found")
    }

    pub fn extend(&mut self, items: Vec<T>) {
        self.items.extend(items);
        self.items.sort();
        self.build_indices();
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
        self.items.sort();
        self.build_indices();
    }


    pub fn select(&mut self, item_id: &String) -> T {
        self.selected_item_id = Some(item_id.clone());
        self.update_state();
        self.item_indices.get(item_id).map(|i| &self.items[*i]).expect("Item not found").clone()
    }

    pub fn unselect(&mut self) {
        self.selected_item_id = None;
        self.update_state();
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

    fn build_indices(&mut self) {
        self.item_indices.clear();
        for (i, item) in self.items.iter().enumerate() {
            self.item_indices.insert(item.internal_id(), i);
        }
        self.update_state();
    }

    fn update_state(&mut self) {
        if self.selected_item_id.is_none() {
            self.state.select(None);
        }

        let selected_item_index = self
            .item_indices
            .get(self.selected_item_id.as_ref().expect("No item selected"))
            .expect("Selected item not found");
        self.state.select(Some(*selected_item_index));
    }
}

impl<T> Default for StatefulOrderedList<T>
    where T: InternalID + Ord + Clone
{
    fn default() -> Self {
        Self {
            state: ListState::default(),
            items: Vec::new(),
            selected_item_id: None,
            item_indices: HashMap::new(),
        }
    }
}
