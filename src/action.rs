use std::{io::BufReader, path::Path};

use duct::{ReaderHandle, cmd};

use crate::project::Project;
use eyre::Result;

#[derive(Default)]
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

    pub fn run(&self, project: &Project, dir: &Path) -> Result<BufReader<ReaderHandle>> {
        match self {
            Action::Run => {
                let path = dir.to_string_lossy();
                let target = &project.key;
                self.build(&path.to_string(), target)
            }
            Action::Build => {
                let path = dir.to_string_lossy();
                let target = &project.key;
                self.build(&path.to_string(), target)
            }
            Action::Debug => {
                let path = dir.to_string_lossy();
                let target = &project.key;
                self.build(&path.to_string(), target)
            }
        }
    }

    fn build(&self, path: &String, target: &String) -> Result<BufReader<ReaderHandle>> {
        Ok(BufReader::new(
            cmd!("cmake", "--build", "build", "-t", target)
                .dir(path)
                .stderr_to_stdout()
                .reader()?,
        ))
    }
}
