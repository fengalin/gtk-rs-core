// This file was generated by gir (c6b70b0) from gir-files (469db10)
// DO NOT EDIT

use AxisFlags;
use DeviceToolType;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DeviceTool(Object<ffi::GdkDeviceTool>);

    match fn {
        get_type => || ffi::gdk_device_tool_get_type(),
    }
}

pub trait DeviceToolExt {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_hardware_id(&self) -> u64;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_serial(&self) -> u64;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_tool_type(&self) -> DeviceToolType;

    fn get_property_axes(&self) -> AxisFlags;

    fn get_property_hardware_id(&self) -> u64;

    fn get_property_serial(&self) -> u64;

    fn get_property_tool_type(&self) -> DeviceToolType;

    fn connect_property_axes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_hardware_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_serial_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tool_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DeviceTool> + IsA<glib::object::Object>> DeviceToolExt for O {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_hardware_id(&self) -> u64 {
        unsafe {
            ffi::gdk_device_tool_get_hardware_id(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_serial(&self) -> u64 {
        unsafe {
            ffi::gdk_device_tool_get_serial(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn get_tool_type(&self) -> DeviceToolType {
        unsafe {
            from_glib(ffi::gdk_device_tool_get_tool_type(self.to_glib_none().0))
        }
    }

    fn get_property_axes(&self) -> AxisFlags {
        unsafe {
            let mut value = Value::from_type(<AxisFlags as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "axes".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_hardware_id(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "hardware-id".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_serial(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "serial".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_tool_type(&self) -> DeviceToolType {
        unsafe {
            let mut value = Value::from_type(<DeviceToolType as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "tool-type".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn connect_property_axes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::axes",
                transmute(notify_axes_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_hardware_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::hardware-id",
                transmute(notify_hardware_id_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_serial_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::serial",
                transmute(notify_serial_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_tool_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::tool-type",
                transmute(notify_tool_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_axes_trampoline<P>(this: *mut ffi::GdkDeviceTool, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DeviceTool> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceTool::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_hardware_id_trampoline<P>(this: *mut ffi::GdkDeviceTool, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DeviceTool> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceTool::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_serial_trampoline<P>(this: *mut ffi::GdkDeviceTool, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DeviceTool> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceTool::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_tool_type_trampoline<P>(this: *mut ffi::GdkDeviceTool, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DeviceTool> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceTool::from_glib_borrow(this).downcast_unchecked())
}
