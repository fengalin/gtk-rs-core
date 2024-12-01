// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, IOStream, TlsAuthenticationMode, TlsCertificate, TlsConnection};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GTlsServerConnection")]
    pub struct TlsServerConnection(Interface<ffi::GTlsServerConnection, ffi::GTlsServerConnectionInterface>) @requires TlsConnection, IOStream;

    match fn {
        type_ => || ffi::g_tls_server_connection_get_type(),
    }
}

impl TlsServerConnection {
    pub const NONE: Option<&'static TlsServerConnection> = None;

    #[doc(alias = "g_tls_server_connection_new")]
    pub fn new<'a, P: IsA<TlsCertificate>>(
        base_io_stream: &impl IsA<IOStream>,
        certificate: impl Into<Option<&'a P>>,
    ) -> Result<TlsServerConnection, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_tls_server_connection_new(
                base_io_stream.as_ref().to_glib_none().0,
                certificate
                    .into()
                    .as_ref()
                    .map(|p| p.as_ref())
                    .to_glib_none()
                    .0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

pub trait TlsServerConnectionExt: IsA<TlsServerConnection> + 'static {
    #[doc(alias = "authentication-mode")]
    fn authentication_mode(&self) -> TlsAuthenticationMode {
        ObjectExt::property(self.as_ref(), "authentication-mode")
    }

    #[doc(alias = "authentication-mode")]
    fn set_authentication_mode(&self, authentication_mode: TlsAuthenticationMode) {
        ObjectExt::set_property(self.as_ref(), "authentication-mode", authentication_mode)
    }

    #[doc(alias = "authentication-mode")]
    fn connect_authentication_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_authentication_mode_trampoline<
            P: IsA<TlsServerConnection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GTlsServerConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TlsServerConnection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::authentication-mode\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_authentication_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<TlsServerConnection>> TlsServerConnectionExt for O {}
