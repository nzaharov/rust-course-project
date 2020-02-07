extern crate gio;
extern crate gtk;
#[macro_use]
extern crate glib;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow, Button, Label};
use std::cell::RefCell;
use std::rc::Rc;

use sysinfo::{ProcessorExt, SystemExt};

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
                    .get_processor_list()
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

fn main() {
    let application =
        Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size(400, 200);

        let button = Button::new_with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Clicked!");
        });

        let system = sysinfo::System::new();
        let system = Rc::new(RefCell::new(system));

        let label = Label::new_with_mnemonic(Some("&cpu_usage"));
        window.add(&label);
        let label_refc = Rc::new(RefCell::new(label));

        setup_system_interval(1000, &system, &label_refc);

        println!("{:?}", system);

        // window.add(&button);

        window.show_all();
    });

    application.run(&[]);
}
