pub mod bin;

pub mod prelude {
    pub use super::bin::BinImpl;
    pub use gio::subclass::prelude::*;
    pub use glib::subclass::prelude::*;
}
