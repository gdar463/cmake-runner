use crate::project::Project;

use super::*;

pub struct StatefulList {
    pub state: ListState,
    pub items: Vec<Project>,
}

impl Default for StatefulList {
    fn default() -> Self {
        Self {
            state: ListState::default().with_selected(Some(0)),
            items: vec![
                Project::from("1;;;test"),
                Project::from("2;;;wow"),
                Project::from("3;;;just like magic"),
            ],
        }
    }
}
