use gtk::prelude::*;

pub struct Logs {
    pub container: gtk::Box,
}

impl Logs {
    pub fn new() -> Logs {
        let container = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Vertical)
            .expand(true)
            .build();

        let test_label = gtk::LabelBuilder::new().label("lfgGFGDFgdgd").build();
        container.add(&test_label);

        Logs { container }
    }
}
