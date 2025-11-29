use std::marker::PhantomData;

use crate::{action::Action, project::Project};

use super::*;
pub mod state;
use state::ListBoxState;

pub struct ListBox<T> {
    title: &'static str,
    phantom: PhantomData<T>,
}

impl<T> ListBox<T> {
    pub fn new(title: &'static str) -> ListBox<T> {
        ListBox {
            title,
            phantom: PhantomData,
        }
    }
}

impl StatefulWidget for &ListBox<Action> {
    type State = ListBoxState<Action>;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let block = if state.active {
            Block::bordered()
                .border_type(BorderType::Double)
                .border_style(Style::new().light_blue())
                .title(self.title)
        } else {
            Block::bordered()
                .border_type(BorderType::Plain)
                .title(self.title)
        };

        let actions: Vec<ListItem> = state
            .list
            .items
            .iter()
            .map(|s| ListItem::new(s.to_str()))
            .collect();
        StatefulWidget::render(
            List::new(actions)
                .block(block)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD).light_green())
                .highlight_symbol(" > "),
            area,
            buf,
            &mut state.list.state,
        );
    }
}

impl StatefulWidget for &ListBox<Project> {
    type State = ListBoxState<Project>;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let block = if state.active {
            Block::bordered()
                .border_type(BorderType::Double)
                .border_style(Style::new().light_blue())
                .title(self.title)
        } else {
            Block::bordered()
                .border_type(BorderType::Plain)
                .title(self.title)
        };

        let projects: Vec<ListItem> = state
            .list
            .items
            .iter()
            .map(|s| ListItem::new(s.key.as_str()))
            .collect();
        StatefulWidget::render(
            List::new(projects)
                .block(block)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD).light_green())
                .highlight_symbol(" > "),
            area,
            buf,
            &mut state.list.state,
        );
    }
}
