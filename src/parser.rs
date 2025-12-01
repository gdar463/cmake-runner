use eyre::Result;
use std::{fs, path::PathBuf};

use crate::project::Project;

pub fn refresh_list(path: &PathBuf) -> Result<Vec<Project>> {
    let file = fs::read_to_string(path)?;
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
    Ok(projects)
}
