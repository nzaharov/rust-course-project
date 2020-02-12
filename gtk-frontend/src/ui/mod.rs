pub mod content;
mod dashboard;
pub mod header;
mod logs;

pub trait Refresh {
    fn refresh(&self, system: &sysinfo::System);
}
