// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GConverter")]
    pub struct Converter(Interface<ffi::GConverter, ffi::GConverterIface>);

    match fn {
        type_ => || ffi::g_converter_get_type(),
    }
}

impl Converter {
    pub const NONE: Option<&'static Converter> = None;
}

pub trait ConverterExt: IsA<Converter> + 'static {
    #[cfg(feature = "v2_82")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_82")))]
    #[doc(alias = "g_converter_convert_bytes")]
    fn convert_bytes(&self, bytes: &glib::Bytes) -> Result<glib::Bytes, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_converter_convert_bytes(
                self.as_ref().to_glib_none().0,
                bytes.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_converter_reset")]
    fn reset(&self) {
        unsafe {
            ffi::g_converter_reset(self.as_ref().to_glib_none().0);
        }
    }
}

impl<O: IsA<Converter>> ConverterExt for O {}
