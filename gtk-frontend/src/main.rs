#[macro_use]
extern crate glib;

mod ui;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow};
use std::cell::RefCell;
use std::rc::Rc;

use sysinfo::{ProcessorExt, System, SystemExt};

use crate::ui::content::Content;
use crate::ui::header::Header;

type BarRefs = Rc<RefCell<Vec<gtk::LevelBar>>>;

pub fn setup_processors_interval(
    refresh_time: u32,
    system: &Rc<RefCell<sysinfo::System>>,
    bar_refs: &BarRefs,
) {
    gtk::timeout_add(
        refresh_time,
        clone!(@strong system, @strong bar_refs => @default-return glib::Continue(true), move || {
            let mut system = system.borrow_mut();
            let bars = bar_refs.borrow();

            system.refresh_system();

            let processors = system.get_processors();

            bars.iter().zip(processors.iter()).for_each(|(bar,processor)| {
                bar.set_value(processor.get_cpu_usage() as f64);
            });

            glib::Continue(true)
            }
        ),
    );
}

fn main() {
    let application = Application::new(Some("com.nzaharov.rtop"), Default::default())
        .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_default_size(800, 500);

        let header = Header::new();
        window.set_titlebar(Some(&header.container));

        let content = Content::new();
        header.stack_switch.set_stack(Some(&content.stack));

        window.add(&content.stack);

        // let system = System::new();

        // let outer_box = gtk::BoxBuilder::new()
        //     .orientation(gtk::Orientation::Horizontal)
        //     .expand(true)
        //     .build();
        // window.add(&outer_box);

        // let bars: Vec<gtk::LevelBar> = system
        //     .get_processors()
        //     .chunks(4)
        //     .flat_map(|processors_chunk| {
        //         let inner_box = gtk::BoxBuilder::new()
        //             .orientation(gtk::Orientation::Vertical)
        //             .expand(true)
        //             .build();
        //         outer_box.add(&inner_box);

        //         processors_chunk.iter().map(move |_| {
        //             let builder = gtk::LevelBarBuilder::new();
        //             let bar = builder
        //                 .min_value(0_f64)
        //                 .max_value(100_f64)
        //                 .height_request(30)
        //                 .build();
        //             inner_box.add(&bar);

        //             bar
        //         })
        //     })
        //     .collect();

        // let system = Rc::new(RefCell::new(system));
        // let bar_refs = Rc::new(RefCell::new(bars));
        // setup_processors_interval(1000, &system, &bar_refs);

        window.show_all();
    });

    application.run(&[]);
}
