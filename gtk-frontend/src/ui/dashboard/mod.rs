use gtk::prelude::*;
mod memory;
mod processes;
mod processors;
use crate::ui::{InitialState, Refresh};
use memory::Memory;
use processes::Processes;
use processors::Processors;

pub struct Dashboard {
    pub container: gtk::Paned,
    pub processors: Processors,
    pub memory: Memory,
    pub processes: Processes,
}

impl Dashboard {
    pub fn new(init_state: InitialState) -> Dashboard {
        let container = gtk::PanedBuilder::new()
            .wide_handle(true)
            .orientation(gtk::Orientation::Vertical)
            .expand(true)
            .build();

        let upper_container = gtk::GridBuilder::new()
            .expand(true)
            .column_homogeneous(true)
            .row_spacing(6)
            .column_spacing(6)
            .build();
        let processors = Processors::new(init_state.processor_count as i32);
        upper_container.attach(&processors.container, 0, 0, 2, 1);

        let memory = Memory::new();
        upper_container.attach(&memory.container, 0, 1, 1, 1);

        let temp_label2 = gtk::LabelBuilder::new().label("Usage placeholder").build();
        upper_container.attach(&temp_label2, 1, 1, 1, 1);

        let processes = Processes::new();

        container.add1(&upper_container);
        container.add2(&processes.container);

        Dashboard {
            container,
            processors,
            memory,
            processes,
        }
    }
}

impl Refresh for Dashboard {
    fn refresh(&self, system: &sysinfo::System) {
        self.processors.refresh(system);
        self.memory.refresh(system);
        self.processes.refresh(system);
    }
}
