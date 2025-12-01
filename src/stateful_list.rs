use super::*;

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn get_selected(&self) -> Option<&T> {
        self.state.selected().and_then(|i| self.items.get(i))
    }

    pub fn prev(&mut self) {
        if self.items.is_empty() {
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

    pub fn next(&mut self) {
        if self.items.is_empty() {
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
}

impl<T> Default for StatefulList<T> {
    fn default() -> Self {
        Self {
            state: ListState::default().with_selected(Some(0)),
            items: Default::default(),
        }
    }
}
