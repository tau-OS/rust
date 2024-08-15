#[doc(hidden)]
use glib::translate::Uninitialized;
#[doc(hidden)]
pub use gtk::prelude::*;
#[doc(hidden)]
use std::mem;

pub use crate::auto::traits::*;

#[doc(hidden)]
impl Uninitialized for crate::RGBColor {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::uninitialized()
    }
}
