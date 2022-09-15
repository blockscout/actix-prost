use crate::config::HttpRule;
use proc_macro2::{Ident, TokenStream};
use std::{collections::HashSet, iter::FromIterator};

pub struct RequestFields {
    name: String,
    fields: Vec<String>,
}

pub struct Request {
    message: syn::ItemStruct,
    method_name: Ident,
    path: RequestFields,
    query: RequestFields,
    body: RequestFields,
}

impl Request {
    pub fn new(message: syn::ItemStruct, method_name: Ident, config: &HttpRule) -> Request {
        let fields: Vec<String> = config
            .pattern
            .path()
            .split('{')
            .skip(1)
            .filter_map(|q| q.split('}').next())
            .map(|x| x.to_owned())
            .collect();

        let (path, query, body) = Self::split_fields(&message, &fields, &config.body);

        Request {
            message,
            method_name,
            path: RequestFields {
                name: "Path".into(),
                fields: path,
            },
            query: RequestFields {
                name: "Query".into(),
                fields: query,
            },
            body: RequestFields {
                name: "Json".into(),
                fields: body,
            },
        }
    }

    fn split_fields(
        message: &syn::ItemStruct,
        path_fields: &[String],
        body_fields: &Option<String>,
    ) -> (Vec<String>, Vec<String>, Vec<String>) {
        let fields = if let syn::Fields::Named(fields) = &message.fields {
            fields
        } else {
            panic!("non named fields aren't supported");
        };

        let path_filter: HashSet<&str> = HashSet::from_iter(path_fields.iter().map(|s| s.as_ref()));
        let (path, non_path): (Vec<_>, Vec<_>) = fields
            .named
            .iter()
            .map(|field| field.ident.as_ref().unwrap().to_string())
            .partition(|field| path_filter.contains(field.as_str()));

        if path_fields.len() != path.len() {
            let found: HashSet<String> = HashSet::from_iter(path.into_iter().map(|x| x));
            panic!(
                "some path fields were not found: {:?}",
                path_fields
                    .iter()
                    .filter(|f| !found.contains(f.as_str()))
                    .collect::<Vec<_>>()
            )
        }

        let (body, query) = if let Some(body_fields) = body_fields {
            if body_fields != "*" {
                non_path.into_iter().partition(|f| f == body_fields)
            } else {
                (non_path, Vec::default())
            }
        } else {
            (Vec::default(), non_path)
        };

        (path, query, body)
    }

    pub fn filter_fields(&self, req: &RequestFields) -> syn::Fields {
        let filter: HashSet<&str> = HashSet::from_iter(req.fields.iter().map(|x| x.as_ref()));
        let fields = self
            .message
            .fields
            .iter()
            .filter(|field| filter.contains(field.ident.as_ref().unwrap().to_string().as_str()))
            .cloned()
            .collect();
        let brace_token = if let syn::Fields::Named(named) = &self.message.fields {
            named.brace_token.clone()
        } else {
            panic!("not named fields not supported");
        };
        syn::Fields::Named(syn::FieldsNamed {
            brace_token,
            named: fields,
        })
    }

    pub fn sub_name(&self, req: &RequestFields) -> Option<Ident> {
        if !req.fields.is_empty() {
            Some(quote::format_ident!("{}{}", self.method_name, req.name))
        } else {
            None
        }
    }

    fn generate_struct(&self, req: &RequestFields) -> Option<TokenStream> {
        // an optimization: do not generate new struct if all the fields are the same as in message
        if req.fields.len() == self.message.fields.len() {
            self.sub_name(req).map(|name| {
                let message_name = &self.message.ident;
                quote::quote!(
                    type #name = #message_name;
                )
            })
        } else {
            self.sub_name(req).map(|name| {
                let mut generated = self.message.clone();
                generated.ident = name;
                generated.fields = self.filter_fields(req);
                quote::quote!(#generated)
            })
        }
    }

    pub fn generate_structs(&self) -> TokenStream {
        let path = self.generate_struct(&self.path);
        let query = self.generate_struct(&self.query);
        let body = self.generate_struct(&self.body);
        quote::quote!(#path #query #body)
    }

    pub fn generate_fields_init<'a>(
        req: &'a RequestFields,
    ) -> impl Iterator<Item = TokenStream> + 'a {
        req.fields
            .iter()
            .map(|f| quote::format_ident!("{}", f))
            .map(|f| {
                let field_name = quote::format_ident!("{}", req.name.to_lowercase());
                quote::quote!(
                    #f: #field_name.#f,
                )
            })
    }

    pub fn generate_new_request(&self) -> TokenStream {
        let name = &self.message.ident;
        let path_fields = Self::generate_fields_init(&self.path);
        let query_fields = Self::generate_fields_init(&self.query);
        let body_fields = Self::generate_fields_init(&self.body);
        quote::quote!(
            #name {
                #(#path_fields)*
                #(#query_fields)*
                #(#body_fields)*
            }
        )
    }

    fn generate_fn_arg(&self, req: &RequestFields) -> Option<TokenStream> {
        let field_name = quote::format_ident!("{}", req.name.to_lowercase());
        let extractor = quote::format_ident!("{}", req.name);
        self.sub_name(req)
            .map(|name| quote::quote!(#field_name: ::actix_web::web::#extractor<#name>,))
    }

    pub fn generate_fn_args(&self) -> TokenStream {
        let path = self.generate_fn_arg(&self.path);
        let query = self.generate_fn_arg(&self.query);
        let body = self.generate_fn_arg(&self.body);
        quote::quote!(#path #query #body)
    }

    fn generate_into_inner(&self, req: &RequestFields) -> Option<TokenStream> {
        let field_name = quote::format_ident!("{}", req.name.to_lowercase());
        self.sub_name(req)
            .map(|_| quote::quote!(let #field_name = #field_name.into_inner();))
    }

    pub fn generate_into_inners(&self) -> TokenStream {
        let path = self.generate_into_inner(&self.path);
        let query = self.generate_into_inner(&self.query);
        let body = self.generate_into_inner(&self.body);
        quote::quote!(#path #query #body)
    }
}
