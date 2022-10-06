use enums::process_enum;
use field::process_field;
use proc_macro::TokenStream;
use quote::quote;

mod enums;
mod field;
mod prost;

#[proc_macro_attribute]
pub fn serde(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut item: syn::Item = syn::parse(item).unwrap();
    let mut result = quote::quote!();
    match &mut item {
        syn::Item::Enum(item) => {
            let mut need_serde_as = false;
            for variant in item.variants.iter_mut() {
                for field in variant.fields.iter_mut() {
                    let (attr, need) = process_field(field);
                    need_serde_as = need || need_serde_as;
                    if let Some(attr) = attr {
                        field.attrs.push(attr);
                    }
                }
            }
            if need_serde_as {
                item.attrs.push(syn::parse_quote!(#[serde_with::serde_as]));
            }
            item.attrs
                .push(syn::parse_quote!(#[derive(serde::Serialize, serde::Deserialize)]));
            let enums = process_enum(item);
            result = quote::quote!(#result #enums);
        }
        syn::Item::Struct(item) => {
            let mut need_serde_as = false;
            for field in item.fields.iter_mut() {
                let (attr, need) = process_field(field);
                need_serde_as = need || need_serde_as;
                if let Some(attr) = attr {
                    field.attrs.push(attr);
                }
            }
            if need_serde_as {
                item.attrs.push(syn::parse_quote!(#[serde_with::serde_as]));
            }
            item.attrs
                .push(syn::parse_quote!(#[derive(serde::Serialize, serde::Deserialize)]));
        }
        _ => {}
    }
    //dbg!(item.to_token_stream().to_string());
    quote!(#item #result).into()
}
