extern crate gtk;

mod ui;

use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let mut main_window = ui::MainWindow::new();
    main_window.show();
    gtk::main();
}
