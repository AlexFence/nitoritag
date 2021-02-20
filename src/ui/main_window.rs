use ui::{FileList, TagEditor, Component};
use gtk::{Window, Builder, ScrolledWindow, ContainerExt, GtkWindowExt, Paned, WidgetExt, PanedExt};
use gtk::prelude::BuilderExtManual;

pub struct MainWindow {
    root: Window,
}

impl MainWindow {
    pub fn new(editor: &mut TagEditor, list: &mut FileList) -> Self {
        let window_src = include_str!("main_window.glade");
        let builder = Builder::new_from_string(window_src);


        //scroll_container for the filelist
        let scroll_container: ScrolledWindow = ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scroll_container.add(list.get_root_widget());

        let root: Window = builder.get_object("main_window").unwrap();
        root.set_title("NitoriTag");

        let paned: Paned = builder.get_object("paned").unwrap();
        paned.add1(editor.get_root_widget());
        paned.add2(&scroll_container);

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
