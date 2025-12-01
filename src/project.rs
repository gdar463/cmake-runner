use crate::list_box::ListItemProvider;

#[derive(Default, Clone, Debug)]
pub struct Project {
    pub target: String,
    pub file_name: String,
}

impl ListItemProvider for Project {
    fn as_str(&self) -> &str {
        &self.target
    }
}

impl From<&str> for Project {
    fn from(s: &str) -> Self {
        let mut split = s.split(";;;");
        let key = split.next().unwrap().to_string();
        let value = split.next().unwrap().to_string();
        Project {
            target: key,
            file_name: value,
        }
    }
}
