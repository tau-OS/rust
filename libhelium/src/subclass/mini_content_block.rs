use crate::subclass::prelude::BinImpl;
use crate::MiniContentBlock;
use glib::subclass::prelude::*;

pub trait MiniContentBlockImpl: BinImpl {}

unsafe impl<T: MiniContentBlockImpl> IsSubclassable<T> for MiniContentBlock {}
