// Take a look at the license at the top of the repository in the LICENSE file.

//! # GTK 4 Macros
//!
//! The crate aims to provide useful macros to use with the GTK 4 Rust bindings.

mod attribute_parser;
mod composite_template_derive;
mod util;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for using a composite template in a widget.
///
/// The `template` attribute specifies where the template should be loaded
/// from;  it can be a `file`, a `resource`, or a `string`.
///
/// The `template_child` attribute is used to mark all internal widgets
/// we need to have programmatic access to.
///
/// # Example
///
/// Specify that `MyWidget` is using a composite template and load the
/// template file the `composite_template.ui` file.
///
/// Then, in the `ObjectSubclass` implementation you will need to call
/// `bind_template` in the `class_init` function, and `init_template` in
/// `instance_init` function.
///
///
/// ```no_run
/// # fn main() {}
/// use gtk::prelude::*;
/// use gtk::glib;
/// use gtk::CompositeTemplate;
/// use gtk::subclass::prelude::*;
///
/// mod imp {
///     use super::*;
///
///     #[derive(Debug, Default, CompositeTemplate)]
///     #[template(file = "test/template.ui")]
///     pub struct MyDialog {
///         #[template_child]
///         pub label: TemplateChild<gtk::Label>,
///         #[template_child(id = "my_button_id")]
///         pub button: TemplateChild<gtk::Button>,
///         #[template_child(internal = true)]
///         pub ok_button: TemplateChild<gtk::Button>,
///     }
///
///     #[glib::object_subclass]
///     impl ObjectSubclass for MyDialog {
///         const NAME: &'static str = "MyDialog";
///         type Type = super::MyDialog;
///         type ParentType = gtk::Dialog;
///
///         fn class_init(klass: &mut Self::Class) {
///             Self::bind_template(klass);
///         }
///
///         fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
///             obj.init_template();
///         }
///     }
///
///     impl ObjectImpl for MyDialog {}
///     impl WidgetImpl for MyDialog {}
///     impl WindowImpl for MyDialog {}
///     impl DialogImpl for MyDialog {}
/// }
///
/// glib::wrapper! {
///     pub struct MyDialog(ObjectSubclass<imp::MyDialog>) @extends gtk::Widget, gtk::Window, gtk::Dialog;
/// }
///
/// impl MyDialog {
///     pub fn new() -> Self {
///         glib::Object::new(&[]).expect("Failed to create an instance of MyDialog")
///     }
/// }
/// ```
#[proc_macro_derive(CompositeTemplate, attributes(template, template_child))]
#[proc_macro_error]
pub fn composite_template_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let gen = composite_template_derive::impl_composite_template(&input);
    gen.into()
}
