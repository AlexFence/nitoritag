use gtk::{Builder, Box};
use gtk::prelude::BuilderExtManual;

use ui::{Component, EditorComponent};
use tags::Tag;
use ui::action_bus::ActionBus;
use std::rc::Rc;
use std::cell::RefCell;

pub struct TagEditor {
    root: Box,
    tags: Vec<Tag>,
    action_bus: Rc<RefCell<ActionBus>>
}

impl TagEditor {
    pub fn new(action_bus: Rc<RefCell<ActionBus>>) -> Self {
        let window_src = include_str!("tag_editor.glade");
        let builder = Builder::new_from_string(window_src);

        let root: Box = builder.get_object("tag_editor").unwrap();
        let tags: Vec<Tag> = Vec::new();
        Self { root, tags, action_bus}
    }
}

impl Component<Box> for TagEditor {
    fn get_root_widget(&mut self) -> &mut Box {
        &mut self.root
    }
}

impl EditorComponent for TagEditor {
    fn set_tags_to_edit(&mut self, tags: Vec<Tag>) {
        println!("Received tags:");
        for t in tags.clone() {
            println!("{:?}", t);
        }
        self.tags = tags;
    }
}
