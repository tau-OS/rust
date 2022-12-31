use crate::Application;

use gtk::subclass::prelude::*;

pub trait HeApplicationImpl: GtkApplicationImpl {}

unsafe impl<T: HeApplicationImpl> IsSubclassable<T> for Application {}
