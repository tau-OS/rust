// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{ffi, AboutWindowLicenses, Colors, Window};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "HeAboutWindow")]
    pub struct AboutWindow(Object<ffi::HeAboutWindow, ffi::HeAboutWindowClass>) @extends Window, gtk::Window, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;

    match fn {
        type_ => || ffi::he_about_window_get_type(),
    }
}

impl AboutWindow {
    pub const NONE: Option<&'static AboutWindow> = None;

    #[doc(alias = "he_about_window_new")]
    pub fn new(
        parent: &impl IsA<gtk::Window>,
        app_name: &str,
        app_id: &str,
        version: Option<&str>,
        icon: Option<&str>,
        translate_url: Option<&str>,
        issue_url: Option<&str>,
        more_info_url: Option<&str>,
        translators: &[&str],
        developers: &[&str],
        copyright_year: i32,
        license: AboutWindowLicenses,
        color: Colors,
    ) -> AboutWindow {
        assert_initialized_main_thread!();
        let translators_length1 = translators.len() as _;
        let developers_length1 = developers.len() as _;
        unsafe {
            from_glib_none(ffi::he_about_window_new(
                parent.as_ref().to_glib_none().0,
                app_name.to_glib_none().0,
                app_id.to_glib_none().0,
                version.to_glib_none().0,
                icon.to_glib_none().0,
                translate_url.to_glib_none().0,
                issue_url.to_glib_none().0,
                more_info_url.to_glib_none().0,
                translators.to_glib_none().0,
                translators_length1,
                developers.to_glib_none().0,
                developers_length1,
                copyright_year,
                license.into_glib(),
                color.into_glib(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`AboutWindow`] objects.
    ///
    /// This method returns an instance of [`AboutWindowBuilder`](crate::builders::AboutWindowBuilder) which can be used to create [`AboutWindow`] objects.
    pub fn builder() -> AboutWindowBuilder {
        AboutWindowBuilder::new()
    }
}

impl Default for AboutWindow {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`AboutWindow`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct AboutWindowBuilder {
    builder: glib::object::ObjectBuilder<'static, AboutWindow>,
}

impl AboutWindowBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn color(self, color: Colors) -> Self {
        Self {
            builder: self.builder.property("color", color),
        }
    }

    pub fn license(self, license: AboutWindowLicenses) -> Self {
        Self {
            builder: self.builder.property("license", license),
        }
    }

    pub fn version(self, version: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("version", version.into()),
        }
    }

    pub fn app_name(self, app_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("app-name", app_name.into()),
        }
    }

    pub fn icon(self, icon: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("icon", icon.into()),
        }
    }

    pub fn translator_names(self, translator_names: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self
                .builder
                .property("translator-names", translator_names.into()),
        }
    }

