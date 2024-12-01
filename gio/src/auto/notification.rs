// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Icon, NotificationPriority};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GNotification")]
    pub struct Notification(Object<ffi::GNotification>);

    match fn {
        type_ => || ffi::g_notification_get_type(),
    }
}

impl Notification {
    #[doc(alias = "g_notification_new")]
    pub fn new(title: &str) -> Notification {
        unsafe { from_glib_full(ffi::g_notification_new(title.to_glib_none().0)) }
    }

    #[doc(alias = "g_notification_add_button")]
    pub fn add_button(&self, label: &str, detailed_action: &str) {
        unsafe {
            ffi::g_notification_add_button(
                self.to_glib_none().0,
                label.to_glib_none().0,
                detailed_action.to_glib_none().0,
            );
        }
    }

    //#[doc(alias = "g_notification_add_button_with_target")]
    //pub fn add_button_with_target<'a>(&self, label: &str, action: &str, target_format: impl Into<Option<&'a str>>, : /*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:g_notification_add_button_with_target() }
    //}

    #[doc(alias = "g_notification_add_button_with_target_value")]
    pub fn add_button_with_target_value<'a>(
        &self,
        label: &str,
        action: &str,
        target: impl Into<Option<&'a glib::Variant>>,
    ) {
        unsafe {
            ffi::g_notification_add_button_with_target_value(
                self.to_glib_none().0,
                label.to_glib_none().0,
                action.to_glib_none().0,
                target.into().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_notification_set_body")]
    pub fn set_body<'a>(&self, body: impl Into<Option<&'a str>>) {
        unsafe {
            ffi::g_notification_set_body(self.to_glib_none().0, body.into().to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_70")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
    #[doc(alias = "g_notification_set_category")]
    pub fn set_category<'a>(&self, category: impl Into<Option<&'a str>>) {
        unsafe {
            ffi::g_notification_set_category(
                self.to_glib_none().0,
                category.into().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_notification_set_default_action")]
    pub fn set_default_action(&self, detailed_action: &str) {
        unsafe {
            ffi::g_notification_set_default_action(
                self.to_glib_none().0,
                detailed_action.to_glib_none().0,
            );
        }
    }

    //#[doc(alias = "g_notification_set_default_action_and_target")]
    //pub fn set_default_action_and_target<'a>(&self, action: &str, target_format: impl Into<Option<&'a str>>, : /*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:g_notification_set_default_action_and_target() }
    //}

    #[doc(alias = "g_notification_set_default_action_and_target_value")]
    pub fn set_default_action_and_target_value<'a>(
        &self,
        action: &str,
        target: impl Into<Option<&'a glib::Variant>>,
    ) {
        unsafe {
            ffi::g_notification_set_default_action_and_target_value(
                self.to_glib_none().0,
                action.to_glib_none().0,
                target.into().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_notification_set_icon")]
    pub fn set_icon(&self, icon: &impl IsA<Icon>) {
        unsafe {
            ffi::g_notification_set_icon(self.to_glib_none().0, icon.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "g_notification_set_priority")]
    pub fn set_priority(&self, priority: NotificationPriority) {
        unsafe {
            ffi::g_notification_set_priority(self.to_glib_none().0, priority.into_glib());
        }
    }

    #[doc(alias = "g_notification_set_title")]
    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::g_notification_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }
}
