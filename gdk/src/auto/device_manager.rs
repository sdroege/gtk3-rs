// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Device, Display};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GdkDeviceManager")]
    pub struct DeviceManager(Object<ffi::GdkDeviceManager>);

    match fn {
        type_ => || ffi::gdk_device_manager_get_type(),
    }
}

impl DeviceManager {
    #[doc(alias = "gdk_device_manager_get_display")]
    #[doc(alias = "get_display")]
    pub fn display(&self) -> Option<Display> {
        unsafe { from_glib_none(ffi::gdk_device_manager_get_display(self.to_glib_none().0)) }
    }

    #[doc(alias = "device-added")]
    pub fn connect_device_added<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn device_added_trampoline<F: Fn(&DeviceManager, &Device) + 'static>(
            this: *mut ffi::GdkDeviceManager,
            device: *mut ffi::GdkDevice,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(device))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"device-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    device_added_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "device-changed")]
    pub fn connect_device_changed<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn device_changed_trampoline<F: Fn(&DeviceManager, &Device) + 'static>(
            this: *mut ffi::GdkDeviceManager,
            device: *mut ffi::GdkDevice,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(device))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"device-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    device_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "device-removed")]
    pub fn connect_device_removed<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn device_removed_trampoline<F: Fn(&DeviceManager, &Device) + 'static>(
            this: *mut ffi::GdkDeviceManager,
            device: *mut ffi::GdkDevice,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(device))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"device-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    device_removed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceManager")
    }
}
