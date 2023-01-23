// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Align, BaselinePosition, Box, Buildable, ButtonBoxStyle, Container, Orientable, Orientation,
    ResizeMode, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkButtonBox")]
    pub struct ButtonBox(Object<ffi::GtkButtonBox, ffi::GtkButtonBoxClass>) @extends Box, Container, Widget, @implements Buildable, Orientable;

    match fn {
        type_ => || ffi::gtk_button_box_get_type(),
    }
}

impl ButtonBox {
    pub const NONE: Option<&'static ButtonBox> = None;

    #[doc(alias = "gtk_button_box_new")]
    pub fn new(orientation: Orientation) -> ButtonBox {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_button_box_new(orientation.into_glib())).unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ButtonBox`] objects.
    ///
    /// This method returns an instance of [`ButtonBoxBuilder`](crate::builders::ButtonBoxBuilder) which can be used to create [`ButtonBox`] objects.
    pub fn builder() -> ButtonBoxBuilder {
        ButtonBoxBuilder::new()
    }
}

impl Default for ButtonBox {
    fn default() -> Self {
        glib::object::Object::new_default::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ButtonBox`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ButtonBoxBuilder {
    builder: glib::object::ObjectBuilder<'static, ButtonBox>,
}

impl ButtonBoxBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn layout_style(self, layout_style: ButtonBoxStyle) -> Self {
        Self {
            builder: self.builder.property("layout-style", layout_style),
        }
    }

    pub fn baseline_position(self, baseline_position: BaselinePosition) -> Self {
        Self {
            builder: self
                .builder
                .property("baseline-position", baseline_position),
        }
    }

    pub fn homogeneous(self, homogeneous: bool) -> Self {
        Self {
            builder: self.builder.property("homogeneous", homogeneous),
        }
    }

    pub fn spacing(self, spacing: i32) -> Self {
        Self {
            builder: self.builder.property("spacing", spacing),
        }
    }

    pub fn border_width(self, border_width: u32) -> Self {
        Self {
            builder: self.builder.property("border-width", border_width),
        }
    }

    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn resize_mode(self, resize_mode: ResizeMode) -> Self {
        Self {
            builder: self.builder.property("resize-mode", resize_mode),
        }
    }

    pub fn app_paintable(self, app_paintable: bool) -> Self {
        Self {
            builder: self.builder.property("app-paintable", app_paintable),
        }
    }

    pub fn can_default(self, can_default: bool) -> Self {
        Self {
            builder: self.builder.property("can-default", can_default),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn events(self, events: gdk::EventMask) -> Self {
        Self {
            builder: self.builder.property("events", events),
        }
    }

    pub fn expand(self, expand: bool) -> Self {
        Self {
            builder: self.builder.property("expand", expand),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_default(self, has_default: bool) -> Self {
        Self {
            builder: self.builder.property("has-default", has_default),
        }
    }

    pub fn has_focus(self, has_focus: bool) -> Self {
        Self {
            builder: self.builder.property("has-focus", has_focus),
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

    pub fn is_focus(self, is_focus: bool) -> Self {
        Self {
            builder: self.builder.property("is-focus", is_focus),
        }
    }

    pub fn margin(self, margin: i32) -> Self {
        Self {
            builder: self.builder.property("margin", margin),
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

    pub fn no_show_all(self, no_show_all: bool) -> Self {
        Self {
            builder: self.builder.property("no-show-all", no_show_all),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn parent(self, parent: &impl IsA<Container>) -> Self {
        Self {
            builder: self.builder.property("parent", parent.clone().upcast()),
        }
    }

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

    pub fn valign(self, valign: Align) -> Self {
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

    pub fn orientation(self, orientation: Orientation) -> Self {
        Self {
            builder: self.builder.property("orientation", orientation),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ButtonBox`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ButtonBox {
        self.builder.build()
    }
}

pub trait ButtonBoxExt: 'static {
    #[doc(alias = "gtk_button_box_get_child_non_homogeneous")]
    #[doc(alias = "get_child_non_homogeneous")]
    fn child_is_non_homogeneous(&self, child: &impl IsA<Widget>) -> bool;

    #[doc(alias = "gtk_button_box_get_child_secondary")]
    #[doc(alias = "get_child_secondary")]
    fn child_is_secondary(&self, child: &impl IsA<Widget>) -> bool;

    #[doc(alias = "gtk_button_box_get_layout")]
    #[doc(alias = "get_layout")]
    fn layout(&self) -> ButtonBoxStyle;

    #[doc(alias = "gtk_button_box_set_child_non_homogeneous")]
    fn set_child_non_homogeneous(&self, child: &impl IsA<Widget>, non_homogeneous: bool);

    #[doc(alias = "gtk_button_box_set_child_secondary")]
    fn set_child_secondary(&self, child: &impl IsA<Widget>, is_secondary: bool);

    #[doc(alias = "gtk_button_box_set_layout")]
    fn set_layout(&self, layout_style: ButtonBoxStyle);

    #[doc(alias = "layout-style")]
    fn layout_style(&self) -> ButtonBoxStyle;

    #[doc(alias = "layout-style")]
    fn set_layout_style(&self, layout_style: ButtonBoxStyle);

    #[doc(alias = "layout-style")]
    fn connect_layout_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ButtonBox>> ButtonBoxExt for O {
    fn child_is_non_homogeneous(&self, child: &impl IsA<Widget>) -> bool {
        unsafe {
            from_glib(ffi::gtk_button_box_get_child_non_homogeneous(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    fn child_is_secondary(&self, child: &impl IsA<Widget>) -> bool {
        unsafe {
            from_glib(ffi::gtk_button_box_get_child_secondary(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            ))
        }
    }

    fn layout(&self) -> ButtonBoxStyle {
        unsafe {
            from_glib(ffi::gtk_button_box_get_layout(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_child_non_homogeneous(&self, child: &impl IsA<Widget>, non_homogeneous: bool) {
        unsafe {
            ffi::gtk_button_box_set_child_non_homogeneous(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                non_homogeneous.into_glib(),
            );
        }
    }

    fn set_child_secondary(&self, child: &impl IsA<Widget>, is_secondary: bool) {
        unsafe {
            ffi::gtk_button_box_set_child_secondary(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                is_secondary.into_glib(),
            );
        }
    }

    fn set_layout(&self, layout_style: ButtonBoxStyle) {
        unsafe {
            ffi::gtk_button_box_set_layout(
                self.as_ref().to_glib_none().0,
                layout_style.into_glib(),
            );
        }
    }

    fn layout_style(&self) -> ButtonBoxStyle {
        glib::ObjectExt::property(self.as_ref(), "layout-style")
    }

    fn set_layout_style(&self, layout_style: ButtonBoxStyle) {
        glib::ObjectExt::set_property(self.as_ref(), "layout-style", &layout_style)
    }

    fn connect_layout_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_layout_style_trampoline<
            P: IsA<ButtonBox>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkButtonBox,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ButtonBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::layout-style\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_layout_style_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ButtonBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ButtonBox")
    }
}
