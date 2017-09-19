use std::path::PathBuf;
use gtk;
use gdk;
use gtk::prelude::*;
use gtk::{TreeView, TreeViewColumn, CellRendererText, TreeStore};
use ui::Component;
use tags;
use tags::TagManager;
use url::Url;
use std::rc::Rc;
use std::cell::RefCell;
use id3::Tag;
use std::str::FromStr;

fn append_text_column(tree: &TreeView, title: &str) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();


    column.set_title(title);
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);
}

pub fn get_model() -> TreeStore {
    TreeStore::new(
        &[
            String::static_type(),
            String::static_type(),
            String::static_type(),
            String::static_type(),
            String::static_type(),
        ],
    )
}

pub struct FileList {
    root: TreeView,
    tags: Rc<RefCell<TagManager>>,
}

impl FileList {
    pub fn new() -> Self {
        let mut tags: Rc<RefCell<TagManager>> = Rc::new(RefCell::new(TagManager::new()));
        let root: TreeView = TreeView::new();
        append_text_column(&root, "Title");
        append_text_column(&root, "Artist");
        append_text_column(&root, "Album Artist");
        append_text_column(&root, "Album");
        append_text_column(&root, "Path");
        root.set_headers_visible(true);
        root.set_model(Some(&get_model()));


        let cloned_tags = tags.clone();

        root.drag_dest_set(gtk::DEST_DEFAULT_ALL, &[], gdk::ACTION_COPY);
        root.drag_dest_add_uri_targets();
        root.connect_drag_data_received(move |_, _, _, _, data, _, _| {
            let uris = data.get_uris();
            let uri = &uris[0];

            let url = Url::parse(uri);
            match url {
                Ok(v) => {
                    match v.to_file_path() {
                        Ok(path) => {
                            cloned_tags.borrow_mut().add_from_path(path);
                        }
                        Err(e) => println!("this is not a local file desu {:?}", e),
                    }
                }
                Err(e) => println!("parse error: {:?}", e),
            }

        });



        FileList { root, tags }
    }
}

impl Component<TreeView> for FileList {
    fn get_root_widget(&mut self) -> &mut TreeView {
        &mut self.root
    }
}
