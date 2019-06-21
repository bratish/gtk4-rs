// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Align;
use Buildable;
use Container;
use LayoutManager;
use Overflow;
use SelectionModel;
use StackPage;
use StackTransitionType;
use Widget;
use gdk;
use glib::GString;
use glib::StaticType;
use glib::ToValue;
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
    pub struct Stack(Object<gtk_sys::GtkStack, gtk_sys::GtkStackClass, StackClass>) @extends Container, Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_stack_get_type(),
    }
}

impl Stack {
    pub fn new() -> Stack {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_stack_new()).unsafe_cast()
        }
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

pub struct StackBuilder {
    hhomogeneous: Option<bool>,
    homogeneous: Option<bool>,
    interpolate_size: Option<bool>,
    transition_duration: Option<u32>,
    transition_type: Option<StackTransitionType>,
    vhomogeneous: Option<bool>,
    visible_child: Option<Widget>,
    visible_child_name: Option<String>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    expand: Option<bool>,
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    layout_manager: Option<LayoutManager>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
}

impl StackBuilder {
    pub fn new() -> Self {
        Self {
            hhomogeneous: None,
            homogeneous: None,
            interpolate_size: None,
            transition_duration: None,
            transition_type: None,
            vhomogeneous: None,
            visible_child: None,
            visible_child_name: None,
            can_focus: None,
            can_target: None,
            css_name: None,
            cursor: None,
            expand: None,
            focus_on_click: None,
            halign: None,
            has_focus: None,
            has_tooltip: None,
            height_request: None,
            hexpand: None,
            hexpand_set: None,
            is_focus: None,
            layout_manager: None,
            margin: None,
            margin_bottom: None,
            margin_end: None,
            margin_start: None,
            margin_top: None,
            name: None,
            opacity: None,
            overflow: None,
            receives_default: None,
            sensitive: None,
            tooltip_markup: None,
            tooltip_text: None,
            valign: None,
            vexpand: None,
            vexpand_set: None,
            visible: None,
            width_request: None,
        }
    }

    pub fn build(self) -> Stack {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref hhomogeneous) = self.hhomogeneous {
            properties.push(("hhomogeneous", hhomogeneous));
        }
        if let Some(ref homogeneous) = self.homogeneous {
            properties.push(("homogeneous", homogeneous));
        }
        if let Some(ref interpolate_size) = self.interpolate_size {
            properties.push(("interpolate-size", interpolate_size));
        }
        if let Some(ref transition_duration) = self.transition_duration {
            properties.push(("transition-duration", transition_duration));
        }
        if let Some(ref transition_type) = self.transition_type {
            properties.push(("transition-type", transition_type));
        }
        if let Some(ref vhomogeneous) = self.vhomogeneous {
            properties.push(("vhomogeneous", vhomogeneous));
        }
        if let Some(ref visible_child) = self.visible_child {
            properties.push(("visible-child", visible_child));
        }
        if let Some(ref visible_child_name) = self.visible_child_name {
            properties.push(("visible-child-name", visible_child_name));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        glib::Object::new(Stack::static_type(), &properties).expect("object new").downcast().expect("downcast")
    }

    pub fn hhomogeneous(mut self, hhomogeneous: bool) -> Self {
        self.hhomogeneous = Some(hhomogeneous);
        self
    }

    pub fn homogeneous(mut self, homogeneous: bool) -> Self {
        self.homogeneous = Some(homogeneous);
        self
    }

    pub fn interpolate_size(mut self, interpolate_size: bool) -> Self {
        self.interpolate_size = Some(interpolate_size);
        self
    }

    pub fn transition_duration(mut self, transition_duration: u32) -> Self {
        self.transition_duration = Some(transition_duration);
        self
    }

    pub fn transition_type(mut self, transition_type: StackTransitionType) -> Self {
        self.transition_type = Some(transition_type);
        self
    }

    pub fn vhomogeneous(mut self, vhomogeneous: bool) -> Self {
        self.vhomogeneous = Some(vhomogeneous);
        self
    }

    pub fn visible_child(mut self, visible_child: &Widget) -> Self {
        self.visible_child = Some(visible_child.clone());
        self
    }

