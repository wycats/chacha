use derive_syn_parse::Parse;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Token};
use syn::{Generics, Ident};
use unicode_xid::UnicodeXID;

#[derive(Parse)]
struct ChachaDataMeta {
    _copy: Option<syn::Ident>,
}

#[proc_macro_attribute]
pub fn chacha_value(metadata: proc_macro::TokenStream, input: TokenStream) -> TokenStream {
    let _meta = parse_macro_input!(metadata as ChachaDataMeta);

    // Parse the string representation
    let ast = parse_macro_input!(input as DeriveInput);

    let attrs = quote! {
        Debug, Hash, Eq, PartialEq, Ord, PartialOrd
    };

    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data: _,
    } = ast.clone();

    let Generics {
        lt_token,
        gt_token,
        params,
        where_clause,
    } = generics;

    let type_params = quote! {
        #lt_token #params #gt_token
    };

    let expanded = quote! {
        #[derive(#attrs)]
        #ast

        impl #type_params Copy for #ident #type_params #where_clause {}

        impl #type_params Clone for #ident #type_params #where_clause {
            fn clone(&self) -> Self {
                *self
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn chacha_obj(metadata: proc_macro::TokenStream, input: TokenStream) -> TokenStream {
    let _meta = parse_macro_input!(metadata as ChachaDataMeta);

    // Parse the string representation
    let ast = parse_macro_input!(input as DeriveInput);

    let attrs = quote! {
        Debug, Hash, Eq, PartialEq, Ord, PartialOrd
    };

    let expanded = quote! {
        #[derive(#attrs)]
        #ast
    };

    proc_macro::TokenStream::from(expanded)
}

#[derive(Parse)]
struct UnitTest {
    desc: syn::LitStr,
    _comma: Token![,],
    _or1: Token![|],
    _or2: Token![|],
    block: syn::Block,
}

#[proc_macro]
pub fn unit_test(input: TokenStream) -> TokenStream {
    let UnitTest { desc, block, .. } = parse_macro_input!(input as UnitTest);

    let id_span = desc.span();
    let desc = desc.value();
    let mut chars = desc.chars();
    let mut id = String::new();

    if let Some(char) = chars.next() {
        if let Some(char) = normalize_char(char, true) {
            id.push(char);
        }
    }

    for char in chars {
        if let Some(char) = normalize_char(char, false) {
            id.push(char)
        }
    }

    let id = Ident::new(&id, id_span);
    let id = format_ident!("test_{}", id);

    let expanded = quote! {
        #[test]
        #[allow(non_snake_case)]
        fn #id() -> Result<(), Box<std::error::Error>>
        {
            #block;
            Ok(())
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn normalize_char(c: char, is_start: bool) -> Option<char> {
    match c {
        char if is_start && UnicodeXID::is_xid_start(char) => Some(char),
        char if !is_start && UnicodeXID::is_xid_continue(char) => Some(char),
        char if char.is_whitespace() || char == '#' => Some('_'),
        _ => None,
    }
}
