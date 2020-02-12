use crate::ui::Refresh;
use gtk::prelude::*;
use sysinfo::{ProcessorExt, SystemExt};

pub struct Processors {
    pub container: gtk::Box,
    pub processors: Vec<gtk::LevelBar>,
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
            .flat_map(|processors_chunk| {
                let inner_box = gtk::BoxBuilder::new()
                    .orientation(gtk::Orientation::Vertical)
                    .expand(true)
                    .build();
                container.add(&inner_box);

                processors_chunk.iter().map(move |_| {
                    let builder = gtk::LevelBarBuilder::new();
                    let bar = builder
                        .min_value(0_f64)
                        .max_value(100_f64)
                        .height_request(30)
                        .build();
                    inner_box.add(&bar);

                    bar
                })
            })
            .collect::<Vec<gtk::LevelBar>>();

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
