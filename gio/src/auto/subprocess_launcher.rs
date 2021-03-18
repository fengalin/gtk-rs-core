// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Subprocess;
use crate::SubprocessFlags;
use glib::translate::*;
#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
use std::boxed::Box as Box_;
use std::fmt;
use std::ptr;

glib::wrapper! {
    pub struct SubprocessLauncher(Object<ffi::GSubprocessLauncher>);

    match fn {
        get_type => || ffi::g_subprocess_launcher_get_type(),
    }
}

impl SubprocessLauncher {
    #[doc(alias = "g_subprocess_launcher_new")]
    pub fn new(flags: SubprocessFlags) -> SubprocessLauncher {
        unsafe { from_glib_full(ffi::g_subprocess_launcher_new(flags.to_glib())) }
    }

    #[doc(alias = "g_subprocess_launcher_getenv")]
    pub fn getenv<P: AsRef<std::path::Path>>(&self, variable: P) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_none(ffi::g_subprocess_launcher_getenv(
                self.to_glib_none().0,
                variable.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    #[doc(alias = "g_subprocess_launcher_set_child_setup")]
    pub fn set_child_setup<P: Fn() + 'static>(&self, child_setup: P) {
        let child_setup_data: Box_<P> = Box_::new(child_setup);
        unsafe extern "C" fn child_setup_func<P: Fn() + 'static>(user_data: glib::ffi::gpointer) {
            let callback: &P = &*(user_data as *mut _);
            (*callback)();
        }
        let child_setup = Some(child_setup_func::<P> as _);
        unsafe extern "C" fn destroy_notify_func<P: Fn() + 'static>(data: glib::ffi::gpointer) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_notify_func::<P> as _);
        let super_callback0: Box_<P> = child_setup_data;
        unsafe {
            ffi::g_subprocess_launcher_set_child_setup(
                self.to_glib_none().0,
                child_setup,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[doc(alias = "g_subprocess_launcher_set_cwd")]
    pub fn set_cwd<P: AsRef<std::path::Path>>(&self, cwd: P) {
        unsafe {
            ffi::g_subprocess_launcher_set_cwd(
                self.to_glib_none().0,
                cwd.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_subprocess_launcher_set_environ")]
    pub fn set_environ(&self, env: &[&std::path::Path]) {
        unsafe {
            ffi::g_subprocess_launcher_set_environ(self.to_glib_none().0, env.to_glib_none().0);
        }
    }

    #[doc(alias = "g_subprocess_launcher_set_flags")]
    pub fn set_flags(&self, flags: SubprocessFlags) {
        unsafe {
            ffi::g_subprocess_launcher_set_flags(self.to_glib_none().0, flags.to_glib());
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    #[doc(alias = "g_subprocess_launcher_set_stderr_file_path")]
    pub fn set_stderr_file_path<P: AsRef<std::path::Path>>(&self, path: P) {
        unsafe {
            ffi::g_subprocess_launcher_set_stderr_file_path(
                self.to_glib_none().0,
                path.as_ref().to_glib_none().0,
            );
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    #[doc(alias = "g_subprocess_launcher_set_stdin_file_path")]
    pub fn set_stdin_file_path(&self, path: &str) {
        unsafe {
            ffi::g_subprocess_launcher_set_stdin_file_path(
                self.to_glib_none().0,
                path.to_glib_none().0,
            );
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    #[doc(alias = "g_subprocess_launcher_set_stdout_file_path")]
    pub fn set_stdout_file_path<P: AsRef<std::path::Path>>(&self, path: P) {
        unsafe {
            ffi::g_subprocess_launcher_set_stdout_file_path(
                self.to_glib_none().0,
                path.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_subprocess_launcher_setenv")]
    pub fn setenv<P: AsRef<std::ffi::OsStr>, Q: AsRef<std::ffi::OsStr>>(
        &self,
        variable: P,
        value: Q,
        overwrite: bool,
    ) {
        unsafe {
            ffi::g_subprocess_launcher_setenv(
                self.to_glib_none().0,
                variable.as_ref().to_glib_none().0,
                value.as_ref().to_glib_none().0,
                overwrite.to_glib(),
            );
        }
    }

    //#[doc(alias = "g_subprocess_launcher_spawn")]
    //pub fn spawn(&self, error: &mut glib::Error, argv0: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Subprocess {
    //    unsafe { TODO: call ffi:g_subprocess_launcher_spawn() }
    //}

    #[doc(alias = "g_subprocess_launcher_spawnv")]
    pub fn spawnv(&self, argv: &[&std::ffi::OsStr]) -> Result<Subprocess, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_subprocess_launcher_spawnv(
                self.to_glib_none().0,
                argv.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_subprocess_launcher_unsetenv")]
    pub fn unsetenv<P: AsRef<std::ffi::OsStr>>(&self, variable: P) {
        unsafe {
            ffi::g_subprocess_launcher_unsetenv(
                self.to_glib_none().0,
                variable.as_ref().to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for SubprocessLauncher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SubprocessLauncher")
    }
}