    pub fn visible_child_name(mut self, visible_child_name: &str) -> Self {
        self.visible_child_name = Some(visible_child_name.to_string());
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn layout_manager(mut self, layout_manager: &LayoutManager) -> Self {
        self.layout_manager = Some(layout_manager.clone());
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }
}

pub const NONE_STACK: Option<&Stack> = None;

pub trait StackExt: 'static {
    fn add_named<P: IsA<Widget>>(&self, child: &P, name: &str);

    fn add_titled<P: IsA<Widget>>(&self, child: &P, name: &str, title: &str);

    fn get_child_by_name(&self, name: &str) -> Option<Widget>;

    fn get_hhomogeneous(&self) -> bool;

    fn get_homogeneous(&self) -> bool;

    fn get_interpolate_size(&self) -> bool;

    fn get_page<P: IsA<Widget>>(&self, child: &P) -> Option<StackPage>;

    fn get_pages(&self) -> Option<SelectionModel>;

    fn get_transition_duration(&self) -> u32;

    fn get_transition_running(&self) -> bool;

    fn get_transition_type(&self) -> StackTransitionType;

    fn get_vhomogeneous(&self) -> bool;

    fn get_visible_child(&self) -> Option<Widget>;

    fn get_visible_child_name(&self) -> Option<GString>;

    fn set_hhomogeneous(&self, hhomogeneous: bool);

    fn set_homogeneous(&self, homogeneous: bool);

    fn set_interpolate_size(&self, interpolate_size: bool);

    fn set_transition_duration(&self, duration: u32);

    fn set_transition_type(&self, transition: StackTransitionType);

    fn set_vhomogeneous(&self, vhomogeneous: bool);

    fn set_visible_child<P: IsA<Widget>>(&self, child: &P);

    fn set_visible_child_full(&self, name: &str, transition: StackTransitionType);

    fn set_visible_child_name(&self, name: &str);

