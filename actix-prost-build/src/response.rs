use proc_macro2::{Ident, TokenStream};

pub struct Response {
    message: syn::ItemStruct,
    method_name: Ident,
    field: Option<String>,
}

impl Response {
    pub fn new(message: syn::ItemStruct, method_name: Ident, field: Option<String>) -> Response {
        Response {
            message,
            method_name,
            field,
        }
    }

    pub fn is_custom(&self) -> bool {
        self.field.is_some()
    }

    pub fn name(&self) -> Ident {
        if self.is_custom() {
            quote::format_ident!("{}Response", self.method_name)
        } else {
            self.message.ident.clone()
        }
    }

    pub fn filter_field(&self, field: &str) -> syn::Field {
        self.message
            .fields
            .iter()
            .filter(|full_field| &full_field.ident.as_ref().unwrap().to_string() == field)
            .next()
            .cloned()
            .unwrap_or_else(|| {
                panic!(
                    "could not find response_body field {} in response message",
                    field
                )
            })
    }

    pub fn generate_struct(&self) -> Option<TokenStream> {
        self.field.as_ref().map(|field| {
            let name = self.name();
            let full_field = self.filter_field(field);
            let field_type = full_field.ty;
            quote::quote!(
                type #name = #field_type;
            )
        })
    }

    pub fn generate_convert(&self) -> Option<TokenStream> {
        self.field.as_ref().map(|field| {
            let field = quote::format_ident!("{}", field);
            quote::quote!(
                let response = response.#field;
            )
        })
    }
}
