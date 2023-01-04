use crate::Fuse;
use glib::subclass::prelude::*;

pub trait FuseImpl: ObjectImpl {
  fn code_name(&self) -> String;
  fn display_name(&self) -> String;
  fn description(&self) -> String;
  fn icon(&self) -> String;
}

unsafe impl<T: FuseImpl> IsSubclassable<T> for Fuse {}
