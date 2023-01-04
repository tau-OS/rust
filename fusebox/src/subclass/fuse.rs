use crate::{Fuse, prelude::FuseExt};
use glib::subclass::prelude::*;

pub trait FuseImpl: FuseExt + ObjectImpl {}

unsafe impl<T: FuseImpl> IsSubclassable<T> for Fuse {}
