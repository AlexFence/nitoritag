use gtk::{Builder, Box};
use gtk::WidgetExt;

pub struct TagEditor {
    root: Box,
}

impl TagEditor {
    pub fn new() -> Self {
        let window_src = include_str!("tag_editor.glade");
        let builder = Builder::new_from_string(window_src);

        let root: Box = builder.get_object("tag_editor").unwrap();
        Self { root }
    }

    pub fn show(&mut self) {
        self.root.show_all();
    }
}
