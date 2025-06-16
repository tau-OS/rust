use crate::subclass::prelude::BinImpl;
use crate::Card;
use glib::subclass::prelude::*;

pub trait CardImpl: BinImpl {}

unsafe impl<T: CardImpl> IsSubclassable<T> for Card {}
