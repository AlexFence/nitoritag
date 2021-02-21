use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;

use gdk;
use gtk;
use gtk::prelude::*;
use gtk::{TreeIter, TreeModel};
use gtk::{CellRendererText, ListStore, SelectionMode, TreeView, TreeViewColumn};
use url::Url;

use tags::Tag;
use tags::TagIndex;
use ui::action_bus::{Action, ActionBus};
use ui::Component;
use gdk::WindowExt;

struct FileListRow {
    pub tag: Tag,
    pub path: PathBuf,
}

pub struct FileList {
    root: TreeView,
    tags: Rc<RefCell<TagIndex>>,
    action_bus: Rc<RefCell<ActionBus>>,
}

impl FileList {
    pub fn new(action_bus: Rc<RefCell<ActionBus>>) -> Self {
        let tags: Rc<RefCell<TagIndex>> = Rc::new(RefCell::new(TagIndex::new()));
        let root: TreeView = TreeView::new();
        Self::append_text_column(&root, "Title", 0);
        Self::append_text_column(&root, "Artist", 1);
        Self::append_text_column(&root, "Album Artist", 2);
        Self::append_text_column(&root, "Album", 3);
        Self::append_text_column(&root, "Path", 4);
        root.set_headers_visible(true);
        root.set_model(Some(&Self::get_empty_model()));

        let cloned_tags = tags.clone();

        // TODO ogg crashes
        root.drag_dest_set(gtk::DestDefaults::ALL, &[], gdk::DragAction::MOVE | gdk::DragAction::COPY);
        root.drag_dest_add_uri_targets();
        // TODO ensure that MOVE is the default action
        // I feel like this isn't possible with the bindings yet, as no callback params are mutable
        // https://valadoc.org/gtk+-3.0/Gtk.drag_dest_set.html
        root.connect_drag_data_received(move |w, ctx, _, _, data, _, _| {
            let action = ctx.get_selected_action();

            println!("{:?}", action);

            let uris = data.get_uris();
            for uri in uris {
                let url = Url::parse(&uri);
                match url {
                    Ok(v) => {
                        match v.to_file_path() {
                            Ok(path) => {
                                match action {
                                    gdk::DragAction::MOVE  => {
                                        let result = cloned_tags.borrow_mut().add_from_path(&path);
                                        if result.is_ok() {
                                            Self::update_table(w, cloned_tags.clone());
                                        }
                                    },
                                    gdk::DragAction::COPY => {
                                        cloned_tags.borrow_mut().clear();
                                        cloned_tags.borrow_mut().add_from_path(&path);
                                        Self::update_table(w, cloned_tags.clone());
                                    },
                                    _ => eprintln!("unsupported DragActon")
                                };
                            }
                            Err(e) => eprintln!("this is not a local file desu {:?}", e),
                        }
                    }
                    Err(e) => eprintln!("parse error: {:?}", e),
                }
            }
        });

        // TODO don't just print out the title, pass the tag to the editor some how
        //      not sure how to do it, all options seem wrong
        let cloned_tags2 = tags.clone();
        let action_bus_clone = action_bus.clone();
        let selection = root.get_selection();
        selection.set_mode(SelectionMode::Multiple);
        selection.connect_changed(move |tree_selection| {
            // get tree paths and model and shit to tired
            let (paths, model) = tree_selection.get_selected_rows();
            // convert the paths to iters
            let mut iters: Vec<TreeIter> = Vec::new();
            for path in paths {
                iters.push(model.get_iter(&path).unwrap());
            }

            let mut selected_tags: Vec<(PathBuf, Tag)> = Vec::new();

            for iter in iters {
                // TODO clean up
                let path_string = model.get_value(&iter, 4).get::<String>().unwrap();
                let path = PathBuf::from(path_string.unwrap());
                let tag = cloned_tags2.borrow_mut().clone().get(&path).unwrap();
                //println!("{}", tag.title().unwrap());
                selected_tags.push((path, tag));
            }
            action_bus_clone.borrow_mut().dispatch(Action::SetTagsToEdit(selected_tags));
        });

        FileList {
            root,
            tags,
            action_bus,
        }
    }

    fn append_text_column(tree: &TreeView, title: &str, position: i32) {
        let column = TreeViewColumn::new();
        let cell = CellRendererText::new();

        column.set_title(title);
        column.set_resizable(true);
        column.pack_start(&cell, true);

        //position is needed so the cell is linked to the column
        column.add_attribute(&cell, "text", position);
        tree.append_column(&column);
    }

    fn get_empty_model() -> ListStore {
        ListStore::new(&[
            String::static_type(),
            String::static_type(),
            String::static_type(),
            String::static_type(),
            String::static_type(),
        ])
    }

    //TODO Try to improve this mess ;w;
    fn update_table(table: &TreeView, tags: Rc<RefCell<TagIndex>>) {
        let model: ListStore = Self::get_empty_model();
        //what even is this?
        let cloned_tags1 = tags.clone();
        let cloned_and_borrowed_tags1 = cloned_tags1.borrow();
        let borrowed_tags = tags.borrow();

        let paths = cloned_and_borrowed_tags1.get_index();
        for path in paths {
            let tag = borrowed_tags.clone().get(&path.to_path_buf()).unwrap();
            let row = FileListRow {
                tag,
                path: path.clone(),
            };
            Self::display_row(&model, row)
        }

        table.set_model(Some(&model));
    }

    fn display_row(model: &ListStore, row: FileListRow) {
        // tag needs to be cloned because it isn't a reference
        // is there a better way for doing this?
        let title = row.tag.clone().title().unwrap_or(String::new()).to_string();
        let artist = row
            .tag
            .clone()
            .artist()
            .unwrap_or(String::new())
            .to_string();
        let album_artist = row
            .tag
            .clone()
            .album_artist()
            .unwrap_or(String::new())
            .to_string();
        let album = row.tag.clone().album().unwrap_or(String::new()).to_string();
        let path = match row.path.clone().into_os_string().into_string() {
            Ok(v) => v,
            Err(e) => String::from_str("TwT").unwrap(),
        };

        let iter = model.append();
        model.set(
            &iter,
            &[0, 1, 2, 3, 4],
            &[&title, &artist, &album_artist, &album, &path],
        );
    }
}

impl Component<TreeView> for FileList {
    fn get_root_widget(&mut self) -> &mut TreeView {
        &mut self.root
    }
}
