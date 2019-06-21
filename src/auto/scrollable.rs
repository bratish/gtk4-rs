// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Adjustment;
use Border;
use ScrollablePolicy;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Scrollable(Interface<gtk_sys::GtkScrollable>);

    match fn {
        get_type => || gtk_sys::gtk_scrollable_get_type(),
    }
}

pub const NONE_SCROLLABLE: Option<&Scrollable> = None;

pub trait ScrollableExt: 'static {
    fn get_border(&self) -> Option<Border>;

    fn get_hadjustment(&self) -> Option<Adjustment>;

    fn get_hscroll_policy(&self) -> ScrollablePolicy;

    fn get_vadjustment(&self) -> Option<Adjustment>;

    fn get_vscroll_policy(&self) -> ScrollablePolicy;

    fn set_hadjustment<P: IsA<Adjustment>>(&self, hadjustment: Option<&P>);

    fn set_hscroll_policy(&self, policy: ScrollablePolicy);

    fn set_vadjustment<P: IsA<Adjustment>>(&self, vadjustment: Option<&P>);

    fn set_vscroll_policy(&self, policy: ScrollablePolicy);

    fn connect_property_hadjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_hscroll_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_vadjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_vscroll_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Scrollable>> ScrollableExt for O {
    fn get_border(&self) -> Option<Border> {
        unsafe {
            let mut border = Border::uninitialized();
            let ret = from_glib(gtk_sys::gtk_scrollable_get_border(self.as_ref().to_glib_none().0, border.to_glib_none_mut().0));
            if ret { Some(border) } else { None }
        }
    }

    fn get_hadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(gtk_sys::gtk_scrollable_get_hadjustment(self.as_ref().to_glib_none().0))
        }
    }

    fn get_hscroll_policy(&self) -> ScrollablePolicy {
        unsafe {
            from_glib(gtk_sys::gtk_scrollable_get_hscroll_policy(self.as_ref().to_glib_none().0))
        }
    }

    fn get_vadjustment(&self) -> Option<Adjustment> {
        unsafe {
            from_glib_none(gtk_sys::gtk_scrollable_get_vadjustment(self.as_ref().to_glib_none().0))
        }
    }

    fn get_vscroll_policy(&self) -> ScrollablePolicy {
        unsafe {
            from_glib(gtk_sys::gtk_scrollable_get_vscroll_policy(self.as_ref().to_glib_none().0))
        }
    }

    fn set_hadjustment<P: IsA<Adjustment>>(&self, hadjustment: Option<&P>) {
        unsafe {
            gtk_sys::gtk_scrollable_set_hadjustment(self.as_ref().to_glib_none().0, hadjustment.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_hscroll_policy(&self, policy: ScrollablePolicy) {
        unsafe {
            gtk_sys::gtk_scrollable_set_hscroll_policy(self.as_ref().to_glib_none().0, policy.to_glib());
        }
    }

    fn set_vadjustment<P: IsA<Adjustment>>(&self, vadjustment: Option<&P>) {
        unsafe {
            gtk_sys::gtk_scrollable_set_vadjustment(self.as_ref().to_glib_none().0, vadjustment.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_vscroll_policy(&self, policy: ScrollablePolicy) {
        unsafe {
            gtk_sys::gtk_scrollable_set_vscroll_policy(self.as_ref().to_glib_none().0, policy.to_glib());
        }
    }

    fn connect_property_hadjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_hadjustment_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkScrollable, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Scrollable>
        {
            let f: &F = &*(f as *const F);
            f(&Scrollable::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::hadjustment\0".as_ptr() as *const _,
                Some(transmute(notify_hadjustment_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_hscroll_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_hscroll_policy_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkScrollable, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Scrollable>
        {
            let f: &F = &*(f as *const F);
            f(&Scrollable::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::hscroll-policy\0".as_ptr() as *const _,
                Some(transmute(notify_hscroll_policy_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_vadjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vadjustment_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkScrollable, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Scrollable>
        {
            let f: &F = &*(f as *const F);
            f(&Scrollable::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::vadjustment\0".as_ptr() as *const _,
                Some(transmute(notify_vadjustment_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_vscroll_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vscroll_policy_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkScrollable, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Scrollable>
        {
            let f: &F = &*(f as *const F);
            f(&Scrollable::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::vscroll-policy\0".as_ptr() as *const _,
                Some(transmute(notify_vscroll_policy_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Scrollable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scrollable")
    }
}
