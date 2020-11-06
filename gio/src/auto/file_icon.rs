// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use File;
use Icon;
use LoadableIcon;

glib_wrapper! {
    pub struct FileIcon(Object<gio_sys::GFileIcon, gio_sys::GFileIconClass>) @implements Icon, LoadableIcon;

    match fn {
        get_type => || gio_sys::g_file_icon_get_type(),
    }
}

impl FileIcon {
    pub fn new<P: IsA<File>>(file: &P) -> FileIcon {
        unsafe { from_glib_full(gio_sys::g_file_icon_new(file.as_ref().to_glib_none().0)) }
    }

    pub fn get_file(&self) -> Option<File> {
        unsafe { from_glib_none(gio_sys::g_file_icon_get_file(self.to_glib_none().0)) }
    }
}

impl fmt::Display for FileIcon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileIcon")
    }
}
