use gtk::TreeView;
use ui::Component;

pub struct FileList {
    root: TreeView,
}

impl FileList {
    pub fn new() -> Self {
        let root: TreeView = TreeView::new();
        FileList { root }
    }
}

impl Component<TreeView> for FileList {
    fn get_root_widget(&mut self) -> &mut TreeView {
        &mut self.root
    }
}
