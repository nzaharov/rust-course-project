mod common;
pub mod content;
mod dashboard;
pub mod header;
mod logs;

pub trait Refresh {
    fn refresh(&self, system: &sysinfo::System);
}

pub struct InitialState {
    pub processor_count: usize,
}
