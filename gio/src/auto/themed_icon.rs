// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Icon;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GThemedIcon")]
    pub struct ThemedIcon(Object<ffi::GThemedIcon, ffi::GThemedIconClass>) @implements Icon;

    match fn {
        type_ => || ffi::g_themed_icon_get_type(),
    }
}

impl ThemedIcon {
    #[doc(alias = "g_themed_icon_new")]
    pub fn new(iconname: &str) -> ThemedIcon {
        unsafe { from_glib_full(ffi::g_themed_icon_new(iconname.to_glib_none().0)) }
    }

    #[doc(alias = "g_themed_icon_new_from_names")]
    #[doc(alias = "new_from_names")]
    pub fn from_names(iconnames: &[&str]) -> ThemedIcon {
        let len = iconnames.len() as _;
        unsafe {
            from_glib_full(ffi::g_themed_icon_new_from_names(
                iconnames.to_glib_none().0,
                len,
            ))
        }
    }

    #[doc(alias = "g_themed_icon_new_with_default_fallbacks")]
    #[doc(alias = "new_with_default_fallbacks")]
    pub fn with_default_fallbacks(iconname: &str) -> ThemedIcon {
        unsafe {
            from_glib_full(ffi::g_themed_icon_new_with_default_fallbacks(
                iconname.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_themed_icon_append_name")]
    pub fn append_name(&self, iconname: &str) {
        unsafe {
            ffi::g_themed_icon_append_name(self.to_glib_none().0, iconname.to_glib_none().0);
        }
    }

    #[doc(alias = "g_themed_icon_get_names")]
    #[doc(alias = "get_names")]
    pub fn names(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_themed_icon_get_names(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_themed_icon_prepend_name")]
    pub fn prepend_name(&self, iconname: &str) {
        unsafe {
            ffi::g_themed_icon_prepend_name(self.to_glib_none().0, iconname.to_glib_none().0);
        }
    }

    #[doc(alias = "use-default-fallbacks")]
    pub fn uses_default_fallbacks(&self) -> bool {
        ObjectExt::property(self, "use-default-fallbacks")
    }

    #[doc(alias = "names")]
    pub fn connect_names_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_names_trampoline<F: Fn(&ThemedIcon) + 'static>(
            this: *mut ffi::GThemedIcon,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::names\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_names_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ThemedIcon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ThemedIcon")
    }
}
