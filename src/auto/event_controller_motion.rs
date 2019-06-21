// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use EventController;
use Widget;
use gdk;
use gdk_sys;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct EventControllerMotion(Object<gtk_sys::GtkEventControllerMotion, gtk_sys::GtkEventControllerMotionClass, EventControllerMotionClass>) @extends EventController;

    match fn {
        get_type => || gtk_sys::gtk_event_controller_motion_get_type(),
    }
}

impl EventControllerMotion {
    pub fn new() -> EventControllerMotion {
        assert_initialized_main_thread!();
        unsafe {
            EventController::from_glib_full(gtk_sys::gtk_event_controller_motion_new()).unsafe_cast()
        }
    }

    pub fn get_pointer_origin(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_event_controller_motion_get_pointer_origin(self.to_glib_none().0))
        }
    }

    pub fn get_pointer_target(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_event_controller_motion_get_pointer_target(self.to_glib_none().0))
        }
    }

    pub fn get_property_contains_pointer_focus(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"contains-pointer-focus\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn get_property_is_pointer_focus(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"is-pointer-focus\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn connect_enter<F: Fn(&EventControllerMotion, f64, f64, gdk::CrossingMode, gdk::NotifyType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn enter_trampoline<F: Fn(&EventControllerMotion, f64, f64, gdk::CrossingMode, gdk::NotifyType) + 'static>(this: *mut gtk_sys::GtkEventControllerMotion, x: libc::c_double, y: libc::c_double, crossing_mode: gdk_sys::GdkCrossingMode, notify_type: gdk_sys::GdkNotifyType, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y, from_glib(crossing_mode), from_glib(notify_type))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"enter\0".as_ptr() as *const _,
                Some(transmute(enter_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_leave<F: Fn(&EventControllerMotion, gdk::CrossingMode, gdk::NotifyType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn leave_trampoline<F: Fn(&EventControllerMotion, gdk::CrossingMode, gdk::NotifyType) + 'static>(this: *mut gtk_sys::GtkEventControllerMotion, crossing_mode: gdk_sys::GdkCrossingMode, notify_type: gdk_sys::GdkNotifyType, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(crossing_mode), from_glib(notify_type))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"leave\0".as_ptr() as *const _,
                Some(transmute(leave_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_motion<F: Fn(&EventControllerMotion, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn motion_trampoline<F: Fn(&EventControllerMotion, f64, f64) + 'static>(this: *mut gtk_sys::GtkEventControllerMotion, x: libc::c_double, y: libc::c_double, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"motion\0".as_ptr() as *const _,
                Some(transmute(motion_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_contains_pointer_focus_notify<F: Fn(&EventControllerMotion) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_contains_pointer_focus_trampoline<F: Fn(&EventControllerMotion) + 'static>(this: *mut gtk_sys::GtkEventControllerMotion, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::contains-pointer-focus\0".as_ptr() as *const _,
                Some(transmute(notify_contains_pointer_focus_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_is_pointer_focus_notify<F: Fn(&EventControllerMotion) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_pointer_focus_trampoline<F: Fn(&EventControllerMotion) + 'static>(this: *mut gtk_sys::GtkEventControllerMotion, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::is-pointer-focus\0".as_ptr() as *const _,
                Some(transmute(notify_is_pointer_focus_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }
}

impl Default for EventControllerMotion {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for EventControllerMotion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EventControllerMotion")
    }
}
