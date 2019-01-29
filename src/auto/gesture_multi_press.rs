// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use EventController;
use Gesture;
use GestureSingle;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use Widget;
use ffi;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use gdk;
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::connect_raw;
use glib::translate::*;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib_ffi;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use libc;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::mem::transmute;

glib_wrapper! {
    pub struct GestureMultiPress(Object<ffi::GtkGestureMultiPress, ffi::GtkGestureMultiPressClass, GestureMultiPressClass>) @extends GestureSingle, Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_multi_press_get_type(),
    }
}

impl GestureMultiPress {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub fn new<P: IsA<Widget>>(widget: &P) -> GestureMultiPress {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_multi_press_new(widget.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_GESTURE_MULTI_PRESS: Option<&GestureMultiPress> = None;

pub trait GestureMultiPressExt: 'static {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_area(&self) -> Option<gdk::Rectangle>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_area<'a, P: Into<Option<&'a gdk::Rectangle>>>(&self, rect: P);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_pressed<F: Fn(&Self, i32, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_released<F: Fn(&Self, i32, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_stopped<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GestureMultiPress>> GestureMultiPressExt for O {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_area(&self) -> Option<gdk::Rectangle> {
        unsafe {
            let mut rect = gdk::Rectangle::uninitialized();
            let ret = from_glib(ffi::gtk_gesture_multi_press_get_area(self.as_ref().to_glib_none().0, rect.to_glib_none_mut().0));
            if ret { Some(rect) } else { None }
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_area<'a, P: Into<Option<&'a gdk::Rectangle>>>(&self, rect: P) {
        let rect = rect.into();
        unsafe {
            ffi::gtk_gesture_multi_press_set_area(self.as_ref().to_glib_none().0, rect.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_pressed<F: Fn(&Self, i32, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"pressed\0".as_ptr() as *const _,
                Some(transmute(pressed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_released<F: Fn(&Self, i32, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"released\0".as_ptr() as *const _,
                Some(transmute(released_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_stopped<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"stopped\0".as_ptr() as *const _,
                Some(transmute(stopped_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn pressed_trampoline<P, F: Fn(&P, i32, f64, f64) + 'static>(this: *mut ffi::GtkGestureMultiPress, n_press: libc::c_int, x: libc::c_double, y: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<GestureMultiPress> {
    let f: &F = transmute(f);
    f(&GestureMultiPress::from_glib_borrow(this).unsafe_cast(), n_press, x, y)
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn released_trampoline<P, F: Fn(&P, i32, f64, f64) + 'static>(this: *mut ffi::GtkGestureMultiPress, n_press: libc::c_int, x: libc::c_double, y: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<GestureMultiPress> {
    let f: &F = transmute(f);
    f(&GestureMultiPress::from_glib_borrow(this).unsafe_cast(), n_press, x, y)
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn stopped_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkGestureMultiPress, f: glib_ffi::gpointer)
where P: IsA<GestureMultiPress> {
    let f: &F = transmute(f);
    f(&GestureMultiPress::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for GestureMultiPress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GestureMultiPress")
    }
}
