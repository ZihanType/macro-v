use macro_v::macro_v;

// Use before declaration
private_macro!();

#[macro_v]
/// Hello
#[doc(hidden)]
macro_rules! private_macro {
    () => {};
}

mod inner {
    super::private_macro!();
    crate::private_macro!();
}

mod a {
    inner::example_macro!();

    // No `#[macro_use]` needed!
    mod inner {
        use macro_v::macro_v;

        #[doc = "pub(crate) docs"]
        #[doc(hidden)]
        #[macro_v(pub(crate))]
        macro_rules! example_macro {
            () => {};
        }
    }
}

pub mod b {
    use macro_v::macro_v;

    #[macro_v(pub)]
    #[doc = "public_macro docs"]
    macro_rules! public_macro {
        () => {};
    }
}

crate::b::public_macro!();

fn main() {}
