use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::BuilderExtManual;
use gtk::{
    accel_groups_activate, AccelFlags, Builder, ContainerExt, GtkMenuItemExt, GtkWindowExt, Paned,
    PanedExt, ScrolledWindow, WidgetExt, Window,
};

use std::borrow::BorrowMut;
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
        let group = gtk::AccelGroup::new();
        root.set_title("NitoriTag");

        let paned: Paned = builder.get_object("paned").unwrap();
        paned.add1(editor.get_root_widget());
        paned.add2(&scroll_container);

        root.connect_delete_event(|_, _| {
            gtk::main_quit();
            gtk::Inhibit(false)
        });

        let menu_save: gtk::MenuItem = builder.get_object("menu_save").unwrap();
        let action_bus_clone = action_bus.clone();
        menu_save.add_accelerator(
            "activate",
            &group,
            gdk::enums::key::S,
            gdk::ModifierType::CONTROL_MASK,
            AccelFlags::MASK,
        );
        //menu_save.connect_activate(move |_| action_bus_clone.into_inner().dispatch(Action::Save));
        menu_save.connect_activate(move |_| println!("nya"));

        MainWindow { root, action_bus }
    }

    pub fn show(&mut self) {
        self.root.show_all();
    }
}
