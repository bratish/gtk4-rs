// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Align;
use AppChooser;
use Application;
use Bin;
use Buildable;
use Container;
use Dialog;
use DialogFlags;
use LayoutManager;
use Overflow;
use Root;
use Widget;
use Window;
use WindowPosition;
use WindowType;
use gdk;
use gio;
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
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct AppChooserDialog(Object<gtk_sys::GtkAppChooserDialog, gtk_sys::GtkAppChooserDialogClass, AppChooserDialogClass>) @extends Dialog, Window, Bin, Container, Widget, @implements Buildable, Root, AppChooser;

    match fn {
        get_type => || gtk_sys::gtk_app_chooser_dialog_get_type(),
    }
}

impl AppChooserDialog {
    pub fn new<P: IsA<Window>, Q: IsA<gio::File>>(parent: Option<&P>, flags: DialogFlags, file: &Q) -> AppChooserDialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_app_chooser_dialog_new(parent.map(|p| p.as_ref()).to_glib_none().0, flags.to_glib(), file.as_ref().to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_for_content_type<P: IsA<Window>>(parent: Option<&P>, flags: DialogFlags, content_type: &str) -> AppChooserDialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_app_chooser_dialog_new_for_content_type(parent.map(|p| p.as_ref()).to_glib_none().0, flags.to_glib(), content_type.to_glib_none().0)).unsafe_cast()
        }
    }
}

