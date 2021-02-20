mod action_bus;
mod tag_editor;
mod file_list;
mod main_window;

use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::{Borrow, BorrowMut};

use gtk;
use gtk::{Builder, Window, Paned, PanedExt, Widget, ScrolledWindow, ContainerExt, WidgetExt, GtkWindowExt};
use gtk::prelude::BuilderExtManual;
use glib::object::IsA;

use tags::Tag;
pub use ui::tag_editor::TagEditor;
pub use ui::file_list::FileList;
use ui::main_window::MainWindow;
use ui::action_bus::ActionBus;

pub trait Component<T>
where
    T: IsA<Widget>,
{
    fn get_root_widget(&mut self) -> &mut T;
}

trait EditorComponent {
    fn set_tags_to_edit(&mut self, tags: Vec<Tag>);
}

pub fn main () {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let mut action_bus = ActionBus::new();
    let mut action_bus_rc = Rc::new(RefCell::new(action_bus));

    let mut tag_editor = TagEditor::new(action_bus_rc.clone());
    let mut file_list = FileList::new(action_bus_rc.clone());
    let mut main_window = MainWindow::new(&mut tag_editor, &mut file_list);

    &action_bus_rc.as_ref().borrow_mut().set_editor(Box::new(tag_editor));

    main_window.show();
    gtk::main();
}
