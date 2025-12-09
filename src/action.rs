use std::path::Path;

use ratatui::text::Text;
use tokio::sync::mpsc;

use crate::{list_box::ListItemProvider, project::Project, utils};
use eyre::Result;

#[derive(Default, PartialEq, Eq, Clone)]
pub enum Action {
    #[default]
    Run,
    Build,
    Debug,
}

impl ListItemProvider for Action {
    fn as_str(&self) -> &str {
        self.to_str()
    }
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
        in_rx: mpsc::Receiver<Result<String>>,
        project: &Project,
        dir: &Path,
    ) -> Result<()> {
        let path = dir.parent().unwrap().to_str().unwrap();
        match self {
            Action::Run => self.build_and_run(out, Some(in_rx), path, project).await,
            Action::Build => self.build(out, path, project).await,
            Action::Debug => self.build_and_debug(out, Some(in_rx), path, project).await,
        }
    }

    async fn build_and_debug(
        &self,
        out: &mpsc::Sender<Result<Text<'static>>>,
        mut in_rx: Option<mpsc::Receiver<Result<String>>>,
        path: &str,
        project: &Project,
    ) -> Result<()> {
        let result = self.build(out, path, project).await;
        match result {
            Err(..) => return Ok(()),
            _ => {}
        };
        utils::spawn_command(
            out,
            in_rx.take(),
            "lldb",
            &[&format!("build/{0}", project.file_name)],
            path,
            "Run",
        )
        .await
    }

    async fn build_and_run(
        &self,
        out: &mpsc::Sender<Result<Text<'static>>>,
        mut in_rx: Option<mpsc::Receiver<Result<String>>>,
        path: &str,
        project: &Project,
    ) -> Result<()> {
        let result = self.build(out, path, project).await;
        match result {
            Err(..) => return Ok(()),
            _ => {}
        };
        utils::spawn_command(
            out,
            in_rx.take(),
            &format!("build/{0}", project.file_name),
            &[],
            path,
            "Run",
        )
        .await
    }

    async fn build(
        &self,
        out: &mpsc::Sender<Result<Text<'static>>>,
        path: &str,
        project: &Project,
    ) -> Result<()> {
        utils::spawn_command(
            out,
            None,
            "cmake",
            &["--build", "build", "-t", &project.target],
            path,
            "Build",
        )
        .await
    }
}
