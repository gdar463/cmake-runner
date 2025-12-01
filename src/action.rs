use std::{path::Path, process::Stdio};

use ansi_to_tui::IntoText;
use ratatui::{style::Stylize, text::Text};
use tokio::{
    io::{AsyncReadExt, BufReader},
    process::Command,
    sync::mpsc,
};

use crate::project::Project;
use eyre::Result;

#[derive(Default, PartialEq, Eq, Clone)]
pub enum Action {
    #[default]
    Run,
    Build,
    Debug,
}

impl Action {
    pub fn to_str(&self) -> &'static str {
        match self {
            Action::Run => "Run",
            Action::Build => "Build",
            Action::Debug => "Debug",
        }
    }

    pub async fn run(
        &self,
        out: &mpsc::Sender<Result<Text<'static>>>,
        project: &Project,
        dir: &Path,
    ) -> Result<()> {
        let path = dir.parent().unwrap().to_str().unwrap();
        let target = &project.key;
        match self {
            Action::Run => self.build_and_run(out, path, project).await,
            Action::Build => {
                self.spawn_command(
                    out,
                    "cmake",
                    &["--build", "build", "-t", target],
                    path,
                    "Build",
                )
                .await
            }
            Action::Debug => self.build_and_run(out, path, project).await,
        }
    }

    async fn build_and_run(
        &self,
        out: &mpsc::Sender<Result<Text<'static>>>,
        path: &str,
        project: &Project,
    ) -> Result<()> {
        let result = self
            .spawn_command(
                out,
                "cmake",
                &["--build", "build", "-t", &project.key],
                path,
                "Build",
            )
            .await;
        match result {
            Err(..) => return Ok(()),
            _ => {}
        };
        self.spawn_command(
            out,
            &format!("build/{0}", project.file_name),
            &[],
            path,
            "Run",
        )
        .await
    }

    async fn spawn_command(
        &self,
        out: &mpsc::Sender<Result<Text<'static>>>,
        command: &str,
        args: &[&str],
        path: &str,
        action: &str,
    ) -> Result<()> {
        let mut child = Command::new(command)
            .args(args)
            .current_dir(path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().expect("stdout not piped");
        let stderr = child.stderr.take().expect("stderr not piped");

        let mut stdout_reader = BufReader::new(stdout);
        let mut stderr_reader = BufReader::new(stderr);

        let out_clone = out.clone();
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            loop {
                match stdout_reader.read(&mut buffer).await {
                    Ok(0) => break,
                    Ok(n) => {
                        let text = (&buffer[..n]).into_text().unwrap();
                        if out_clone.send(Ok(text)).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        if out_clone.send(Err(e.into())).await.is_err() {
                            break;
                        }
                    }
                }
            }
        });

        let out_clone = out.clone();
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            loop {
                match stderr_reader.read(&mut buffer).await {
                    Ok(0) => break,
                    Ok(n) => {
                        let text = (&buffer[..n]).into_text().unwrap();
                        if out_clone.send(Ok(text)).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        if out_clone.send(Err(e.into())).await.is_err() {
                            break;
                        }
                    }
                }
            }
        });

        let status = child.wait().await?;
        out.send(Ok(Text::raw("\n"))).await?;
        if status.success() {
            out.send(Ok(Text::raw(format!("=== {action} finished")).light_green()))
                .await?;
            out.send(Ok(Text::raw("\n"))).await?;

            Ok(())
        } else {
            out.send(Ok(Text::raw(format!("=== {action} failed")).light_red()))
                .await?;
            out.send(Ok(Text::raw("\n"))).await?;
            Err(eyre::ErrReport::msg(1))
        }
    }
}
