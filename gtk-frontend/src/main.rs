extern crate gio;
extern crate gtk;
#[macro_use]
extern crate glib;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow, Button, Label};
use std::cell::RefCell;
use std::rc::Rc;

use sysinfo::{ProcessorExt, System, SystemExt};

type BarRefs = Rc<RefCell<Vec<gtk::LevelBar>>>;

pub fn setup_system_interval(
    refresh_time: u32,
    system: &Rc<RefCell<sysinfo::System>>,
    label: &Rc<RefCell<Label>>,
) {
    gtk::timeout_add(
        refresh_time,
        clone!(@strong system, @strong label => @default-return glib::Continue(true), move || {
            let mut system = system.borrow_mut();
            let label = label.borrow();

            system.refresh_system();
            let cpu_usage =
                system
                    .get_processors()
                    .iter()
                    .fold(String::new(), |mut acc, processor| {
                        acc.push_str(&format!("{}: {:.2}%\n", processor.get_name(), processor.get_cpu_usage() * 100_f32));
                        acc
                    });
            label.set_label(&cpu_usage);

            // println!("{}",&cpu_usage);

            glib::Continue(true)
            }
        ),
    );
}

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
        window.set_title("rtop");
        window.set_default_size(400, 200);

        let button = Button::new_with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Clicked!");
        });

        let system = System::new();

        let outer_box = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Horizontal)
            .expand(true)
            .build();
        window.add(&outer_box);

        let bars: Vec<gtk::LevelBar> = system
            .get_processors()
            .chunks(4)
            .flat_map(|processors_chunk| {
                let inner_box = gtk::BoxBuilder::new()
                    .orientation(gtk::Orientation::Vertical)
                    .expand(true)
                    .build();
                outer_box.add(&inner_box);

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
            .collect();

        let system = Rc::new(RefCell::new(system));
        let bar_refs = Rc::new(RefCell::new(bars));
        // let label = Label::new_with_mnemonic(Some("&cpu_usage"));
        // window.add(&label);
        // let label_refc = Rc::new(RefCell::new(label));

        setup_processors_interval(1000, &system, &bar_refs);

        // println!("{:?}", system);

        // window.add(&button);

        window.show_all();
    });

    application.run(&[]);
}
