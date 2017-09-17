mod tageditor;

use gtk;
use gtk::{Builder, Window};
use gtk::WidgetExt;

pub struct MainWindow {
    root: Window,
}

impl MainWindow {
    pub fn new() -> Self {
        let window_src = include_str!("main_window.glade");
        let builder = Builder::new_from_string(window_src);

        let root: Window = builder.get_object("main_window").unwrap();

        root.connect_delete_event(|_, _| {
                                      gtk::main_quit();
                                      gtk::Inhibit(false)
                                  });

        MainWindow { root }
    }

    pub fn show(&mut self) {
        self.root.show_all();
    }
}
