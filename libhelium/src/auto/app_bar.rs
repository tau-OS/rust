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
    #[doc(alias = "HeAppBar")]
    pub struct AppBar(Object<ffi::HeAppBar, ffi::HeAppBarClass>) @extends Bin, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::he_app_bar_get_type(),
    }
}

impl AppBar {
    pub const NONE: Option<&'static AppBar> = None;

    #[doc(alias = "he_app_bar_new")]
    pub fn new() -> AppBar {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::he_app_bar_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`AppBar`] objects.
    ///
    /// This method returns an instance of [`AppBarBuilder`](crate::builders::AppBarBuilder) which can be used to create [`AppBar`] objects.
    pub fn builder() -> AppBarBuilder {
        AppBarBuilder::new()
    }
}

impl Default for AppBar {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`AppBar`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct AppBarBuilder {
    builder: glib::object::ObjectBuilder<'static, AppBar>,
}

impl AppBarBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn stack(self, stack: &gtk::Stack) -> Self {
        Self {
            builder: self.builder.property("stack", stack.clone()),
        }
    }

    pub fn scroller(self, scroller: &gtk::ScrolledWindow) -> Self {
        Self {
            builder: self.builder.property("scroller", scroller.clone()),
        }
    }

    pub fn is_compact(self, is_compact: bool) -> Self {
        Self {
            builder: self.builder.property("is-compact", is_compact),
        }
    }

