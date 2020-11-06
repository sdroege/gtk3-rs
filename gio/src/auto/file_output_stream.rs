// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::pin::Pin;
use std::ptr;
use Cancellable;
use FileInfo;
use OutputStream;
use Seekable;

glib_wrapper! {
    pub struct FileOutputStream(Object<gio_sys::GFileOutputStream, gio_sys::GFileOutputStreamClass>) @extends OutputStream, @implements Seekable;

    match fn {
        get_type => || gio_sys::g_file_output_stream_get_type(),
    }
}

pub const NONE_FILE_OUTPUT_STREAM: Option<&FileOutputStream> = None;

pub trait FileOutputStreamExt: 'static {
    fn get_etag(&self) -> Option<GString>;

    fn query_info<P: IsA<Cancellable>>(
        &self,
        attributes: &str,
        cancellable: Option<&P>,
    ) -> Result<FileInfo, glib::Error>;

    fn query_info_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<FileInfo, glib::Error>) + Send + 'static,
    >(
        &self,
        attributes: &str,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn query_info_async_future(
        &self,
        attributes: &str,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<FileInfo, glib::Error>> + 'static>>;
}

impl<O: IsA<FileOutputStream>> FileOutputStreamExt for O {
    fn get_etag(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gio_sys::g_file_output_stream_get_etag(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn query_info<P: IsA<Cancellable>>(
        &self,
        attributes: &str,
        cancellable: Option<&P>,
    ) -> Result<FileInfo, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_file_output_stream_query_info(
                self.as_ref().to_glib_none().0,
                attributes.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn query_info_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<FileInfo, glib::Error>) + Send + 'static,
    >(
        &self,
        attributes: &str,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn query_info_async_trampoline<
            Q: FnOnce(Result<FileInfo, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_file_output_stream_query_info_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = query_info_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_file_output_stream_query_info_async(
                self.as_ref().to_glib_none().0,
                attributes.to_glib_none().0,
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn query_info_async_future(
        &self,
        attributes: &str,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<FileInfo, glib::Error>> + 'static>> {
        let attributes = String::from(attributes);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.query_info_async(&attributes, io_priority, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }
}

impl fmt::Display for FileOutputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileOutputStream")
    }
}
