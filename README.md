# macro-v

[![Crates.io version](https://img.shields.io/crates/v/macro-v.svg?style=flat-square)](https://crates.io/crates/macro-v)

[![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/macro-v)

This crate provides an attribute macro for making the visibility of the `macro_rules!` macro the same as other items.

The visibility of declarative macros is not consistent with the behavior of other items in rust, necessitating the use of `#[macro_use]` and `#[macro_export]` instead of `pub` or `pub(...)`, such inconsistencies make the mental burden and cognitive cost significant. Now with this crate, you are allowed to use `#[macro_v]` or `#[macro_v(pub)]` or `#[macro_v(pub(...))]` on any `macro_rules!` macro, giving declarative macros the same visibility as other items, no more writing confusing `#[macro_use]` and `#[macro_export]`.

## Inspired

Inspired by [macro-vis](https://github.com/Kestrer/macro-vis) and even named after a part of it, but there are two problems of `macro-vis`:

1. you have to add `#![allow(uncommon_codepoints)]`.

2. the modified macro is shown in the documentation as a function instead of a macro.

To solve these two problems, I've reimplemented an attribute macro.

## How it works

It's very simple, see the code:

```rust
#[macro_v(pub(crate))]
macro_rules! example_macro { () => {}; }
```

... will expand to this:

```rust
#[doc(hidden)]
macro_rules! __example_macro_2228885075611141983 { () => {}; }

#[doc(inline)]
pub(crate) use __example_macro_2228885075611141983 as example_macro;
```

If you are using `#[macro_v(pub)]`, then the expanded code will then have `#[macro_export]` added to it:

```rust
#[doc(hidden)]
#[macro_export]
macro_rules! __example_macro_2228885075611141983 { () => {}; }

#[doc(inline)]
pub use __example_macro_2228885075611141983 as example_macro;
```

But because of using `#[doc(hidden)]`, you must use `#[doc(inline)]` attribute when re-exporting, otherwise re-exported macro won't be visible in the document. When using `#[macro_v]`, `#[doc(inline)]` will be added automatically, but if you want to re-export manually, you must remember to add `#[doc(inline)]`, which is the only problem.
