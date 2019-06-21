// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use CellArea;
use CellLayout;
use TreeIter;
use TreeModel;
use Widget;
use glib::GString;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use libc;
use signal::Inhibit;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct EntryCompletion(Object<gtk_sys::GtkEntryCompletion, gtk_sys::GtkEntryCompletionClass, EntryCompletionClass>) @implements Buildable, CellLayout;

    match fn {
        get_type => || gtk_sys::gtk_entry_completion_get_type(),
    }
}

impl EntryCompletion {
    pub fn new() -> EntryCompletion {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_entry_completion_new())
        }
    }

    pub fn new_with_area<P: IsA<CellArea>>(area: &P) -> EntryCompletion {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gtk_sys::gtk_entry_completion_new_with_area(area.as_ref().to_glib_none().0))
        }
    }
}

impl Default for EntryCompletion {
    fn default() -> Self {
        Self::new()
    }
}

pub struct EntryCompletionBuilder {
    cell_area: Option<CellArea>,
    inline_completion: Option<bool>,
    inline_selection: Option<bool>,
    minimum_key_length: Option<i32>,
    model: Option<TreeModel>,
    popup_completion: Option<bool>,
    popup_set_width: Option<bool>,
    popup_single_match: Option<bool>,
    text_column: Option<i32>,
}

impl EntryCompletionBuilder {
    pub fn new() -> Self {
        Self {
            cell_area: None,
            inline_completion: None,
            inline_selection: None,
            minimum_key_length: None,
            model: None,
            popup_completion: None,
            popup_set_width: None,
            popup_single_match: None,
            text_column: None,
        }
    }

    pub fn build(self) -> EntryCompletion {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref cell_area) = self.cell_area {
            properties.push(("cell-area", cell_area));
        }
        if let Some(ref inline_completion) = self.inline_completion {
            properties.push(("inline-completion", inline_completion));
        }
        if let Some(ref inline_selection) = self.inline_selection {
            properties.push(("inline-selection", inline_selection));
        }
        if let Some(ref minimum_key_length) = self.minimum_key_length {
            properties.push(("minimum-key-length", minimum_key_length));
        }
        if let Some(ref model) = self.model {
            properties.push(("model", model));
        }
        if let Some(ref popup_completion) = self.popup_completion {
            properties.push(("popup-completion", popup_completion));
        }
        if let Some(ref popup_set_width) = self.popup_set_width {
            properties.push(("popup-set-width", popup_set_width));
        }
        if let Some(ref popup_single_match) = self.popup_single_match {
            properties.push(("popup-single-match", popup_single_match));
        }
        if let Some(ref text_column) = self.text_column {
            properties.push(("text-column", text_column));
        }
        glib::Object::new(EntryCompletion::static_type(), &properties).expect("object new").downcast().expect("downcast")
    }

    pub fn cell_area(mut self, cell_area: &CellArea) -> Self {
        self.cell_area = Some(cell_area.clone());
        self
    }

    pub fn inline_completion(mut self, inline_completion: bool) -> Self {
        self.inline_completion = Some(inline_completion);
        self
    }

    pub fn inline_selection(mut self, inline_selection: bool) -> Self {
        self.inline_selection = Some(inline_selection);
        self
    }

    pub fn minimum_key_length(mut self, minimum_key_length: i32) -> Self {
        self.minimum_key_length = Some(minimum_key_length);
        self
    }

    pub fn model(mut self, model: &TreeModel) -> Self {
        self.model = Some(model.clone());
        self
    }

    pub fn popup_completion(mut self, popup_completion: bool) -> Self {
        self.popup_completion = Some(popup_completion);
        self
    }

    pub fn popup_set_width(mut self, popup_set_width: bool) -> Self {
        self.popup_set_width = Some(popup_set_width);
        self
    }

    pub fn popup_single_match(mut self, popup_single_match: bool) -> Self {
        self.popup_single_match = Some(popup_single_match);
        self
    }

    pub fn text_column(mut self, text_column: i32) -> Self {
        self.text_column = Some(text_column);
        self
    }
}

pub const NONE_ENTRY_COMPLETION: Option<&EntryCompletion> = None;

