// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use std;
use std::fmt;
#[cfg(any(feature = "v2_60", feature = "dox"))]
use std::mem;
use AppInfo;
use AppLaunchContext;

glib_wrapper! {
    pub struct DesktopAppInfo(Object<gio_sys::GDesktopAppInfo, gio_sys::GDesktopAppInfoClass, DesktopAppInfoClass>) @implements AppInfo;

    match fn {
        get_type => || gio_sys::g_desktop_app_info_get_type(),
    }
}

impl DesktopAppInfo {
    pub fn new(desktop_id: &str) -> Option<DesktopAppInfo> {
        unsafe { from_glib_full(gio_sys::g_desktop_app_info_new(desktop_id.to_glib_none().0)) }
    }

    pub fn new_from_filename<P: AsRef<std::path::Path>>(filename: P) -> Option<DesktopAppInfo> {
        unsafe {
            from_glib_full(gio_sys::g_desktop_app_info_new_from_filename(
                filename.as_ref().to_glib_none().0,
            ))
        }
    }

    pub fn new_from_keyfile(key_file: &glib::KeyFile) -> Option<DesktopAppInfo> {
        unsafe {
            from_glib_full(gio_sys::g_desktop_app_info_new_from_keyfile(
                key_file.to_glib_none().0,
            ))
        }
    }

    pub fn get_implementations(interface: &str) -> Vec<DesktopAppInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gio_sys::g_desktop_app_info_get_implementations(
                interface.to_glib_none().0,
            ))
        }
    }
}

pub const NONE_DESKTOP_APP_INFO: Option<&DesktopAppInfo> = None;

pub trait DesktopAppInfoExt: 'static {
    fn get_action_name(&self, action_name: &str) -> Option<GString>;

    fn get_boolean(&self, key: &str) -> bool;

    fn get_categories(&self) -> Option<GString>;

    fn get_filename(&self) -> Option<std::path::PathBuf>;

    fn get_generic_name(&self) -> Option<GString>;

    fn get_is_hidden(&self) -> bool;

    fn get_keywords(&self) -> Vec<GString>;

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn get_locale_string(&self, key: &str) -> Option<GString>;

    fn get_nodisplay(&self) -> bool;

    fn get_show_in(&self, desktop_env: Option<&str>) -> bool;

    fn get_startup_wm_class(&self) -> Option<GString>;

    fn get_string(&self, key: &str) -> Option<GString>;

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    fn get_string_list(&self, key: &str) -> Vec<GString>;

    fn has_key(&self, key: &str) -> bool;

    fn launch_action<P: IsA<AppLaunchContext>>(
        &self,
        action_name: &str,
        launch_context: Option<&P>,
    );

    //fn launch_uris_as_manager<P: IsA<AppLaunchContext>>(&self, uris: &[&str], launch_context: Option<&P>, spawn_flags: /*Ignored*/glib::SpawnFlags, user_setup: Option<Box_<dyn FnOnce() + 'static>>, pid_callback: /*Unimplemented*/FnMut(&DesktopAppInfo, /*Ignored*/glib::Pid), pid_callback_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> Result<(), Error>;

    //#[cfg(any(feature = "v2_58", feature = "dox"))]
    //fn launch_uris_as_manager_with_fds<P: IsA<AppLaunchContext>>(&self, uris: &[&str], launch_context: Option<&P>, spawn_flags: /*Ignored*/glib::SpawnFlags, user_setup: Option<Box_<dyn FnOnce() + 'static>>, pid_callback: /*Unimplemented*/FnMut(&DesktopAppInfo, /*Ignored*/glib::Pid), pid_callback_data: /*Unimplemented*/Option<Fundamental: Pointer>, stdin_fd: i32, stdout_fd: i32, stderr_fd: i32) -> Result<(), Error>;

    fn list_actions(&self) -> Vec<GString>;
}