    pub fn viewtitle_widget(self, viewtitle_widget: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("viewtitle-widget", viewtitle_widget.clone().upcast()),
        }
    }

    pub fn viewsubtitle_label(self, viewsubtitle_label: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("viewsubtitle-label", viewsubtitle_label.into()),
        }
    }

    pub fn show_left_title_buttons(self, show_left_title_buttons: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("show-left-title-buttons", show_left_title_buttons),
        }
    }

    pub fn show_right_title_buttons(self, show_right_title_buttons: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("show-right-title-buttons", show_right_title_buttons),
        }
    }

    pub fn decoration_layout(self, decoration_layout: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("decoration-layout", decoration_layout.into()),
        }
    }

    pub fn show_back(self, show_back: bool) -> Self {
        Self {
            builder: self.builder.property("show-back", show_back),
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
    /// Build the [`AppBar`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> AppBar {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}

pub trait AppBarExt: IsA<AppBar> + 'static {
    #[doc(alias = "he_app_bar_append")]
    fn append(&self, child: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::he_app_bar_append(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "he_app_bar_append_toggle")]
    fn append_toggle(&self, child: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::he_app_bar_append_toggle(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "he_app_bar_append_menu")]
    fn append_menu(&self, child: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::he_app_bar_append_menu(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "he_app_bar_remove")]
    fn remove(&self, child: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::he_app_bar_remove(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "he_app_bar_get_stack")]
    #[doc(alias = "get_stack")]
    fn stack(&self) -> gtk::Stack {
        unsafe { from_glib_none(ffi::he_app_bar_get_stack(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "he_app_bar_set_stack")]
    fn set_stack(&self, value: &gtk::Stack) {
        unsafe {
            ffi::he_app_bar_set_stack(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "he_app_bar_get_scroller")]
    #[doc(alias = "get_scroller")]
    fn scroller(&self) -> gtk::ScrolledWindow {
        unsafe { from_glib_none(ffi::he_app_bar_get_scroller(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "he_app_bar_set_scroller")]
    fn set_scroller(&self, value: &gtk::ScrolledWindow) {
        unsafe {
            ffi::he_app_bar_set_scroller(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "he_app_bar_get_is_compact")]
    #[doc(alias = "get_is_compact")]
    fn is_compact(&self) -> bool {
        unsafe {
            from_glib(ffi::he_app_bar_get_is_compact(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_app_bar_set_is_compact")]
    fn set_is_compact(&self, value: bool) {
        unsafe {
            ffi::he_app_bar_set_is_compact(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "he_app_bar_get_viewtitle_widget")]
    #[doc(alias = "get_viewtitle_widget")]
    fn viewtitle_widget(&self) -> Option<gtk::Widget> {
        unsafe {
            from_glib_none(ffi::he_app_bar_get_viewtitle_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_app_bar_set_viewtitle_widget")]
    fn set_viewtitle_widget(&self, value: Option<&impl IsA<gtk::Widget>>) {
        unsafe {
            ffi::he_app_bar_set_viewtitle_widget(
                self.as_ref().to_glib_none().0,
                value.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "he_app_bar_get_viewsubtitle_label")]
    #[doc(alias = "get_viewsubtitle_label")]
    fn viewsubtitle_label(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::he_app_bar_get_viewsubtitle_label(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_app_bar_set_viewsubtitle_label")]
    fn set_viewsubtitle_label(&self, value: &str) {
        unsafe {
            ffi::he_app_bar_set_viewsubtitle_label(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "he_app_bar_get_show_left_title_buttons")]
    #[doc(alias = "get_show_left_title_buttons")]
    fn shows_left_title_buttons(&self) -> bool {
        unsafe {
            from_glib(ffi::he_app_bar_get_show_left_title_buttons(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_app_bar_set_show_left_title_buttons")]
    fn set_show_left_title_buttons(&self, value: bool) {
        unsafe {
            ffi::he_app_bar_set_show_left_title_buttons(
                self.as_ref().to_glib_none().0,
                value.into_glib(),
            );
        }
    }

    #[doc(alias = "he_app_bar_get_show_right_title_buttons")]
    #[doc(alias = "get_show_right_title_buttons")]
    fn shows_right_title_buttons(&self) -> bool {
        unsafe {
            from_glib(ffi::he_app_bar_get_show_right_title_buttons(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_app_bar_set_show_right_title_buttons")]
    fn set_show_right_title_buttons(&self, value: bool) {
        unsafe {
            ffi::he_app_bar_set_show_right_title_buttons(
                self.as_ref().to_glib_none().0,
                value.into_glib(),
            );
        }
    }

    #[doc(alias = "he_app_bar_get_decoration_layout")]
    #[doc(alias = "get_decoration_layout")]
    fn decoration_layout(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::he_app_bar_get_decoration_layout(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_app_bar_set_decoration_layout")]
    fn set_decoration_layout(&self, value: &str) {
        unsafe {
            ffi::he_app_bar_set_decoration_layout(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "he_app_bar_get_show_back")]
    #[doc(alias = "get_show_back")]
    fn shows_back(&self) -> bool {
        unsafe {
            from_glib(ffi::he_app_bar_get_show_back(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_app_bar_set_show_back")]
    fn set_show_back(&self, value: bool) {
        unsafe {
            ffi::he_app_bar_set_show_back(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "stack")]
    fn connect_stack_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_stack_trampoline<P: IsA<AppBar>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeAppBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::stack".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_stack_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "scroller")]
    fn connect_scroller_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_scroller_trampoline<P: IsA<AppBar>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeAppBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::scroller".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_scroller_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "is-compact")]
    fn connect_is_compact_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_compact_trampoline<P: IsA<AppBar>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeAppBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::is-compact".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_is_compact_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "viewtitle-widget")]
    fn connect_viewtitle_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_viewtitle_widget_trampoline<
            P: IsA<AppBar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HeAppBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::viewtitle-widget".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_viewtitle_widget_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "viewsubtitle-label")]
    fn connect_viewsubtitle_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_viewsubtitle_label_trampoline<
            P: IsA<AppBar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HeAppBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::viewsubtitle-label".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_viewsubtitle_label_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-left-title-buttons")]
    fn connect_show_left_title_buttons_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_left_title_buttons_trampoline<
            P: IsA<AppBar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HeAppBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::show-left-title-buttons".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_show_left_title_buttons_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-right-title-buttons")]
    fn connect_show_right_title_buttons_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_right_title_buttons_trampoline<
            P: IsA<AppBar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HeAppBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::show-right-title-buttons".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_show_right_title_buttons_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "decoration-layout")]
    fn connect_decoration_layout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_decoration_layout_trampoline<
            P: IsA<AppBar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HeAppBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::decoration-layout".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_decoration_layout_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-back")]
    fn connect_show_back_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_back_trampoline<P: IsA<AppBar>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeAppBar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppBar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::show-back".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_show_back_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<AppBar>> AppBarExt for O {}
