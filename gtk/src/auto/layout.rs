// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Adjustment;
use crate::Align;
use crate::Buildable;
use crate::Container;
use crate::ResizeMode;
use crate::Scrollable;
use crate::ScrollablePolicy;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkLayout")]
    pub struct Layout(Object<ffi::GtkLayout, ffi::GtkLayoutClass>) @extends Container, Widget, @implements Buildable, Scrollable;

    match fn {
        type_ => || ffi::gtk_layout_get_type(),
    }
}

impl Layout {
    #[doc(alias = "gtk_layout_new")]
    pub fn new<P: IsA<Adjustment>, Q: IsA<Adjustment>>(
        hadjustment: Option<&P>,
        vadjustment: Option<&Q>,
    ) -> Layout {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_layout_new(
                hadjustment.map(|p| p.as_ref()).to_glib_none().0,
                vadjustment.map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`Layout`].
    ///
    /// This method returns an instance of [`LayoutBuilder`] which can be used to create a [`Layout`].
    pub fn builder() -> LayoutBuilder {
        LayoutBuilder::default()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`Layout`].
pub struct LayoutBuilder {
    height: Option<u32>,
    width: Option<u32>,
    border_width: Option<u32>,
    child: Option<Widget>,
    resize_mode: Option<ResizeMode>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    hadjustment: Option<Adjustment>,
    hscroll_policy: Option<ScrollablePolicy>,
    vadjustment: Option<Adjustment>,
    vscroll_policy: Option<ScrollablePolicy>,
}

impl LayoutBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`LayoutBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Layout`].
    pub fn build(self) -> Layout {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref height) = self.height {
            properties.push(("height", height));
        }
        if let Some(ref width) = self.width {
            properties.push(("width", width));
        }
        if let Some(ref border_width) = self.border_width {
            properties.push(("border-width", border_width));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref resize_mode) = self.resize_mode {
            properties.push(("resize-mode", resize_mode));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        #[cfg(any(feature = "v3_20", feature = "dox"))]
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref hadjustment) = self.hadjustment {
            properties.push(("hadjustment", hadjustment));
        }
        if let Some(ref hscroll_policy) = self.hscroll_policy {
            properties.push(("hscroll-policy", hscroll_policy));
        }
        if let Some(ref vadjustment) = self.vadjustment {
            properties.push(("vadjustment", vadjustment));
        }
        if let Some(ref vscroll_policy) = self.vscroll_policy {
            properties.push(("vscroll-policy", vscroll_policy));
        }
        glib::Object::new::<Layout>(&properties).expect("Failed to create an instance of Layout")
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child<P: IsA<Widget>>(mut self, child: &P) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn resize_mode(mut self, resize_mode: ResizeMode) -> Self {
        self.resize_mode = Some(resize_mode);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent<P: IsA<Container>>(mut self, parent: &P) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn hadjustment<P: IsA<Adjustment>>(mut self, hadjustment: &P) -> Self {
        self.hadjustment = Some(hadjustment.clone().upcast());
        self
    }

    pub fn hscroll_policy(mut self, hscroll_policy: ScrollablePolicy) -> Self {
        self.hscroll_policy = Some(hscroll_policy);
        self
    }

    pub fn vadjustment<P: IsA<Adjustment>>(mut self, vadjustment: &P) -> Self {
        self.vadjustment = Some(vadjustment.clone().upcast());
        self
    }

    pub fn vscroll_policy(mut self, vscroll_policy: ScrollablePolicy) -> Self {
        self.vscroll_policy = Some(vscroll_policy);
        self
    }
}

pub const NONE_LAYOUT: Option<&Layout> = None;

pub trait LayoutExt: 'static {
    #[doc(alias = "gtk_layout_get_bin_window")]
    #[doc(alias = "get_bin_window")]
    fn bin_window(&self) -> Option<gdk::Window>;

    #[doc(alias = "gtk_layout_get_size")]
    #[doc(alias = "get_size")]
    fn size(&self) -> (u32, u32);

