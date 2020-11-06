// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use atk_sys;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Action;
use Object;

glib_wrapper! {
    pub struct Hyperlink(Object<atk_sys::AtkHyperlink, atk_sys::AtkHyperlinkClass>) @implements Action;

    match fn {
        get_type => || atk_sys::atk_hyperlink_get_type(),
    }
}

pub const NONE_HYPERLINK: Option<&Hyperlink> = None;

pub trait HyperlinkExt: 'static {
    fn get_end_index(&self) -> i32;

    fn get_n_anchors(&self) -> i32;

    fn get_object(&self, i: i32) -> Option<Object>;

    fn get_start_index(&self) -> i32;

    fn get_uri(&self, i: i32) -> Option<GString>;

    fn is_inline(&self) -> bool;

    fn is_valid(&self) -> bool;

    fn get_property_number_of_anchors(&self) -> i32;

    fn connect_link_activated<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_end_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_number_of_anchors_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_start_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Hyperlink>> HyperlinkExt for O {
    fn get_end_index(&self) -> i32 {
        unsafe { atk_sys::atk_hyperlink_get_end_index(self.as_ref().to_glib_none().0) }
    }

    fn get_n_anchors(&self) -> i32 {
        unsafe { atk_sys::atk_hyperlink_get_n_anchors(self.as_ref().to_glib_none().0) }
    }

    fn get_object(&self, i: i32) -> Option<Object> {
        unsafe {
            from_glib_none(atk_sys::atk_hyperlink_get_object(
                self.as_ref().to_glib_none().0,
                i,
            ))
        }
    }

    fn get_start_index(&self) -> i32 {
        unsafe { atk_sys::atk_hyperlink_get_start_index(self.as_ref().to_glib_none().0) }
    }

    fn get_uri(&self, i: i32) -> Option<GString> {
        unsafe {
            from_glib_full(atk_sys::atk_hyperlink_get_uri(
                self.as_ref().to_glib_none().0,
                i,
            ))
        }
    }

    fn is_inline(&self) -> bool {
        unsafe {
            from_glib(atk_sys::atk_hyperlink_is_inline(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_valid(&self) -> bool {
        unsafe {
            from_glib(atk_sys::atk_hyperlink_is_valid(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_property_number_of_anchors(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"number-of-anchors\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `number-of-anchors` getter")
                .unwrap()
        }
    }

    fn connect_link_activated<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn link_activated_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut atk_sys::AtkHyperlink,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Hyperlink>,
        {
            let f: &F = &*(f as *const F);
            f(&Hyperlink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"link-activated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    link_activated_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_end_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_end_index_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut atk_sys::AtkHyperlink,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Hyperlink>,
        {
            let f: &F = &*(f as *const F);
            f(&Hyperlink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::end-index\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_end_index_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_number_of_anchors_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_number_of_anchors_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut atk_sys::AtkHyperlink,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Hyperlink>,
        {
            let f: &F = &*(f as *const F);
            f(&Hyperlink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::number-of-anchors\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_number_of_anchors_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_start_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_start_index_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut atk_sys::AtkHyperlink,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Hyperlink>,
        {
            let f: &F = &*(f as *const F);
            f(&Hyperlink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::start-index\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_start_index_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Hyperlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hyperlink")
    }
}
