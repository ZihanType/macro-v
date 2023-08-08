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

mod private_module {
    inner::public_crate_macro!();

    // No `#[macro_use]` needed!
    mod inner {
        use macro_v::macro_v;

        #[doc = "public_crate_macro docs"]
        #[macro_v(pub(crate))]
        macro_rules! public_crate_macro {
            () => {};
        }
    }
}

crate::public_module::public_macro!();

crate::public_module::hidden_public_macro!();

pub mod public_module {
    use macro_v::macro_v;

    #[macro_v(pub)]
    #[doc = "public_macro docs"]
    macro_rules! public_macro {
        () => {};
    }

    #[doc(hidden)]
    #[macro_v(pub)]
    #[doc = "hidden_public_macro docs"]
    macro_rules! hidden_public_macro {
        () => {};
    }
}

crate::public_macro!();

#[macro_v(pub)]
#[doc = "public_macro docs"]
macro_rules! public_macro {
    () => {};
}

crate::hidden_public_macro!();

#[doc(hidden)]
#[macro_v(pub)]
#[doc = "hidden_public_macro docs"]
macro_rules! hidden_public_macro {
    () => {};
}

fn main() {}
