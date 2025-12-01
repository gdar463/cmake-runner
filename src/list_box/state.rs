use crate::stateful_list::StatefulList;

pub struct ListBoxState<T> {
    pub list: StatefulList<T>,
    pub active: bool,
}

impl<T> ListBoxState<T> {
    pub fn get_selected(&self) -> Option<&T> {
        self.list.get_selected()
    }

    pub fn prev(&mut self) {
        self.list.prev();
    }

    pub fn next(&mut self) {
        self.list.next();
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
