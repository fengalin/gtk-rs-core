// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GListModel")]
    pub struct ListModel(Interface<ffi::GListModel, ffi::GListModelInterface>);

    match fn {
        type_ => || ffi::g_list_model_get_type(),
    }
}

impl ListModel {
    pub const NONE: Option<&'static ListModel> = None;
}

pub trait ListModelExt: IsA<ListModel> + 'static {
    #[doc(alias = "g_list_model_get_item_type")]
    #[doc(alias = "get_item_type")]
    fn item_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::g_list_model_get_item_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_list_model_get_n_items")]
    #[doc(alias = "get_n_items")]
    fn n_items(&self) -> u32 {
        unsafe { ffi::g_list_model_get_n_items(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "g_list_model_get_object")]
    #[doc(alias = "get_object")]
    fn item(&self, position: u32) -> Option<glib::Object> {
        unsafe {
            from_glib_full(ffi::g_list_model_get_object(
                self.as_ref().to_glib_none().0,
                position,
            ))
        }
    }

    #[doc(alias = "g_list_model_items_changed")]
    fn items_changed(&self, position: u32, removed: u32, added: u32) {
        unsafe {
            ffi::g_list_model_items_changed(
                self.as_ref().to_glib_none().0,
                position,
                removed,
                added,
            );
        }
    }

    #[doc(alias = "items-changed")]
    fn connect_items_changed<F: Fn(&Self, u32, u32, u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn items_changed_trampoline<
            P: IsA<ListModel>,
            F: Fn(&P, u32, u32, u32) + 'static,
        >(
            this: *mut ffi::GListModel,
            position: std::ffi::c_uint,
            removed: std::ffi::c_uint,
            added: std::ffi::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                ListModel::from_glib_borrow(this).unsafe_cast_ref(),
                position,
                removed,
                added,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"items-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    items_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<ListModel>> ListModelExt for O {}
