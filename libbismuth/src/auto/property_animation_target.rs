// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{ffi, AnimationTarget};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "BisPropertyAnimationTarget")]
    pub struct PropertyAnimationTarget(Object<ffi::BisPropertyAnimationTarget, ffi::BisPropertyAnimationTargetClass>) @extends AnimationTarget;

    match fn {
        type_ => || ffi::bis_property_animation_target_get_type(),
    }
}

impl PropertyAnimationTarget {
    #[doc(alias = "bis_property_animation_target_new")]
    pub fn new(object: &impl IsA<glib::Object>, property_name: &str) -> PropertyAnimationTarget {
        assert_initialized_main_thread!();
        unsafe {
            AnimationTarget::from_glib_full(ffi::bis_property_animation_target_new(
                object.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "bis_property_animation_target_new_for_pspec")]
    #[doc(alias = "new_for_pspec")]
    pub fn for_pspec(
        object: &impl IsA<glib::Object>,
        pspec: impl AsRef<glib::ParamSpec>,
    ) -> PropertyAnimationTarget {
        assert_initialized_main_thread!();
        unsafe {
            AnimationTarget::from_glib_full(ffi::bis_property_animation_target_new_for_pspec(
                object.as_ref().to_glib_none().0,
                pspec.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`PropertyAnimationTarget`] objects.
    ///
    /// This method returns an instance of [`PropertyAnimationTargetBuilder`](crate::builders::PropertyAnimationTargetBuilder) which can be used to create [`PropertyAnimationTarget`] objects.
    pub fn builder() -> PropertyAnimationTargetBuilder {
        PropertyAnimationTargetBuilder::new()
    }

    #[doc(alias = "bis_property_animation_target_get_object")]
    #[doc(alias = "get_object")]
    pub fn object(&self) -> glib::Object {
        unsafe {
            from_glib_none(ffi::bis_property_animation_target_get_object(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "bis_property_animation_target_get_pspec")]
    #[doc(alias = "get_pspec")]
    pub fn pspec(&self) -> glib::ParamSpec {
        unsafe {
            from_glib_none(ffi::bis_property_animation_target_get_pspec(
                self.to_glib_none().0,
            ))
        }
    }
}

#[cfg(feature = "v1_2")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
impl Default for PropertyAnimationTarget {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`PropertyAnimationTarget`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PropertyAnimationTargetBuilder {
    builder: glib::object::ObjectBuilder<'static, PropertyAnimationTarget>,
}

impl PropertyAnimationTargetBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    pub fn object(self, object: &impl IsA<glib::Object>) -> Self {
        Self {
            builder: self.builder.property("object", object.clone().upcast()),
        }
    }

    #[cfg(feature = "v1_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    pub fn pspec(self, pspec: impl AsRef<glib::ParamSpec>) -> Self {
        Self {
            builder: self.builder.property("pspec", pspec.as_ref().clone()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`PropertyAnimationTarget`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> PropertyAnimationTarget {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
