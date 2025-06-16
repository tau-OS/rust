use crate::subclass::prelude::CardImpl;
use crate::MiniContentBlock;
use glib::subclass::prelude::*;

pub trait MiniContentBlockImpl: CardImpl {}

unsafe impl<T: MiniContentBlockImpl> IsSubclassable<T> for MiniContentBlock {}
