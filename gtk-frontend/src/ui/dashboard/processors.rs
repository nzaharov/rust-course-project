use gtk::prelude::*;

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
