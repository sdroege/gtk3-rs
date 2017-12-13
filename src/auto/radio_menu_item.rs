// This file was generated by gir (13f739b) from gir-files (db49619)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use CheckMenuItem;
use Container;
use MenuItem;
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct RadioMenuItem(Object<ffi::GtkRadioMenuItem, ffi::GtkRadioMenuItemClass>): CheckMenuItem, MenuItem, Bin, Container, Widget, Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_radio_menu_item_get_type(),
    }
}

impl RadioMenuItem {
    pub fn new_from_widget(group: &RadioMenuItem) -> RadioMenuItem {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_menu_item_new_from_widget(group.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_label_from_widget<'a, P: Into<Option<&'a str>>>(group: &RadioMenuItem, label: P) -> RadioMenuItem {
        skip_assert_initialized!();
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_menu_item_new_with_label_from_widget(group.to_glib_none().0, label.0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic_from_widget<'a, P: Into<Option<&'a str>>>(group: &RadioMenuItem, label: P) -> RadioMenuItem {
        skip_assert_initialized!();
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_radio_menu_item_new_with_mnemonic_from_widget(group.to_glib_none().0, label.0)).downcast_unchecked()
        }
    }
}

pub trait RadioMenuItemExt {
    fn get_group(&self) -> Vec<RadioMenuItem>;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn join_group<'a, P: Into<Option<&'a RadioMenuItem>>>(&self, group_source: P);

    fn connect_group_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RadioMenuItem> + IsA<glib::object::Object>> RadioMenuItemExt for O {
    fn get_group(&self) -> Vec<RadioMenuItem> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_radio_menu_item_get_group(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn join_group<'a, P: Into<Option<&'a RadioMenuItem>>>(&self, group_source: P) {
        let group_source = group_source.into();
        let group_source = group_source.to_glib_none();
        unsafe {
            ffi::gtk_radio_menu_item_join_group(self.to_glib_none().0, group_source.0);
        }
    }

    fn connect_group_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "group-changed",
                transmute(group_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn group_changed_trampoline<P>(this: *mut ffi::GtkRadioMenuItem, f: glib_ffi::gpointer)
where P: IsA<RadioMenuItem> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&RadioMenuItem::from_glib_borrow(this).downcast_unchecked())
}
