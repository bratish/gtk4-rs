// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use LayoutChild;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gsk;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct FixedLayoutChild(Object<gtk_sys::GtkFixedLayoutChild, gtk_sys::GtkFixedLayoutChildClass, FixedLayoutChildClass>) @extends LayoutChild;

    match fn {
        get_type => || gtk_sys::gtk_fixed_layout_child_get_type(),
    }
}

pub const NONE_FIXED_LAYOUT_CHILD: Option<&FixedLayoutChild> = None;

pub trait FixedLayoutChildExt: 'static {
    fn get_position(&self) -> Option<gsk::Transform>;

    fn set_position(&self, position: &gsk::Transform);

    fn connect_property_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FixedLayoutChild>> FixedLayoutChildExt for O {
    fn get_position(&self) -> Option<gsk::Transform> {
        unsafe {
            from_glib_none(gtk_sys::gtk_fixed_layout_child_get_position(self.as_ref().to_glib_none().0))
        }
    }

    fn set_position(&self, position: &gsk::Transform) {
        unsafe {
            gtk_sys::gtk_fixed_layout_child_set_position(self.as_ref().to_glib_none().0, position.to_glib_none().0);
        }
    }

    fn connect_property_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkFixedLayoutChild, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<FixedLayoutChild>
        {
            let f: &F = &*(f as *const F);
            f(&FixedLayoutChild::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::position\0".as_ptr() as *const _,
                Some(transmute(notify_position_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for FixedLayoutChild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FixedLayoutChild")
    }
}
