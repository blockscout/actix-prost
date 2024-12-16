use enums::process_enum;
use field::process_field;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, AttributeArgs, Item};

mod enums;
mod field;
mod prost;

fn find_rename_all(attrs: &[syn::NestedMeta]) -> Option<String> {
    for attr in attrs {
        match attr {
            syn::NestedMeta::Meta(syn::Meta::NameValue(meta))
                if meta.path == syn::parse_quote!(rename_all) =>
            {
                if let syn::Lit::Str(s) = &meta.lit {
                    return Some(s.value());
                }
            }
            _ => {}
        }
    }
    None
}

/// Checks if any other `actix_prost_macros::serde` attribute exist
/// for the item defined after the current attribute.
fn has_other_actix_prost_serde_attributes(item_attributes: &[Attribute]) -> bool {
    for attribute in item_attributes {
        if attribute.path == syn::parse_quote!(actix_prost_macros::serde) {
            return true;
        }
    }

    false
}

#[proc_macro_attribute]
pub fn serde(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = syn::parse_macro_input!(item as Item);

    let attrs = syn::parse_macro_input!(attrs as AttributeArgs);
    let maybe_rename = match find_rename_all(&attrs) {
        // 'none' option makes it possible to use rust default case
        // by default (which is snake_case for structs and PascalCase for enums),
        // and overwrite that value for some of the messages. For us, it is a way
        // to make most of the messages using snake_case, while small part of them
        // using camelCase.
        Some(rename) if rename.to_lowercase() == "none" => None,
        Some(rename) => Some(rename),
        None => Some("camelCase".to_owned()),
    };

    let mut result = quote::quote!();
    // Checking that no other `actix_prost_macros::serde` attributes exist for an item
    // allows us to override 'rename_all' attribute for the type.
    // That allows to specify a default serialization case convention and override it
    // for some of the types only (e.g., for legacy reasons).
    match &mut item {
        syn::Item::Enum(item) if !has_other_actix_prost_serde_attributes(&item.attrs) => {
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
            let enums = process_enum(item, maybe_rename.clone());
            result = quote::quote!(#result #enums);
        }
        syn::Item::Struct(item) if !has_other_actix_prost_serde_attributes(&item.attrs) => {
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
            if let Some(rename) = maybe_rename {
                item.attrs
                    .push(syn::parse_quote!(#[serde(rename_all = #rename)]));
            }
        }
        _ => {}
    }
    quote!(#item #result).into()
}