    fn connect_property_hhomogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_interpolate_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pages_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_transition_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_transition_running_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_transition_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_vhomogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_visible_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_visible_child_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Stack>> StackExt for O {
    fn add_named<P: IsA<Widget>>(&self, child: &P, name: &str) {
        unsafe {
            gtk_sys::gtk_stack_add_named(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn add_titled<P: IsA<Widget>>(&self, child: &P, name: &str, title: &str) {
        unsafe {
            gtk_sys::gtk_stack_add_titled(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, name.to_glib_none().0, title.to_glib_none().0);
        }
    }

    fn get_child_by_name(&self, name: &str) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_stack_get_child_by_name(self.as_ref().to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_hhomogeneous(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_stack_get_hhomogeneous(self.as_ref().to_glib_none().0))
        }
    }

    fn get_homogeneous(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_stack_get_homogeneous(self.as_ref().to_glib_none().0))
        }
    }

    fn get_interpolate_size(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_stack_get_interpolate_size(self.as_ref().to_glib_none().0))
        }
    }

    fn get_page<P: IsA<Widget>>(&self, child: &P) -> Option<StackPage> {
        unsafe {
            from_glib_none(gtk_sys::gtk_stack_get_page(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0))
        }
    }

    fn get_pages(&self) -> Option<SelectionModel> {
        unsafe {
            from_glib_full(gtk_sys::gtk_stack_get_pages(self.as_ref().to_glib_none().0))
        }
    }

    fn get_transition_duration(&self) -> u32 {
        unsafe {
            gtk_sys::gtk_stack_get_transition_duration(self.as_ref().to_glib_none().0)
        }
    }

    fn get_transition_running(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_stack_get_transition_running(self.as_ref().to_glib_none().0))
        }
    }

    fn get_transition_type(&self) -> StackTransitionType {
        unsafe {
            from_glib(gtk_sys::gtk_stack_get_transition_type(self.as_ref().to_glib_none().0))
        }
    }

    fn get_vhomogeneous(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_stack_get_vhomogeneous(self.as_ref().to_glib_none().0))
        }
    }

    fn get_visible_child(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_stack_get_visible_child(self.as_ref().to_glib_none().0))
        }
    }

    fn get_visible_child_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_stack_get_visible_child_name(self.as_ref().to_glib_none().0))
        }
    }

    fn set_hhomogeneous(&self, hhomogeneous: bool) {
        unsafe {
            gtk_sys::gtk_stack_set_hhomogeneous(self.as_ref().to_glib_none().0, hhomogeneous.to_glib());
        }
    }

    fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            gtk_sys::gtk_stack_set_homogeneous(self.as_ref().to_glib_none().0, homogeneous.to_glib());
        }
    }

    fn set_interpolate_size(&self, interpolate_size: bool) {
        unsafe {
            gtk_sys::gtk_stack_set_interpolate_size(self.as_ref().to_glib_none().0, interpolate_size.to_glib());
        }
    }

    fn set_transition_duration(&self, duration: u32) {
        unsafe {
            gtk_sys::gtk_stack_set_transition_duration(self.as_ref().to_glib_none().0, duration);
        }
    }

    fn set_transition_type(&self, transition: StackTransitionType) {
        unsafe {
            gtk_sys::gtk_stack_set_transition_type(self.as_ref().to_glib_none().0, transition.to_glib());
        }
    }

    fn set_vhomogeneous(&self, vhomogeneous: bool) {
        unsafe {
            gtk_sys::gtk_stack_set_vhomogeneous(self.as_ref().to_glib_none().0, vhomogeneous.to_glib());
        }
    }

    fn set_visible_child<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            gtk_sys::gtk_stack_set_visible_child(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    fn set_visible_child_full(&self, name: &str, transition: StackTransitionType) {
        unsafe {
            gtk_sys::gtk_stack_set_visible_child_full(self.as_ref().to_glib_none().0, name.to_glib_none().0, transition.to_glib());
        }
    }

    fn set_visible_child_name(&self, name: &str) {
        unsafe {
            gtk_sys::gtk_stack_set_visible_child_name(self.as_ref().to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn connect_property_hhomogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_hhomogeneous_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStack, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Stack>
        {
            let f: &F = &*(f as *const F);
            f(&Stack::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::hhomogeneous\0".as_ptr() as *const _,
                Some(transmute(notify_hhomogeneous_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_homogeneous_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStack, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Stack>
        {
            let f: &F = &*(f as *const F);
            f(&Stack::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::homogeneous\0".as_ptr() as *const _,
                Some(transmute(notify_homogeneous_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_interpolate_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_interpolate_size_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStack, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Stack>
        {
            let f: &F = &*(f as *const F);
            f(&Stack::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::interpolate-size\0".as_ptr() as *const _,
                Some(transmute(notify_interpolate_size_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_pages_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pages_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStack, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Stack>
        {
            let f: &F = &*(f as *const F);
            f(&Stack::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::pages\0".as_ptr() as *const _,
                Some(transmute(notify_pages_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_transition_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_transition_duration_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStack, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Stack>
        {
            let f: &F = &*(f as *const F);
            f(&Stack::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::transition-duration\0".as_ptr() as *const _,
                Some(transmute(notify_transition_duration_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_transition_running_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_transition_running_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStack, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Stack>
        {
            let f: &F = &*(f as *const F);
            f(&Stack::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::transition-running\0".as_ptr() as *const _,
                Some(transmute(notify_transition_running_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_transition_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_transition_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStack, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Stack>
        {
            let f: &F = &*(f as *const F);
            f(&Stack::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::transition-type\0".as_ptr() as *const _,
                Some(transmute(notify_transition_type_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_vhomogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vhomogeneous_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStack, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Stack>
        {
            let f: &F = &*(f as *const F);
            f(&Stack::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::vhomogeneous\0".as_ptr() as *const _,
                Some(transmute(notify_vhomogeneous_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_visible_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_child_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStack, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Stack>
        {
            let f: &F = &*(f as *const F);
            f(&Stack::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::visible-child\0".as_ptr() as *const _,
                Some(transmute(notify_visible_child_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_visible_child_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_child_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkStack, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Stack>
        {
            let f: &F = &*(f as *const F);
            f(&Stack::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::visible-child-name\0".as_ptr() as *const _,
                Some(transmute(notify_visible_child_name_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stack")
    }
}
