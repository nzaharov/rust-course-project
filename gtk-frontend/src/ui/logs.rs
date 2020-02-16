use crate::ui::common::table::Table;
use crate::ui::Refresh;
use gtk::prelude::*;

pub struct Logs {
    pub table: Table,
}

impl Logs {
    pub fn new() -> Self {
        let column_names = ["Timestamp", "CPU Usage", "RAM"];
        let column_types = [
            u32::static_type(),
            String::static_type(),
            String::static_type(),
        ];

        let table = Table::new(&column_names, &column_types);

        Self { table }
    }
}

impl Refresh for Logs {
    fn refresh(&self, system: &sysinfo::System) {
        unimplemented!()
    }
}
