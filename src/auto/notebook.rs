// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Align;
use Buildable;
use Container;
use DirectionType;
use LayoutManager;
use NotebookPage;
use NotebookTab;
use Overflow;
use PackType;
use PositionType;
use Widget;
use gdk;
use gio;
use glib;
use glib::GString;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
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
    pub struct Notebook(Object<gtk_sys::GtkNotebook, gtk_sys::GtkNotebookClass, NotebookClass>) @extends Container, Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_notebook_get_type(),
    }
}

impl Notebook {
    pub fn new() -> Notebook {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_notebook_new()).unsafe_cast()
        }
    }
}

impl Default for Notebook {
    fn default() -> Self {
        Self::new()
    }
}

pub struct NotebookBuilder {
    enable_popup: Option<bool>,
    group_name: Option<String>,
    page: Option<i32>,
    scrollable: Option<bool>,
    show_border: Option<bool>,
    show_tabs: Option<bool>,
    tab_pos: Option<PositionType>,
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

impl NotebookBuilder {
    pub fn new() -> Self {
        Self {
            enable_popup: None,
            group_name: None,
            page: None,
            scrollable: None,
            show_border: None,
            show_tabs: None,
            tab_pos: None,
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

    pub fn build(self) -> Notebook {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref enable_popup) = self.enable_popup {
            properties.push(("enable-popup", enable_popup));
        }
        if let Some(ref group_name) = self.group_name {
            properties.push(("group-name", group_name));
        }
        if let Some(ref page) = self.page {
            properties.push(("page", page));
        }
        if let Some(ref scrollable) = self.scrollable {
            properties.push(("scrollable", scrollable));
        }
        if let Some(ref show_border) = self.show_border {
            properties.push(("show-border", show_border));
        }
        if let Some(ref show_tabs) = self.show_tabs {
            properties.push(("show-tabs", show_tabs));
        }
        if let Some(ref tab_pos) = self.tab_pos {
            properties.push(("tab-pos", tab_pos));
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
        glib::Object::new(Notebook::static_type(), &properties).expect("object new").downcast().expect("downcast")
    }

    pub fn enable_popup(mut self, enable_popup: bool) -> Self {
        self.enable_popup = Some(enable_popup);
        self
    }

    pub fn group_name(mut self, group_name: &str) -> Self {
        self.group_name = Some(group_name.to_string());
        self
    }

    pub fn page(mut self, page: i32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn scrollable(mut self, scrollable: bool) -> Self {
        self.scrollable = Some(scrollable);
        self
    }

    pub fn show_border(mut self, show_border: bool) -> Self {
        self.show_border = Some(show_border);
        self
    }

    pub fn show_tabs(mut self, show_tabs: bool) -> Self {
        self.show_tabs = Some(show_tabs);
        self
    }

    pub fn tab_pos(mut self, tab_pos: PositionType) -> Self {
        self.tab_pos = Some(tab_pos);
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

pub const NONE_NOTEBOOK: Option<&Notebook> = None;

pub trait NotebookExt: 'static {
    fn detach_tab<P: IsA<Widget>>(&self, child: &P);

    fn get_action_widget(&self, pack_type: PackType) -> Option<Widget>;

    fn get_group_name(&self) -> Option<GString>;

    fn get_menu_label<P: IsA<Widget>>(&self, child: &P) -> Option<Widget>;

    fn get_menu_label_text<P: IsA<Widget>>(&self, child: &P) -> Option<GString>;

    fn get_page<P: IsA<Widget>>(&self, child: &P) -> Option<NotebookPage>;

    fn get_pages(&self) -> Option<gio::ListModel>;

    fn get_scrollable(&self) -> bool;

    fn get_show_border(&self) -> bool;

    fn get_show_tabs(&self) -> bool;

    fn get_tab_detachable<P: IsA<Widget>>(&self, child: &P) -> bool;

    fn get_tab_label<P: IsA<Widget>>(&self, child: &P) -> Option<Widget>;

    fn get_tab_label_text<P: IsA<Widget>>(&self, child: &P) -> Option<GString>;

    fn get_tab_pos(&self) -> PositionType;

    fn get_tab_reorderable<P: IsA<Widget>>(&self, child: &P) -> bool;

    fn next_page(&self);

    fn popup_disable(&self);

    fn popup_enable(&self);

    fn prev_page(&self);

    fn set_action_widget<P: IsA<Widget>>(&self, widget: &P, pack_type: PackType);

    fn set_group_name(&self, group_name: Option<&str>);

    fn set_menu_label<P: IsA<Widget>, Q: IsA<Widget>>(&self, child: &P, menu_label: Option<&Q>);

    fn set_menu_label_text<P: IsA<Widget>>(&self, child: &P, menu_text: &str);

    fn set_scrollable(&self, scrollable: bool);

    fn set_show_border(&self, show_border: bool);

    fn set_show_tabs(&self, show_tabs: bool);

    fn set_tab_detachable<P: IsA<Widget>>(&self, child: &P, detachable: bool);

    fn set_tab_label<P: IsA<Widget>, Q: IsA<Widget>>(&self, child: &P, tab_label: Option<&Q>);

    fn set_tab_label_text<P: IsA<Widget>>(&self, child: &P, tab_text: &str);

    fn set_tab_pos(&self, pos: PositionType);

    fn set_tab_reorderable<P: IsA<Widget>>(&self, child: &P, reorderable: bool);

    fn get_property_enable_popup(&self) -> bool;

    fn set_property_enable_popup(&self, enable_popup: bool);

    fn set_property_page(&self, page: i32);

    fn connect_change_current_page<F: Fn(&Self, i32) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_change_current_page(&self, object: i32) -> bool;

    fn connect_create_window<F: Fn(&Self, &Widget, i32, i32) -> Notebook + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_focus_tab<F: Fn(&Self, NotebookTab) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_focus_tab(&self, object: NotebookTab) -> bool;

    fn connect_move_focus_out<F: Fn(&Self, DirectionType) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_focus_out(&self, object: DirectionType);

    fn connect_page_added<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_page_removed<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_page_reordered<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_reorder_tab<F: Fn(&Self, DirectionType, bool) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_reorder_tab(&self, object: DirectionType, p0: bool) -> bool;

    fn connect_select_page<F: Fn(&Self, bool) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_select_page(&self, object: bool) -> bool;

    fn connect_switch_page<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_enable_popup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_group_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_page_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pages_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_scrollable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_border_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_tabs_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tab_pos_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Notebook>> NotebookExt for O {
    fn detach_tab<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            gtk_sys::gtk_notebook_detach_tab(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0);
        }
    }

    fn get_action_widget(&self, pack_type: PackType) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_notebook_get_action_widget(self.as_ref().to_glib_none().0, pack_type.to_glib()))
        }
    }

