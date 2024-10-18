// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, TlsDatabase};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GTlsBackend")]
    pub struct TlsBackend(Interface<ffi::GTlsBackend, ffi::GTlsBackendInterface>);

    match fn {
        type_ => || ffi::g_tls_backend_get_type(),
    }
}

impl TlsBackend {
    pub const NONE: Option<&'static TlsBackend> = None;

    #[doc(alias = "g_tls_backend_get_default")]
    #[doc(alias = "get_default")]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> TlsBackend {
        unsafe { from_glib_none(ffi::g_tls_backend_get_default()) }
    }
}

pub trait TlsBackendExt: IsA<TlsBackend> + 'static {
    #[doc(alias = "g_tls_backend_get_certificate_type")]
    #[doc(alias = "get_certificate_type")]
    fn certificate_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::g_tls_backend_get_certificate_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_tls_backend_get_client_connection_type")]
    #[doc(alias = "get_client_connection_type")]
    fn client_connection_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::g_tls_backend_get_client_connection_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_tls_backend_get_default_database")]
    #[doc(alias = "get_default_database")]
    fn default_database(&self) -> TlsDatabase {
        unsafe {
            from_glib_full(ffi::g_tls_backend_get_default_database(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_tls_backend_get_dtls_client_connection_type")]
    #[doc(alias = "get_dtls_client_connection_type")]
    fn dtls_client_connection_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::g_tls_backend_get_dtls_client_connection_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_tls_backend_get_dtls_server_connection_type")]
    #[doc(alias = "get_dtls_server_connection_type")]
    fn dtls_server_connection_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::g_tls_backend_get_dtls_server_connection_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_tls_backend_get_file_database_type")]
    #[doc(alias = "get_file_database_type")]
    fn file_database_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::g_tls_backend_get_file_database_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_tls_backend_get_server_connection_type")]
    #[doc(alias = "get_server_connection_type")]
    fn server_connection_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::g_tls_backend_get_server_connection_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v2_60")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_60")))]
    #[doc(alias = "g_tls_backend_set_default_database")]
    fn set_default_database(&self, database: Option<&impl IsA<TlsDatabase>>) {
        unsafe {
            ffi::g_tls_backend_set_default_database(
                self.as_ref().to_glib_none().0,
                database.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_tls_backend_supports_dtls")]
    fn supports_dtls(&self) -> bool {
        unsafe {
            from_glib(ffi::g_tls_backend_supports_dtls(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_tls_backend_supports_tls")]
    fn supports_tls(&self) -> bool {
        unsafe {
            from_glib(ffi::g_tls_backend_supports_tls(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<TlsBackend>> TlsBackendExt for O {}
