use std::io::BufReader;

use duct::ReaderHandle;

#[derive(Default)]
pub enum Reader {
    Done(String),
    Present(BufReader<ReaderHandle>),
    #[default]
    None,
}

impl Reader {
    pub fn get_reader(&mut self) -> Option<&mut BufReader<ReaderHandle>> {
        match self {
            Reader::Present(r) => Some(r),
            _ => None,
        }
    }

    pub fn get_string(&mut self) -> Option<&String> {
        match self {
            Reader::Done(s) => Some(s),
            _ => None,
        }
    }
}
