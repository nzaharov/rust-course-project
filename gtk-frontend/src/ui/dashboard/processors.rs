use crate::ui::common::labeledbar::LabeledBar;
use crate::ui::Refresh;
use gtk::prelude::*;
use sysinfo::{ProcessorExt, SystemExt};

pub struct Processors {
    pub container: gtk::Box,
    pub processors: Vec<LabeledBar>,
}

impl Processors {
    pub fn new(processor_count: i32) -> Self {
        let container = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Horizontal)
            .expand(true)
            .build();

        let processors = (0..processor_count)
            .collect::<Vec<_>>()
            .chunks(4)
            .enumerate()
            .flat_map(|(_, processors_chunk)| {
                let inner_box = gtk::BoxBuilder::new()
                    .orientation(gtk::Orientation::Vertical)
                    .expand(true)
                    .build();
                container.add(&inner_box);

                processors_chunk.iter().map(move |i| {
                    let bar = LabeledBar::new(&(i + 1).to_string());
                    bar.set_max_value(100_f64);
                    inner_box.add(&bar.container);

                    bar
                })
            })
            .collect::<Vec<LabeledBar>>();

        Processors {
            container,
            processors,
        }
    }
}

impl Refresh for Processors {
    fn refresh(&self, system: &sysinfo::System) {
        let processors = system.get_processors();

        self.processors
            .iter()
            .zip(processors.iter())
            .for_each(|(bar, processor)| {
                bar.set_value(processor.get_cpu_usage() as f64);
            });
    }
}
