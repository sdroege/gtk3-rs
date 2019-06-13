// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk_sys;
use glib::object::IsA;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use glib::object::ObjectType as ObjectType_;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use glib_sys;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use std::mem::transmute;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use Cursor;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use Device;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use DeviceTool;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use Display;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use Event;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use GrabStatus;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use SeatCapabilities;
use Window;

glib_wrapper! {
    pub struct Seat(Object<gdk_sys::GdkSeat, SeatClass>);

    match fn {
        get_type => || gdk_sys::gdk_seat_get_type(),
    }
}

impl Seat {
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn get_capabilities(&self) -> SeatCapabilities {
        unsafe { from_glib(gdk_sys::gdk_seat_get_capabilities(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn get_display(&self) -> Option<Display> {
        unsafe { from_glib_none(gdk_sys::gdk_seat_get_display(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn get_keyboard(&self) -> Option<Device> {
        unsafe { from_glib_none(gdk_sys::gdk_seat_get_keyboard(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn get_pointer(&self) -> Option<Device> {
        unsafe { from_glib_none(gdk_sys::gdk_seat_get_pointer(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn get_slaves(&self, capabilities: SeatCapabilities) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(gdk_sys::gdk_seat_get_slaves(
                self.to_glib_none().0,
                capabilities.to_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn grab<P: IsA<Window>>(
        &self,
        window: &P,
        capabilities: SeatCapabilities,
        owner_events: bool,
        cursor: Option<&Cursor>,
        event: Option<&Event>,
        prepare_func: Option<&mut dyn (FnMut(&Seat, &Window))>,
    ) -> GrabStatus {
        let prepare_func_data: Option<&mut dyn (FnMut(&Seat, &Window))> = prepare_func;
        unsafe extern "C" fn prepare_func_func<P: IsA<Window>>(
            seat: *mut gdk_sys::GdkSeat,
            window: *mut gdk_sys::GdkWindow,
            user_data: glib_sys::gpointer,
        ) {
            let seat = from_glib_borrow(seat);
            let window = from_glib_borrow(window);
            let callback: *mut Option<&mut dyn (FnMut(&Seat, &Window))> =
                user_data as *const _ as usize as *mut Option<&mut dyn (FnMut(&Seat, &Window))>;
            if let Some(ref mut callback) = *callback {
                callback(&seat, &window)
            } else {
                panic!("cannot get closure...")
            };
        }
        let prepare_func = if prepare_func_data.is_some() {
            Some(prepare_func_func::<P> as _)
        } else {
            None
        };
        let super_callback0: &Option<&mut dyn (FnMut(&Seat, &Window))> = &prepare_func_data;
        unsafe {
            from_glib(gdk_sys::gdk_seat_grab(
                self.to_glib_none().0,
                window.as_ref().to_glib_none().0,
                capabilities.to_glib(),
                owner_events.to_glib(),
                cursor.to_glib_none().0,
                event.to_glib_none().0,
                prepare_func,
                super_callback0 as *const _ as usize as *mut _,
            ))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn ungrab(&self) {
        unsafe {
            gdk_sys::gdk_seat_ungrab(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn connect_device_added<F: Fn(&Seat, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn device_added_trampoline<F: Fn(&Seat, &Device) + 'static>(
            this: *mut gdk_sys::GdkSeat,
            device: *mut gdk_sys::GdkDevice,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(device))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"device-added\0".as_ptr() as *const _,
                Some(transmute(device_added_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub fn connect_device_removed<F: Fn(&Seat, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn device_removed_trampoline<F: Fn(&Seat, &Device) + 'static>(
            this: *mut gdk_sys::GdkSeat,
            device: *mut gdk_sys::GdkDevice,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(device))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"device-removed\0".as_ptr() as *const _,
                Some(transmute(device_removed_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn connect_tool_added<F: Fn(&Seat, &DeviceTool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn tool_added_trampoline<F: Fn(&Seat, &DeviceTool) + 'static>(
            this: *mut gdk_sys::GdkSeat,
            tool: *mut gdk_sys::GdkDeviceTool,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(tool))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"tool-added\0".as_ptr() as *const _,
                Some(transmute(tool_added_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub fn connect_tool_removed<F: Fn(&Seat, &DeviceTool) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn tool_removed_trampoline<F: Fn(&Seat, &DeviceTool) + 'static>(
            this: *mut gdk_sys::GdkSeat,
            tool: *mut gdk_sys::GdkDeviceTool,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(tool))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"tool-removed\0".as_ptr() as *const _,
                Some(transmute(tool_removed_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Seat")
    }
}
