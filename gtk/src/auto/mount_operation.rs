// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Window;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkMountOperation")]
    pub struct MountOperation(Object<ffi::GtkMountOperation, ffi::GtkMountOperationClass>) @extends gio::MountOperation;

    match fn {
        type_ => || ffi::gtk_mount_operation_get_type(),
    }
}

impl MountOperation {
    pub const NONE: Option<&'static MountOperation> = None;

    #[doc(alias = "gtk_mount_operation_new")]
    pub fn new(parent: Option<&impl IsA<Window>>) -> MountOperation {
        assert_initialized_main_thread!();
        unsafe {
            gio::MountOperation::from_glib_full(ffi::gtk_mount_operation_new(
                parent.map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`MountOperation`] objects.
    ///
    /// This method returns an instance of [`MountOperationBuilder`](crate::builders::MountOperationBuilder) which can be used to create [`MountOperation`] objects.
    pub fn builder() -> MountOperationBuilder {
        MountOperationBuilder::new()
    }
}

impl Default for MountOperation {
    fn default() -> Self {
        glib::object::Object::new_default::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`MountOperation`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct MountOperationBuilder {
    builder: glib::object::ObjectBuilder<'static, MountOperation>,
}

impl MountOperationBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn parent(self, parent: &impl IsA<Window>) -> Self {
        Self {
            builder: self.builder.property("parent", parent.clone().upcast()),
        }
    }

    pub fn screen(self, screen: &gdk::Screen) -> Self {
        Self {
            builder: self.builder.property("screen", screen.clone()),
        }
    }

    pub fn anonymous(self, anonymous: bool) -> Self {
        Self {
            builder: self.builder.property("anonymous", anonymous),
        }
    }

    pub fn choice(self, choice: i32) -> Self {
        Self {
            builder: self.builder.property("choice", choice),
        }
    }

    pub fn domain(self, domain: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("domain", domain.into()),
        }
    }

    #[cfg(any(feature = "gio_v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_58")))]
    pub fn is_tcrypt_hidden_volume(self, is_tcrypt_hidden_volume: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("is-tcrypt-hidden-volume", is_tcrypt_hidden_volume),
        }
    }

    #[cfg(any(feature = "gio_v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_58")))]
    pub fn is_tcrypt_system_volume(self, is_tcrypt_system_volume: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("is-tcrypt-system-volume", is_tcrypt_system_volume),
        }
    }

    pub fn password(self, password: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("password", password.into()),
        }
    }

    pub fn password_save(self, password_save: gio::PasswordSave) -> Self {
        Self {
            builder: self.builder.property("password-save", password_save),
        }
    }

    #[cfg(any(feature = "gio_v2_58", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_58")))]
    pub fn pim(self, pim: u32) -> Self {
        Self {
            builder: self.builder.property("pim", pim),
        }
    }

    pub fn username(self, username: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("username", username.into()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`MountOperation`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> MountOperation {
        self.builder.build()
    }
}

pub trait MountOperationExt: 'static {
    #[doc(alias = "gtk_mount_operation_get_parent")]
    #[doc(alias = "get_parent")]
    fn parent(&self) -> Option<Window>;

    #[doc(alias = "gtk_mount_operation_get_screen")]
    #[doc(alias = "get_screen")]
    fn screen(&self) -> Option<gdk::Screen>;

    #[doc(alias = "gtk_mount_operation_is_showing")]
    fn is_showing(&self) -> bool;

    #[doc(alias = "gtk_mount_operation_set_parent")]
    fn set_parent(&self, parent: Option<&impl IsA<Window>>);

    #[doc(alias = "gtk_mount_operation_set_screen")]
    fn set_screen(&self, screen: &gdk::Screen);

    #[doc(alias = "is-showing")]
    fn connect_is_showing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "parent")]
    fn connect_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "screen")]
    fn connect_screen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MountOperation>> MountOperationExt for O {
    fn parent(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gtk_mount_operation_get_parent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn screen(&self) -> Option<gdk::Screen> {
        unsafe {
            from_glib_none(ffi::gtk_mount_operation_get_screen(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_showing(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_mount_operation_is_showing(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_parent(&self, parent: Option<&impl IsA<Window>>) {
        unsafe {
            ffi::gtk_mount_operation_set_parent(
                self.as_ref().to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_screen(&self, screen: &gdk::Screen) {
        unsafe {
            ffi::gtk_mount_operation_set_screen(
                self.as_ref().to_glib_none().0,
                screen.to_glib_none().0,
            );
        }
    }

    fn connect_is_showing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_showing_trampoline<
            P: IsA<MountOperation>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkMountOperation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-showing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_showing_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<
            P: IsA<MountOperation>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkMountOperation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::parent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_parent_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_screen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_screen_trampoline<
            P: IsA<MountOperation>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkMountOperation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(MountOperation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::screen\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_screen_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for MountOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MountOperation")
    }
}
