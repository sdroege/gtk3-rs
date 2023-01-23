// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::prelude::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkHeaderBarAccessible")]
    pub struct HeaderBarAccessible(Object<ffi::GtkHeaderBarAccessible, ffi::GtkHeaderBarAccessibleClass>) @extends atk::Object;

    match fn {
        type_ => || ffi::gtk_header_bar_accessible_get_type(),
    }
}

impl HeaderBarAccessible {
    pub const NONE: Option<&'static HeaderBarAccessible> = None;

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`HeaderBarAccessible`] objects.
    ///
    /// This method returns an instance of [`HeaderBarAccessibleBuilder`](crate::builders::HeaderBarAccessibleBuilder) which can be used to create [`HeaderBarAccessible`] objects.
    pub fn builder() -> HeaderBarAccessibleBuilder {
        HeaderBarAccessibleBuilder::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`HeaderBarAccessible`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct HeaderBarAccessibleBuilder {
    builder: glib::object::ObjectBuilder<'static, HeaderBarAccessible>,
}

impl HeaderBarAccessibleBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn accessible_description(self, accessible_description: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("accessible-description", accessible_description.into()),
        }
    }

    pub fn accessible_name(self, accessible_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("accessible-name", accessible_name.into()),
        }
    }

    pub fn accessible_parent(self, accessible_parent: &impl IsA<atk::Object>) -> Self {
        Self {
            builder: self
                .builder
                .property("accessible-parent", accessible_parent.clone().upcast()),
        }
    }

    pub fn accessible_role(self, accessible_role: atk::Role) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    pub fn accessible_table_caption(
        self,
        accessible_table_caption: impl Into<glib::GString>,
    ) -> Self {
        Self {
            builder: self
                .builder
                .property("accessible-table-caption", accessible_table_caption.into()),
        }
    }

    pub fn accessible_table_caption_object(
        self,
        accessible_table_caption_object: &impl IsA<atk::Object>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "accessible-table-caption-object",
                accessible_table_caption_object.clone().upcast(),
            ),
        }
    }

    pub fn accessible_table_column_description(
        self,
        accessible_table_column_description: impl Into<glib::GString>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "accessible-table-column-description",
                accessible_table_column_description.into(),
            ),
        }
    }

    pub fn accessible_table_column_header(
        self,
        accessible_table_column_header: &impl IsA<atk::Object>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "accessible-table-column-header",
                accessible_table_column_header.clone().upcast(),
            ),
        }
    }

    pub fn accessible_table_row_description(
        self,
        accessible_table_row_description: impl Into<glib::GString>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "accessible-table-row-description",
                accessible_table_row_description.into(),
            ),
        }
    }

    pub fn accessible_table_row_header(
        self,
        accessible_table_row_header: &impl IsA<atk::Object>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "accessible-table-row-header",
                accessible_table_row_header.clone().upcast(),
            ),
        }
    }

    pub fn accessible_table_summary(
        self,
        accessible_table_summary: &impl IsA<atk::Object>,
    ) -> Self {
        Self {
            builder: self.builder.property(
                "accessible-table-summary",
                accessible_table_summary.clone().upcast(),
            ),
        }
    }

    pub fn accessible_value(self, accessible_value: f64) -> Self {
        Self {
            builder: self.builder.property("accessible-value", accessible_value),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`HeaderBarAccessible`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> HeaderBarAccessible {
        self.builder.build()
    }
}

impl fmt::Display for HeaderBarAccessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("HeaderBarAccessible")
    }
}
