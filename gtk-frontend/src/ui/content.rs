use crate::ui::dashboard::Dashboard;
use crate::ui::logs::Logs;
use crate::ui::{Refresh, State};
use gtk::prelude::*;
use gtk::StackBuilder;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Content {
    pub stack: gtk::Stack,
    pub dashboard: Dashboard,
    pub logs: Logs,
}

impl Content {
    pub fn new(state: &Rc<RefCell<State>>) -> Content {
        let stack = StackBuilder::new()
            .transition_type(gtk::StackTransitionType::SlideLeftRight)
            .transition_duration(100)
            .build();

        stack.connect_property_visible_child_notify(|r| {
            match reqwest::blocking::get("http://localhost:8080/api/sysinfo") {
                Ok(res) => match res.json::<Vec<String>>() {
                    Ok(resp) => println!("{:?}", resp),
                    Err(_) => (),
                },
                Err(_) => (),
            };
            println!("{:?}", r.get_visible_child_name().unwrap().as_str());
        });

        let dashboard = Dashboard::new(state);
        stack.add_titled(&dashboard.container, "dashboard", "Dashboard");

        let logs = Logs::new();
        stack.add_titled(&logs.container, "logs", "Logs");

        Content {
            stack,
            dashboard,
            logs,
        }
    }
}

impl Refresh for Content {
    fn refresh(&self, system: &sysinfo::System) {
        self.dashboard.refresh(system);
    }
}
