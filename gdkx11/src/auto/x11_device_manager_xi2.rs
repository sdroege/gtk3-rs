// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use gdk_x11_sys;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use gobject_sys;
use std::fmt;
use X11DeviceManagerCore;

glib_wrapper! {
    pub struct X11DeviceManagerXI2(Object<gdk_x11_sys::GdkX11DeviceManagerXI2, gdk_x11_sys::GdkX11DeviceManagerXI2Class>) @extends X11DeviceManagerCore, gdk::DeviceManager;

    match fn {
        get_type => || gdk_x11_sys::gdk_x11_device_manager_xi2_get_type(),
    }
}

impl X11DeviceManagerXI2 {
    pub fn get_property_major(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"major\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `major` getter")
                .unwrap()
        }
    }

    pub fn get_property_minor(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"minor\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `minor` getter")
                .unwrap()
        }
    }

    pub fn get_property_opcode(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"opcode\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `opcode` getter")
                .unwrap()
        }
    }
}

#[derive(Clone, Default)]
pub struct X11DeviceManagerXI2Builder {
    major: Option<i32>,
    minor: Option<i32>,
    opcode: Option<i32>,
    display: Option<gdk::Display>,
}

impl X11DeviceManagerXI2Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> X11DeviceManagerXI2 {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref major) = self.major {
            properties.push(("major", major));
        }
        if let Some(ref minor) = self.minor {
            properties.push(("minor", minor));
        }
        if let Some(ref opcode) = self.opcode {
            properties.push(("opcode", opcode));
        }
        if let Some(ref display) = self.display {
            properties.push(("display", display));
        }
        let ret = glib::Object::new(X11DeviceManagerXI2::static_type(), &properties)
            .expect("object new")
            .downcast::<X11DeviceManagerXI2>()
            .expect("downcast");
        ret
    }

    pub fn major(mut self, major: i32) -> Self {
        self.major = Some(major);
        self
    }

    pub fn minor(mut self, minor: i32) -> Self {
        self.minor = Some(minor);
        self
    }

    pub fn opcode(mut self, opcode: i32) -> Self {
        self.opcode = Some(opcode);
        self
    }

    pub fn display<P: IsA<gdk::Display>>(mut self, display: &P) -> Self {
        self.display = Some(display.clone().upcast());
        self
    }
}

impl fmt::Display for X11DeviceManagerXI2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X11DeviceManagerXI2")
    }
}
