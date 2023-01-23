// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Align, BaselinePosition, Box, Buildable, Container, Orientable, Orientation, RecentChooser,
    RecentFilter, RecentManager, RecentSortType, ResizeMode, Widget,
};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkRecentChooserWidget")]
    pub struct RecentChooserWidget(Object<ffi::GtkRecentChooserWidget, ffi::GtkRecentChooserWidgetClass>) @extends Box, Container, Widget, @implements Buildable, Orientable, RecentChooser;

    match fn {
        type_ => || ffi::gtk_recent_chooser_widget_get_type(),
    }
}

impl RecentChooserWidget {
    pub const NONE: Option<&'static RecentChooserWidget> = None;

    #[doc(alias = "gtk_recent_chooser_widget_new")]
    pub fn new() -> RecentChooserWidget {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_recent_chooser_widget_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_recent_chooser_widget_new_for_manager")]
    #[doc(alias = "new_for_manager")]
    pub fn for_manager(manager: &impl IsA<RecentManager>) -> RecentChooserWidget {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_recent_chooser_widget_new_for_manager(
                manager.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`RecentChooserWidget`] objects.
    ///
    /// This method returns an instance of [`RecentChooserWidgetBuilder`](crate::builders::RecentChooserWidgetBuilder) which can be used to create [`RecentChooserWidget`] objects.
    pub fn builder() -> RecentChooserWidgetBuilder {
        RecentChooserWidgetBuilder::new()
    }
}

impl Default for RecentChooserWidget {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`RecentChooserWidget`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct RecentChooserWidgetBuilder {
    builder: glib::object::ObjectBuilder<'static, RecentChooserWidget>,
}

impl RecentChooserWidgetBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
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

    pub fn filter(self, filter: &RecentFilter) -> Self {
        Self {
            builder: self.builder.property("filter", filter.clone()),
        }
    }

    pub fn limit(self, limit: i32) -> Self {
        Self {
            builder: self.builder.property("limit", limit),
        }
    }

    pub fn local_only(self, local_only: bool) -> Self {
        Self {
            builder: self.builder.property("local-only", local_only),
        }
    }

    pub fn recent_manager(self, recent_manager: &impl IsA<RecentManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("recent-manager", recent_manager.clone().upcast()),
        }
    }

    pub fn select_multiple(self, select_multiple: bool) -> Self {
        Self {
            builder: self.builder.property("select-multiple", select_multiple),
        }
    }

    pub fn show_icons(self, show_icons: bool) -> Self {
        Self {
            builder: self.builder.property("show-icons", show_icons),
        }
    }

    pub fn show_not_found(self, show_not_found: bool) -> Self {
        Self {
            builder: self.builder.property("show-not-found", show_not_found),
        }
    }

    pub fn show_private(self, show_private: bool) -> Self {
        Self {
            builder: self.builder.property("show-private", show_private),
        }
    }

    pub fn show_tips(self, show_tips: bool) -> Self {
        Self {
            builder: self.builder.property("show-tips", show_tips),
        }
    }

    pub fn sort_type(self, sort_type: RecentSortType) -> Self {
        Self {
            builder: self.builder.property("sort-type", sort_type),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`RecentChooserWidget`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> RecentChooserWidget {
        self.builder.build()
    }
}

impl fmt::Display for RecentChooserWidget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("RecentChooserWidget")
    }
}
