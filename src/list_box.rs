use crate::stateful_list::StatefulList;

use super::*;

pub struct ListBox {
    title: &'static str,
}

impl ListBox {
    pub fn new(title: &'static str) -> ListBox {
        ListBox { title }
    }
}

impl StatefulWidget for &ListBox {
    type State = StatefulList;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let block = Block::bordered()
            .border_type(BorderType::Double)
            .title(self.title);

        let projects: Vec<ListItem> = state
            .items
            .iter()
            .map(|s| ListItem::new(s.key.as_str()))
            .collect();
        StatefulWidget::render(
            List::new(projects)
                .block(block)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(" > "),
            area,
            buf,
            &mut state.state,
        );
    }
}
