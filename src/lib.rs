extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Meta, parse_macro_input};

#[proc_macro_attribute]
pub fn variants_vec(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let enum_name = input.ident;

    let data = match input.data {
        syn::Data::Enum(e) => e,
        _ => panic!("variants_vec can only be used on enums"),
    };

    let mut variants = Vec::new();
    let mut variants_desc = Vec::new();

    for variant in &data.variants {
        let name = variant.ident.to_string();
        let mut desc = String::new();

        // extract #[description("...")]
        for attr in &variant.attrs {
            if attr.path().is_ident("description")
                && let Meta::NameValue(nv) = &attr.meta
                && let syn::Expr::Lit(expr_lit) = &nv.value
                && let syn::Lit::Str(lit_str) = &expr_lit.lit
            {
                desc = lit_str.value();
            }
        }

        variants.push(name.clone());
        variants_desc.push(quote! {
            (stringify!(#variant), #desc)
        });
    }

    let expanded = quote! {
        impl #enum_name {
            pub fn variants() -> &'static [&'static str] {
                &[#(#variants),*]
            }
            pub fn variants_desc() -> Vec<(&'static str, &'static str)> {
                vec![#(#variants_desc),*]
            }
        }
    };

    TokenStream::from(expanded)
}

