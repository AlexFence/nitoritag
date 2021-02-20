extern crate gtk;
extern crate gdk;
extern crate glib;
extern crate id3;
extern crate metaflac;
extern crate taglib;
extern crate url;

mod ui;
mod tags;


// TODO add some decent logging
fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let mut tag_editor = ui::TagEditor::new();
    let mut file_list = ui::FileList::new();
    let mut main_window = ui::MainWindow::new(&mut tag_editor, &mut file_list);

    main_window.show();
    gtk::main();
}
