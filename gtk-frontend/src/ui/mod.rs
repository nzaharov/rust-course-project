mod common;
pub mod content;
mod dashboard;
pub mod header;
mod logs;

pub trait Refresh {
    fn refresh(&self, system: &sysinfo::System);
}

#[derive(Debug)]
pub struct State {
    pub processor_count: usize,
    pub logging_on: bool,
    pub pc_name: String,
}

impl State {
    pub fn new(processor_count: usize) -> Self {
        Self {
            processor_count,
            logging_on: false,
            pc_name: String::new(),
        }
    }

    pub fn toggle_logging(&mut self) {
        self.logging_on = !self.logging_on;
    }

    pub fn set_name(&mut self, text: &str) {
        self.pc_name = String::from(text);
    }
}
