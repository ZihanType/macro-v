use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hasher},
};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    Attribute, Ident, Meta, Token, Visibility,
};

pub(crate) struct MacroDefinition {
    attrs: Vec<Attribute>,
    macro_rules: Ident,
    bang_token: Token![!],
    name: Ident,
    tokens: TokenStream,
}

impl Parse for MacroDefinition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;

        let macro_rules = input.parse::<Ident>()?;
        if macro_rules != "macro_rules" {
            return Err(syn::Error::new(
                macro_rules.span(),
                "expected `macro_rules`",
            ));
        }

        let bang_token = input.parse::<Token![!]>()?;

        let name = input.parse::<Ident>()?;

        let tokens;
        braced!(tokens in input);

        Ok(Self {
            attrs,
            macro_rules,
            bang_token,
            name,
            tokens: tokens.parse()?,
        })
    }
}

pub(crate) fn generate(vis: Visibility, macro_def: MacroDefinition) -> syn::Result<TokenStream> {
    let MacroDefinition {
        attrs,
        macro_rules,
        bang_token,
        name,
        tokens,
    } = macro_def;

    let real_name = format_ident!("__{}_{}", name, RandomState::new().build_hasher().finish());

    let mut has_doc_comment = false;
    let mut has_doc_hidden = false;

    for attr in attrs.iter() {
        if has_doc_comment && has_doc_hidden {
            break;
        }

        if !attr.path().is_ident("doc") {
            continue;
        }

        if !has_doc_comment && matches!(attr.meta, Meta::NameValue(_)) {
            has_doc_comment = true;
            continue;
        }

        if !has_doc_hidden && matches!(attr.meta, Meta::List(_)) {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("hidden") {
                    has_doc_hidden = true;
                    return Ok(());
                }
                Ok(())
            })?;
        }
    }

    let blank_line = if has_doc_comment {
        quote! {
            #[doc = ""]
        }
    } else {
        quote! {}
    };

    let (doc_hidden, doc_inline) = if has_doc_hidden {
        (quote! {}, quote! {})
    } else {
        (
            quote! {
                #[doc(hidden)]
            },
            quote! {
                #[doc(inline)]
            },
        )
    };

    let export = if let Visibility::Public(_) = vis {
        quote! {
            #[macro_export]
        }
    } else {
        quote! {}
    };

    let expand = quote! {
        #doc_hidden
        #(#attrs)*
        #blank_line
        #[doc = "**[macro-v]**: If you want to use `pub use` to re-export and see the macro in the doc, you must add `#[doc(inline)]`."]
        #export
        #macro_rules #bang_token #real_name { #tokens }

        #doc_inline
        #vis use #real_name as #name;
    };

    Ok(expand)
}
