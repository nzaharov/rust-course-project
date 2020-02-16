use gtk::prelude::*;

pub struct Table {
    pub container: gtk::ScrolledWindow,
    pub store: gtk::ListStore,
}

impl Table {
    pub fn new(column_names: &[&str], column_types: &[glib::Type]) -> Self {
        let store = gtk::ListStore::new(&column_types);
        let tree_view = gtk::TreeViewBuilder::new()
            .expand(true)
            .headers_visible(true)
            .enable_grid_lines(gtk::TreeViewGridLines::Vertical)
            .build();

        column_names
            .iter()
            .enumerate()
            .for_each(|(i, column)| Self::append_column(&tree_view, column, i as i32));

        tree_view.set_model(Some(&store));

        let container = gtk::ScrolledWindowBuilder::new()
            .expand(true)
            .child(&tree_view)
            .build();

        Self { container, store }
    }

    pub fn get_container(&self) -> &gtk::ScrolledWindow {
        &self.container
    }

    pub fn get_store(&self) -> &gtk::ListStore {
        &self.store
    }

    fn append_column(tree_view: &gtk::TreeView, column_name: &str, column_index: i32) {
        let column = gtk::TreeViewColumnBuilder::new().title(column_name).build(); // TODO: export column creation in method
        let cell = gtk::CellRendererText::new();
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", column_index);

        tree_view.append_column(&column);
    }
}
