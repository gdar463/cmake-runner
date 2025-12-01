use std::{path::PathBuf, time::Duration};

use crate::{
    action::Action,
    list_box::{ListBox, state::ListBoxState},
    project::Project,
    stateful_list::StatefulList,
};

use super::*;
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyModifiers};
use eyre::Result;
use ratatui::DefaultTerminal;
use tokio::sync::{Mutex, mpsc};
use tokio_stream::StreamExt;

pub struct App {
    projects: ListBoxState<Project>,
    actions: ListBoxState<Action>,
    path: PathBuf,
    out_channel: Mutex<(mpsc::Sender<String>, mpsc::Receiver<String>)>,
    exit: bool,
}

impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;

    pub fn new(path: PathBuf) -> Self {
        Self {
            actions: ListBoxState {
                list: StatefulList {
                    items: vec![Action::Run, Action::Build, Action::Debug],
                    ..Default::default()
                },
                active: false,
            },
            projects: Default::default(),
            path,
            out_channel: Mutex::new(mpsc::channel(50)),
            exit: false,
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.refresh_list()?;

        let period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
        let mut interval = tokio::time::interval(period);
        let mut events = EventStream::new();

        while !self.exit {
            tokio::select! {
                _ = interval.tick() => {terminal.draw(|frame| self.draw(frame))?;},
                Some(Ok(event)) = events.next() => self.handle_events(&event)?,
            };
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self, event: &Event) -> Result<()> {
        if let Some(key_event) = event.as_key_press_event() {
            self.handle_key_event(key_event)?
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => self.exit = true,
            KeyCode::Char('r') if key_event.modifiers == KeyModifiers::SHIFT => {
                self.refresh_list()?
            }
            KeyCode::Char('a') => {
                self.projects.active = !self.projects.active;
                self.actions.active = !self.actions.active
            }
            KeyCode::Up if self.projects.active => self.projects.prev(),
            KeyCode::Up if self.actions.active => self.actions.prev(),
            KeyCode::Down if self.projects.active => self.projects.next(),
            KeyCode::Down if self.actions.active => self.actions.next(),
            KeyCode::Enter if self.actions.active => {}
            _ => {}
        }
        Ok(())
    }

    fn refresh_list(&mut self) -> Result<()> {
        self.projects.list.items = parser::refresh_list(&self.path)?;
        Ok(())
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let projects_area = Rect {
            x: area.x + 1,
            y: area.y,
            width: area.width / 4,
            height: area.height / 2,
        };
        ListBox::<Project>::new(" Projects ").render(projects_area, buf, &mut self.projects);

        let actions_area = Rect {
            x: area.x + 1,
            y: projects_area.y + projects_area.height,
            width: area.width / 4,
            height: area.height / 2,
        };
        ListBox::<Action>::new(" Actions ").render(actions_area, buf, &mut self.actions);

        let out_area = Rect {
            x: projects_area.x + projects_area.width,
            y: area.y,
            width: area.width / 4 * 3 - 1,
            height: area.height - 1,
        };
        let block = Block::bordered().title(" Output ");
        let value = " <empty>".to_string();
        Paragraph::new(value).block(block).render(out_area, buf);
    }
}
