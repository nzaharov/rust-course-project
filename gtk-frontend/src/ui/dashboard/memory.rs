use crate::ui::common::labeledbar::LabeledBar;
use crate::ui::Refresh;
use gtk::prelude::*;
use sysinfo::SystemExt;

pub struct Memory {
    pub container: gtk::Box,
    pub ram: LabeledBar,
    pub swap: LabeledBar,
}

impl Memory {
    pub fn new() -> Self {
        let container = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Vertical)
            .expand(true)
            .build();

        let ram = LabeledBar::new("RAM");
        container.add(&ram.container);

        let swap = LabeledBar::new("Swap");
        container.add(&swap.container);

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

fn set_bar_values_in_mb(bar: &LabeledBar, max_value: u64, curr_value: u64) {
    let max_value: f64 = (max_value / 1000) as f64 * 1.024;
    let curr_value: f64 = (curr_value / 1000) as f64 * 1.024;
    bar.set_max_value(max_value);
    bar.set_value(curr_value);
}