    #[doc(alias = "gtk_layout_move")]
    #[doc(alias = "move")]
    fn move_<P: IsA<Widget>>(&self, child_widget: &P, x: i32, y: i32);

    #[doc(alias = "gtk_layout_put")]
    fn put<P: IsA<Widget>>(&self, child_widget: &P, x: i32, y: i32);

    #[doc(alias = "gtk_layout_set_size")]
    fn set_size(&self, width: u32, height: u32);

    fn height(&self) -> u32;

    fn set_height(&self, height: u32);

    fn width(&self) -> u32;

    fn set_width(&self, width: u32);

    fn child_x<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_child_x<T: IsA<Widget>>(&self, item: &T, x: i32);

    fn child_y<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_child_y<T: IsA<Widget>>(&self, item: &T, y: i32);

    #[doc(alias = "height")]
    fn connect_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "width")]
    fn connect_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Layout>> LayoutExt for O {
    fn bin_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_layout_get_bin_window(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn size(&self) -> (u32, u32) {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            ffi::gtk_layout_get_size(
                self.as_ref().to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            let width = width.assume_init();
            let height = height.assume_init();
            (width, height)
        }
    }

    fn move_<P: IsA<Widget>>(&self, child_widget: &P, x: i32, y: i32) {
        unsafe {
            ffi::gtk_layout_move(
                self.as_ref().to_glib_none().0,
                child_widget.as_ref().to_glib_none().0,
                x,
                y,
            );
        }
    }

    fn put<P: IsA<Widget>>(&self, child_widget: &P, x: i32, y: i32) {
        unsafe {
            ffi::gtk_layout_put(
                self.as_ref().to_glib_none().0,
                child_widget.as_ref().to_glib_none().0,
                x,
                y,
            );
        }
    }

    fn set_size(&self, width: u32, height: u32) {
        unsafe {
            ffi::gtk_layout_set_size(self.as_ref().to_glib_none().0, width, height);
        }
    }

    fn height(&self) -> u32 {
        unsafe {
            let mut value = glib::Value::from_type(<u32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"height\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `height` getter")
        }
    }

    fn set_height(&self, height: u32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"height\0".as_ptr() as *const _,
                height.to_value().to_glib_none().0,
            );
        }
    }

    fn width(&self) -> u32 {
        unsafe {
            let mut value = glib::Value::from_type(<u32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"width\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `width` getter")
        }
    }

    fn set_width(&self, width: u32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"width\0".as_ptr() as *const _,
                width.to_value().to_glib_none().0,
            );
        }
    }

    fn child_x<T: IsA<Widget>>(&self, item: &T) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            crate::ffi::gtk_container_child_get_property(
                self.to_glib_none().0 as *mut crate::ffi::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"x\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().expect("Return Value for property `x` getter")
        }
    }

    fn set_child_x<T: IsA<Widget>>(&self, item: &T, x: i32) {
        unsafe {
            crate::ffi::gtk_container_child_set_property(
                self.to_glib_none().0 as *mut crate::ffi::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"x\0".as_ptr() as *const _,
                x.to_value().to_glib_none().0,
            );
        }
    }

    fn child_y<T: IsA<Widget>>(&self, item: &T) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            crate::ffi::gtk_container_child_get_property(
                self.to_glib_none().0 as *mut crate::ffi::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"y\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get().expect("Return Value for property `y` getter")
        }
    }

    fn set_child_y<T: IsA<Widget>>(&self, item: &T, y: i32) {
        unsafe {
            crate::ffi::gtk_container_child_set_property(
                self.to_glib_none().0 as *mut crate::ffi::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"y\0".as_ptr() as *const _,
                y.to_value().to_glib_none().0,
            );
        }
    }

    fn connect_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_height_trampoline<P: IsA<Layout>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkLayout,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Layout::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_height_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_trampoline<P: IsA<Layout>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkLayout,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Layout::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Layout")
    }
}
