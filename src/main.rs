extern crate gtk;
extern crate gdk;
extern crate id3;
extern crate url;

mod ui;
mod tags;

use gtk::prelude::*;

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
