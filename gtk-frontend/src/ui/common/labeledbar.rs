use gtk::prelude::*;

pub struct LabeledBar {
    pub container: gtk::Box,
    pub label: gtk::Label,
    pub bar: gtk::LevelBar,
}

impl LabeledBar {
    pub fn new(label_text: &str) -> Self {
        let container = gtk::BoxBuilder::new()
            .expand(true)
            .spacing(2)
            .orientation(gtk::Orientation::Horizontal)
            .build();

        let label = gtk::LabelBuilder::new()
            .width_chars(5)
            .label(label_text)
            .build();

        let bar = gtk::LevelBarBuilder::new()
            .height_request(30)
            .expand(true)
            .orientation(gtk::Orientation::Horizontal)
            .min_value(0_f64)
            .max_value(0_f64)
            .build();

        container.add(&label);
        container.add(&bar);

        Self {
            container,
            label,
            bar,
        }
    }

    pub fn set_max_value(&self, value: f64) {
        self.bar.set_max_value(value);
    }

    pub fn set_value(&self, value: f64) {
        self.bar.set_value(value);
    }
}
