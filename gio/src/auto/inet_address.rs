// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use SocketFamily;

glib_wrapper! {
    pub struct InetAddress(Object<gio_sys::GInetAddress, gio_sys::GInetAddressClass>);

    match fn {
        get_type => || gio_sys::g_inet_address_get_type(),
    }
}

impl InetAddress {
    pub fn new_any(family: SocketFamily) -> InetAddress {
        unsafe { from_glib_full(gio_sys::g_inet_address_new_any(family.to_glib())) }
    }

    pub fn from_string(string: &str) -> Option<InetAddress> {
        unsafe {
            from_glib_full(gio_sys::g_inet_address_new_from_string(
                string.to_glib_none().0,
            ))
        }
    }

    pub fn new_loopback(family: SocketFamily) -> InetAddress {
        unsafe { from_glib_full(gio_sys::g_inet_address_new_loopback(family.to_glib())) }
    }
}

unsafe impl Send for InetAddress {}
unsafe impl Sync for InetAddress {}

pub const NONE_INET_ADDRESS: Option<&InetAddress> = None;

pub trait InetAddressExt: 'static {
    fn equal<P: IsA<InetAddress>>(&self, other_address: &P) -> bool;

    fn get_family(&self) -> SocketFamily;

    fn get_is_any(&self) -> bool;

    fn get_is_link_local(&self) -> bool;

    fn get_is_loopback(&self) -> bool;

    fn get_is_mc_global(&self) -> bool;

    fn get_is_mc_link_local(&self) -> bool;

    fn get_is_mc_node_local(&self) -> bool;

    fn get_is_mc_org_local(&self) -> bool;

    fn get_is_mc_site_local(&self) -> bool;

    fn get_is_multicast(&self) -> bool;

    fn get_is_site_local(&self) -> bool;

    fn get_native_size(&self) -> usize;

    fn to_string(&self) -> GString;

    //fn get_property_bytes(&self) -> /*Unimplemented*/Fundamental: Pointer;

    fn connect_property_is_any_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_is_link_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_is_loopback_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_is_mc_global_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_is_mc_link_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_is_mc_node_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_is_mc_org_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_is_mc_site_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_is_multicast_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_is_site_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<InetAddress>> InetAddressExt for O {
    fn equal<P: IsA<InetAddress>>(&self, other_address: &P) -> bool {
        unsafe {
            from_glib(gio_sys::g_inet_address_equal(
                self.as_ref().to_glib_none().0,
                other_address.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_family(&self) -> SocketFamily {
        unsafe {
            from_glib(gio_sys::g_inet_address_get_family(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_is_any(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_inet_address_get_is_any(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_is_link_local(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_inet_address_get_is_link_local(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_is_loopback(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_inet_address_get_is_loopback(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_is_mc_global(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_inet_address_get_is_mc_global(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_is_mc_link_local(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_inet_address_get_is_mc_link_local(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_is_mc_node_local(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_inet_address_get_is_mc_node_local(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_is_mc_org_local(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_inet_address_get_is_mc_org_local(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_is_mc_site_local(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_inet_address_get_is_mc_site_local(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_is_multicast(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_inet_address_get_is_multicast(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_is_site_local(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_inet_address_get_is_site_local(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_native_size(&self) -> usize {
        unsafe { gio_sys::g_inet_address_get_native_size(self.as_ref().to_glib_none().0) }
    }

    fn to_string(&self) -> GString {
        unsafe {
            from_glib_full(gio_sys::g_inet_address_to_string(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn get_property_bytes(&self) -> /*Unimplemented*/Fundamental: Pointer {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"bytes\0".as_ptr() as *const _, value.to_glib_none_mut().0);
    //        value.get().expect("Return Value for property `bytes` getter").unwrap()
    //    }
    //}

    fn connect_property_is_any_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_any_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gio_sys::GInetAddress,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<InetAddress>,
        {
            let f: &F = &*(f as *const F);
            f(&InetAddress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-any\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_any_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_is_link_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_link_local_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gio_sys::GInetAddress,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<InetAddress>,
        {
            let f: &F = &*(f as *const F);
            f(&InetAddress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-link-local\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_link_local_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_is_loopback_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_loopback_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gio_sys::GInetAddress,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<InetAddress>,
        {
            let f: &F = &*(f as *const F);
            f(&InetAddress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-loopback\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_loopback_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_is_mc_global_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_mc_global_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gio_sys::GInetAddress,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<InetAddress>,
        {
            let f: &F = &*(f as *const F);
            f(&InetAddress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-mc-global\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_mc_global_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_is_mc_link_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_mc_link_local_trampoline<
            P,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut gio_sys::GInetAddress,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<InetAddress>,
        {
            let f: &F = &*(f as *const F);
            f(&InetAddress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-mc-link-local\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_mc_link_local_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_is_mc_node_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_mc_node_local_trampoline<
            P,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut gio_sys::GInetAddress,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<InetAddress>,
        {
            let f: &F = &*(f as *const F);
            f(&InetAddress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-mc-node-local\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_mc_node_local_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_is_mc_org_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_mc_org_local_trampoline<
            P,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut gio_sys::GInetAddress,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<InetAddress>,
        {
            let f: &F = &*(f as *const F);
            f(&InetAddress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-mc-org-local\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_mc_org_local_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_is_mc_site_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_mc_site_local_trampoline<
            P,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut gio_sys::GInetAddress,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<InetAddress>,
        {
            let f: &F = &*(f as *const F);
            f(&InetAddress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-mc-site-local\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_mc_site_local_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_is_multicast_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_multicast_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gio_sys::GInetAddress,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<InetAddress>,
        {
            let f: &F = &*(f as *const F);
            f(&InetAddress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-multicast\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_multicast_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_is_site_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_site_local_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gio_sys::GInetAddress,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<InetAddress>,
        {
            let f: &F = &*(f as *const F);
            f(&InetAddress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-site-local\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_site_local_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for InetAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InetAddress")
    }
}
