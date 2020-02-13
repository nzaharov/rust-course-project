use crate::ui::Refresh;
use gtk::prelude::*;
use gtk::{ListStore, TreeView, TreeViewBuilder, TreeViewColumn, TreeViewColumnBuilder};

pub struct Processes {
    pub container: gtk::ScrolledWindow,
    pub tree_model: ListStore,
}

impl Processes {
    pub fn new() -> Self {
        let tree_model = ListStore::new(&[u32::static_type(), String::static_type()]);
        let tree_view = TreeViewBuilder::new()
            .expand(true)
            .headers_visible(true)
            .enable_grid_lines(gtk::TreeViewGridLines::Vertical)
            .build();

        let column_pid = TreeViewColumnBuilder::new().title("PID").build(); // TODO: export column creation in method
        let cell1 = gtk::CellRendererText::new();
        column_pid.pack_start(&cell1, true);
        column_pid.add_attribute(&cell1, "text", 0);

        let column_name = TreeViewColumnBuilder::new().title("Command").build();
        let cell2 = gtk::CellRendererText::new();
        column_name.pack_start(&cell2, true);
        column_name.add_attribute(&cell2, "text", 1);

        tree_view.append_column(&column_pid);
        tree_view.append_column(&column_name);
        tree_view.set_model(Some(&tree_model));

        let container = gtk::ScrolledWindowBuilder::new()
            .expand(true)
            .child(&tree_view)
            .build();

        Self {
            container,
            tree_model,
        }
    }
}

impl Refresh for Processes {
    fn refresh(&self, system: &sysinfo::System) {
        let entries = &[
            "Michel",
            "Sara",
            "Liam",
            "Zelda",
            "Neo",
            "Octopus master",
            "Michel",
            "Sara",
            "Liam",
            "Zelda",
            "Neo",
            "Octopus master",
            "Michel",
            "Sara",
            "Liam",
            "Zelda",
            "Neo",
            "Octopus master",
            "Michel",
            "Sara",
            "Liam",
            "Zelda",
            "Neo",
            "Octopus master",
            "Michel",
            "Sara",
            "Liam",
            "Zelda",
            "Neo",
            "Octopus master",
            "Michel",
            "Sara",
            "Liam",
            "Zelda",
            "Neo",
            "Octopus master",
        ];
        self.tree_model.clear();

        for (i, entry) in entries.iter().enumerate() {
            self.tree_model
                .insert_with_values(None, &[0, 1], &[&(i as u32 + 1), &entry]);
        }
    }
}
