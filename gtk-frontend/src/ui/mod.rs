mod common;
pub mod content;
mod dashboard;
pub mod header;
mod logs;

pub trait Refresh {
    fn refresh(&self, system: &sysinfo::System);
}

pub struct State {
    pub processor_count: usize,
}

impl State {
    pub fn new(processor_count: usize) -> Self {
        Self { processor_count }
    }
}
