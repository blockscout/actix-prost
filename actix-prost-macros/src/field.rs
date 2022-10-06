use crate::prost::parse_attrs;

pub struct Field {
    attr: Option<syn::Attribute>,
}

impl Field {
    #[allow(clippy::if_same_then_else)]
    fn parse_path(&mut self, name: &syn::Path) {
        if name == &syn::parse_quote!(i64) {
            self.attr = Some(syn::parse_quote!(#[serde_as(as = "serde_with::DisplayFromStr")]))
        } else if name == &syn::parse_quote!(u64) {
            self.attr = Some(syn::parse_quote!(#[serde_as(as = "serde_with::DisplayFromStr")]))
        } else if name == &syn::parse_quote!(::prost::bytes::Bytes) {
            self.attr = Some(syn::parse_quote!(#[serde_as(as = "serde_with::base64::Base64")]))
        }
    }

    fn parse_meta(&mut self, meta: &syn::Meta) {
        let meta = match meta {
            syn::Meta::NameValue(value) => value,
            _ => return,
        };
        if meta.path == syn::parse_quote!(enumeration) {
            let enum_name = match &meta.lit {
                syn::Lit::Str(name) => name,
                _ => return,
            };
            let as_value = format!("serde_with::TryFromInto<{}>", enum_name.value());
            self.attr = Some(syn::parse_quote!(#[serde_as(as = #as_value)]));
        } else if meta.path == syn::parse_quote!(oneof) {
            self.attr = Some(syn::parse_quote!(#[serde(flatten)]));
        }
    }

    fn generate(&mut self, f: &syn::Field) {
        let metas = parse_attrs(f.attrs.clone());
        for m in metas {
            self.parse_meta(&m);
        }

        if let syn::Type::Path(ty) = &f.ty {
            self.parse_path(&ty.path);
        }
    }

    pub fn new(f: &syn::Field) -> Self {
        let mut field = Self { attr: None };
        field.generate(f);
        field
    }

    pub fn take_attribute(&mut self) -> Option<syn::Attribute> {
        self.attr.take()
    }
}
