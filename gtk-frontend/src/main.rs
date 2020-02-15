#[macro_use]
extern crate glib;

mod http;
mod ui;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow};
use std::cell::RefCell;
use std::rc::Rc;

use sysinfo::{RefreshKind, System, SystemExt};

use crate::ui::content::Content;
use crate::ui::header::Header;
use crate::ui::Refresh;

pub fn system_loop(
    refresh_time: u32,
    system: &Rc<RefCell<sysinfo::System>>,
    state: &Rc<RefCell<ui::State>>,
    content: &Rc<RefCell<Content>>,
) {
    gtk::timeout_add(
        refresh_time,
        clone!(@strong system, @strong state, @strong content => @default-return glib::Continue(true), move || {
            let content = content.borrow();
            let state = state.borrow();
            let mut system = system.borrow_mut();

            if state.logging_on && state.pc_name.len() > 2 {
                gtk::timeout_add(0, || {
                    let list = http::get_sys_list().ok();
                    println!("{:?}", list);
                    glib::Continue(false)
                });
            }

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

        let mut state = ui::State::new(system.get_processors().len());

        let header = Header::new(&mut state);
        window.set_titlebar(Some(&header.container));

        let content = Content::new(&state);
        header.stack_switch.set_stack(Some(&content.stack));
        window.add(&content.stack);

        let system = Rc::new(RefCell::new(system));
        let state = Rc::new(RefCell::new(state));
        let content = Rc::new(RefCell::new(content));
        system_loop(1500, &system, &state, &content);

        window.show_all();
    });

    application.run(&[]);
}
