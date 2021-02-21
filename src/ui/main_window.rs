use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::BuilderExtManual;
use gtk::{
    accel_groups_activate, AccelFlags, Builder, ContainerExt, GtkMenuItemExt, GtkWindowExt, Paned,
    PanedExt, ScrolledWindow, WidgetExt, Window,
};

use ui::action_bus::{Action, ActionBus};
use ui::{Component, FileList, TagEditor};

pub struct MainWindow {
    root: Window,
    action_bus: Rc<RefCell<ActionBus>>,
}

impl MainWindow {
    pub fn new(
        editor: &mut TagEditor,
        list: &mut FileList,
        action_bus: Rc<RefCell<ActionBus>>,
    ) -> Self {
        let window_src = include_str!("main_window.glade");
        let builder = Builder::new_from_string(window_src);

        //scroll_container for the filelist
        let scroll_container: ScrolledWindow =
            ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        scroll_container.add(list.get_root_widget());

        let root: Window = builder.get_object("main_window").unwrap();
        root.set_title("NitoriTag");
        root.set_default_size(800, 400);


        let paned: Paned = builder.get_object("paned").unwrap();
        paned.pack1(editor.get_root_widget(), false, false);
        paned.add2(&scroll_container);

        root.connect_delete_event(|_, _| {
            gtk::main_quit();
            gtk::Inhibit(false)
        });

        let menu_save: gtk::MenuItem = builder.get_object("menu_save").unwrap();
        let mut action_bus_clone = action_bus.clone();
        menu_save.connect_activate(move |_| action_bus_clone.borrow_mut().dispatch(Action::Save));

        MainWindow { root, action_bus }
    }

    pub fn show(&mut self) {
        self.root.show_all();
    }
}
