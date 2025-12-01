use std::{path::Path, process::Stdio};

use tokio::{
    io::{AsyncBufReadExt, BufReader, Lines},
    process::{ChildStdout, Command},
    sync::mpsc,
};

use crate::project::Project;
use eyre::Result;

#[derive(Default)]
pub enum Action {
    #[default]
    Run,
    Build,
    Debug,
}

pub type LinesChild = Lines<BufReader<ChildStdout>>;

impl Action {
    pub fn to_str(&self) -> &'static str {
        match self {
            Action::Run => "Run",
            Action::Build => "Build",
            Action::Debug => "Debug",
        }
    }

    pub fn run(
        &self,
        out: &mpsc::Sender<Option<String>>,
        project: &Project,
        dir: &Path,
    ) -> Result<()> {
        match self {
            Action::Run => {
                let path = dir.to_str().unwrap();
                let target = &project.key;
                self.build(out, &path, target)
            }
            Action::Build => {
                let path = dir.to_str().unwrap();
                let target = &project.key;
                self.build(out, &path, target)
            }
            Action::Debug => {
                let path = dir.to_str().unwrap();
                let target = &project.key;
                self.build(out, &path, target)
            }
        }
    }

    fn build(&self, out: &mpsc::Sender<Option<String>>, path: &str, target: &String) -> Result<()> {
        let mut child = Command::new("cmake")
            .args(&["--build", "build", "-t", target])
            .current_dir(path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let stdout = child.stdout.take().expect("stdout not piped");
        let mut reader = BufReader::new(stdout).lines();

        tokio::spawn(async move {
            out.send(reader.next_line().await.unwrap());
        });

        Ok(())
    }
}
