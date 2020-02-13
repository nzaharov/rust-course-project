#[macro_use]
extern crate glib;

mod ui;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow};
use std::cell::RefCell;
use std::rc::Rc;

use sysinfo::{ProcessorExt, RefreshKind, System, SystemExt};

use crate::ui::content::Content;
use crate::ui::header::Header;
use crate::ui::Refresh;

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

pub fn test_loop(
    refresh_time: u32,
    system: &Rc<RefCell<sysinfo::System>>,
    content: &Rc<RefCell<Content>>,
) {
    gtk::timeout_add(
        refresh_time,
        clone!(@strong system, @strong content => @default-return glib::Continue(true), move || {
            let content = content.borrow();
            let mut system = system.borrow_mut();

            system.refresh_system();

            content.refresh(&system);

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

        let system = System::new_with_specifics(
            RefreshKind::new().with_memory().with_cpu().with_processes(),
        );

        let header = Header::new();
        window.set_titlebar(Some(&header.container));

        let content = Content::new(ui::InitialState {
            processor_count: system.get_processors().len(),
        });
        header.stack_switch.set_stack(Some(&content.stack));
        window.add(&content.stack);

        let system = Rc::new(RefCell::new(system));
        let content = Rc::new(RefCell::new(content));
        test_loop(1000, &system, &content);

        window.show_all();
    });

    application.run(&[]);
}
