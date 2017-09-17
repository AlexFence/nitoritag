use gtk;
use gdk;
use gtk::prelude::*;
use gtk::TreeView;
use ui::Component;

pub struct FileList {
    root: TreeView,
}

impl FileList {
    pub fn new() -> Self {
        let root: TreeView = TreeView::new();

        root.drag_dest_set(gtk::DEST_DEFAULT_ALL, &[], gdk::ACTION_COPY);
        root.drag_dest_add_uri_targets();
        root.connect_drag_data_received(move |_, _, _, _, data, _, _| {
            let uris = data.get_uris();
            let uri = &uris[0];
            if uri.starts_with("file://") {
                println!("this is a file :3");
            } else {
                println!("this is a shitty url :v");
            };
        });

        FileList { root }
    }
}

impl Component<TreeView> for FileList {
    fn get_root_widget(&mut self) -> &mut TreeView {
        &mut self.root
    }
}
