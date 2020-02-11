use crate::ui::dashboard::Dashboard;
use crate::ui::logs::Logs;
use gtk::prelude::*;
use gtk::StackBuilder;

pub struct Content {
    pub stack: gtk::Stack,
    pub dashboard: Dashboard,
    pub logs: Logs,
}

impl Content {
    pub fn new() -> Content {
        let stack = StackBuilder::new()
            .transition_type(gtk::StackTransitionType::SlideLeftRight)
            .transition_duration(100)
            .build();

        let dashboard = Dashboard::new();
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
