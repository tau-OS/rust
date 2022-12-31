use crate::Window;
use gtk::subclass::prelude::*;

pub trait HeWindowImpl: WindowImpl {}

unsafe impl<T: HeWindowImpl> IsSubclassable<T> for Window {}
