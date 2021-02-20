extern crate gtk;
extern crate gdk;
extern crate glib;
extern crate id3;
extern crate metaflac;
extern crate taglib;
extern crate url;

use std::rc::Rc;
use std::cell::RefCell;

mod ui;
mod tags;


// TODO add some decent logging
fn main() {
   ui::main()
}
