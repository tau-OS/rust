// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::ffi;
use glib::{
    object::ObjectType as _,
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "HeBottomSheet")]
    pub struct BottomSheet(Object<ffi::HeBottomSheet, ffi::HeBottomSheetClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::he_bottom_sheet_get_type(),
    }
}

impl BottomSheet {
    pub const NONE: Option<&'static BottomSheet> = None;

    #[doc(alias = "he_bottom_sheet_new")]
    pub fn new() -> BottomSheet {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::he_bottom_sheet_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`BottomSheet`] objects.
    ///
    /// This method returns an instance of [`BottomSheetBuilder`](crate::builders::BottomSheetBuilder) which can be used to create [`BottomSheet`] objects.
    pub fn builder() -> BottomSheetBuilder {
        BottomSheetBuilder::new()
    }
}

impl Default for BottomSheet {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`BottomSheet`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct BottomSheetBuilder {
    builder: glib::object::ObjectBuilder<'static, BottomSheet>,
}

impl BottomSheetBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn sheet(self, sheet: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("sheet", sheet.clone().upcast()),
        }
    }

    pub fn sheet_stack(self, sheet_stack: &gtk::Stack) -> Self {
        Self {
            builder: self.builder.property("sheet-stack", sheet_stack.clone()),
        }
    }

    pub fn button(self, button: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("button", button.clone().upcast()),
        }
    }

    pub fn title(self, title: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("title", title.into()),
        }
    }

    pub fn show_sheet(self, show_sheet: bool) -> Self {
        Self {
            builder: self.builder.property("show-sheet", show_sheet),
        }
    }

    pub fn modal(self, modal: bool) -> Self {
        Self {
            builder: self.builder.property("modal", modal),
        }
    }

    pub fn show_handle(self, show_handle: bool) -> Self {
        Self {
            builder: self.builder.property("show-handle", show_handle),
        }
    }

    pub fn preferred_sheet_height(self, preferred_sheet_height: i32) -> Self {
        Self {
            builder: self
                .builder
                .property("preferred-sheet-height", preferred_sheet_height),
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
    /// Build the [`BottomSheet`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> BottomSheet {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}

pub trait BottomSheetExt: IsA<BottomSheet> + 'static {
    #[doc(alias = "he_bottom_sheet_get_sheet")]
    #[doc(alias = "get_sheet")]
    fn sheet(&self) -> Option<gtk::Widget> {
        unsafe {
            from_glib_none(ffi::he_bottom_sheet_get_sheet(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_bottom_sheet_set_sheet")]
    fn set_sheet(&self, value: Option<&impl IsA<gtk::Widget>>) {
        unsafe {
            ffi::he_bottom_sheet_set_sheet(
                self.as_ref().to_glib_none().0,
                value.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "he_bottom_sheet_get_sheet_stack")]
    #[doc(alias = "get_sheet_stack")]
    fn sheet_stack(&self) -> Option<gtk::Stack> {
        unsafe {
            from_glib_none(ffi::he_bottom_sheet_get_sheet_stack(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_bottom_sheet_set_sheet_stack")]
    fn set_sheet_stack(&self, value: Option<&gtk::Stack>) {
        unsafe {
            ffi::he_bottom_sheet_set_sheet_stack(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "he_bottom_sheet_get_button")]
    #[doc(alias = "get_button")]
    fn button(&self) -> Option<gtk::Widget> {
        unsafe {
            from_glib_none(ffi::he_bottom_sheet_get_button(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_bottom_sheet_set_button")]
    fn set_button(&self, value: Option<&impl IsA<gtk::Widget>>) {
        unsafe {
            ffi::he_bottom_sheet_set_button(
                self.as_ref().to_glib_none().0,
                value.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "he_bottom_sheet_get_title")]
    #[doc(alias = "get_title")]
    fn title(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::he_bottom_sheet_get_title(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_bottom_sheet_set_title")]
    fn set_title(&self, value: Option<&str>) {
        unsafe {
            ffi::he_bottom_sheet_set_title(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "he_bottom_sheet_get_show_sheet")]
    #[doc(alias = "get_show_sheet")]
    fn shows_sheet(&self) -> bool {
        unsafe {
            from_glib(ffi::he_bottom_sheet_get_show_sheet(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_bottom_sheet_set_show_sheet")]
    fn set_show_sheet(&self, value: bool) {
        unsafe {
            ffi::he_bottom_sheet_set_show_sheet(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "he_bottom_sheet_get_modal")]
    #[doc(alias = "get_modal")]
    fn is_modal(&self) -> bool {
        unsafe {
            from_glib(ffi::he_bottom_sheet_get_modal(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_bottom_sheet_set_modal")]
    fn set_modal(&self, value: bool) {
        unsafe {
            ffi::he_bottom_sheet_set_modal(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "he_bottom_sheet_get_show_handle")]
    #[doc(alias = "get_show_handle")]
    fn shows_handle(&self) -> bool {
        unsafe {
            from_glib(ffi::he_bottom_sheet_get_show_handle(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_bottom_sheet_set_show_handle")]
    fn set_show_handle(&self, value: bool) {
        unsafe {
            ffi::he_bottom_sheet_set_show_handle(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "he_bottom_sheet_get_preferred_sheet_height")]
    #[doc(alias = "get_preferred_sheet_height")]
    fn preferred_sheet_height(&self) -> i32 {
        unsafe { ffi::he_bottom_sheet_get_preferred_sheet_height(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "he_bottom_sheet_set_preferred_sheet_height")]
    fn set_preferred_sheet_height(&self, value: i32) {
        unsafe {
            ffi::he_bottom_sheet_set_preferred_sheet_height(self.as_ref().to_glib_none().0, value);
        }
    }

    #[doc(alias = "hidden")]
    fn connect_hidden<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn hidden_trampoline<P: IsA<BottomSheet>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeBottomSheet,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BottomSheet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"hidden".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    hidden_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "sheet")]
    fn connect_sheet_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sheet_trampoline<P: IsA<BottomSheet>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeBottomSheet,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BottomSheet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::sheet".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_sheet_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "sheet-stack")]
    fn connect_sheet_stack_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sheet_stack_trampoline<
            P: IsA<BottomSheet>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HeBottomSheet,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BottomSheet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::sheet-stack".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_sheet_stack_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "button")]
    fn connect_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_button_trampoline<P: IsA<BottomSheet>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeBottomSheet,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BottomSheet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::button".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_button_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "title")]
    fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<P: IsA<BottomSheet>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeBottomSheet,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BottomSheet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::title".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_title_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-sheet")]
    fn connect_show_sheet_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_sheet_trampoline<
            P: IsA<BottomSheet>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HeBottomSheet,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BottomSheet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::show-sheet".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_show_sheet_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "modal")]
    fn connect_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_modal_trampoline<P: IsA<BottomSheet>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeBottomSheet,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BottomSheet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::modal".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_modal_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-handle")]
    fn connect_show_handle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_handle_trampoline<
            P: IsA<BottomSheet>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HeBottomSheet,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BottomSheet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::show-handle".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_show_handle_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "preferred-sheet-height")]
    fn connect_preferred_sheet_height_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_preferred_sheet_height_trampoline<
            P: IsA<BottomSheet>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HeBottomSheet,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(BottomSheet::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::preferred-sheet-height".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_preferred_sheet_height_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<BottomSheet>> BottomSheetExt for O {}
