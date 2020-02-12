use gtk::prelude::*;
mod processors;
use crate::ui::Refresh;
use processors::Processors;

pub struct Dashboard {
    pub container: gtk::Box,
    pub processors: Processors,
}

impl Dashboard {
    pub fn new() -> Dashboard {
        let container = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Vertical)
            .expand(true)
            .build();

        let processors = Processors::new(8);
        container.add(&processors.container);

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
