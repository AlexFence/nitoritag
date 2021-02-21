extern crate gdk;
extern crate glib;
extern crate gtk;
extern crate id3;
extern crate metaflac;
extern crate taglib;
extern crate url;

mod tags;
mod ui;

fn main() {
    ui::main()
}
