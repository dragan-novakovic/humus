pub mod node;
pub mod render;
pub mod vdom;

use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack(support_nested)]
pub use jsx_macro::html;