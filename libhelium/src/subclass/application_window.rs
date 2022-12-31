use crate::ApplicationWindow;
use gtk::subclass::prelude::*;

pub trait HeApplicationWindowImpl: ApplicationWindowImpl {}

unsafe impl<T: HeApplicationWindowImpl> IsSubclassable<T> for ApplicationWindow {}
