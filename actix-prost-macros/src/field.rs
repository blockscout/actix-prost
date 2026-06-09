use crate::prost::parse_attrs;

pub fn process_field(f: &syn::Field) -> (Option<syn::Attribute>, bool) {
    let metas = parse_attrs(f.attrs.clone());

    // Enum fields use a `TryFromInto` serde adapter so they (de)serialize via the
    // proto enum names instead of the underlying i32. Repeated and optional enum
    // fields are stored as `Vec<i32>` / `Option<i32>`, so the adapter has to be
    // wrapped to match that container; the scalar adapter alone does not apply.
    if let Some(enum_name) = metas.iter().find_map(enum_name) {
        let inner = format!("serde_with::TryFromInto<{enum_name}>");
        let as_value = match cardinality(&metas) {
            Cardinality::Repeated => format!("Vec<{inner}>"),
            Cardinality::Optional => format!("Option<{inner}>"),
            Cardinality::Scalar => inner,
        };
        return (Some(syn::parse_quote!(#[serde_as(as = #as_value)])), true);
    }

    for m in metas {
        if let Some(attr) = parse_meta(&m) {
            return (Some(attr), false);
        }
    }

    if let syn::Type::Path(ty) = &f.ty {
        if let (Some(attr), need_serde_as) = parse_path(&ty.path) {
            return (Some(attr), need_serde_as);
        }
    }

    (None, false)
}

enum Cardinality {
    Scalar,
    Optional,
    Repeated,
}

/// Returns the enum type name referenced by a prost `enumeration = "..."` meta.
fn enum_name(meta: &syn::Meta) -> Option<String> {
    match meta {
        syn::Meta::NameValue(nv) if nv.path == syn::parse_quote!(enumeration) => match &nv.lit {
            syn::Lit::Str(name) => Some(name.value()),
            _ => None,
        },
        _ => None,
    }
}

/// Derives a field's cardinality from prost's bare `repeated` / `optional` flags.
fn cardinality(metas: &[syn::Meta]) -> Cardinality {
    for m in metas {
        if let syn::Meta::Path(path) = m {
            if path.is_ident("repeated") {
                return Cardinality::Repeated;
            }
            if path.is_ident("optional") {
                return Cardinality::Optional;
            }
        }
    }
    Cardinality::Scalar
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

fn parse_meta(meta: &syn::Meta) -> Option<syn::Attribute> {
    let meta = match meta {
        syn::Meta::NameValue(value) => value,
        _ => return None,
    };
    if meta.path == syn::parse_quote!(oneof) {
        Some(syn::parse_quote!(#[serde(flatten)]))
    } else {
        None
    }
}
