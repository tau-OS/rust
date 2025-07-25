// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{ffi, AvatarStatusColor};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "HeAvatar")]
    pub struct Avatar(Object<ffi::HeAvatar, ffi::HeAvatarClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::he_avatar_get_type(),
    }
}

impl Avatar {
    pub const NONE: Option<&'static Avatar> = None;

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Avatar`] objects.
    ///
    /// This method returns an instance of [`AvatarBuilder`](crate::builders::AvatarBuilder) which can be used to create [`Avatar`] objects.
    pub fn builder() -> AvatarBuilder {
        AvatarBuilder::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Avatar`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct AvatarBuilder {
    builder: glib::object::ObjectBuilder<'static, Avatar>,
}

impl AvatarBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn size(self, size: i32) -> Self {
        Self {
            builder: self.builder.property("size", size),
        }
    }

    pub fn text(self, text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("text", text.into()),
        }
    }

    pub fn status(self, status: bool) -> Self {
        Self {
            builder: self.builder.property("status", status),
        }
    }

    pub fn status_color(self, status_color: AvatarStatusColor) -> Self {
        Self {
            builder: self.builder.property("status-color", status_color),
        }
    }

    pub fn image(self, image: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("image", image.into()),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    //pub fn cursor(self, cursor: /*Ignored*/&gdk::Cursor) -> Self {
    //    Self { builder: self.builder.property("cursor", cursor), }
    //}

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: gtk::Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    //pub fn layout_manager(self, layout_manager: &impl IsA</*Ignored*/gtk::LayoutManager>) -> Self {
    //    Self { builder: self.builder.property("layout-manager", layout_manager.clone().upcast()), }
    //}

    #[cfg(feature = "gtk_v4_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v4_18")))]
    pub fn limit_events(self, limit_events: bool) -> Self {
        Self {
            builder: self.builder.property("limit-events", limit_events),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    //pub fn overflow(self, overflow: /*Ignored*/gtk::Overflow) -> Self {
    //    Self { builder: self.builder.property("overflow", overflow), }
    //}

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: gtk::Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    //pub fn accessible_role(self, accessible_role: /*Ignored*/gtk::AccessibleRole) -> Self {
    //    Self { builder: self.builder.property("accessible-role", accessible_role), }
    //}

    // rustdoc-stripper-ignore-next
    /// Build the [`Avatar`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Avatar {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}

pub trait AvatarExt: IsA<Avatar> + 'static {
    #[doc(alias = "he_avatar_get_size")]
    #[doc(alias = "get_size")]
    fn size(&self) -> i32 {
        unsafe { ffi::he_avatar_get_size(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "he_avatar_set_size")]
    fn set_size(&self, value: i32) {
        unsafe {
            ffi::he_avatar_set_size(self.as_ref().to_glib_none().0, value);
        }
    }

    #[doc(alias = "he_avatar_get_text")]
    #[doc(alias = "get_text")]
    fn text(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::he_avatar_get_text(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "he_avatar_set_text")]
    fn set_text(&self, value: Option<&str>) {
        unsafe {
            ffi::he_avatar_set_text(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "he_avatar_get_status")]
    #[doc(alias = "get_status")]
    fn is_status(&self) -> bool {
        unsafe { from_glib(ffi::he_avatar_get_status(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "he_avatar_set_status")]
    fn set_status(&self, value: bool) {
        unsafe {
            ffi::he_avatar_set_status(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "he_avatar_get_status_color")]
    #[doc(alias = "get_status_color")]
    fn status_color(&self) -> AvatarStatusColor {
        unsafe {
            from_glib(ffi::he_avatar_get_status_color(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_avatar_set_status_color")]
    fn set_status_color(&self, value: AvatarStatusColor) {
        unsafe {
            ffi::he_avatar_set_status_color(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "he_avatar_get_image")]
    #[doc(alias = "get_image")]
    fn image(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::he_avatar_get_image(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "he_avatar_set_image")]
    fn set_image(&self, value: Option<&str>) {
        unsafe {
            ffi::he_avatar_set_image(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "size")]
    fn connect_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_size_trampoline<P: IsA<Avatar>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeAvatar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Avatar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::size".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "text")]
    fn connect_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<P: IsA<Avatar>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeAvatar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Avatar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::text".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "status")]
    fn connect_status_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_status_trampoline<P: IsA<Avatar>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeAvatar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Avatar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::status".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_status_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "status-color")]
    fn connect_status_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_status_color_trampoline<P: IsA<Avatar>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeAvatar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Avatar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::status-color".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_status_color_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "image")]
    fn connect_image_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_image_trampoline<P: IsA<Avatar>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeAvatar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Avatar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::image".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_image_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Avatar>> AvatarExt for O {}
