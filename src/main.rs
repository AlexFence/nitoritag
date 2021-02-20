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

    let mut tageditor = ui::TagEditor::new();
    let mut filelist = ui::FileList::new();
    let mut main_window = ui::MainWindow::new(&mut tageditor, &mut filelist);

    main_window.show();
    gtk::main();
}
