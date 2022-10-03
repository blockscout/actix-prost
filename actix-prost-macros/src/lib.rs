use enums::process_enum;
use field::Field;
use proc_macro::TokenStream;
use quote::quote;
use syn::__private::ToTokens;

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
                    let mut generated = Field::new(field);
                    if let Some(attr) = generated.take_attribute() {
                        field.attrs.push(attr);
                        need_serde_as = true;
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
                let mut generated = Field::new(field);
                if let Some(attr) = generated.take_attribute() {
                    field.attrs.push(attr);
                    need_serde_as = true;
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
