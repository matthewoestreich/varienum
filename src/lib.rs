extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Meta, parse_macro_input};

#[proc_macro_derive(VariantsVec, attributes(description))]
pub fn variants_vec(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let enum_name = input.ident;

    let data = match input.data {
        syn::Data::Enum(e) => e,
        _ => panic!("VariantsVec can only be used on enums"),
    };

    let mut variants = Vec::new();
    let mut variants_desc = Vec::new();

    for variant in &data.variants {
        let ident = &variant.ident;

        // default empty description
        let mut desc = String::new();

        for attr in &variant.attrs {
            if attr.path().is_ident("description")
                && let Meta::NameValue(nv) = &attr.meta
                && let syn::Expr::Lit(expr_lit) = &nv.value
                && let syn::Lit::Str(lit_str) = &expr_lit.lit
            {
                desc = lit_str.value();
            }
        }

        variants.push(ident.to_string());
        variants_desc.push(quote! {
            (stringify!(#ident), #desc)
        });
    }

    let expanded = quote! {
        impl #enum_name {
            pub fn variants() -> &'static [&'static str] {
                &[#(#variants),*]
            }

            pub fn variants_desc() -> &'static [(&'static str, &'static str)] {
                &[#(#variants_desc),*]
            }
        }
    };

    TokenStream::from(expanded)
}

