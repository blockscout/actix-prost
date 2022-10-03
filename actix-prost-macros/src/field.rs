use crate::prost::parse_attrs;
use proc_macro2::TokenStream;

pub struct Field {
    attr: Option<syn::Attribute>,
    from: Option<TokenStream>,
}

impl Field {
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

            let enum_ident = quote::format_ident!("{}", enum_name.value());
            self.from = Some(quote::quote!(
                impl TryFrom<i32> for #enum_ident {
                    type Error = String;
                    fn try_from(value: i32) -> Result<Self, Self::Error> {
                        Self::from_i32(value).ok_or("enum value out of range".into())
                    }
                }
            ));
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
        let mut field = Self {
            attr: None,
            from: None,
        };
        field.generate(f);
        field
    }

    pub fn take_attribute(&mut self) -> Option<syn::Attribute> {
        self.attr.take()
    }

    pub fn take_from_impl(&mut self) -> Option<TokenStream> {
        self.from.take()
    }
}
