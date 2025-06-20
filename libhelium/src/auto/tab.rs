// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{ffi, Bin};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "HeTab")]
    pub struct Tab(Object<ffi::HeTab, ffi::HeTabClass>) @extends Bin, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::he_tab_get_type(),
    }
}

impl Tab {
    pub const NONE: Option<&'static Tab> = None;

    #[doc(alias = "he_tab_new")]
    pub fn new(label: Option<&str>, page: Option<&impl IsA<gtk::Widget>>) -> Tab {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::he_tab_new(
                label.to_glib_none().0,
                page.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Tab`] objects.
    ///
    /// This method returns an instance of [`TabBuilder`](crate::builders::TabBuilder) which can be used to create [`Tab`] objects.
    pub fn builder() -> TabBuilder {
        TabBuilder::new()
    }
}

impl Default for Tab {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Tab`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct TabBuilder {
    builder: glib::object::ObjectBuilder<'static, Tab>,
}

impl TabBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn label(self, label: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("label", label.into()),
        }
    }

    pub fn tooltip(self, tooltip: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip", tooltip.into()),
        }
    }

    pub fn pinned(self, pinned: bool) -> Self {
        Self {
            builder: self.builder.property("pinned", pinned),
        }
    }

    pub fn can_pin(self, can_pin: bool) -> Self {
        Self {
            builder: self.builder.property("can-pin", can_pin),
        }
    }

    pub fn can_close(self, can_close: bool) -> Self {
        Self {
            builder: self.builder.property("can-close", can_close),
        }
    }

    pub fn page(self, page: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("page", page.clone().upcast()),
        }
    }

    pub fn menu(self, menu: &gio::Menu) -> Self {
        Self {
            builder: self.builder.property("menu", menu.clone()),
        }
    }

    pub fn actions(self, actions: &impl IsA<gio::SimpleActionGroup>) -> Self {
        Self {
            builder: self.builder.property("actions", actions.clone().upcast()),
        }
    }

    pub fn child(self, child: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
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
    /// Build the [`Tab`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Tab {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}

pub trait TabExt: IsA<Tab> + 'static {
    #[doc(alias = "he_tab_get_label")]
    #[doc(alias = "get_label")]
    fn label(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::he_tab_get_label(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "he_tab_set_label")]
    fn set_label(&self, value: &str) {
        unsafe {
            ffi::he_tab_set_label(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "he_tab_set_tooltip")]
    fn set_tooltip(&self, value: &str) {
        unsafe {
            ffi::he_tab_set_tooltip(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "he_tab_get_pinned")]
    #[doc(alias = "get_pinned")]
    fn is_pinned(&self) -> bool {
        unsafe { from_glib(ffi::he_tab_get_pinned(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "he_tab_set_pinned")]
    fn set_pinned(&self, value: bool) {
        unsafe {
            ffi::he_tab_set_pinned(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "he_tab_get_can_pin")]
    #[doc(alias = "get_can_pin")]
    fn can_pin(&self) -> bool {
        unsafe { from_glib(ffi::he_tab_get_can_pin(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "he_tab_set_can_pin")]
    fn set_can_pin(&self, value: bool) {
        unsafe {
            ffi::he_tab_set_can_pin(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "he_tab_get_can_close")]
    #[doc(alias = "get_can_close")]
    fn can_close(&self) -> bool {
        unsafe { from_glib(ffi::he_tab_get_can_close(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "he_tab_set_can_close")]
    fn set_can_close(&self, value: bool) {
        unsafe {
            ffi::he_tab_set_can_close(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "he_tab_get_page")]
    #[doc(alias = "get_page")]
    fn page(&self) -> gtk::Widget {
        unsafe { from_glib_none(ffi::he_tab_get_page(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "he_tab_set_page")]
    fn set_page(&self, value: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::he_tab_set_page(
                self.as_ref().to_glib_none().0,
                value.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "he_tab_get_menu")]
    #[doc(alias = "get_menu")]
    fn menu(&self) -> gio::Menu {
        unsafe { from_glib_none(ffi::he_tab_get_menu(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "he_tab_get_actions")]
    #[doc(alias = "get_actions")]
    fn actions(&self) -> gio::SimpleActionGroup {
        unsafe { from_glib_none(ffi::he_tab_get_actions(self.as_ref().to_glib_none().0)) }
    }

    fn set_menu(&self, menu: Option<&gio::Menu>) {
        ObjectExt::set_property(self.as_ref(), "menu", menu)
    }

    #[doc(alias = "label")]
    fn connect_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_label_trampoline<P: IsA<Tab>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeTab,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Tab::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::label".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_label_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "tooltip")]
    fn connect_tooltip_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tooltip_trampoline<P: IsA<Tab>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeTab,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Tab::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::tooltip".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_tooltip_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pinned")]
    fn connect_pinned_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pinned_trampoline<P: IsA<Tab>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeTab,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Tab::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::pinned".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_pinned_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "can-pin")]
    fn connect_can_pin_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_pin_trampoline<P: IsA<Tab>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeTab,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Tab::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::can-pin".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_can_pin_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "can-close")]
    fn connect_can_close_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_close_trampoline<P: IsA<Tab>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeTab,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Tab::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::can-close".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_can_close_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "page")]
    fn connect_page_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_page_trampoline<P: IsA<Tab>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeTab,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Tab::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::page".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_page_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "menu")]
    fn connect_menu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_menu_trampoline<P: IsA<Tab>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeTab,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Tab::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::menu".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_menu_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Tab>> TabExt for O {}
