// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Buildable;
use crate::SizeGroupMode;
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
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkSizeGroup")]
    pub struct SizeGroup(Object<ffi::GtkSizeGroup, ffi::GtkSizeGroupClass>) @implements Buildable;

    match fn {
        type_ => || ffi::gtk_size_group_get_type(),
    }
}

impl SizeGroup {
    #[doc(alias = "gtk_size_group_new")]
    pub fn new(mode: SizeGroupMode) -> SizeGroup {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_size_group_new(mode.into_glib())) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`SizeGroup`].
    ///
    /// This method returns an instance of [`SizeGroupBuilder`] which can be used to create a [`SizeGroup`].
    pub fn builder() -> SizeGroupBuilder {
        SizeGroupBuilder::default()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`SizeGroup`].
pub struct SizeGroupBuilder {
    #[cfg_attr(feature = "v3_22", deprecated = "Since 3.22")]
    ignore_hidden: Option<bool>,
    mode: Option<SizeGroupMode>,
}

impl SizeGroupBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`SizeGroupBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`SizeGroup`].
    pub fn build(self) -> SizeGroup {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref ignore_hidden) = self.ignore_hidden {
            properties.push(("ignore-hidden", ignore_hidden));
        }
        if let Some(ref mode) = self.mode {
            properties.push(("mode", mode));
        }
        glib::Object::new::<SizeGroup>(&properties)
            .expect("Failed to create an instance of SizeGroup")
    }

    #[cfg_attr(feature = "v3_22", deprecated = "Since 3.22")]
    pub fn ignore_hidden(mut self, ignore_hidden: bool) -> Self {
        self.ignore_hidden = Some(ignore_hidden);
        self
    }

    pub fn mode(mut self, mode: SizeGroupMode) -> Self {
        self.mode = Some(mode);
        self
    }
}

pub const NONE_SIZE_GROUP: Option<&SizeGroup> = None;

pub trait SizeGroupExt: 'static {
    #[doc(alias = "gtk_size_group_add_widget")]
    fn add_widget<P: IsA<Widget>>(&self, widget: &P);

    #[cfg_attr(feature = "v3_22", deprecated = "Since 3.22")]
    #[doc(alias = "gtk_size_group_get_ignore_hidden")]
    #[doc(alias = "get_ignore_hidden")]
    fn ignores_hidden(&self) -> bool;

    #[doc(alias = "gtk_size_group_get_mode")]
    #[doc(alias = "get_mode")]
    fn mode(&self) -> SizeGroupMode;

    #[doc(alias = "gtk_size_group_get_widgets")]
    #[doc(alias = "get_widgets")]
    fn widgets(&self) -> Vec<Widget>;

    #[doc(alias = "gtk_size_group_remove_widget")]
    fn remove_widget<P: IsA<Widget>>(&self, widget: &P);

    #[cfg_attr(feature = "v3_22", deprecated = "Since 3.22")]
    #[doc(alias = "gtk_size_group_set_ignore_hidden")]
    fn set_ignore_hidden(&self, ignore_hidden: bool);

    #[doc(alias = "gtk_size_group_set_mode")]
    fn set_mode(&self, mode: SizeGroupMode);

    #[cfg_attr(feature = "v3_22", deprecated = "Since 3.22")]
    #[doc(alias = "ignore-hidden")]
    fn connect_ignore_hidden_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "mode")]
    fn connect_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SizeGroup>> SizeGroupExt for O {
    fn add_widget<P: IsA<Widget>>(&self, widget: &P) {
        unsafe {
            ffi::gtk_size_group_add_widget(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            );
        }
    }

    fn ignores_hidden(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_size_group_get_ignore_hidden(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn mode(&self) -> SizeGroupMode {
        unsafe { from_glib(ffi::gtk_size_group_get_mode(self.as_ref().to_glib_none().0)) }
    }

    fn widgets(&self) -> Vec<Widget> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_size_group_get_widgets(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove_widget<P: IsA<Widget>>(&self, widget: &P) {
        unsafe {
            ffi::gtk_size_group_remove_widget(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_ignore_hidden(&self, ignore_hidden: bool) {
        unsafe {
            ffi::gtk_size_group_set_ignore_hidden(
                self.as_ref().to_glib_none().0,
                ignore_hidden.into_glib(),
            );
        }
    }

    fn set_mode(&self, mode: SizeGroupMode) {
        unsafe {
            ffi::gtk_size_group_set_mode(self.as_ref().to_glib_none().0, mode.into_glib());
        }
    }

    fn connect_ignore_hidden_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ignore_hidden_trampoline<
            P: IsA<SizeGroup>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkSizeGroup,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SizeGroup::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ignore-hidden\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ignore_hidden_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mode_trampoline<P: IsA<SizeGroup>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSizeGroup,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SizeGroup::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SizeGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SizeGroup")
    }
}
