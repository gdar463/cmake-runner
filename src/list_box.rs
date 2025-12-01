use std::marker::PhantomData;

use super::*;
pub mod state;
use state::ListBoxState;

pub trait ListItemProvider {
    fn as_str(&self) -> &str;
}

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

impl<T: ListItemProvider> StatefulWidget for &ListBox<T> {
    type State = ListBoxState<T>;
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
                .border_style(Style::new().gray())
                .title(self.title)
        };

        let items: Vec<ListItem> = state
            .list
            .items
            .iter()
            .map(|s| ListItem::new(s.as_str()))
            .collect();

        StatefulWidget::render(
            List::new(items)
                .block(block)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD).light_green())
                .highlight_symbol(" > "),
            area,
            buf,
            &mut state.list.state,
        );
    }
}
