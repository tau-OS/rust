// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "HeDatePicker")]
    pub struct DatePicker(Object<ffi::HeDatePicker, ffi::HeDatePickerClass>) @extends gtk::Entry, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::CellEditable, gtk::Editable;

    match fn {
        type_ => || ffi::he_date_picker_get_type(),
    }
}

impl DatePicker {
    pub const NONE: Option<&'static DatePicker> = None;

    #[doc(alias = "he_date_picker_new_with_format")]
    pub fn with_format(format: &str) -> DatePicker {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::he_date_picker_new_with_format(format.to_glib_none().0)) }
    }

    #[doc(alias = "he_date_picker_new")]
    pub fn new() -> DatePicker {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::he_date_picker_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`DatePicker`] objects.
    ///
    /// This method returns an instance of [`DatePickerBuilder`](crate::builders::DatePickerBuilder) which can be used to create [`DatePicker`] objects.
    pub fn builder() -> DatePickerBuilder {
        DatePickerBuilder::new()
    }
}

impl Default for DatePicker {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`DatePicker`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct DatePickerBuilder {
    builder: glib::object::ObjectBuilder<'static, DatePicker>,
}

impl DatePickerBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn format(self, format: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("format", format.into()),
        }
    }

    pub fn date(self, date: &glib::DateTime) -> Self {
        Self {
            builder: self.builder.property("date", date.clone()),
        }
    }

    pub fn activates_default(self, activates_default: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("activates-default", activates_default),
        }
    }

    //pub fn attributes(self, attributes: /*Ignored*/&pango::AttrList) -> Self {
    //    Self { builder: self.builder.property("attributes", attributes), }
    //}

    //pub fn buffer(self, buffer: &impl IsA</*Ignored*/gtk::EntryBuffer>) -> Self {
    //    Self { builder: self.builder.property("buffer", buffer.clone().upcast()), }
    //}

    //    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    //pub fn completion(self, completion: /*Ignored*/&gtk::EntryCompletion) -> Self {
    //    Self { builder: self.builder.property("completion", completion), }
    //}

    pub fn enable_emoji_completion(self, enable_emoji_completion: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("enable-emoji-completion", enable_emoji_completion),
        }
    }

    pub fn extra_menu(self, extra_menu: &impl IsA<gio::MenuModel>) -> Self {
        Self {
            builder: self
                .builder
                .property("extra-menu", extra_menu.clone().upcast()),
        }
    }

    pub fn has_frame(self, has_frame: bool) -> Self {
        Self {
            builder: self.builder.property("has-frame", has_frame),
        }
    }

    pub fn im_module(self, im_module: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("im-module", im_module.into()),
        }
    }

    //pub fn input_hints(self, input_hints: /*Ignored*/gtk::InputHints) -> Self {
    //    Self { builder: self.builder.property("input-hints", input_hints), }
    //}

    //pub fn input_purpose(self, input_purpose: /*Ignored*/gtk::InputPurpose) -> Self {
    //    Self { builder: self.builder.property("input-purpose", input_purpose), }
    //}

    pub fn invisible_char(self, invisible_char: u32) -> Self {
        Self {
            builder: self.builder.property("invisible-char", invisible_char),
        }
    }

    pub fn invisible_char_set(self, invisible_char_set: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("invisible-char-set", invisible_char_set),
        }
    }

    pub fn max_length(self, max_length: i32) -> Self {
        Self {
            builder: self.builder.property("max-length", max_length),
        }
    }

    pub fn overwrite_mode(self, overwrite_mode: bool) -> Self {
        Self {
            builder: self.builder.property("overwrite-mode", overwrite_mode),
        }
    }

    pub fn placeholder_text(self, placeholder_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("placeholder-text", placeholder_text.into()),
        }
    }

    pub fn primary_icon_activatable(self, primary_icon_activatable: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("primary-icon-activatable", primary_icon_activatable),
        }
    }

    pub fn primary_icon_gicon(self, primary_icon_gicon: &impl IsA<gio::Icon>) -> Self {
        Self {
            builder: self
                .builder
                .property("primary-icon-gicon", primary_icon_gicon.clone().upcast()),
        }
    }

    pub fn primary_icon_name(self, primary_icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("primary-icon-name", primary_icon_name.into()),
        }
    }

    pub fn primary_icon_paintable(self, primary_icon_paintable: &impl IsA<gdk::Paintable>) -> Self {
        Self {
            builder: self.builder.property(
                "primary-icon-paintable",
                primary_icon_paintable.clone().upcast(),
            ),
        }
    }

    pub fn primary_icon_sensitive(self, primary_icon_sensitive: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("primary-icon-sensitive", primary_icon_sensitive),
        }
    }

    pub fn primary_icon_tooltip_markup(
        self,
        primary_icon_tooltip_markup: impl Into<glib::GString>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "primary-icon-tooltip-markup",
                primary_icon_tooltip_markup.into(),
            ),
        }
    }

    pub fn primary_icon_tooltip_text(
        self,
        primary_icon_tooltip_text: impl Into<glib::GString>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "primary-icon-tooltip-text",
                primary_icon_tooltip_text.into(),
            ),
        }
    }

    pub fn progress_fraction(self, progress_fraction: f64) -> Self {
        Self {
            builder: self
                .builder
                .property("progress-fraction", progress_fraction),
        }
    }

    pub fn progress_pulse_step(self, progress_pulse_step: f64) -> Self {
        Self {
            builder: self
                .builder
                .property("progress-pulse-step", progress_pulse_step),
        }
    }

    pub fn secondary_icon_activatable(self, secondary_icon_activatable: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("secondary-icon-activatable", secondary_icon_activatable),
        }
    }

    pub fn secondary_icon_gicon(self, secondary_icon_gicon: &impl IsA<gio::Icon>) -> Self {
        Self {
            builder: self.builder.property(
                "secondary-icon-gicon",
                secondary_icon_gicon.clone().upcast(),
            ),
        }
    }

    pub fn secondary_icon_name(self, secondary_icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("secondary-icon-name", secondary_icon_name.into()),
        }
    }

    pub fn secondary_icon_paintable(
        self,
        secondary_icon_paintable: &impl IsA<gdk::Paintable>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "secondary-icon-paintable",
                secondary_icon_paintable.clone().upcast(),
            ),
        }
    }

    pub fn secondary_icon_sensitive(self, secondary_icon_sensitive: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("secondary-icon-sensitive", secondary_icon_sensitive),
        }
    }

    pub fn secondary_icon_tooltip_markup(
        self,
        secondary_icon_tooltip_markup: impl Into<glib::GString>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "secondary-icon-tooltip-markup",
                secondary_icon_tooltip_markup.into(),
            ),
        }
    }

    pub fn secondary_icon_tooltip_text(
        self,
        secondary_icon_tooltip_text: impl Into<glib::GString>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "secondary-icon-tooltip-text",
                secondary_icon_tooltip_text.into(),
            ),
        }
    }

    pub fn show_emoji_icon(self, show_emoji_icon: bool) -> Self {
        Self {
            builder: self.builder.property("show-emoji-icon", show_emoji_icon),
        }
    }

    //pub fn tabs(self, tabs: /*Ignored*/&pango::TabArray) -> Self {
    //    Self { builder: self.builder.property("tabs", tabs), }
    //}

    pub fn truncate_multiline(self, truncate_multiline: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("truncate-multiline", truncate_multiline),
        }
    }

    pub fn visibility(self, visibility: bool) -> Self {
        Self {
            builder: self.builder.property("visibility", visibility),
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

    pub fn editing_canceled(self, editing_canceled: bool) -> Self {
        Self {
            builder: self.builder.property("editing-canceled", editing_canceled),
        }
    }

    pub fn editable(self, editable: bool) -> Self {
        Self {
            builder: self.builder.property("editable", editable),
        }
    }

    pub fn enable_undo(self, enable_undo: bool) -> Self {
        Self {
            builder: self.builder.property("enable-undo", enable_undo),
        }
    }

    pub fn max_width_chars(self, max_width_chars: i32) -> Self {
        Self {
            builder: self.builder.property("max-width-chars", max_width_chars),
        }
    }

    pub fn text(self, text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("text", text.into()),
        }
    }

    pub fn width_chars(self, width_chars: i32) -> Self {
        Self {
            builder: self.builder.property("width-chars", width_chars),
        }
    }

    pub fn xalign(self, xalign: f32) -> Self {
        Self {
            builder: self.builder.property("xalign", xalign),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`DatePicker`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> DatePicker {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DatePicker>> Sealed for T {}
}

pub trait DatePickerExt: IsA<DatePicker> + sealed::Sealed + 'static {
    #[doc(alias = "he_date_picker_get_format")]
    #[doc(alias = "get_format")]
    fn format(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::he_date_picker_get_format(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "he_date_picker_get_date")]
    #[doc(alias = "get_date")]
    fn date(&self) -> glib::DateTime {
        unsafe { from_glib_none(ffi::he_date_picker_get_date(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "he_date_picker_set_date")]
    fn set_date(&self, value: &glib::DateTime) {
        unsafe {
            ffi::he_date_picker_set_date(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "date")]
    fn connect_date_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_date_trampoline<P: IsA<DatePicker>, F: Fn(&P) + 'static>(
            this: *mut ffi::HeDatePicker,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DatePicker::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::date\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_date_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<DatePicker>> DatePickerExt for O {}