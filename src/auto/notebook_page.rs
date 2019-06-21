// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Widget;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::ObjectType as ObjectType_;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct NotebookPage(Object<gtk_sys::GtkNotebookPage, gtk_sys::GtkNotebookPageClass, NotebookPageClass>);

    match fn {
        get_type => || gtk_sys::gtk_notebook_page_get_type(),
    }
}

impl NotebookPage {
    pub fn get_child(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_notebook_page_get_child(self.to_glib_none().0))
        }
    }

    pub fn get_property_detachable(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"detachable\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_detachable(&self, detachable: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"detachable\0".as_ptr() as *const _, Value::from(&detachable).to_glib_none().0);
        }
    }

    pub fn get_property_menu(&self) -> Option<Widget> {
        unsafe {
            let mut value = Value::from_type(<Widget as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"menu\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    pub fn get_property_menu_label(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"menu-label\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    pub fn set_property_menu_label(&self, menu_label: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"menu-label\0".as_ptr() as *const _, Value::from(menu_label).to_glib_none().0);
        }
    }

    pub fn get_property_position(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"position\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_position(&self, position: i32) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"position\0".as_ptr() as *const _, Value::from(&position).to_glib_none().0);
        }
    }

    pub fn get_property_reorderable(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"reorderable\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_reorderable(&self, reorderable: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"reorderable\0".as_ptr() as *const _, Value::from(&reorderable).to_glib_none().0);
        }
    }

    pub fn get_property_tab(&self) -> Option<Widget> {
        unsafe {
            let mut value = Value::from_type(<Widget as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"tab\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    pub fn get_property_tab_expand(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"tab-expand\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_tab_expand(&self, tab_expand: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"tab-expand\0".as_ptr() as *const _, Value::from(&tab_expand).to_glib_none().0);
        }
    }

    pub fn get_property_tab_fill(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"tab-fill\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_tab_fill(&self, tab_fill: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"tab-fill\0".as_ptr() as *const _, Value::from(&tab_fill).to_glib_none().0);
        }
    }

    pub fn get_property_tab_label(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"tab-label\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    pub fn set_property_tab_label(&self, tab_label: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"tab-label\0".as_ptr() as *const _, Value::from(tab_label).to_glib_none().0);
        }
    }

    pub fn connect_property_detachable_notify<F: Fn(&NotebookPage) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_detachable_trampoline<F: Fn(&NotebookPage) + 'static>(this: *mut gtk_sys::GtkNotebookPage, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::detachable\0".as_ptr() as *const _,
                Some(transmute(notify_detachable_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_menu_label_notify<F: Fn(&NotebookPage) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_menu_label_trampoline<F: Fn(&NotebookPage) + 'static>(this: *mut gtk_sys::GtkNotebookPage, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::menu-label\0".as_ptr() as *const _,
                Some(transmute(notify_menu_label_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_position_notify<F: Fn(&NotebookPage) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_trampoline<F: Fn(&NotebookPage) + 'static>(this: *mut gtk_sys::GtkNotebookPage, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::position\0".as_ptr() as *const _,
                Some(transmute(notify_position_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_reorderable_notify<F: Fn(&NotebookPage) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_reorderable_trampoline<F: Fn(&NotebookPage) + 'static>(this: *mut gtk_sys::GtkNotebookPage, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::reorderable\0".as_ptr() as *const _,
                Some(transmute(notify_reorderable_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_tab_expand_notify<F: Fn(&NotebookPage) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tab_expand_trampoline<F: Fn(&NotebookPage) + 'static>(this: *mut gtk_sys::GtkNotebookPage, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::tab-expand\0".as_ptr() as *const _,
                Some(transmute(notify_tab_expand_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_tab_fill_notify<F: Fn(&NotebookPage) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tab_fill_trampoline<F: Fn(&NotebookPage) + 'static>(this: *mut gtk_sys::GtkNotebookPage, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::tab-fill\0".as_ptr() as *const _,
                Some(transmute(notify_tab_fill_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_tab_label_notify<F: Fn(&NotebookPage) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tab_label_trampoline<F: Fn(&NotebookPage) + 'static>(this: *mut gtk_sys::GtkNotebookPage, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::tab-label\0".as_ptr() as *const _,
                Some(transmute(notify_tab_label_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for NotebookPage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NotebookPage")
    }
}
