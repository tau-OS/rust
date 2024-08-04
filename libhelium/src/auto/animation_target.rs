// This file was generated by gir (https://github.com/gtk-rs/gir)
// from 
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{ffi};
use glib::{prelude::*,translate::*};

glib::wrapper! {
    #[doc(alias = "HeAnimationTarget")]
    pub struct AnimationTarget(Object<ffi::HeAnimationTarget, ffi::HeAnimationTargetClass>);

    match fn {
        type_ => || ffi::he_animation_target_get_type(),
    }
}

impl AnimationTarget {
        pub const NONE: Option<&'static AnimationTarget> = None;
    
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::AnimationTarget>> Sealed for T {}
}

pub trait AnimationTargetExt: IsA<AnimationTarget> + sealed::Sealed + 'static {
    #[doc(alias = "he_animation_target_set_value")]
    fn set_value(&self, value: f64) {
        unsafe {
            ffi::he_animation_target_set_value(self.as_ref().to_glib_none().0, value);
        }
    }
}

impl<O: IsA<AnimationTarget>> AnimationTargetExt for O {}