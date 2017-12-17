// This file was generated by gir (5a68ad0) from gir-files (469db10)
// DO NOT EDIT

use Error;
use Pixbuf;
use PixbufAnimation;
use PixbufFormat;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct PixbufLoader(Object<ffi::GdkPixbufLoader, ffi::GdkPixbufLoaderClass>);

    match fn {
        get_type => || ffi::gdk_pixbuf_loader_get_type(),
    }
}

impl PixbufLoader {
    pub fn new() -> PixbufLoader {
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_loader_new())
        }
    }

    pub fn new_with_mime_type(mime_type: &str) -> Result<PixbufLoader, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gdk_pixbuf_loader_new_with_mime_type(mime_type.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn new_with_type(image_type: &str) -> Result<PixbufLoader, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gdk_pixbuf_loader_new_with_type(image_type.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

impl Default for PixbufLoader {
    fn default() -> Self {
        Self::new()
    }
}

pub trait PixbufLoaderExt {
    fn close(&self) -> Result<(), Error>;

    fn get_animation(&self) -> Option<PixbufAnimation>;

    fn get_format(&self) -> Option<PixbufFormat>;

    fn get_pixbuf(&self) -> Option<Pixbuf>;

    fn set_size(&self, width: i32, height: i32);

    fn write(&self, buf: &[u8]) -> Result<(), Error>;

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    fn write_bytes(&self, buffer: &glib::Bytes) -> Result<(), Error>;

    fn connect_area_prepared<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_area_updated<F: Fn(&Self, i32, i32, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_size_prepared<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<PixbufLoader> + IsA<glib::object::Object>> PixbufLoaderExt for O {
    fn close(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gdk_pixbuf_loader_close(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_animation(&self) -> Option<PixbufAnimation> {
        unsafe {
            from_glib_none(ffi::gdk_pixbuf_loader_get_animation(self.to_glib_none().0))
        }
    }

    fn get_format(&self) -> Option<PixbufFormat> {
        unsafe {
            from_glib_none(ffi::gdk_pixbuf_loader_get_format(self.to_glib_none().0))
        }
    }

    fn get_pixbuf(&self) -> Option<Pixbuf> {
        unsafe {
            from_glib_none(ffi::gdk_pixbuf_loader_get_pixbuf(self.to_glib_none().0))
        }
    }

    fn set_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gdk_pixbuf_loader_set_size(self.to_glib_none().0, width, height);
        }
    }

    fn write(&self, buf: &[u8]) -> Result<(), Error> {
        let count = buf.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gdk_pixbuf_loader_write(self.to_glib_none().0, buf.to_glib_none().0, count, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    fn write_bytes(&self, buffer: &glib::Bytes) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gdk_pixbuf_loader_write_bytes(self.to_glib_none().0, buffer.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_area_prepared<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "area-prepared",
                transmute(area_prepared_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_area_updated<F: Fn(&Self, i32, i32, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32, i32, i32, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "area-updated",
                transmute(area_updated_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "closed",
                transmute(closed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_size_prepared<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "size-prepared",
                transmute(size_prepared_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn area_prepared_trampoline<P>(this: *mut ffi::GdkPixbufLoader, f: glib_ffi::gpointer)
where P: IsA<PixbufLoader> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PixbufLoader::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn area_updated_trampoline<P>(this: *mut ffi::GdkPixbufLoader, x: libc::c_int, y: libc::c_int, width: libc::c_int, height: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<PixbufLoader> {
    callback_guard!();
    let f: &&(Fn(&P, i32, i32, i32, i32) + 'static) = transmute(f);
    f(&PixbufLoader::from_glib_borrow(this).downcast_unchecked(), x, y, width, height)
}

unsafe extern "C" fn closed_trampoline<P>(this: *mut ffi::GdkPixbufLoader, f: glib_ffi::gpointer)
where P: IsA<PixbufLoader> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PixbufLoader::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn size_prepared_trampoline<P>(this: *mut ffi::GdkPixbufLoader, width: libc::c_int, height: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<PixbufLoader> {
    callback_guard!();
    let f: &&(Fn(&P, i32, i32) + 'static) = transmute(f);
    f(&PixbufLoader::from_glib_borrow(this).downcast_unchecked(), width, height)
}
