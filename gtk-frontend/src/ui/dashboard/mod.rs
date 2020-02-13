use gtk::prelude::*;
mod memory;
mod processors;
use crate::ui::Refresh;
use memory::Memory;
use processors::Processors;

pub struct Dashboard {
    pub container: gtk::Paned,
    pub processors: Processors,
    pub memory: Memory,
}

impl Dashboard {
    pub fn new() -> Dashboard {
        let container = gtk::PanedBuilder::new()
            .wide_handle(true)
            .orientation(gtk::Orientation::Vertical)
            .expand(true)
            .build();

        let upper_container = gtk::GridBuilder::new()
            .expand(true)
            .column_homogeneous(true)
            .build();
        let processors = Processors::new(8);
        upper_container.attach(&processors.container, 0, 0, 2, 1);

        let memory = Memory::new();
        upper_container.attach(&memory.container, 0, 1, 1, 1);

        let temp_label2 = gtk::LabelBuilder::new().label("Usage placeholder").build();
        upper_container.attach(&temp_label2, 1, 1, 1, 1);

        let temp_label = gtk::LabelBuilder::new()
            .label("Processes placeholder")
            .build();
        container.add1(&upper_container);
        container.add2(&temp_label);

        Dashboard {
            container,
            processors,
            memory,
        }
    }
}

impl Refresh for Dashboard {
    fn refresh(&self, system: &sysinfo::System) {
        self.processors.refresh(system);
        self.memory.refresh(system);
    }
}
