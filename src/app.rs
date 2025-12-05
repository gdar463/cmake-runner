use std::{path::PathBuf, time::Duration};

pub mod io;
use super::*;
use crate::{
    action::Action,
    list_box::{ListBox, state::ListBoxState},
    project::Project,
    stateful_list::StatefulList,
};
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyModifiers};
use eyre::Result;
use io::AppIo;
use ratatui::{
    DefaultTerminal,
    text::{Span, Text},
};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

#[derive(Default)]
pub struct App {
    projects: ListBoxState<Project>,
    actions: ListBoxState<Action>,
    path: PathBuf,
    output: Vec<Text<'static>>,
    io: AppIo,
    in_tx: Option<mpsc::Sender<Result<String>>>,
    offset: u16,
    exit: bool,
    input: bool,
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
            path,
            input: false,
            in_tx: None,
            ..Default::default()
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
                Some(line) = self.io.out_rx.recv() => {
                    match line {
                        Ok(line) => self.output.push(line),
                        Err(e) => self.output.push(Text::raw(e.to_string()).light_red()),
                    }
                }
            };
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self, event: &Event) -> Result<()> {
        if let Some(key_event) = event.as_key_press_event() {
            self.handle_key_event(key_event.to_owned())?
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        if self.input {
            if key_event.code == KeyCode::Char('i') && key_event.modifiers == KeyModifiers::ALT {
                self.input = false;
                self.in_tx = None;
            } else if let Some(in_tx) = &self.in_tx {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    match key_event.code {
                        KeyCode::Char('j') => {
                            self.offset += 1;
                        }
                        KeyCode::Char('k') => {
                            self.offset = self.offset.saturating_sub(1);
                        }
                        _ => {}
                    }
                    return Ok(());
                }

                let mut buf = [0; 4];
                let str_buf = match key_event.code {
                    KeyCode::Char(c) => c.encode_utf8(&mut buf).to_string(),
                    KeyCode::Enter => "\n".to_string(),
                    _ => "".to_string(),
                };

                if !str_buf.is_empty() {
                    if let Some(l) = self.output.last_mut() {
                        l.push_span(format!("{str_buf}"));
                    }
                    in_tx.try_send(Ok(str_buf)).ok();
                }
            }
            return Ok(());
        }

        match key_event.code {
            KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.exit = true;
            }
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
            KeyCode::Enter => {
                if let (Some(action), Some(project)) = (
                    self.actions.get_selected().cloned(),
                    self.projects.get_selected().cloned(),
                ) {
                    self.output.clear();

                    let out_tx = self.io.out_tx.clone();
                    let (command_in_tx, command_in_rx) = mpsc::channel(50);
                    self.in_tx = Some(command_in_tx);

                    let path = self.path.clone();
                    tokio::spawn(async move {
                        if let Err(e) = action.run(&out_tx, command_in_rx, &project, &path).await {
                            out_tx.send(Err(e)).await.ok();
                        }
                    });
                }
            }
            KeyCode::Char('j') => {
                self.offset += 1;
            }
            KeyCode::Char('k') => {
                self.offset = self.offset.saturating_sub(1);
            }
            KeyCode::Char('i') if key_event.modifiers == KeyModifiers::ALT => {
                self.input = true;
            }
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
        let block = if self.input {
            Block::bordered()
                .border_style(Style::new().light_yellow())
                .title(" Output ")
        } else {
            Block::bordered()
                .border_style(Style::new().gray())
                .title(" Output ")
        };
        let lines: Vec<_> = self
            .output
            .iter()
            .flat_map(|text| {
                text.lines.clone().into_iter().map(|mut line| {
                    let mut new_spans = vec![Span::raw(" ")];
                    new_spans.extend(line.spans.into_iter());
                    line.spans = new_spans;
                    line.patch_style(text.style)
                })
            })
            .collect();
        Paragraph::new(lines)
            .block(block)
            .scroll((self.offset, 0))
            .render(out_area, buf);
    }
}
