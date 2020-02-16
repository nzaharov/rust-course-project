#[macro_use]
extern crate glib;

mod http;
mod ui;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::SystemTime;

use sysinfo::{RefreshKind, System, SystemExt};

use crate::ui::content::Content;
use crate::ui::header::Header;
use crate::ui::Refresh;

pub fn system_loop(
    refresh_time: u32,
    system: &Rc<RefCell<sysinfo::System>>,
    state: &Rc<RefCell<ui::State>>,
    content: &Rc<RefCell<Content>>,
    http_client: &Rc<RefCell<http::HttpClient>>,
) {
    gtk::timeout_add(
        refresh_time,
        clone!(@strong system, @strong state, @strong content, @strong http_client => @default-return glib::Continue(true), move || {
            let content = content.borrow();
            let state = state.borrow();
            let mut system = system.borrow_mut();
            let http_client = http_client.clone();

            system.refresh_system();
            content.refresh(&system);
            if state.logging_on && state.pc_name.len() > 2 {
                let cpu = format!("{:?}", system.get_processors());
                let memory = format!("{}KiB/{}KiB", system.get_used_memory(), system.get_total_memory());
                let pc_name = String::from(&state.pc_name);

                gtk::timeout_add(0, move || {
                    let http_client = http_client.borrow();
                    let current_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                        Ok(n) =>  n.as_secs(),
                        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
                    };
                    let res = http_client.post_sys_snapshot(&pc_name,&cpu,&memory,current_time as i64);

                    match res {
                        Ok(_) => (),
                        Err(err) => eprintln!("{}", err)
                    };
                    glib::Continue(false)
                });
            }
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

        let http_client = http::HttpClient::new();
        let system = System::new_with_specifics(
            RefreshKind::new().with_memory().with_cpu().with_processes(),
        );
        let state = ui::State::new(system.get_processors().len());
        let state = Rc::new(RefCell::new(state));
        let header = Header::new(&state);
        window.set_titlebar(Some(&header.container));
        let content = Content::new(&state);
        header.stack_switch.set_stack(Some(&content.stack));
        window.add(&content.stack);
        let system = Rc::new(RefCell::new(system));
        let content = Rc::new(RefCell::new(content));
        let http_client = Rc::new(RefCell::new(http_client));
        system_loop(1500, &system, &state, &content, &http_client);

        window.show_all();
    });

    application.run(&[]);
}
