use std::{fs, io::Read, path::PathBuf, time::Duration};

use crate::{
    action::Action,
    list_box::{ListBox, state::ListBoxState},
    project::Project,
    reader::Reader,
    stateful_list::StatefulList,
};

use super::*;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use eyre::Result;
use ratatui::DefaultTerminal;

#[derive(Default)]
pub struct App {
    projects: ListBoxState<Project>,
    actions: ListBoxState<Action>,
    path: PathBuf,
    reader: Reader,
    exit: bool,
}

impl App {
    pub fn new(path: PathBuf) -> Self {
        Self {
            actions: ListBoxState {
                list: StatefulList {
                    items: vec![Action::Run, Action::Build, Action::Debug],
                    ..Default::default()
                },
                active: false,
            },
            path,
            ..Default::default()
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.refresh_list()?;
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_secs(0)).unwrap() == true {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)?
                }
                _ => {}
            }
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
            KeyCode::Up if self.projects.active => self.projects.list.state.select_previous(),
            KeyCode::Up if self.actions.active => self.actions.list.state.select_previous(),
            KeyCode::Down if self.projects.active => self.projects.list.state.select_next(),
            KeyCode::Down if self.actions.active => self.actions.list.state.select_next(),
            KeyCode::Enter if self.actions.active => {
                self.reader = Reader::Present(
                    self.actions.list.items[self.actions.list.state.selected().unwrap_or_default()]
                        .run(
                            &self.projects.list.items
                                [self.projects.list.state.selected().unwrap_or_default()],
                            self.path.parent().unwrap(),
                        )?,
                );
            }
            _ => {}
        }
        Ok(())
    }

    fn refresh_list(&mut self) -> Result<()> {
        let file = fs::read_to_string(&self.path)?;
        let mut projects = Vec::new();
        for line in file.lines() {
            if line.starts_with("add_executable") {
                projects.push(Project {
                    key: line
                        .split("(")
                        .nth(1)
                        .unwrap()
                        .split(" ")
                        .nth(0)
                        .unwrap()
                        .to_string(),
                    file_name: "".to_string(),
                })
            } else if line.starts_with("set_target_properties") {
                let target = line
                    .split("(")
                    .nth(1)
                    .unwrap()
                    .split(" ")
                    .nth(0)
                    .unwrap()
                    .to_string();
                if let Some(element) = projects
                    .iter_mut()
                    .find(|item| if item.key == target { true } else { false })
                {
                    element.file_name = line
                        .split("\"")
                        .nth(1)
                        .unwrap()
                        .split("\"")
                        .nth(0)
                        .unwrap()
                        .to_string();
                }
            }
        }
        self.projects.list.items = projects;
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
        let value = match &self.reader {
            Reader::Present(_r) => {
                let mut lines = String::new();
                let _ = self.reader.get_reader().unwrap().read_to_string(&mut lines);
                let lines = lines
                    .lines()
                    .fold(String::new(), |s, l| s + " " + &l + "\n");
                self.reader = Reader::Done(lines.clone());
                lines
            }
            Reader::Done(l) => l.to_string(),
            Reader::None => " <empty>".to_string(),
        };
        Paragraph::new(value).block(block).render(out_area, buf);
    }
}
