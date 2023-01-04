#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(clippy::needless_doctest_main)]

// Re-export the -sys bindings
pub use ffi;
pub use gdk;
pub use gio;
pub use glib;
pub use gtk;

macro_rules! assert_initialized_main_thread {
    () => {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("fusebox may only be used from the main thread.");
            } else {
                panic!("Gtk has to be initialized before using fusebox.");
            }
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(unused_imports)]
#[allow(clippy::let_and_return)]
#[allow(clippy::type_complexity)]
mod auto;

pub use auto::*;

pub mod prelude;
pub mod subclass;
