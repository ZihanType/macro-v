//! This crate provides an attribute macro for making the visibility of the
//! `macro_rules!` macro the same as other items.
//!
//! The visibility of declarative macros is not consistent with the behavior of
//! other items in rust, necessitating the use of `#[macro_use]` and
//! `#[macro_export]` instead of `pub` or `pub(...)`, such inconsistencies make
//! the mental burden and cognitive cost significant. Now with this crate, you
//! are allowed to use `#[macro_v]` or `#[macro_v(pub)]` or
//! `#[macro_v(pub(...))]` on any `macro_rules!` macro, giving declarative
//! macros the same visibility as other items, no more writing confusing
//! `#[macro_use]` and `#[macro_export]`.
//!
//! ## Inspired
//!
//! Inspired by [macro-vis](https://github.com/Kestrer/macro-vis) and even named
//! after a part of it, but there are two problems of `macro-vis`:
//!
//! 1. you have to add `#![allow(uncommon_codepoints)]`.
//!
//! 2. the modified macro is shown in the documentation as a function instead of
//! a macro.
//!
//! To solve these two problems, I've reimplemented an attribute macro.
//!
//! ## How it works
//!
//! It's very simple, see the code:
//!
//! ```rust
//! # use macro_v::macro_v;
//! #[macro_v(pub(crate))]
//! macro_rules! example_macro {
//!     () => {};
//! }
//! ```
//!
//! ... will expand to this:
//!
//! ```rust
//! #[doc(hidden)]
//! macro_rules! __example_macro_2228885075611141983 {
//!     () => {};
//! }
//!
//! #[doc(inline)]
//! pub(crate) use __example_macro_2228885075611141983 as example_macro;
//! ```
//!
//! If you are using `#[macro_v(pub)]`, then the expanded code will then have
//! `#[macro_export]` added to it:
//!
//! ```rust
//! #[doc(hidden)]
//! #[macro_export]
//! macro_rules! __example_macro_2228885075611141983 {
//!     () => {};
//! }
//!
//! #[doc(inline)]
//! pub use __example_macro_2228885075611141983 as example_macro;
//! ```
//!
//! ## Limitations
//!
//! Because of using `#[doc(hidden)]`, you must use `#[doc(inline)]` attribute
//! when re-exporting, otherwise re-exported macro won't be visible in the
//! document. When using `#[macro_v]`, `#[doc(inline)]` will be added
//! automatically, but **if you want to manually re-export the macro, you must
//! also manually add `#[doc(inline)]`**, which is the only problem.

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
