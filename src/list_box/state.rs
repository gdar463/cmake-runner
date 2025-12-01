use crate::stateful_list::StatefulList;

pub struct ListBoxState<T> {
    pub list: StatefulList<T>,
    pub active: bool,
}

impl<T> ListBoxState<T> {
    pub fn get_selected(&self) -> &T {
        &self.list.items[self.list.state.selected().unwrap_or_default()]
    }

    pub fn prev(&mut self) {
        self.list.state.select_previous();
    }

    pub fn next(&mut self) {
        self.list.state.select_next();
    }
}

impl<T> Default for ListBoxState<T> {
    fn default() -> Self {
        Self {
            active: true,
            list: Default::default(),
        }
    }
}
