use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hasher},
};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    Attribute, Ident, Lit, Meta, MetaList, MetaNameValue, NestedMeta, Token, Visibility,
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

        if !attr.path.is_ident("doc") {
            continue;
        }

        if !has_doc_comment {
            if let Ok(Meta::NameValue(MetaNameValue {
                lit: Lit::Str(_), ..
            })) = attr.parse_meta()
            {
                has_doc_comment = true;
                continue;
            }
        }

        if !has_doc_hidden {
            if let Ok(Meta::List(MetaList { nested, .. })) = attr.parse_meta() {
                has_doc_hidden = nested
                    .iter()
                    .any(|n| matches!(n, NestedMeta::Meta(Meta::Path(p)) if p.is_ident("hidden")));
            }
        }
    }

    let blank_line = if has_doc_comment {
        Some(quote!(#[doc = ""]))
    } else {
        None
    };

    let (doc_hidden, doc_inline) = if has_doc_hidden {
        (None, None)
    } else {
        (Some(quote!(#[doc(hidden)])), Some(quote!(#[doc(inline)])))
    };

    let export = if let Visibility::Public(_) = vis {
        Some(quote!(#[macro_export]))
    } else {
        None
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
