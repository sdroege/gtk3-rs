// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::value::SetValueOptional;
use glib::GString;
use glib::Value;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use InetAddress;
use SocketFamily;

glib_wrapper! {
    pub struct InetAddressMask(Object<gio_sys::GInetAddressMask, gio_sys::GInetAddressMaskClass>);

    match fn {
        get_type => || gio_sys::g_inet_address_mask_get_type(),
    }
}

impl InetAddressMask {
    pub fn new<P: IsA<InetAddress>>(addr: &P, length: u32) -> Result<InetAddressMask, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_inet_address_mask_new(
                addr.as_ref().to_glib_none().0,
                length,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn from_string(mask_string: &str) -> Result<InetAddressMask, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_inet_address_mask_new_from_string(
                mask_string.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

unsafe impl Send for InetAddressMask {}
unsafe impl Sync for InetAddressMask {}

pub const NONE_INET_ADDRESS_MASK: Option<&InetAddressMask> = None;

pub trait InetAddressMaskExt: 'static {
    fn equal<P: IsA<InetAddressMask>>(&self, mask2: &P) -> bool;

    fn get_address(&self) -> InetAddress;

    fn get_family(&self) -> SocketFamily;

    fn get_length(&self) -> u32;

    fn matches<P: IsA<InetAddress>>(&self, address: &P) -> bool;

    fn to_string(&self) -> GString;

    fn set_property_address<P: IsA<InetAddress> + SetValueOptional>(&self, address: Option<&P>);

    fn set_property_length(&self, length: u32);

    fn connect_property_address_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_family_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_length_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<InetAddressMask>> InetAddressMaskExt for O {
    fn equal<P: IsA<InetAddressMask>>(&self, mask2: &P) -> bool {
        unsafe {
            from_glib(gio_sys::g_inet_address_mask_equal(
                self.as_ref().to_glib_none().0,
                mask2.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_address(&self) -> InetAddress {
        unsafe {
            from_glib_none(gio_sys::g_inet_address_mask_get_address(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_family(&self) -> SocketFamily {
        unsafe {
            from_glib(gio_sys::g_inet_address_mask_get_family(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_length(&self) -> u32 {
        unsafe { gio_sys::g_inet_address_mask_get_length(self.as_ref().to_glib_none().0) }
    }

    fn matches<P: IsA<InetAddress>>(&self, address: &P) -> bool {
        unsafe {
            from_glib(gio_sys::g_inet_address_mask_matches(
                self.as_ref().to_glib_none().0,
                address.as_ref().to_glib_none().0,
            ))
        }
    }

    fn to_string(&self) -> GString {
        unsafe {
            from_glib_full(gio_sys::g_inet_address_mask_to_string(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_property_address<P: IsA<InetAddress> + SetValueOptional>(&self, address: Option<&P>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"address\0".as_ptr() as *const _,
                Value::from(address).to_glib_none().0,
            );
        }
    }

    fn set_property_length(&self, length: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"length\0".as_ptr() as *const _,
                Value::from(&length).to_glib_none().0,
            );
        }
    }

    fn connect_property_address_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_address_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gio_sys::GInetAddressMask,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<InetAddressMask>,
        {
            let f: &F = &*(f as *const F);
            f(&InetAddressMask::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::address\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_address_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_family_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_family_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gio_sys::GInetAddressMask,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<InetAddressMask>,
        {
            let f: &F = &*(f as *const F);
            f(&InetAddressMask::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::family\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_family_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_length_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_length_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gio_sys::GInetAddressMask,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<InetAddressMask>,
        {
            let f: &F = &*(f as *const F);
            f(&InetAddressMask::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::length\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_length_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for InetAddressMask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InetAddressMask")
    }
}