pub struct AppChooserDialogBuilder {
    gfile: Option<gio::File>,
    heading: Option<String>,
    use_header_bar: Option<i32>,
    accept_focus: Option<bool>,
    application: Option<Application>,
    attached_to: Option<Widget>,
    decorated: Option<bool>,
    default_height: Option<i32>,
    default_widget: Option<Widget>,
    default_width: Option<i32>,
    deletable: Option<bool>,
    destroy_with_parent: Option<bool>,
    display: Option<gdk::Display>,
    focus_on_map: Option<bool>,
    focus_visible: Option<bool>,
    hide_on_close: Option<bool>,
    icon_name: Option<String>,
    mnemonics_visible: Option<bool>,
    modal: Option<bool>,
    resizable: Option<bool>,
    startup_id: Option<String>,
    title: Option<String>,
    transient_for: Option<Window>,
    type_: Option<WindowType>,
    type_hint: Option<gdk::SurfaceTypeHint>,
    window_position: Option<WindowPosition>,
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

impl AppChooserDialogBuilder {
    pub fn new() -> Self {
        Self {
            gfile: None,
            heading: None,
            use_header_bar: None,
            accept_focus: None,
            application: None,
            attached_to: None,
            decorated: None,
            default_height: None,
            default_widget: None,
            default_width: None,
            deletable: None,
            destroy_with_parent: None,
            display: None,
            focus_on_map: None,
            focus_visible: None,
            hide_on_close: None,
            icon_name: None,
            mnemonics_visible: None,
            modal: None,
            resizable: None,
            startup_id: None,
            title: None,
            transient_for: None,
            type_: None,
            type_hint: None,
            window_position: None,
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

    pub fn build(self) -> AppChooserDialog {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref gfile) = self.gfile {
            properties.push(("gfile", gfile));
        }
        if let Some(ref heading) = self.heading {
            properties.push(("heading", heading));
        }
        if let Some(ref use_header_bar) = self.use_header_bar {
            properties.push(("use-header-bar", use_header_bar));
        }
        if let Some(ref accept_focus) = self.accept_focus {
            properties.push(("accept-focus", accept_focus));
        }
        if let Some(ref application) = self.application {
            properties.push(("application", application));
        }
        if let Some(ref attached_to) = self.attached_to {
            properties.push(("attached-to", attached_to));
        }
        if let Some(ref decorated) = self.decorated {
            properties.push(("decorated", decorated));
        }
        if let Some(ref default_height) = self.default_height {
            properties.push(("default-height", default_height));
        }
        if let Some(ref default_widget) = self.default_widget {
            properties.push(("default-widget", default_widget));
        }
        if let Some(ref default_width) = self.default_width {
            properties.push(("default-width", default_width));
        }
        if let Some(ref deletable) = self.deletable {
            properties.push(("deletable", deletable));
        }
        if let Some(ref destroy_with_parent) = self.destroy_with_parent {
            properties.push(("destroy-with-parent", destroy_with_parent));
        }
        if let Some(ref display) = self.display {
            properties.push(("display", display));
        }
        if let Some(ref focus_on_map) = self.focus_on_map {
            properties.push(("focus-on-map", focus_on_map));
        }
        if let Some(ref focus_visible) = self.focus_visible {
            properties.push(("focus-visible", focus_visible));
        }
        if let Some(ref hide_on_close) = self.hide_on_close {
            properties.push(("hide-on-close", hide_on_close));
        }
        if let Some(ref icon_name) = self.icon_name {
            properties.push(("icon-name", icon_name));
        }
        if let Some(ref mnemonics_visible) = self.mnemonics_visible {
            properties.push(("mnemonics-visible", mnemonics_visible));
        }
        if let Some(ref modal) = self.modal {
            properties.push(("modal", modal));
        }
        if let Some(ref resizable) = self.resizable {
            properties.push(("resizable", resizable));
        }
        if let Some(ref startup_id) = self.startup_id {
            properties.push(("startup-id", startup_id));
        }
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        if let Some(ref transient_for) = self.transient_for {
            properties.push(("transient-for", transient_for));
        }
        if let Some(ref type_) = self.type_ {
            properties.push(("type", type_));
        }
        if let Some(ref type_hint) = self.type_hint {
            properties.push(("type-hint", type_hint));
        }
        if let Some(ref window_position) = self.window_position {
            properties.push(("window-position", window_position));
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
        glib::Object::new(AppChooserDialog::static_type(), &properties).expect("object new").downcast().expect("downcast")
    }

    pub fn gfile(mut self, gfile: &gio::File) -> Self {
        self.gfile = Some(gfile.clone());
        self
    }

    pub fn heading(mut self, heading: &str) -> Self {
        self.heading = Some(heading.to_string());
        self
    }

    pub fn use_header_bar(mut self, use_header_bar: i32) -> Self {
        self.use_header_bar = Some(use_header_bar);
        self
    }

    pub fn accept_focus(mut self, accept_focus: bool) -> Self {
        self.accept_focus = Some(accept_focus);
        self
    }

    pub fn application(mut self, application: &Application) -> Self {
        self.application = Some(application.clone());
        self
    }

    pub fn attached_to(mut self, attached_to: &Widget) -> Self {
        self.attached_to = Some(attached_to.clone());
        self
    }

    pub fn decorated(mut self, decorated: bool) -> Self {
        self.decorated = Some(decorated);
        self
    }

    pub fn default_height(mut self, default_height: i32) -> Self {
        self.default_height = Some(default_height);
        self
    }

    pub fn default_widget(mut self, default_widget: &Widget) -> Self {
        self.default_widget = Some(default_widget.clone());
        self
    }

    pub fn default_width(mut self, default_width: i32) -> Self {
        self.default_width = Some(default_width);
        self
    }

    pub fn deletable(mut self, deletable: bool) -> Self {
        self.deletable = Some(deletable);
        self
    }

    pub fn destroy_with_parent(mut self, destroy_with_parent: bool) -> Self {
        self.destroy_with_parent = Some(destroy_with_parent);
        self
    }

    pub fn display(mut self, display: &gdk::Display) -> Self {
        self.display = Some(display.clone());
        self
    }

    pub fn focus_on_map(mut self, focus_on_map: bool) -> Self {
        self.focus_on_map = Some(focus_on_map);
        self
    }

    pub fn focus_visible(mut self, focus_visible: bool) -> Self {
        self.focus_visible = Some(focus_visible);
        self
    }

    pub fn hide_on_close(mut self, hide_on_close: bool) -> Self {
        self.hide_on_close = Some(hide_on_close);
        self
    }

    pub fn icon_name(mut self, icon_name: &str) -> Self {
        self.icon_name = Some(icon_name.to_string());
        self
    }

    pub fn mnemonics_visible(mut self, mnemonics_visible: bool) -> Self {
        self.mnemonics_visible = Some(mnemonics_visible);
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = Some(modal);
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = Some(resizable);
        self
    }

    pub fn startup_id(mut self, startup_id: &str) -> Self {
        self.startup_id = Some(startup_id.to_string());
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn transient_for(mut self, transient_for: &Window) -> Self {
        self.transient_for = Some(transient_for.clone());
        self
    }

    pub fn type_(mut self, type_: WindowType) -> Self {
        self.type_ = Some(type_);
        self
    }

    pub fn type_hint(mut self, type_hint: gdk::SurfaceTypeHint) -> Self {
        self.type_hint = Some(type_hint);
        self
    }

    pub fn window_position(mut self, window_position: WindowPosition) -> Self {
        self.window_position = Some(window_position);
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

pub const NONE_APP_CHOOSER_DIALOG: Option<&AppChooserDialog> = None;

pub trait AppChooserDialogExt: 'static {
    fn get_heading(&self) -> Option<GString>;

    fn get_widget(&self) -> Widget;

    fn set_heading(&self, heading: &str);

    fn get_property_gfile(&self) -> Option<gio::File>;

    fn connect_property_heading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AppChooserDialog>> AppChooserDialogExt for O {
    fn get_heading(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_app_chooser_dialog_get_heading(self.as_ref().to_glib_none().0))
        }
    }

    fn get_widget(&self) -> Widget {
        unsafe {
            from_glib_none(gtk_sys::gtk_app_chooser_dialog_get_widget(self.as_ref().to_glib_none().0))
        }
    }

    fn set_heading(&self, heading: &str) {
        unsafe {
            gtk_sys::gtk_app_chooser_dialog_set_heading(self.as_ref().to_glib_none().0, heading.to_glib_none().0);
        }
    }

    fn get_property_gfile(&self) -> Option<gio::File> {
        unsafe {
            let mut value = Value::from_type(<gio::File as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"gfile\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_property_heading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_heading_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkAppChooserDialog, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<AppChooserDialog>
        {
            let f: &F = &*(f as *const F);
            f(&AppChooserDialog::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::heading\0".as_ptr() as *const _,
                Some(transmute(notify_heading_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for AppChooserDialog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AppChooserDialog")
    }
}