impl<O: IsA<DesktopAppInfo>> DesktopAppInfoExt for O {
    fn get_action_name(&self, action_name: &str) -> Option<GString> {
        unsafe {
            from_glib_full(gio_sys::g_desktop_app_info_get_action_name(
                self.as_ref().to_glib_none().0,
                action_name.to_glib_none().0,
            ))
        }
    }

    fn get_boolean(&self, key: &str) -> bool {
        unsafe {
            from_glib(gio_sys::g_desktop_app_info_get_boolean(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    fn get_categories(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gio_sys::g_desktop_app_info_get_categories(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_filename(&self) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_none(gio_sys::g_desktop_app_info_get_filename(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_generic_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gio_sys::g_desktop_app_info_get_generic_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_is_hidden(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_desktop_app_info_get_is_hidden(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_keywords(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(gio_sys::g_desktop_app_info_get_keywords(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn get_locale_string(&self, key: &str) -> Option<GString> {
        unsafe {
            from_glib_full(gio_sys::g_desktop_app_info_get_locale_string(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    fn get_nodisplay(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_desktop_app_info_get_nodisplay(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_show_in(&self, desktop_env: Option<&str>) -> bool {
        unsafe {
            from_glib(gio_sys::g_desktop_app_info_get_show_in(
                self.as_ref().to_glib_none().0,
                desktop_env.to_glib_none().0,
            ))
        }
    }

    fn get_startup_wm_class(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gio_sys::g_desktop_app_info_get_startup_wm_class(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_string(&self, key: &str) -> Option<GString> {
        unsafe {
            from_glib_full(gio_sys::g_desktop_app_info_get_string(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    fn get_string_list(&self, key: &str) -> Vec<GString> {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(
                gio_sys::g_desktop_app_info_get_string_list(
                    self.as_ref().to_glib_none().0,
                    key.to_glib_none().0,
                    length.as_mut_ptr(),
                ),
                length.assume_init() as usize,
            );
            ret
        }
    }

    fn has_key(&self, key: &str) -> bool {
        unsafe {
            from_glib(gio_sys::g_desktop_app_info_has_key(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    fn launch_action<P: IsA<AppLaunchContext>>(
        &self,
        action_name: &str,
        launch_context: Option<&P>,
    ) {
        unsafe {
            gio_sys::g_desktop_app_info_launch_action(
                self.as_ref().to_glib_none().0,
                action_name.to_glib_none().0,
                launch_context.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    //fn launch_uris_as_manager<P: IsA<AppLaunchContext>>(&self, uris: &[&str], launch_context: Option<&P>, spawn_flags: /*Ignored*/glib::SpawnFlags, user_setup: Option<Box_<dyn FnOnce() + 'static>>, pid_callback: /*Unimplemented*/FnMut(&DesktopAppInfo, /*Ignored*/glib::Pid), pid_callback_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> Result<(), Error> {
    //    unsafe { TODO: call gio_sys:g_desktop_app_info_launch_uris_as_manager() }
    //}

    //#[cfg(any(feature = "v2_58", feature = "dox"))]
    //fn launch_uris_as_manager_with_fds<P: IsA<AppLaunchContext>>(&self, uris: &[&str], launch_context: Option<&P>, spawn_flags: /*Ignored*/glib::SpawnFlags, user_setup: Option<Box_<dyn FnOnce() + 'static>>, pid_callback: /*Unimplemented*/FnMut(&DesktopAppInfo, /*Ignored*/glib::Pid), pid_callback_data: /*Unimplemented*/Option<Fundamental: Pointer>, stdin_fd: i32, stdout_fd: i32, stderr_fd: i32) -> Result<(), Error> {
    //    unsafe { TODO: call gio_sys:g_desktop_app_info_launch_uris_as_manager_with_fds() }
    //}

    fn list_actions(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(gio_sys::g_desktop_app_info_list_actions(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for DesktopAppInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DesktopAppInfo")
    }
}
