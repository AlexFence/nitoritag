use std::path::PathBuf;
use tags::{write_tags, Tag};
use ui::EditorComponent;

/// The actionBus exists to decouple the different ui components from each other.
/// As an extra benefit it also makes replaces gtk with something else, a bit easier.
/// It also stores some common state.
pub struct ActionBus {
    selected_paths: Vec<PathBuf>,
    editor: Option<Box<EditorComponent>>,
}

pub enum Action {
    SetTagsToEdit(Vec<(PathBuf, Tag)>),
    WriteTags(Tag),
    Save,
}

impl ActionBus {
    pub fn new() -> Self {
        ActionBus {
            selected_paths: Vec::new(),
            editor: None,
        }
    }

    pub fn set_editor(&mut self, editor: Box<EditorComponent>) {
        self.editor = Some(editor)
    }

    pub fn dispatch(&mut self, action: Action) {
        match action {
            Action::SetTagsToEdit(tag_pairs) => {
                println!("SetTagsToEdit dispatched!");
                self.selected_paths = tag_pairs.iter().map(|(path, _)| path.clone()).collect();

                match &mut self.editor {
                    Some(editor) => editor
                        .as_mut()
                        .set_tags_to_edit(tag_pairs.iter().map(|(_, tag)| tag.clone()).collect()),
                    None => {
                        eprintln!("no EditorComponent set")
                    }
                }
            }
            Action::WriteTags(new_values) => {
                println!("WriteTags dispatched!");
                write_tags(&new_values, &self.selected_paths)
            }
            Action::Save => {
                println!("Save dispatched!");
                if self.selected_paths.len() > 0 {
                    match &mut self.editor {
                        Some(editor) => {
                            let new_values = editor.get_new_values();
                            self.dispatch(Action::WriteTags(new_values))
                        }
                        None => {
                            eprintln!("no EditorComponent set")
                        }
                    }
                }
            }
        }
    }
}
