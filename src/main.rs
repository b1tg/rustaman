#![feature(proc_macro)]

extern crate gtk;

#[macro_use]
extern crate relm;
extern crate relm_attributes;
#[macro_use]
extern crate relm_derive;

mod ui;

use relm::Widget;
use ui::window::Window;

fn main() {
    Window::run(()).unwrap();
}