    pub fn developer_names(self, developer_names: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self
                .builder
                .property("developer-names", developer_names.into()),
        }
    }

    pub fn copyright_year(self, copyright_year: i32) -> Self {
        Self {
            builder: self.builder.property("copyright-year", copyright_year),
        }
    }

    pub fn app_id(self, app_id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("app-id", app_id.into()),
        }
    }

    pub fn translate_url(self, translate_url: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("translate-url", translate_url.into()),
        }
    }

    pub fn issue_url(self, issue_url: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("issue-url", issue_url.into()),
        }
    }

    pub fn more_info_url(self, more_info_url: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("more-info-url", more_info_url.into()),
        }
    }

    pub fn parent(self, parent: &impl IsA<gtk::Window>) -> Self {
        Self {
            builder: self.builder.property("parent", parent.clone().upcast()),
        }
    }

    pub fn has_title(self, has_title: bool) -> Self {
        Self {
            builder: self.builder.property("has-title", has_title),
        }
    }

    pub fn has_back_button(self, has_back_button: bool) -> Self {
        Self {
            builder: self.builder.property("has-back-button", has_back_button),
        }
    }

    pub fn application(self, application: &impl IsA<gtk::Application>) -> Self {
        Self {
            builder: self
                .builder
                .property("application", application.clone().upcast()),
        }
    }

    pub fn child(self, child: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn decorated(self, decorated: bool) -> Self {
        Self {
            builder: self.builder.property("decorated", decorated),
        }
    }

    pub fn default_height(self, default_height: i32) -> Self {
        Self {
            builder: self.builder.property("default-height", default_height),
        }
    }

    pub fn default_widget(self, default_widget: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("default-widget", default_widget.clone().upcast()),
        }
    }

    pub fn default_width(self, default_width: i32) -> Self {
        Self {
            builder: self.builder.property("default-width", default_width),
        }
    }

    pub fn deletable(self, deletable: bool) -> Self {
        Self {
            builder: self.builder.property("deletable", deletable),
        }
    }

    pub fn destroy_with_parent(self, destroy_with_parent: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("destroy-with-parent", destroy_with_parent),
        }
    }

    //pub fn display(self, display: /*Ignored*/&gdk::Display) -> Self {
    //    Self { builder: self.builder.property("display", display), }
    //}

    pub fn focus_visible(self, focus_visible: bool) -> Self {
        Self {
            builder: self.builder.property("focus-visible", focus_visible),
        }
    }

    pub fn focus_widget(self, focus_widget: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("focus-widget", focus_widget.clone().upcast()),
        }
    }

    pub fn fullscreened(self, fullscreened: bool) -> Self {
        Self {
            builder: self.builder.property("fullscreened", fullscreened),
        }
    }

    #[cfg(feature = "gtk_v4_2")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v4_2")))]
    pub fn handle_menubar_accel(self, handle_menubar_accel: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("handle-menubar-accel", handle_menubar_accel),
        }
    }

    pub fn hide_on_close(self, hide_on_close: bool) -> Self {
        Self {
            builder: self.builder.property("hide-on-close", hide_on_close),
        }
    }

    pub fn icon_name(self, icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("icon-name", icon_name.into()),
        }
    }

    pub fn maximized(self, maximized: bool) -> Self {
        Self {
            builder: self.builder.property("maximized", maximized),
        }
    }

    pub fn mnemonics_visible(self, mnemonics_visible: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("mnemonics-visible", mnemonics_visible),
        }
    }

    pub fn modal(self, modal: bool) -> Self {
        Self {
            builder: self.builder.property("modal", modal),
        }
    }

    pub fn resizable(self, resizable: bool) -> Self {
        Self {
            builder: self.builder.property("resizable", resizable),
        }
    }

    pub fn startup_id(self, startup_id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("startup-id", startup_id.into()),
        }
    }

    pub fn title(self, title: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("title", title.into()),
        }
    }

    #[cfg(feature = "gtk_v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gtk_v4_6")))]
    pub fn titlebar(self, titlebar: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("titlebar", titlebar.clone().upcast()),
        }
    }

    pub fn transient_for(self, transient_for: &impl IsA<gtk::Window>) -> Self {
        Self {
            builder: self
                .builder
                .property("transient-for", transient_for.clone().upcast()),
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
    /// Build the [`AboutWindow`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> AboutWindow {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::AboutWindow>> Sealed for T {}
}

pub trait AboutWindowExt: IsA<AboutWindow> + sealed::Sealed + 'static {
    #[doc(alias = "he_about_window_get_color")]
    #[doc(alias = "get_color")]
    fn color(&self) -> Colors {
        unsafe {
            from_glib(ffi::he_about_window_get_color(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_about_window_set_color")]
    fn set_color(&self, value: Colors) {
        unsafe {
            ffi::he_about_window_set_color(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "he_about_window_get_license")]
    #[doc(alias = "get_license")]
    fn license(&self) -> AboutWindowLicenses {
        unsafe {
            from_glib(ffi::he_about_window_get_license(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_about_window_set_license")]
    fn set_license(&self, value: AboutWindowLicenses) {
        unsafe {
            ffi::he_about_window_set_license(self.as_ref().to_glib_none().0, value.into_glib());
        }
    }

    #[doc(alias = "he_about_window_get_version")]
    #[doc(alias = "get_version")]
    fn version(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::he_about_window_get_version(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_about_window_set_version")]
    fn set_version(&self, value: &str) {
        unsafe {
            ffi::he_about_window_set_version(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "he_about_window_get_app_name")]
    #[doc(alias = "get_app_name")]
    fn app_name(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::he_about_window_get_app_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_about_window_set_app_name")]
    fn set_app_name(&self, value: &str) {
        unsafe {
            ffi::he_about_window_set_app_name(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "he_about_window_get_icon")]
    #[doc(alias = "get_icon")]
    fn icon(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::he_about_window_get_icon(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_about_window_set_icon")]
    fn set_icon(&self, value: &str) {
        unsafe {
            ffi::he_about_window_set_icon(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "he_about_window_get_translator_names")]
    #[doc(alias = "get_translator_names")]
    fn translator_names(&self) -> Vec<glib::GString> {
        unsafe {
            let mut result_length1 = std::mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(
                ffi::he_about_window_get_translator_names(
                    self.as_ref().to_glib_none().0,
                    result_length1.as_mut_ptr(),
                ),
                result_length1.assume_init() as _,
            );
            ret
        }
    }

    #[doc(alias = "he_about_window_set_translator_names")]
    fn set_translator_names(&self, value: &[&str]) {
        let value_length1 = value.len() as _;
        unsafe {
            ffi::he_about_window_set_translator_names(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
                value_length1,
            );
        }
    }

    #[doc(alias = "he_about_window_get_developer_names")]
    #[doc(alias = "get_developer_names")]
    fn developer_names(&self) -> Vec<glib::GString> {
        unsafe {
            let mut result_length1 = std::mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(
                ffi::he_about_window_get_developer_names(
                    self.as_ref().to_glib_none().0,
                    result_length1.as_mut_ptr(),
                ),
                result_length1.assume_init() as _,
            );
            ret
        }
    }

    #[doc(alias = "he_about_window_set_developer_names")]
    fn set_developer_names(&self, value: &[&str]) {
        let value_length1 = value.len() as _;
        unsafe {
            ffi::he_about_window_set_developer_names(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
                value_length1,
            );
        }
    }

    #[doc(alias = "he_about_window_get_copyright_year")]
    #[doc(alias = "get_copyright_year")]
    fn copyright_year(&self) -> i32 {
        unsafe { ffi::he_about_window_get_copyright_year(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "he_about_window_set_copyright_year")]
    fn set_copyright_year(&self, value: i32) {
        unsafe {
            ffi::he_about_window_set_copyright_year(self.as_ref().to_glib_none().0, value);
        }
    }

    #[doc(alias = "he_about_window_get_app_id")]
    #[doc(alias = "get_app_id")]
    fn app_id(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::he_about_window_get_app_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_about_window_set_app_id")]
    fn set_app_id(&self, value: &str) {
        unsafe {
            ffi::he_about_window_set_app_id(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "he_about_window_get_translate_url")]
    #[doc(alias = "get_translate_url")]
    fn translate_url(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::he_about_window_get_translate_url(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_about_window_set_translate_url")]
    fn set_translate_url(&self, value: Option<&str>) {
        unsafe {
            ffi::he_about_window_set_translate_url(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "he_about_window_get_issue_url")]
    #[doc(alias = "get_issue_url")]
    fn issue_url(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::he_about_window_get_issue_url(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_about_window_set_issue_url")]
    fn set_issue_url(&self, value: Option<&str>) {
        unsafe {
            ffi::he_about_window_set_issue_url(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "he_about_window_get_more_info_url")]
    #[doc(alias = "get_more_info_url")]
    fn more_info_url(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::he_about_window_get_more_info_url(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_about_window_set_more_info_url")]
    fn set_more_info_url(&self, value: Option<&str>) {
        unsafe {
            ffi::he_about_window_set_more_info_url(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "color")]
    fn connect_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_color_trampoline<P: IsA<AboutWindow>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeAboutWindow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AboutWindow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::color\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_color_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "license")]
    fn connect_license_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_license_trampoline<P: IsA<AboutWindow>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeAboutWindow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AboutWindow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::license\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_license_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "version")]
    fn connect_version_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_version_trampoline<P: IsA<AboutWindow>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeAboutWindow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AboutWindow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::version\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_version_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "app-name")]
    fn connect_app_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_app_name_trampoline<
            P: IsA<AboutWindow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HeAboutWindow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AboutWindow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::app-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_app_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "icon")]
    fn connect_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_trampoline<P: IsA<AboutWindow>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeAboutWindow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AboutWindow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_icon_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "translator-names")]
    fn connect_translator_names_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_translator_names_trampoline<
            P: IsA<AboutWindow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HeAboutWindow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AboutWindow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::translator-names\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_translator_names_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "developer-names")]
    fn connect_developer_names_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_developer_names_trampoline<
            P: IsA<AboutWindow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HeAboutWindow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AboutWindow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::developer-names\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_developer_names_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "copyright-year")]
    fn connect_copyright_year_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_copyright_year_trampoline<
            P: IsA<AboutWindow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HeAboutWindow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AboutWindow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::copyright-year\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_copyright_year_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "app-id")]
    fn connect_app_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_app_id_trampoline<P: IsA<AboutWindow>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeAboutWindow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AboutWindow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::app-id\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_app_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "translate-url")]
    fn connect_translate_url_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_translate_url_trampoline<
            P: IsA<AboutWindow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HeAboutWindow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AboutWindow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::translate-url\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_translate_url_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "issue-url")]
    fn connect_issue_url_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_issue_url_trampoline<
            P: IsA<AboutWindow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HeAboutWindow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AboutWindow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::issue-url\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_issue_url_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "more-info-url")]
    fn connect_more_info_url_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_more_info_url_trampoline<
            P: IsA<AboutWindow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::HeAboutWindow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AboutWindow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::more-info-url\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_more_info_url_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<AboutWindow>> AboutWindowExt for O {}
