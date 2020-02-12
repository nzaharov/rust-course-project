use gtk::prelude::*;
mod processors;
use crate::ui::Refresh;
use processors::Processors;

pub struct Dashboard {
    pub container: gtk::Paned,
    pub processors: Processors,
}

impl Dashboard {
    pub fn new() -> Dashboard {
        let container = gtk::PanedBuilder::new()
            .wide_handle(true)
            .orientation(gtk::Orientation::Vertical)
            .expand(true)
            .build();
        let processors = Processors::new(8);
        let temp_label = gtk::LabelBuilder::new().label("Temp label").build();
        container.add1(&processors.container);
        container.add2(&temp_label);

        Dashboard {
            container,
            processors,
        }
    }
}

impl Refresh for Dashboard {
    fn refresh(&self, system: &sysinfo::System) {
        self.processors.refresh(system);
    }
}
