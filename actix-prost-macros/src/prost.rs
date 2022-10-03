// Copypasted from prost-derive, original name `prost_attrs`
/// Get the items belonging to the 'prost' list attribute, e.g. `#[prost(foo, bar="baz")]`.
pub fn parse_attrs(attrs: Vec<syn::Attribute>) -> Vec<syn::Meta> {
    attrs
        .iter()
        .flat_map(syn::Attribute::parse_meta)
        .flat_map(|meta| match meta {
            syn::Meta::List(syn::MetaList { path, nested, .. }) => {
                if path.is_ident("prost") {
                    nested.into_iter().collect()
                } else {
                    Vec::new()
                }
            }
            _ => Vec::new(),
        })
        .flat_map(|attr| -> Option<_> {
            match attr {
                syn::NestedMeta::Meta(attr) => Some(attr),
                syn::NestedMeta::Lit(_) => None,
            }
        })
        .collect()
}
