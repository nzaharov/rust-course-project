use gtk::prelude::*;
use gtk::{HeaderBarBuilder, StackSwitcherBuilder};
mod settings;
use crate::ui::State;
use settings::Settings;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Header {
    pub container: gtk::HeaderBar,
    pub stack_switch: gtk::StackSwitcher,
    pub settings: Settings,
}

impl Header {
    pub fn new(state: &Rc<RefCell<State>>) -> Header {
        let container = HeaderBarBuilder::new().show_close_button(true).build();

        let stack_switch = StackSwitcherBuilder::new().build();
        container.set_custom_title(Some(&stack_switch));

        let settings = Settings::new(state);
        container.pack_end(&settings.menu_button);

        Header {
            container,
            stack_switch,
            settings,
        }
    }
}
