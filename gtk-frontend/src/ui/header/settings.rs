use crate::ui::State;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Settings {
    pub menu_button: gtk::MenuButton,
    pub popover: gtk::Popover,
    pub logging_check: gtk::ToggleButton,
    pub pc_name: gtk::Entry,
}

impl Settings {
    pub fn new(state: &Rc<RefCell<State>>) -> Self {
        let menu_button = gtk::MenuButtonBuilder::new()
            .use_popover(true)
            .tooltip_text("Settings")
            .build();
        let popover = gtk::Popover::new(Some(&menu_button));
        menu_button.set_popover(Some(&popover));
        
        let container = gtk::BoxBuilder::new()
            .spacing(3)
            .orientation(gtk::Orientation::Vertical)
            .build();

        let logging_check = gtk::ToggleButtonBuilder::new()
            .label("Logging disabled")
            .build();

        let state_cpy = state.clone();
        logging_check.connect_toggled(move |button| {
            let mut state = state_cpy.borrow_mut();
            state.toggle_logging();
            match button.get_active() {
                true => button.set_label("Logging enabled"),
                false => button.set_label("Logging disabled")
            };
        });
        
        let field_container = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Horizontal)
            .build();
        let label = gtk::LabelBuilder::new().label("Name: ").build();
        let pc_name = gtk::EntryBuilder::new().build();
        let state_cpy = state.clone();
        pc_name.connect_activate(move |entry| {
            let mut state = state_cpy.borrow_mut();
            state.set_name(entry.get_text().unwrap().as_str());
        });

        field_container.add(&label);
        field_container.add(&pc_name);
        
        container.add(&field_container);
        container.add(&logging_check);
        
        popover.add(&container);
        popover.show_all();
        popover.hide();
        
        Self {
            menu_button,
            popover,
            logging_check,
            pc_name
        }
    }
}
