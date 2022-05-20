// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::mem;

glib::wrapper! {
    #[doc(alias = "GtkEditable")]
    pub struct Editable(Interface<ffi::GtkEditable, ffi::GtkEditableInterface>);

    match fn {
        type_ => || ffi::gtk_editable_get_type(),
    }
}

impl Editable {
    pub const NONE: Option<&'static Editable> = None;
}

pub trait EditableExt: 'static {
    #[doc(alias = "gtk_editable_copy_clipboard")]
    fn copy_clipboard(&self);

    #[doc(alias = "gtk_editable_cut_clipboard")]
    fn cut_clipboard(&self);

    #[doc(alias = "gtk_editable_delete_selection")]
    fn delete_selection(&self);

    #[doc(alias = "gtk_editable_delete_text")]
    fn delete_text(&self, start_pos: i32, end_pos: i32);

    #[doc(alias = "gtk_editable_get_chars")]
    #[doc(alias = "get_chars")]
    fn chars(&self, start_pos: i32, end_pos: i32) -> Option<glib::GString>;

    #[doc(alias = "gtk_editable_get_editable")]
    #[doc(alias = "get_editable")]
    fn is_editable(&self) -> bool;

    #[doc(alias = "gtk_editable_get_position")]
    #[doc(alias = "get_position")]
    fn position(&self) -> i32;

    #[doc(alias = "gtk_editable_get_selection_bounds")]
    #[doc(alias = "get_selection_bounds")]
    fn selection_bounds(&self) -> Option<(i32, i32)>;

    #[doc(alias = "gtk_editable_insert_text")]
    fn insert_text(&self, new_text: &str, position: &mut i32);

    #[doc(alias = "gtk_editable_paste_clipboard")]
    fn paste_clipboard(&self);

    #[doc(alias = "gtk_editable_select_region")]
    fn select_region(&self, start_pos: i32, end_pos: i32);

    #[doc(alias = "gtk_editable_set_editable")]
    fn set_editable(&self, is_editable: bool);

    #[doc(alias = "gtk_editable_set_position")]
    fn set_position(&self, position: i32);
}

impl<O: IsA<Editable>> EditableExt for O {
    fn copy_clipboard(&self) {
        unsafe {
            ffi::gtk_editable_copy_clipboard(self.as_ref().to_glib_none().0);
        }
    }

    fn cut_clipboard(&self) {
        unsafe {
            ffi::gtk_editable_cut_clipboard(self.as_ref().to_glib_none().0);
        }
    }

    fn delete_selection(&self) {
        unsafe {
            ffi::gtk_editable_delete_selection(self.as_ref().to_glib_none().0);
        }
    }

    fn delete_text(&self, start_pos: i32, end_pos: i32) {
        unsafe {
            ffi::gtk_editable_delete_text(self.as_ref().to_glib_none().0, start_pos, end_pos);
        }
    }

    fn chars(&self, start_pos: i32, end_pos: i32) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_editable_get_chars(
                self.as_ref().to_glib_none().0,
                start_pos,
                end_pos,
            ))
        }
    }

    fn is_editable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_editable_get_editable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn position(&self) -> i32 {
        unsafe { ffi::gtk_editable_get_position(self.as_ref().to_glib_none().0) }
    }

    fn selection_bounds(&self) -> Option<(i32, i32)> {
        unsafe {
            let mut start_pos = mem::MaybeUninit::uninit();
            let mut end_pos = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_editable_get_selection_bounds(
                self.as_ref().to_glib_none().0,
                start_pos.as_mut_ptr(),
                end_pos.as_mut_ptr(),
            ));
            if ret {
                Some((start_pos.assume_init(), end_pos.assume_init()))
            } else {
                None
            }
        }
    }

    fn insert_text(&self, new_text: &str, position: &mut i32) {
        let new_text_length = new_text.len() as i32;
        unsafe {
            ffi::gtk_editable_insert_text(
                self.as_ref().to_glib_none().0,
                new_text.to_glib_none().0,
                new_text_length,
                position,
            );
        }
    }

    fn paste_clipboard(&self) {
        unsafe {
            ffi::gtk_editable_paste_clipboard(self.as_ref().to_glib_none().0);
        }
    }

    fn select_region(&self, start_pos: i32, end_pos: i32) {
        unsafe {
            ffi::gtk_editable_select_region(self.as_ref().to_glib_none().0, start_pos, end_pos);
        }
    }

    fn set_editable(&self, is_editable: bool) {
        unsafe {
            ffi::gtk_editable_set_editable(self.as_ref().to_glib_none().0, is_editable.into_glib());
        }
    }

    fn set_position(&self, position: i32) {
        unsafe {
            ffi::gtk_editable_set_position(self.as_ref().to_glib_none().0, position);
        }
    }
}

impl fmt::Display for Editable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Editable")
    }
}
