use proc_macro2::TokenStream;

enum Kind {
    Enum,
    OneOf,
}

pub fn process_enum(item: &mut syn::ItemEnum) -> Option<TokenStream> {
    let derive = item
        .attrs
        .iter()
        .filter_map(|attr| attr.parse_meta().ok())
        .find(|meta| {
            if let Some(ident) = meta.path().get_ident() {
                *ident == "derive"
            } else {
                false
            }
        });
    let derive = match derive {
        Some(syn::Meta::List(derive)) => derive,
        _ => return None,
    };
    let kind = derive
        .nested
        .into_iter()
        .filter_map(|meta| match meta {
            syn::NestedMeta::Meta(syn::Meta::Path(path)) => {
                if path == syn::parse_quote!(::prost::Enumeration) {
                    Some(Kind::Enum)
                } else if path == syn::parse_quote!(::prost::Oneof) {
                    Some(Kind::OneOf)
                } else {
                    None
                }
            }
            _ => None,
        })
        .next();
    let kind = match kind {
        Some(kind) => kind,
        None => return None,
    };
    match kind {
        Kind::OneOf => {
            item.attrs
                .push(syn::parse_quote!(#[serde(rename_all="camelCase")]));
            None
        }
        Kind::Enum => {
            item.attrs
                .push(syn::parse_quote!(#[serde(rename_all="SCREAMING_SNAKE_CASE")]));

            let name = &item.ident;
            Some(quote::quote!(
                impl TryFrom<i32> for #name {
                    type Error = String;
                    fn try_from(value: i32) -> Result<Self, Self::Error> {
                        Self::from_i32(value).ok_or("enum value out of range".into())
                    }
                }
            ))
        }
    }
}
