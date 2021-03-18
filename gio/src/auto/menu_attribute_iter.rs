// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    pub struct MenuAttributeIter(Object<ffi::GMenuAttributeIter, ffi::GMenuAttributeIterClass>);

    match fn {
        get_type => || ffi::g_menu_attribute_iter_get_type(),
    }
}

impl fmt::Display for MenuAttributeIter {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&MenuAttributeIterExt::get_name(self))
    }
}

pub const NONE_MENU_ATTRIBUTE_ITER: Option<&MenuAttributeIter> = None;

pub trait MenuAttributeIterExt: 'static {
    #[doc(alias = "g_menu_attribute_iter_get_name")]
    fn get_name(&self) -> glib::GString;

    #[doc(alias = "g_menu_attribute_iter_get_next")]
    fn get_next(&self) -> Option<(glib::GString, glib::Variant)>;

    #[doc(alias = "g_menu_attribute_iter_get_value")]
    fn get_value(&self) -> glib::Variant;

    #[doc(alias = "g_menu_attribute_iter_next")]
    fn next(&self) -> bool;
}

impl<O: IsA<MenuAttributeIter>> MenuAttributeIterExt for O {
    fn get_name(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::g_menu_attribute_iter_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_next(&self) -> Option<(glib::GString, glib::Variant)> {
        unsafe {
            let mut out_name = ptr::null();
            let mut value = ptr::null_mut();
            let ret = from_glib(ffi::g_menu_attribute_iter_get_next(
                self.as_ref().to_glib_none().0,
                &mut out_name,
                &mut value,
            ));
            if ret {
                Some((from_glib_none(out_name), from_glib_full(value)))
            } else {
                None
            }
        }
    }

    fn get_value(&self) -> glib::Variant {
        unsafe {
            from_glib_full(ffi::g_menu_attribute_iter_get_value(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn next(&self) -> bool {
        unsafe {
            from_glib(ffi::g_menu_attribute_iter_next(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}
