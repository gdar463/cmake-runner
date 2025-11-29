use crate::stateful_list::StatefulList;

pub struct ListBoxState<T> {
    pub list: StatefulList<T>,
    pub active: bool,
}

impl<T> Default for ListBoxState<T> {
    fn default() -> Self {
        Self {
            active: true,
            list: Default::default(),
        }
    }
}
