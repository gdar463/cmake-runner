#[derive(Default)]
pub struct Project {
    pub key: String,
    pub file_name: String,
}

impl From<&str> for Project {
    fn from(s: &str) -> Self {
        let mut split = s.split(";;;");
        let key = split.next().unwrap().to_string();
        let value = split.next().unwrap().to_string();
        Project {
            key,
            file_name: value,
        }
    }
}
