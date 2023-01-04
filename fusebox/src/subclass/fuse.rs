use crate::Fuse;
use glib::subclass::prelude::*;

pub trait FuseImpl: ObjectImpl {}

unsafe impl<T: FuseImpl> IsSubclassable<T> for Fuse {}
