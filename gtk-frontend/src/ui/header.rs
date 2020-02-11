use gtk::prelude::*;
use gtk::{HeaderBarBuilder, StackSwitcherBuilder};

pub struct Header {
    pub container: gtk::HeaderBar,
    pub stack_switch: gtk::StackSwitcher,
}

impl Header {
    pub fn new() -> Header {
        let container = HeaderBarBuilder::new().show_close_button(true).build();

        let stack_switch = StackSwitcherBuilder::new().build();

        container.set_custom_title(Some(&stack_switch));

        Header {
            container,
            stack_switch,
        }
    }
}