pub trait EntryCompletionExt: 'static {
    fn complete(&self);

    fn compute_prefix(&self, key: &str) -> Option<GString>;

    fn delete_action(&self, index_: i32);

    fn get_completion_prefix(&self) -> Option<GString>;

    fn get_entry(&self) -> Option<Widget>;

    fn get_inline_completion(&self) -> bool;

    fn get_inline_selection(&self) -> bool;

    fn get_minimum_key_length(&self) -> i32;

    fn get_model(&self) -> Option<TreeModel>;

    fn get_popup_completion(&self) -> bool;

    fn get_popup_set_width(&self) -> bool;

    fn get_popup_single_match(&self) -> bool;

    fn get_text_column(&self) -> i32;

    fn insert_action_markup(&self, index_: i32, markup: &str);

    fn insert_action_text(&self, index_: i32, text: &str);

    fn insert_prefix(&self);

    fn set_inline_completion(&self, inline_completion: bool);

    fn set_inline_selection(&self, inline_selection: bool);

    fn set_match_func<P: Fn(&EntryCompletion, &str, &TreeIter) -> bool + 'static>(&self, func: P);

    fn set_minimum_key_length(&self, length: i32);

    fn set_model<P: IsA<TreeModel>>(&self, model: Option<&P>);

    fn set_popup_completion(&self, popup_completion: bool);

    fn set_popup_set_width(&self, popup_set_width: bool);

    fn set_popup_single_match(&self, popup_single_match: bool);

    fn set_text_column(&self, column: i32);

    fn get_property_cell_area(&self) -> Option<CellArea>;

    fn connect_action_activated<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_cursor_on_match<F: Fn(&Self, &TreeModel, &TreeIter) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_insert_prefix<F: Fn(&Self, &str) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_match_selected<F: Fn(&Self, &TreeModel, &TreeIter) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_no_matches<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_inline_completion_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_inline_selection_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_minimum_key_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_popup_completion_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_popup_set_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_popup_single_match_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<EntryCompletion>> EntryCompletionExt for O {
    fn complete(&self) {
        unsafe {
            gtk_sys::gtk_entry_completion_complete(self.as_ref().to_glib_none().0);
        }
    }

    fn compute_prefix(&self, key: &str) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_sys::gtk_entry_completion_compute_prefix(self.as_ref().to_glib_none().0, key.to_glib_none().0))
        }
    }

    fn delete_action(&self, index_: i32) {
        unsafe {
            gtk_sys::gtk_entry_completion_delete_action(self.as_ref().to_glib_none().0, index_);
        }
    }

    fn get_completion_prefix(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_entry_completion_get_completion_prefix(self.as_ref().to_glib_none().0))
        }
    }

    fn get_entry(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_entry_completion_get_entry(self.as_ref().to_glib_none().0))
        }
    }

    fn get_inline_completion(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_entry_completion_get_inline_completion(self.as_ref().to_glib_none().0))
        }
    }

    fn get_inline_selection(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_entry_completion_get_inline_selection(self.as_ref().to_glib_none().0))
        }
    }

    fn get_minimum_key_length(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_entry_completion_get_minimum_key_length(self.as_ref().to_glib_none().0)
        }
    }

    fn get_model(&self) -> Option<TreeModel> {
        unsafe {
            from_glib_none(gtk_sys::gtk_entry_completion_get_model(self.as_ref().to_glib_none().0))
        }
    }

    fn get_popup_completion(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_entry_completion_get_popup_completion(self.as_ref().to_glib_none().0))
        }
    }

    fn get_popup_set_width(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_entry_completion_get_popup_set_width(self.as_ref().to_glib_none().0))
        }
    }

    fn get_popup_single_match(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_entry_completion_get_popup_single_match(self.as_ref().to_glib_none().0))
        }
    }

    fn get_text_column(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_entry_completion_get_text_column(self.as_ref().to_glib_none().0)
        }
    }

    fn insert_action_markup(&self, index_: i32, markup: &str) {
        unsafe {
            gtk_sys::gtk_entry_completion_insert_action_markup(self.as_ref().to_glib_none().0, index_, markup.to_glib_none().0);
        }
    }

    fn insert_action_text(&self, index_: i32, text: &str) {
        unsafe {
            gtk_sys::gtk_entry_completion_insert_action_text(self.as_ref().to_glib_none().0, index_, text.to_glib_none().0);
        }
    }

    fn insert_prefix(&self) {
        unsafe {
            gtk_sys::gtk_entry_completion_insert_prefix(self.as_ref().to_glib_none().0);
        }
    }

    fn set_inline_completion(&self, inline_completion: bool) {
        unsafe {
            gtk_sys::gtk_entry_completion_set_inline_completion(self.as_ref().to_glib_none().0, inline_completion.to_glib());
        }
    }

    fn set_inline_selection(&self, inline_selection: bool) {
        unsafe {
            gtk_sys::gtk_entry_completion_set_inline_selection(self.as_ref().to_glib_none().0, inline_selection.to_glib());
        }
    }

    fn set_match_func<P: Fn(&EntryCompletion, &str, &TreeIter) -> bool + 'static>(&self, func: P) {
        let func_data: Box_<P> = Box::new(func);
        unsafe extern "C" fn func_func<P: Fn(&EntryCompletion, &str, &TreeIter) -> bool + 'static>(completion: *mut gtk_sys::GtkEntryCompletion, key: *const libc::c_char, iter: *mut gtk_sys::GtkTreeIter, user_data: glib_sys::gpointer) -> glib_sys::gboolean {
            let completion = from_glib_borrow(completion);
            let key: GString = from_glib_borrow(key);
            let iter = from_glib_borrow(iter);
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)(&completion, key.as_str(), &iter);
            res.to_glib()
        }
        let func = Some(func_func::<P> as _);
        unsafe extern "C" fn func_notify_func<P: Fn(&EntryCompletion, &str, &TreeIter) -> bool + 'static>(data: glib_sys::gpointer) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(func_notify_func::<P> as _);
        let super_callback0: Box_<P> = func_data;
        unsafe {
            gtk_sys::gtk_entry_completion_set_match_func(self.as_ref().to_glib_none().0, func, Box::into_raw(super_callback0) as *mut _, destroy_call3);
        }
    }

    fn set_minimum_key_length(&self, length: i32) {
        unsafe {
            gtk_sys::gtk_entry_completion_set_minimum_key_length(self.as_ref().to_glib_none().0, length);
        }
    }

    fn set_model<P: IsA<TreeModel>>(&self, model: Option<&P>) {
        unsafe {
            gtk_sys::gtk_entry_completion_set_model(self.as_ref().to_glib_none().0, model.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_popup_completion(&self, popup_completion: bool) {
        unsafe {
            gtk_sys::gtk_entry_completion_set_popup_completion(self.as_ref().to_glib_none().0, popup_completion.to_glib());
        }
    }

    fn set_popup_set_width(&self, popup_set_width: bool) {
        unsafe {
            gtk_sys::gtk_entry_completion_set_popup_set_width(self.as_ref().to_glib_none().0, popup_set_width.to_glib());
        }
    }

    fn set_popup_single_match(&self, popup_single_match: bool) {
        unsafe {
            gtk_sys::gtk_entry_completion_set_popup_single_match(self.as_ref().to_glib_none().0, popup_single_match.to_glib());
        }
    }

    fn set_text_column(&self, column: i32) {
        unsafe {
            gtk_sys::gtk_entry_completion_set_text_column(self.as_ref().to_glib_none().0, column);
        }
    }

    fn get_property_cell_area(&self) -> Option<CellArea> {
        unsafe {
            let mut value = Value::from_type(<CellArea as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"cell-area\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_action_activated<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn action_activated_trampoline<P, F: Fn(&P, i32) + 'static>(this: *mut gtk_sys::GtkEntryCompletion, index: libc::c_int, f: glib_sys::gpointer)
            where P: IsA<EntryCompletion>
        {
            let f: &F = &*(f as *const F);
            f(&EntryCompletion::from_glib_borrow(this).unsafe_cast(), index)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"action-activated\0".as_ptr() as *const _,
                Some(transmute(action_activated_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_cursor_on_match<F: Fn(&Self, &TreeModel, &TreeIter) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cursor_on_match_trampoline<P, F: Fn(&P, &TreeModel, &TreeIter) -> Inhibit + 'static>(this: *mut gtk_sys::GtkEntryCompletion, model: *mut gtk_sys::GtkTreeModel, iter: *mut gtk_sys::GtkTreeIter, f: glib_sys::gpointer) -> glib_sys::gboolean
            where P: IsA<EntryCompletion>
        {
            let f: &F = &*(f as *const F);
            f(&EntryCompletion::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(model), &from_glib_borrow(iter)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"cursor-on-match\0".as_ptr() as *const _,
                Some(transmute(cursor_on_match_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_insert_prefix<F: Fn(&Self, &str) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn insert_prefix_trampoline<P, F: Fn(&P, &str) -> Inhibit + 'static>(this: *mut gtk_sys::GtkEntryCompletion, prefix: *mut libc::c_char, f: glib_sys::gpointer) -> glib_sys::gboolean
            where P: IsA<EntryCompletion>
        {
            let f: &F = &*(f as *const F);
            f(&EntryCompletion::from_glib_borrow(this).unsafe_cast(), &GString::from_glib_borrow(prefix)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"insert-prefix\0".as_ptr() as *const _,
                Some(transmute(insert_prefix_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_match_selected<F: Fn(&Self, &TreeModel, &TreeIter) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn match_selected_trampoline<P, F: Fn(&P, &TreeModel, &TreeIter) -> Inhibit + 'static>(this: *mut gtk_sys::GtkEntryCompletion, model: *mut gtk_sys::GtkTreeModel, iter: *mut gtk_sys::GtkTreeIter, f: glib_sys::gpointer) -> glib_sys::gboolean
            where P: IsA<EntryCompletion>
        {
            let f: &F = &*(f as *const F);
            f(&EntryCompletion::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(model), &from_glib_borrow(iter)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"match-selected\0".as_ptr() as *const _,
                Some(transmute(match_selected_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_no_matches<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn no_matches_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkEntryCompletion, f: glib_sys::gpointer)
            where P: IsA<EntryCompletion>
        {
            let f: &F = &*(f as *const F);
            f(&EntryCompletion::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"no-matches\0".as_ptr() as *const _,
                Some(transmute(no_matches_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_inline_completion_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_inline_completion_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkEntryCompletion, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<EntryCompletion>
        {
            let f: &F = &*(f as *const F);
            f(&EntryCompletion::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::inline-completion\0".as_ptr() as *const _,
                Some(transmute(notify_inline_completion_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_inline_selection_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_inline_selection_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkEntryCompletion, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<EntryCompletion>
        {
            let f: &F = &*(f as *const F);
            f(&EntryCompletion::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::inline-selection\0".as_ptr() as *const _,
                Some(transmute(notify_inline_selection_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_minimum_key_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_minimum_key_length_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkEntryCompletion, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<EntryCompletion>
        {
            let f: &F = &*(f as *const F);
            f(&EntryCompletion::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::minimum-key-length\0".as_ptr() as *const _,
                Some(transmute(notify_minimum_key_length_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkEntryCompletion, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<EntryCompletion>
        {
            let f: &F = &*(f as *const F);
            f(&EntryCompletion::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::model\0".as_ptr() as *const _,
                Some(transmute(notify_model_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_popup_completion_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_popup_completion_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkEntryCompletion, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<EntryCompletion>
        {
            let f: &F = &*(f as *const F);
            f(&EntryCompletion::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::popup-completion\0".as_ptr() as *const _,
                Some(transmute(notify_popup_completion_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_popup_set_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_popup_set_width_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkEntryCompletion, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<EntryCompletion>
        {
            let f: &F = &*(f as *const F);
            f(&EntryCompletion::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::popup-set-width\0".as_ptr() as *const _,
                Some(transmute(notify_popup_set_width_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_popup_single_match_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_popup_single_match_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkEntryCompletion, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<EntryCompletion>
        {
            let f: &F = &*(f as *const F);
            f(&EntryCompletion::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::popup-single-match\0".as_ptr() as *const _,
                Some(transmute(notify_popup_single_match_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_text_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_column_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkEntryCompletion, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<EntryCompletion>
        {
            let f: &F = &*(f as *const F);
            f(&EntryCompletion::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::text-column\0".as_ptr() as *const _,
                Some(transmute(notify_text_column_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for EntryCompletion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EntryCompletion")
    }
}
