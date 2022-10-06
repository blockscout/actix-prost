use proc_macro2::TokenStream;

pub struct Response {
    message: syn::ItemStruct,
    field: Option<String>,
}

impl Response {
    pub fn new(message: syn::ItemStruct, field: Option<String>) -> Response {
        Response { message, field }
    }

    pub fn filter_field(&self, field: &str) -> syn::Field {
        self.message
            .fields
            .iter()
            .find(|full_field| *full_field.ident.as_ref().unwrap() == field)
            .cloned()
            .unwrap_or_else(|| {
                panic!(
                    "could not find response_body field {} in response message",
                    field
                )
            })
    }

    pub fn name(&self) -> TokenStream {
        match &self.field {
            Some(field) => {
                let full_field = self.filter_field(field);
                let field_type = full_field.ty;
                quote::quote!(#field_type)
            }
            None => {
                let name = self.message.ident.clone();
                quote::quote!(#name)
            }
        }
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
