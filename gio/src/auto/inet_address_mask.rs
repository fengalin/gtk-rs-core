// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{InetAddress, Initable, SocketFamily};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute, ptr};

glib::wrapper! {
    #[doc(alias = "GInetAddressMask")]
    pub struct InetAddressMask(Object<ffi::GInetAddressMask, ffi::GInetAddressMaskClass>) @implements Initable;

    match fn {
        type_ => || ffi::g_inet_address_mask_get_type(),
    }
}

impl InetAddressMask {
    pub const NONE: Option<&'static InetAddressMask> = None;

    #[doc(alias = "g_inet_address_mask_new")]
    pub fn new(addr: &impl IsA<InetAddress>, length: u32) -> Result<InetAddressMask, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_inet_address_mask_new(addr.as_ref().to_glib_none().0, length, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_inet_address_mask_new_from_string")]
    #[doc(alias = "new_from_string")]
    pub fn from_string(mask_string: &str) -> Result<InetAddressMask, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_inet_address_mask_new_from_string(mask_string.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl fmt::Display for InetAddressMask {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&InetAddressMaskExt::to_str(self))
    }
}

unsafe impl Send for InetAddressMask {}
unsafe impl Sync for InetAddressMask {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::InetAddressMask>> Sealed for T {}
}

pub trait InetAddressMaskExt: IsA<InetAddressMask> + sealed::Sealed + 'static {
    #[doc(alias = "g_inet_address_mask_equal")]
    fn equal(&self, mask2: &impl IsA<InetAddressMask>) -> bool {
        unsafe {
            from_glib(ffi::g_inet_address_mask_equal(
                self.as_ref().to_glib_none().0,
                mask2.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_inet_address_mask_get_address")]
    #[doc(alias = "get_address")]
    fn address(&self) -> InetAddress {
        unsafe {
            from_glib_none(ffi::g_inet_address_mask_get_address(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_inet_address_mask_get_family")]
    #[doc(alias = "get_family")]
    fn family(&self) -> SocketFamily {
        unsafe {
            from_glib(ffi::g_inet_address_mask_get_family(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_inet_address_mask_get_length")]
    #[doc(alias = "get_length")]
    fn length(&self) -> u32 {
        unsafe { ffi::g_inet_address_mask_get_length(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "g_inet_address_mask_matches")]
    fn matches(&self, address: &impl IsA<InetAddress>) -> bool {
        unsafe {
            from_glib(ffi::g_inet_address_mask_matches(
                self.as_ref().to_glib_none().0,
                address.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_inet_address_mask_to_string")]
    #[doc(alias = "to_string")]
    fn to_str(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::g_inet_address_mask_to_string(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_address<P: IsA<InetAddress>>(&self, address: Option<&P>) {
        ObjectExt::set_property(self.as_ref(), "address", address)
    }

    fn set_length(&self, length: u32) {
        ObjectExt::set_property(self.as_ref(), "length", length)
    }

    #[doc(alias = "address")]
    fn connect_address_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_address_trampoline<
            P: IsA<InetAddressMask>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GInetAddressMask,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InetAddressMask::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::address\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_address_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "family")]
    fn connect_family_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_family_trampoline<
            P: IsA<InetAddressMask>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GInetAddressMask,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InetAddressMask::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::family\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_family_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "length")]
    fn connect_length_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_length_trampoline<
            P: IsA<InetAddressMask>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GInetAddressMask,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InetAddressMask::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::length\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_length_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<InetAddressMask>> InetAddressMaskExt for O {}
