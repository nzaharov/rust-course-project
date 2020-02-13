use crate::ui::Refresh;
use gtk::prelude::*;
use sysinfo::SystemExt;

pub struct Memory {
    pub container: gtk::Box,
    pub ram: gtk::LevelBar,
    pub swap: gtk::LevelBar,
}

impl Memory {
    pub fn new() -> Self {
        let container = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Vertical)
            .expand(true)
            .build();

        let ram = gtk::LevelBarBuilder::new()
            .min_value(0_f64)
            .max_value(0_f64)
            .height_request(30)
            .build();
        container.add(&ram);

        let swap = gtk::LevelBarBuilder::new()
            .min_value(0_f64)
            .max_value(0_f64)
            .height_request(30)
            .build();
        container.add(&swap);

        Self {
            container,
            ram,
            swap,
        }
    }
}

impl Refresh for Memory {
    fn refresh(&self, system: &sysinfo::System) {
        set_bar_values_in_mb(
            &self.ram,
            system.get_total_memory(),
            system.get_used_memory(),
        );

        set_bar_values_in_mb(&self.swap, system.get_total_swap(), system.get_used_swap());
    }
}

fn set_bar_values_in_mb(bar: &gtk::LevelBar, max_value: u64, curr_value: u64) {
    let max_value: f64 = (max_value / 1000) as f64 * 1.024;
    let curr_value: f64 = (curr_value / 1000) as f64 * 1.024;
    bar.set_max_value(max_value);
    bar.set_value(curr_value);
}
