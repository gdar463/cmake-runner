use eyre::Result;
use std::{collections::BTreeMap, fs, path::PathBuf};

use crate::project::Project;

pub fn refresh_list(path: &PathBuf) -> Result<Vec<Project>> {
    let file = fs::read_to_string(path)?;
    let mut projects: BTreeMap<String, Project> = BTreeMap::new();
    for line in file.lines() {
        if line.starts_with("add_executable") {
            let target = line
                .split("(")
                .nth(1)
                .unwrap()
                .split(" ")
                .nth(0)
                .unwrap()
                .to_string();
            projects.insert(
                target.clone(),
                Project {
                    target,
                    file_name: "".to_string(),
                },
            );
        } else if line.starts_with("set_target_properties") {
            let target = line
                .split("(")
                .nth(1)
                .unwrap()
                .split(" ")
                .nth(0)
                .unwrap()
                .to_string();
            if let Some(element) = projects.get_mut(&target) {
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
    Ok(projects.into_values().collect())
}
