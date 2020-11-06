// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v3_24", feature = "dox"))]
use glib::object::Cast;
#[cfg(any(feature = "v3_24", feature = "dox"))]
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use EventController;
#[cfg(any(feature = "v3_24", feature = "dox"))]
use EventControllerScrollFlags;
#[cfg(any(feature = "v3_24", feature = "dox"))]
use Widget;

glib_wrapper! {
    pub struct EventControllerScroll(Object<gtk_sys::GtkEventControllerScroll, gtk_sys::GtkEventControllerScrollClass>) @extends EventController;

    match fn {
        get_type => || gtk_sys::gtk_event_controller_scroll_get_type(),
    }
}

impl EventControllerScroll {
    #[cfg(any(feature = "v3_24", feature = "dox"))]
    pub fn new<P: IsA<Widget>>(
        widget: &P,
        flags: EventControllerScrollFlags,
    ) -> EventControllerScroll {
        skip_assert_initialized!();
        unsafe {
            EventController::from_glib_full(gtk_sys::gtk_event_controller_scroll_new(
                widget.as_ref().to_glib_none().0,
                flags.to_glib(),
            ))
            .unsafe_cast()
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    pub fn get_flags(&self) -> EventControllerScrollFlags {
        unsafe {
            from_glib(gtk_sys::gtk_event_controller_scroll_get_flags(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    pub fn set_flags(&self, flags: EventControllerScrollFlags) {
        unsafe {
            gtk_sys::gtk_event_controller_scroll_set_flags(self.to_glib_none().0, flags.to_glib());
        }
    }

    pub fn connect_decelerate<F: Fn(&EventControllerScroll, f64, f64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn decelerate_trampoline<
            F: Fn(&EventControllerScroll, f64, f64) + 'static,
        >(
            this: *mut gtk_sys::GtkEventControllerScroll,
            vel_x: libc::c_double,
            vel_y: libc::c_double,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), vel_x, vel_y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"decelerate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    decelerate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_scroll<F: Fn(&EventControllerScroll, f64, f64) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn scroll_trampoline<
            F: Fn(&EventControllerScroll, f64, f64) + 'static,
        >(
            this: *mut gtk_sys::GtkEventControllerScroll,
            dx: libc::c_double,
            dy: libc::c_double,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), dx, dy)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"scroll\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    scroll_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_scroll_begin<F: Fn(&EventControllerScroll) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn scroll_begin_trampoline<F: Fn(&EventControllerScroll) + 'static>(
            this: *mut gtk_sys::GtkEventControllerScroll,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"scroll-begin\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    scroll_begin_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_scroll_end<F: Fn(&EventControllerScroll) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn scroll_end_trampoline<F: Fn(&EventControllerScroll) + 'static>(
            this: *mut gtk_sys::GtkEventControllerScroll,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"scroll-end\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    scroll_end_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    pub fn connect_property_flags_notify<F: Fn(&EventControllerScroll) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_flags_trampoline<F: Fn(&EventControllerScroll) + 'static>(
            this: *mut gtk_sys::GtkEventControllerScroll,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_flags_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for EventControllerScroll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EventControllerScroll")
    }
}
