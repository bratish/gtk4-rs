// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::mem;
use std::ptr;

glib::glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RecentInfo(Shared<ffi::GtkRecentInfo>);

    match fn {
        ref => |ptr| ffi::gtk_recent_info_ref(ptr),
        unref => |ptr| ffi::gtk_recent_info_unref(ptr),
        get_type => || ffi::gtk_recent_info_get_type(),
    }
}

impl RecentInfo {
    pub fn create_app_info(
        &self,
        app_name: Option<&str>,
    ) -> Result<Option<gio::AppInfo>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_recent_info_create_app_info(
                self.to_glib_none().0,
                app_name.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn exists(&self) -> bool {
        unsafe { from_glib(ffi::gtk_recent_info_exists(self.to_glib_none().0)) }
    }

    pub fn get_added(&self) -> glib::DateTime {
        unsafe { from_glib_none(ffi::gtk_recent_info_get_added(self.to_glib_none().0)) }
    }

    pub fn get_age(&self) -> i32 {
        unsafe { ffi::gtk_recent_info_get_age(self.to_glib_none().0) }
    }

    pub fn get_application_info(
        &self,
        app_name: &str,
    ) -> Option<(glib::GString, u32, glib::DateTime)> {
        unsafe {
            let mut app_exec = ptr::null();
            let mut count = mem::MaybeUninit::uninit();
            let mut stamp = ptr::null_mut();
            let ret = from_glib(ffi::gtk_recent_info_get_application_info(
                self.to_glib_none().0,
                app_name.to_glib_none().0,
                &mut app_exec,
                count.as_mut_ptr(),
                &mut stamp,
            ));
            let count = count.assume_init();
            if ret {
                Some((from_glib_none(app_exec), count, from_glib_none(stamp)))
            } else {
                None
            }
        }
    }

    pub fn get_applications(&self) -> Vec<glib::GString> {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(
                ffi::gtk_recent_info_get_applications(self.to_glib_none().0, length.as_mut_ptr()),
                length.assume_init() as usize,
            );
            ret
        }
    }

    pub fn get_description(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gtk_recent_info_get_description(self.to_glib_none().0)) }
    }

    pub fn get_display_name(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gtk_recent_info_get_display_name(self.to_glib_none().0)) }
    }

    pub fn get_gicon(&self) -> Option<gio::Icon> {
        unsafe { from_glib_full(ffi::gtk_recent_info_get_gicon(self.to_glib_none().0)) }
    }

    pub fn get_groups(&self) -> Vec<glib::GString> {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_full_num(
                ffi::gtk_recent_info_get_groups(self.to_glib_none().0, length.as_mut_ptr()),
                length.assume_init() as usize,
            );
            ret
        }
    }

    pub fn get_mime_type(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gtk_recent_info_get_mime_type(self.to_glib_none().0)) }
    }

    pub fn get_modified(&self) -> glib::DateTime {
        unsafe { from_glib_none(ffi::gtk_recent_info_get_modified(self.to_glib_none().0)) }
    }

    pub fn get_private_hint(&self) -> bool {
        unsafe { from_glib(ffi::gtk_recent_info_get_private_hint(self.to_glib_none().0)) }
    }

    pub fn get_short_name(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::gtk_recent_info_get_short_name(self.to_glib_none().0)) }
    }

    pub fn get_uri(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gtk_recent_info_get_uri(self.to_glib_none().0)) }
    }

    pub fn get_uri_display(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::gtk_recent_info_get_uri_display(self.to_glib_none().0)) }
    }

    pub fn get_visited(&self) -> glib::DateTime {
        unsafe { from_glib_none(ffi::gtk_recent_info_get_visited(self.to_glib_none().0)) }
    }

    pub fn has_application(&self, app_name: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_info_has_application(
                self.to_glib_none().0,
                app_name.to_glib_none().0,
            ))
        }
    }

    pub fn has_group(&self, group_name: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_info_has_group(
                self.to_glib_none().0,
                group_name.to_glib_none().0,
            ))
        }
    }

    pub fn is_local(&self) -> bool {
        unsafe { from_glib(ffi::gtk_recent_info_is_local(self.to_glib_none().0)) }
    }

    pub fn last_application(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::gtk_recent_info_last_application(self.to_glib_none().0)) }
    }

    pub fn match_(&self, info_b: &RecentInfo) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_info_match(
                self.to_glib_none().0,
                info_b.to_glib_none().0,
            ))
        }
    }
}
