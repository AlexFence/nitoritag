mod action_bus;
mod file_list;
mod main_window;
mod tag_editor;

use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;

use glib::object::IsA;
use gtk;
use gtk::prelude::BuilderExtManual;
use gtk::{
    Builder, ContainerExt, GtkWindowExt, Paned, PanedExt, ScrolledWindow, Widget, WidgetExt, Window,
};

use tags::Tag;
use ui::action_bus::ActionBus;
pub use ui::file_list::FileList;
use ui::main_window::MainWindow;
pub use ui::tag_editor::TagEditor;

pub trait Component<T>
where
    T: IsA<Widget>,
{
    fn get_root_widget(&mut self) -> &mut T;
}

trait EditorComponent {
    fn set_tags_to_edit(&mut self, tags: Vec<Tag>);
    fn get_new_values(&self) -> Tag;
}

pub fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let mut action_bus = ActionBus::new();
    let mut action_bus_rc = Rc::new(RefCell::new(action_bus));

    let mut tag_editor = TagEditor::new(action_bus_rc.clone());
    let mut file_list = FileList::new(action_bus_rc.clone());
    let mut main_window = MainWindow::new(&mut tag_editor, &mut file_list, action_bus_rc.clone());

    &action_bus_rc
        .as_ref()
        .borrow_mut()
        .set_editor(Box::new(tag_editor));

    main_window.show();
    gtk::main();
}
