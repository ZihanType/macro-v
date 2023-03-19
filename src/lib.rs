#![doc = include_str!("../README.md")]

mod v;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Visibility};
use v::MacroDefinition;

/// Attribute that make the visibility of the `macro_rules!` macro the same as
/// other items.
///
/// So the usage of `#[macro-v]` is private by default, as is the visibility of
/// other items. For example:
///
/// ```rust
/// // Use before declaration
/// private_macro!();
///
/// #[macro_v]
/// macro_rules! private_macro {
///     () => {};
/// }
///
/// mod inner {
///     // You must use the prefix `super::` or `crate::` to call the macro,
///     // because it is not in the current scope
///     super::private_macro!();
///     crate::private_macro!();
/// }
/// ```
///
/// You can also use `#[macro_v(pub(crate))]` to make the macro visible in the
/// current crate. For example:
///
/// ```rust
/// inner::example_macro!();
///
/// // No `#[macro_use]` needed!
/// mod inner {
///     use macro_v::macro_v;
///
///     #[macro_v(pub(crate))]
///     macro_rules! example_macro {
///         () => {};
///     }
/// }
/// ```
///
/// Defining public macros also no longer requires `#[macro_export]`, but
/// instead uses `#[macro-v(pub)]`. For example:
///
/// ```rust
/// pub mod inner {
///     use macro_v::macro_v;
///
///     #[macro_v(pub)]
///     macro_rules! public_macro {
///         () => {};
///     }
/// }
///
/// crate::inner::public_macro!();
/// ```
#[proc_macro_attribute]
pub fn macro_v(attr: TokenStream, item: TokenStream) -> TokenStream {
    let vis = parse_macro_input!(attr as Visibility);
    let macro_def = parse_macro_input!(item as MacroDefinition);

    v::generate(vis, macro_def)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
