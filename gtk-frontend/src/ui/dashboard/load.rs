use crate::ui::Refresh;
use gtk::prelude::*;
use sysinfo::SystemExt;

pub struct Load {
    pub container: gtk::Box,
    pub label: gtk::Label,
    pub values: gtk::Label,
}

impl Load {
    pub fn new() -> Self {
        let container = gtk::BoxBuilder::new()
            .expand(true)
            .spacing(4)
            .orientation(gtk::Orientation::Horizontal)
            .build();

        let label = gtk::LabelBuilder::new()
            .expand(true)
            .label("Load average: ")
            .build();

        let values = gtk::LabelBuilder::new()
            .expand(true)
            .label("Loading...")
            .build();

        container.add(&label);
        container.add(&values);

        Self {
            container,
            label,
            values,
        }
    }

    pub fn set_values(&self, load_one: f64, load_five: f64, load_fifteen: f64) {
        let label_text = format!(
            "1\" : {:.2}%   5\" : {:.2}%   15\" : {:.2}%",
            load_one, load_five, load_fifteen
        );
        self.values.set_label(&label_text);
    }
}

impl Refresh for Load {
    fn refresh(&self, system: &sysinfo::System) {
        let load_avg = system.get_load_average();
        self.set_values(load_avg.one, load_avg.five, load_avg.fifteen);
    }
}