    fn get_group_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_notebook_get_group_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_menu_label<P: IsA<Widget>>(&self, child: &P) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_notebook_get_menu_label(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0))
        }
    }

    fn get_menu_label_text<P: IsA<Widget>>(&self, child: &P) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_notebook_get_menu_label_text(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0))
        }
    }

    fn get_page<P: IsA<Widget>>(&self, child: &P) -> Option<NotebookPage> {
        unsafe {
            from_glib_none(gtk_sys::gtk_notebook_get_page(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0))
        }
    }

    fn get_pages(&self) -> Option<gio::ListModel> {
        unsafe {
            from_glib_full(gtk_sys::gtk_notebook_get_pages(self.as_ref().to_glib_none().0))
        }
    }

    fn get_scrollable(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_notebook_get_scrollable(self.as_ref().to_glib_none().0))
        }
    }

    fn get_show_border(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_notebook_get_show_border(self.as_ref().to_glib_none().0))
        }
    }

    fn get_show_tabs(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_notebook_get_show_tabs(self.as_ref().to_glib_none().0))
        }
    }

    fn get_tab_detachable<P: IsA<Widget>>(&self, child: &P) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_notebook_get_tab_detachable(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0))
        }
    }

    fn get_tab_label<P: IsA<Widget>>(&self, child: &P) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_notebook_get_tab_label(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0))
        }
    }

    fn get_tab_label_text<P: IsA<Widget>>(&self, child: &P) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_notebook_get_tab_label_text(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0))
        }
    }

    fn get_tab_pos(&self) -> PositionType {
        unsafe {
            from_glib(gtk_sys::gtk_notebook_get_tab_pos(self.as_ref().to_glib_none().0))
        }
    }

    fn get_tab_reorderable<P: IsA<Widget>>(&self, child: &P) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_notebook_get_tab_reorderable(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0))
        }
    }

    fn next_page(&self) {
        unsafe {
            gtk_sys::gtk_notebook_next_page(self.as_ref().to_glib_none().0);
        }
    }

    fn popup_disable(&self) {
        unsafe {
            gtk_sys::gtk_notebook_popup_disable(self.as_ref().to_glib_none().0);
        }
    }

    fn popup_enable(&self) {
        unsafe {
            gtk_sys::gtk_notebook_popup_enable(self.as_ref().to_glib_none().0);
        }
    }

    fn prev_page(&self) {
        unsafe {
            gtk_sys::gtk_notebook_prev_page(self.as_ref().to_glib_none().0);
        }
    }

    fn set_action_widget<P: IsA<Widget>>(&self, widget: &P, pack_type: PackType) {
        unsafe {
            gtk_sys::gtk_notebook_set_action_widget(self.as_ref().to_glib_none().0, widget.as_ref().to_glib_none().0, pack_type.to_glib());
        }
    }

    fn set_group_name(&self, group_name: Option<&str>) {
        unsafe {
            gtk_sys::gtk_notebook_set_group_name(self.as_ref().to_glib_none().0, group_name.to_glib_none().0);
        }
    }

    fn set_menu_label<P: IsA<Widget>, Q: IsA<Widget>>(&self, child: &P, menu_label: Option<&Q>) {
        unsafe {
            gtk_sys::gtk_notebook_set_menu_label(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, menu_label.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_menu_label_text<P: IsA<Widget>>(&self, child: &P, menu_text: &str) {
        unsafe {
            gtk_sys::gtk_notebook_set_menu_label_text(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, menu_text.to_glib_none().0);
        }
    }

    fn set_scrollable(&self, scrollable: bool) {
        unsafe {
            gtk_sys::gtk_notebook_set_scrollable(self.as_ref().to_glib_none().0, scrollable.to_glib());
        }
    }

    fn set_show_border(&self, show_border: bool) {
        unsafe {
            gtk_sys::gtk_notebook_set_show_border(self.as_ref().to_glib_none().0, show_border.to_glib());
        }
    }

    fn set_show_tabs(&self, show_tabs: bool) {
        unsafe {
            gtk_sys::gtk_notebook_set_show_tabs(self.as_ref().to_glib_none().0, show_tabs.to_glib());
        }
    }

    fn set_tab_detachable<P: IsA<Widget>>(&self, child: &P, detachable: bool) {
        unsafe {
            gtk_sys::gtk_notebook_set_tab_detachable(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, detachable.to_glib());
        }
    }

    fn set_tab_label<P: IsA<Widget>, Q: IsA<Widget>>(&self, child: &P, tab_label: Option<&Q>) {
        unsafe {
            gtk_sys::gtk_notebook_set_tab_label(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, tab_label.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_tab_label_text<P: IsA<Widget>>(&self, child: &P, tab_text: &str) {
        unsafe {
            gtk_sys::gtk_notebook_set_tab_label_text(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, tab_text.to_glib_none().0);
        }
    }

    fn set_tab_pos(&self, pos: PositionType) {
        unsafe {
            gtk_sys::gtk_notebook_set_tab_pos(self.as_ref().to_glib_none().0, pos.to_glib());
        }
    }

    fn set_tab_reorderable<P: IsA<Widget>>(&self, child: &P, reorderable: bool) {
        unsafe {
            gtk_sys::gtk_notebook_set_tab_reorderable(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, reorderable.to_glib());
        }
    }

    fn get_property_enable_popup(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"enable-popup\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_enable_popup(&self, enable_popup: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"enable-popup\0".as_ptr() as *const _, Value::from(&enable_popup).to_glib_none().0);
        }
    }

    fn set_property_page(&self, page: i32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"page\0".as_ptr() as *const _, Value::from(&page).to_glib_none().0);
        }
    }

    fn connect_change_current_page<F: Fn(&Self, i32) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn change_current_page_trampoline<P, F: Fn(&P, i32) -> bool + 'static>(this: *mut gtk_sys::GtkNotebook, object: libc::c_int, f: glib_sys::gpointer) -> glib_sys::gboolean
            where P: IsA<Notebook>
        {
            let f: &F = &*(f as *const F);
            f(&Notebook::from_glib_borrow(this).unsafe_cast(), object).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"change-current-page\0".as_ptr() as *const _,
                Some(transmute(change_current_page_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_change_current_page(&self, object: i32) -> bool {
        let res = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("change-current-page", &[&object]).unwrap() };
        res.unwrap().get().unwrap()
    }

    fn connect_create_window<F: Fn(&Self, &Widget, i32, i32) -> Notebook + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn create_window_trampoline<P, F: Fn(&P, &Widget, i32, i32) -> Notebook + 'static>(this: *mut gtk_sys::GtkNotebook, page: *mut gtk_sys::GtkWidget, x: libc::c_int, y: libc::c_int, f: glib_sys::gpointer) -> *mut gtk_sys::GtkNotebook
            where P: IsA<Notebook>
        {
            let f: &F = &*(f as *const F);
            f(&Notebook::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(page), x, y)/*Not checked*/.to_glib_none().0
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"create-window\0".as_ptr() as *const _,
                Some(transmute(create_window_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_focus_tab<F: Fn(&Self, NotebookTab) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn focus_tab_trampoline<P, F: Fn(&P, NotebookTab) -> bool + 'static>(this: *mut gtk_sys::GtkNotebook, object: gtk_sys::GtkNotebookTab, f: glib_sys::gpointer) -> glib_sys::gboolean
            where P: IsA<Notebook>
        {
            let f: &F = &*(f as *const F);
            f(&Notebook::from_glib_borrow(this).unsafe_cast(), from_glib(object)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"focus-tab\0".as_ptr() as *const _,
                Some(transmute(focus_tab_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_focus_tab(&self, object: NotebookTab) -> bool {
        let res = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("focus-tab", &[&object]).unwrap() };
        res.unwrap().get().unwrap()
    }

    fn connect_move_focus_out<F: Fn(&Self, DirectionType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn move_focus_out_trampoline<P, F: Fn(&P, DirectionType) + 'static>(this: *mut gtk_sys::GtkNotebook, object: gtk_sys::GtkDirectionType, f: glib_sys::gpointer)
            where P: IsA<Notebook>
        {
            let f: &F = &*(f as *const F);
            f(&Notebook::from_glib_borrow(this).unsafe_cast(), from_glib(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"move-focus-out\0".as_ptr() as *const _,
                Some(transmute(move_focus_out_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_move_focus_out(&self, object: DirectionType) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("move-focus-out", &[&object]).unwrap() };
    }

    fn connect_page_added<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn page_added_trampoline<P, F: Fn(&P, &Widget, u32) + 'static>(this: *mut gtk_sys::GtkNotebook, child: *mut gtk_sys::GtkWidget, page_num: libc::c_uint, f: glib_sys::gpointer)
            where P: IsA<Notebook>
        {
            let f: &F = &*(f as *const F);
            f(&Notebook::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(child), page_num)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"page-added\0".as_ptr() as *const _,
                Some(transmute(page_added_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_page_removed<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn page_removed_trampoline<P, F: Fn(&P, &Widget, u32) + 'static>(this: *mut gtk_sys::GtkNotebook, child: *mut gtk_sys::GtkWidget, page_num: libc::c_uint, f: glib_sys::gpointer)
            where P: IsA<Notebook>
        {
            let f: &F = &*(f as *const F);
            f(&Notebook::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(child), page_num)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"page-removed\0".as_ptr() as *const _,
                Some(transmute(page_removed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_page_reordered<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn page_reordered_trampoline<P, F: Fn(&P, &Widget, u32) + 'static>(this: *mut gtk_sys::GtkNotebook, child: *mut gtk_sys::GtkWidget, page_num: libc::c_uint, f: glib_sys::gpointer)
            where P: IsA<Notebook>
        {
            let f: &F = &*(f as *const F);
            f(&Notebook::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(child), page_num)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"page-reordered\0".as_ptr() as *const _,
                Some(transmute(page_reordered_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_reorder_tab<F: Fn(&Self, DirectionType, bool) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn reorder_tab_trampoline<P, F: Fn(&P, DirectionType, bool) -> bool + 'static>(this: *mut gtk_sys::GtkNotebook, object: gtk_sys::GtkDirectionType, p0: glib_sys::gboolean, f: glib_sys::gpointer) -> glib_sys::gboolean
            where P: IsA<Notebook>
        {
            let f: &F = &*(f as *const F);
            f(&Notebook::from_glib_borrow(this).unsafe_cast(), from_glib(object), from_glib(p0)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"reorder-tab\0".as_ptr() as *const _,
                Some(transmute(reorder_tab_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_reorder_tab(&self, object: DirectionType, p0: bool) -> bool {
        let res = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("reorder-tab", &[&object, &p0]).unwrap() };
        res.unwrap().get().unwrap()
    }

    fn connect_select_page<F: Fn(&Self, bool) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn select_page_trampoline<P, F: Fn(&P, bool) -> bool + 'static>(this: *mut gtk_sys::GtkNotebook, object: glib_sys::gboolean, f: glib_sys::gpointer) -> glib_sys::gboolean
            where P: IsA<Notebook>
        {
            let f: &F = &*(f as *const F);
            f(&Notebook::from_glib_borrow(this).unsafe_cast(), from_glib(object)).to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"select-page\0".as_ptr() as *const _,
                Some(transmute(select_page_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_select_page(&self, object: bool) -> bool {
        let res = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("select-page", &[&object]).unwrap() };
        res.unwrap().get().unwrap()
    }

    fn connect_switch_page<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn switch_page_trampoline<P, F: Fn(&P, &Widget, u32) + 'static>(this: *mut gtk_sys::GtkNotebook, page: *mut gtk_sys::GtkWidget, page_num: libc::c_uint, f: glib_sys::gpointer)
            where P: IsA<Notebook>
        {
            let f: &F = &*(f as *const F);
            f(&Notebook::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(page), page_num)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"switch-page\0".as_ptr() as *const _,
                Some(transmute(switch_page_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_enable_popup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_enable_popup_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkNotebook, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Notebook>
        {
            let f: &F = &*(f as *const F);
            f(&Notebook::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::enable-popup\0".as_ptr() as *const _,
                Some(transmute(notify_enable_popup_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_group_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_group_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkNotebook, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Notebook>
        {
            let f: &F = &*(f as *const F);
            f(&Notebook::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::group-name\0".as_ptr() as *const _,
                Some(transmute(notify_group_name_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_page_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_page_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkNotebook, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Notebook>
        {
            let f: &F = &*(f as *const F);
            f(&Notebook::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::page\0".as_ptr() as *const _,
                Some(transmute(notify_page_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_pages_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pages_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkNotebook, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Notebook>
        {
            let f: &F = &*(f as *const F);
            f(&Notebook::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::pages\0".as_ptr() as *const _,
                Some(transmute(notify_pages_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_scrollable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_scrollable_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkNotebook, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Notebook>
        {
            let f: &F = &*(f as *const F);
            f(&Notebook::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::scrollable\0".as_ptr() as *const _,
                Some(transmute(notify_scrollable_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_border_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_border_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkNotebook, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Notebook>
        {
            let f: &F = &*(f as *const F);
            f(&Notebook::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-border\0".as_ptr() as *const _,
                Some(transmute(notify_show_border_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_tabs_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_tabs_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkNotebook, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Notebook>
        {
            let f: &F = &*(f as *const F);
            f(&Notebook::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-tabs\0".as_ptr() as *const _,
                Some(transmute(notify_show_tabs_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_tab_pos_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tab_pos_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkNotebook, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<Notebook>
        {
            let f: &F = &*(f as *const F);
            f(&Notebook::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::tab-pos\0".as_ptr() as *const _,
                Some(transmute(notify_tab_pos_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Notebook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Notebook")
    }
}
