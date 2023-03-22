use crate::prost::parse_attrs;

pub fn process_field(f: &syn::Field) -> (Option<syn::Attribute>, bool) {
    let metas = parse_attrs(f.attrs.clone());

    if let syn::Type::Path(ty) = &f.ty {
        for m in metas {
            if let (Some(attr), need_serde_as) = parse_meta(&ty.path, &m) {
                return (Some(attr), need_serde_as);
            }
        }

        if let (Some(attr), need_serde_as) = parse_path(&ty.path) {
            return (Some(attr), need_serde_as);
        }
    }

    (None, false)
}

fn parse_path(name: &syn::Path) -> (Option<syn::Attribute>, bool) {
    if name == &syn::parse_quote!(i64) || name == &syn::parse_quote!(u64) {
        (
            Some(syn::parse_quote!(#[serde_as(as = "serde_with::DisplayFromStr")])),
            true,
        )
    } else if name == &syn::parse_quote!(::core::option::Option<i64>) {
        (
            Some(syn::parse_quote!(#[serde(default,with="actix_prost::serde::option_i64")])),
            false,
        )
    } else if name == &syn::parse_quote!(::core::option::Option<u64>) {
        (
            Some(syn::parse_quote!(#[serde(default,with="actix_prost::serde::option_u64")])),
            false,
        )
    } else if name == &syn::parse_quote!(::prost::bytes::Bytes) {
        (
            Some(syn::parse_quote!(#[serde_as(as = "serde_with::base64::Base64")])),
            true,
        )
    } else if name == &syn::parse_quote!(::core::option::Option<::prost::bytes::Bytes>) {
        (
            Some(syn::parse_quote!(#[serde(default,with="actix_prost::serde::option_bytes")])),
            false,
        )
    } else {
        (None, false)
    }
}

fn parse_meta(name: &syn::Path, meta: &syn::Meta) -> (Option<syn::Attribute>, bool) {
    let meta = match meta {
        syn::Meta::NameValue(value) => value,
        _ => return (None, false),
    };
    if meta.path == syn::parse_quote!(enumeration) {
        let enum_name = match &meta.lit {
            syn::Lit::Str(name) => name,
            _ => return (None, false),
        };
        let option: syn::Path = syn::parse_quote!(::core::option::Option);
        if name.segments.len() == option.segments.len()
            && name
                .segments
                .iter()
                .zip(option.segments.iter())
                .all(|(a, b)| a.ident == b.ident)
        {
            (
                Some(syn::parse_quote!(#[serde(default,with="actix_prost::serde::option_enum")])),
                false,
            )
        } else {
            let as_value = format!("serde_with::TryFromInto<{}>", enum_name.value());
            (Some(syn::parse_quote!(#[serde_as(as = #as_value)])), true)
        }
    } else if meta.path == syn::parse_quote!(oneof) {
        (Some(syn::parse_quote!(#[serde(flatten)])), false)
    } else {
        (None, false)
    }
}
