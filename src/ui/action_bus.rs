use tags::Tag;
use ui::EditorComponent;
use std::rc::Rc;

/// The actionBus exists to decouple the different ui components from each other.
/// As an extra beinfit it also makes replaces gtk with something else, a bit easier.
pub struct ActionBus {
    editor: Option<Box<EditorComponent>>
}

pub enum Action {
    SetTagsToEdit(Vec<Tag>)
}

impl ActionBus {
    pub fn new() -> Self {
        ActionBus {
            editor: None
        }
    }

    pub fn set_editor(&mut self, editor: Box<EditorComponent>) {
        self.editor = Some(editor)
    }

    pub fn dispatch(&mut self, action: Action) {
       match action {
            Action::SetTagsToEdit(tags) => {
                println!("SetTagsToEdit dispatched!");
                match &mut self.editor {
                    Some(editor) => editor.as_mut().set_tags_to_edit(tags),
                    None => {
                        eprintln!("no EditorComponent set")
                    }
                }
            }
       }
    }
}
