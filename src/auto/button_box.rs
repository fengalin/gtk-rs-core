// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Box;
use Buildable;
use ButtonBoxStyle;
use Container;
use Orientable;
use Orientation;
use Widget;
use ffi;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct ButtonBox(Object<ffi::GtkButtonBox, ffi::GtkButtonBoxClass, ButtonBoxClass>) @extends Box, Container, Widget, @implements Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_button_box_get_type(),
    }
}

impl ButtonBox {
    pub fn new(orientation: Orientation) -> ButtonBox {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_button_box_new(orientation.to_glib())).unsafe_cast()
        }
    }
}

pub const NONE_BUTTON_BOX: Option<&ButtonBox> = None;

pub trait ButtonBoxExt: 'static {
    fn get_child_non_homogeneous<P: IsA<Widget>>(&self, child: &P) -> bool;

    fn get_child_secondary<P: IsA<Widget>>(&self, child: &P) -> bool;

    fn get_layout(&self) -> ButtonBoxStyle;

    fn set_child_non_homogeneous<P: IsA<Widget>>(&self, child: &P, non_homogeneous: bool);

    fn set_child_secondary<P: IsA<Widget>>(&self, child: &P, is_secondary: bool);

    fn set_layout(&self, layout_style: ButtonBoxStyle);

    fn get_property_layout_style(&self) -> ButtonBoxStyle;

    fn set_property_layout_style(&self, layout_style: ButtonBoxStyle);

    fn connect_property_layout_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ButtonBox>> ButtonBoxExt for O {
    fn get_child_non_homogeneous<P: IsA<Widget>>(&self, child: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_button_box_get_child_non_homogeneous(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0))
        }
    }

    fn get_child_secondary<P: IsA<Widget>>(&self, child: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_button_box_get_child_secondary(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0))
        }
    }

    fn get_layout(&self) -> ButtonBoxStyle {
        unsafe {
            from_glib(ffi::gtk_button_box_get_layout(self.as_ref().to_glib_none().0))
        }
    }

    fn set_child_non_homogeneous<P: IsA<Widget>>(&self, child: &P, non_homogeneous: bool) {
        unsafe {
            ffi::gtk_button_box_set_child_non_homogeneous(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, non_homogeneous.to_glib());
        }
    }

    fn set_child_secondary<P: IsA<Widget>>(&self, child: &P, is_secondary: bool) {
        unsafe {
            ffi::gtk_button_box_set_child_secondary(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, is_secondary.to_glib());
        }
    }

    fn set_layout(&self, layout_style: ButtonBoxStyle) {
        unsafe {
            ffi::gtk_button_box_set_layout(self.as_ref().to_glib_none().0, layout_style.to_glib());
        }
    }

    fn get_property_layout_style(&self) -> ButtonBoxStyle {
        unsafe {
            let mut value = Value::from_type(<ButtonBoxStyle as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"layout-style\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_layout_style(&self, layout_style: ButtonBoxStyle) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"layout-style\0".as_ptr() as *const _, Value::from(&layout_style).to_glib_none().0);
        }
    }

    fn connect_property_layout_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::layout-style\0".as_ptr() as *const _,
                Some(transmute(notify_layout_style_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_layout_style_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkButtonBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ButtonBox> {
    let f: &F = transmute(f);
    f(&ButtonBox::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for ButtonBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ButtonBox")
    }
}
