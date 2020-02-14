use crate::ui::Refresh;
use gtk::prelude::*;
use gtk::{ListStore, TreeView, TreeViewBuilder, TreeViewColumn, TreeViewColumnBuilder};
use sysinfo::{ProcessExt, SystemExt};

pub struct Processes {
    pub container: gtk::ScrolledWindow,
    pub tree_model: ListStore,
}

impl Processes {
    pub fn new() -> Self {
        let column_names = [
            "PID",
            "User ID",
            "Status",
            "CPU%",
            "MEM KiB",
            "Process Name",
        ];
        let column_types = [
            u32::static_type(),
            String::static_type(),
            String::static_type(),
            String::static_type(),
            u32::static_type(),
            String::static_type(),
        ];

        let tree_model = ListStore::new(&column_types);
        let tree_view = TreeViewBuilder::new()
            .expand(true)
            .headers_visible(true)
            .enable_grid_lines(gtk::TreeViewGridLines::Vertical)
            .build();

        column_names
            .iter()
            .enumerate()
            .for_each(|(i, column)| Self::append_column(&tree_view, column, i as i32));

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

    fn append_column(tree_view: &TreeView, column_name: &str, column_index: i32) {
        let column = TreeViewColumnBuilder::new().title(column_name).build(); // TODO: export column creation in method
        let cell = gtk::CellRendererText::new();
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", column_index);

        tree_view.append_column(&column);
    }
}

impl Refresh for Processes {
    fn refresh(&self, system: &sysinfo::System) {
        let processes_list = system.get_processes();
        self.tree_model.clear();

        for (pid, process) in processes_list.iter() {
            self.tree_model.insert_with_values(
                None,
                &[0, 1, 2, 3, 4, 5],
                &[
                    pid,
                    &extract_username(process.environ()),
                    &format!("{:?}", process.status()),
                    &format!("{:.1}", process.cpu_usage()),
                    &process.memory(),
                    &process.name(),
                ],
            );
        }
    }
}

fn extract_username(environ: &[String]) -> String {
    let user = environ.get(17);
    let user = match user {
        Some(user) => user.split("=").nth(1).unwrap(),
        None => "Unknown",
    };

    String::from(user)
}
