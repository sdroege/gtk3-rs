// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{X11DeviceCore, X11DeviceManagerCore, X11Display, X11Window};
use glib::{prelude::*, translate::*};
use x11::xlib;

#[doc(alias = "gdk_x11_atom_to_xatom")]
pub fn x11_atom_to_xatom(atom: &gdk::Atom) -> xlib::Atom {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_x11_atom_to_xatom(atom.to_glib_none().0) }
}

#[doc(alias = "gdk_x11_atom_to_xatom_for_display")]
pub fn x11_atom_to_xatom_for_display(display: &X11Display, atom: &gdk::Atom) -> xlib::Atom {
    skip_assert_initialized!();
    unsafe {
        ffi::gdk_x11_atom_to_xatom_for_display(display.to_glib_none().0, atom.to_glib_none().0)
    }
}

#[doc(alias = "gdk_x11_device_get_id")]
pub fn x11_device_get_id(device: &X11DeviceCore) -> i32 {
    skip_assert_initialized!();
    unsafe { ffi::gdk_x11_device_get_id(device.to_glib_none().0) }
}

#[doc(alias = "gdk_x11_device_manager_lookup")]
pub fn x11_device_manager_lookup(
    device_manager: &impl IsA<X11DeviceManagerCore>,
    device_id: i32,
) -> Option<X11DeviceCore> {
    skip_assert_initialized!();
    unsafe {
        from_glib_none(ffi::gdk_x11_device_manager_lookup(
            device_manager.as_ref().to_glib_none().0,
            device_id,
        ))
    }
}

#[doc(alias = "gdk_x11_get_default_root_xwindow")]
pub fn x11_get_default_root_xwindow() -> xlib::Window {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_x11_get_default_root_xwindow() }
}

#[doc(alias = "gdk_x11_get_default_screen")]
pub fn x11_get_default_screen() -> i32 {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_x11_get_default_screen() }
}

//#[cfg_attr(feature = "v3_24", deprecated = "Since 3.24")]
//#[cfg(any(feature = "v3_24_2", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24_2")))]
//#[allow(deprecated)]
//#[doc(alias = "gdk_x11_get_parent_relative_pattern")]
//pub fn x11_get_parent_relative_pattern() -> /*Ignored*/Option<cairo::Pattern> {
//    unsafe { TODO: call ffi:gdk_x11_get_parent_relative_pattern() }
//}

#[doc(alias = "gdk_x11_get_server_time")]
pub fn x11_get_server_time(window: &X11Window) -> u32 {
    skip_assert_initialized!();
    unsafe { ffi::gdk_x11_get_server_time(window.to_glib_none().0) }
}

#[doc(alias = "gdk_x11_get_xatom_by_name")]
pub fn x11_get_xatom_by_name(atom_name: &str) -> xlib::Atom {
    assert_initialized_main_thread!();
    unsafe { ffi::gdk_x11_get_xatom_by_name(atom_name.to_glib_none().0) }
}

#[doc(alias = "gdk_x11_get_xatom_by_name_for_display")]
pub fn x11_get_xatom_by_name_for_display(display: &X11Display, atom_name: &str) -> xlib::Atom {
    skip_assert_initialized!();
    unsafe {
        ffi::gdk_x11_get_xatom_by_name_for_display(
            display.to_glib_none().0,
            atom_name.to_glib_none().0,
        )
    }
}

#[doc(alias = "gdk_x11_get_xatom_name")]
pub fn x11_get_xatom_name(xatom: xlib::Atom) -> Option<glib::GString> {
    assert_initialized_main_thread!();
    unsafe { from_glib_none(ffi::gdk_x11_get_xatom_name(xatom)) }
}

#[doc(alias = "gdk_x11_get_xatom_name_for_display")]
pub fn x11_get_xatom_name_for_display(
    display: &X11Display,
    xatom: xlib::Atom,
) -> Option<glib::GString> {
    skip_assert_initialized!();
    unsafe {
        from_glib_none(ffi::gdk_x11_get_xatom_name_for_display(
            display.to_glib_none().0,
            xatom,
        ))
    }
}

#[doc(alias = "gdk_x11_grab_server")]
pub fn x11_grab_server() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_x11_grab_server();
    }
}

#[doc(alias = "gdk_x11_register_standard_event_type")]
pub fn x11_register_standard_event_type(display: &X11Display, event_base: i32, n_events: i32) {
    skip_assert_initialized!();
    unsafe {
        ffi::gdk_x11_register_standard_event_type(display.to_glib_none().0, event_base, n_events);
    }
}

#[doc(alias = "gdk_x11_set_sm_client_id")]
pub fn x11_set_sm_client_id(sm_client_id: Option<&str>) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_x11_set_sm_client_id(sm_client_id.to_glib_none().0);
    }
}

#[doc(alias = "gdk_x11_ungrab_server")]
pub fn x11_ungrab_server() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gdk_x11_ungrab_server();
    }
}

#[doc(alias = "gdk_x11_xatom_to_atom")]
pub fn x11_xatom_to_atom(xatom: xlib::Atom) -> Option<gdk::Atom> {
    assert_initialized_main_thread!();
    unsafe { from_glib_none(ffi::gdk_x11_xatom_to_atom(xatom)) }
}

#[doc(alias = "gdk_x11_xatom_to_atom_for_display")]
pub fn x11_xatom_to_atom_for_display(display: &X11Display, xatom: xlib::Atom) -> Option<gdk::Atom> {
    skip_assert_initialized!();
    unsafe {
        from_glib_none(ffi::gdk_x11_xatom_to_atom_for_display(
            display.to_glib_none().0,
            xatom,
        ))
    }
}
