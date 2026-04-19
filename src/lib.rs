extern crate proc_macro;
use proc_macro::{Delimiter, TokenStream, TokenTree};

#[proc_macro_attribute]
pub fn variants_vec(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut tokens = item.clone().into_iter();
    let mut enum_name = String::new();
    let mut variants = Vec::new();
    let mut variants_with_desc = Vec::new();
    let mut current_desc = String::new();

    // Find the enum name and the braced body
    while let Some(ref tree) = tokens.next() {
        // Find enum name.
        if let TokenTree::Ident(ident) = tree
            && ident.to_string() == "enum"
            && let Some(TokenTree::Ident(name)) = tokens.next()
        {
            enum_name = name.to_string();
        }

        // Find varaints in enum.
        if let TokenTree::Group(group) = tree
            && group.delimiter() == Delimiter::Brace
        {
            let mut group_tokens = group.stream().into_iter();

            while let Some(group_tree) = group_tokens.next() {
                match group_tree {
                    // Detect #[Description = "..."]
                    TokenTree::Punct(p) if p.as_char() == '#' => {
                        if let Some(TokenTree::Group(attr_group)) = group_tokens.next() {
                            let mut attr_tokens = attr_group.stream().into_iter();
                            while let Some(t) = attr_tokens.next() {
                                if let TokenTree::Ident(id) = &t
                                    && id.to_string() == "Description"
                                {
                                    attr_tokens.next(); // skip '='
                                    if let Some(TokenTree::Literal(lit)) = attr_tokens.next() {
                                        current_desc =
                                            lit.to_string().trim_matches('"').to_string();
                                    }
                                }
                            }
                        }
                    }
                    TokenTree::Ident(v_name) => {
                        let name = v_name.to_string();
                        // existing behavior
                        variants.push(name.clone());
                        // NEW behavior (add tuple output)
                        variants_with_desc.push((name.clone(), current_desc.clone()));
                        // reset description after use
                        current_desc.clear();
                        // skip until comma
                        for t in group_tokens.by_ref() {
                            if let TokenTree::Punct(p) = t
                                && p.as_char() == ','
                            {
                                break;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    let variant_names = variants
        .iter()
        .map(|v| format!("\"{}\"", v))
        .collect::<Vec<_>>()
        .join(",");

    let variants_desc = variants_with_desc
        .iter()
        .map(|(v, d)| format!("(\"{}\", \"{}\")", v, d))
        .collect::<Vec<_>>()
        .join(",");

    let mut generated = item.to_string();
    generated.push_str(&format!(
        "impl {} {{ 
            pub fn variants() -> &'static [&'static str] {{ 
                &[{}] 
            }} 
            pub fn variants_desc() -> &'static [(&'static str, &'static str)] {{
                &[{}]
            }}
        }}",
        enum_name, variant_names, variants_desc
    ));

    generated.parse().expect("Failed to parse generated code")
}
