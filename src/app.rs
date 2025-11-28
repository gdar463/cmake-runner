use std::{fs, path::PathBuf, time::Duration};

use crate::{list_box::ListBox, project::Project, stateful_list::StatefulList};

use super::*;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use eyre::Result;
use ratatui::DefaultTerminal;

#[derive(Default)]
pub struct App {
    projects: StatefulList,
    path: PathBuf,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal, path: PathBuf) -> Result<()> {
        self.path = path;
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
            KeyCode::Up => self.projects.state.select_previous(),
            KeyCode::Down => self.projects.state.select_next(),
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
        self.projects.items = projects;
        Ok(())
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let projects_area = Rect {
            x: area.x + 1,
            y: area.y,
            width: area.width / 4,
            height: area.height - 1,
        };
        ListBox::new(" Projects ").render(projects_area, buf, &mut self.projects);

        let desc_area = Rect {
            x: projects_area.x + projects_area.width,
            y: area.y,
            width: area.width / 4 * 3 - 1,
            height: area.height - 1,
        };
        let block = Block::bordered().title(" Description ");
        let value = &self
            .projects
            .items
            .get(self.projects.state.selected().unwrap_or(0))
            .unwrap()
            .file_name;
        Paragraph::new(format!(" {value}"))
            .block(block)
            .render(desc_area, buf);
    }
}
