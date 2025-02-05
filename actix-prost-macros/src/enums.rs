enum Kind {
    Enum,
    OneOf,
}

pub fn process_enum(item: &mut syn::ItemEnum, maybe_rename: Option<String>) {
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
        _ => return,
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
        None => return,
    };
    match kind {
        Kind::OneOf => {
            if let Some(rename) = maybe_rename {
                item.attrs
                    .push(syn::parse_quote!(#[serde(rename_all=#rename)]));
            }
        }
        Kind::Enum => {
            item.attrs
                .push(syn::parse_quote!(#[serde(rename_all="SCREAMING_SNAKE_CASE")]));
        }
    }
}
