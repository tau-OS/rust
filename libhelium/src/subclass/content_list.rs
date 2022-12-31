use crate::subclass::prelude::BinImpl;
use crate::ContentList;
use glib::subclass::prelude::*;

pub trait ContentListImpl: BinImpl {}

unsafe impl<T: ContentListImpl> IsSubclassable<T> for ContentList {}
