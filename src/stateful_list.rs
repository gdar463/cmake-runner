use super::*;

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> Default for StatefulList<T> {
    fn default() -> Self {
        Self {
            state: ListState::default().with_selected(Some(0)),
            items: Default::default(),
        }
    }
}
