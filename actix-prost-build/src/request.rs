use crate::config::HttpRule;
use proc_macro2::{Ident, TokenStream};
use std::{collections::HashSet, iter::FromIterator};

pub struct Request {
    message: syn::ItemStruct,
    method_name: Ident,
    path: Vec<String>,
    body: Vec<String>,
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

        let (path, body) = Self::split_fields(&message, &fields, &config.body);

        Request {
            message,
            method_name,
            path,
            body,
        }
    }

    fn split_fields(
        message: &syn::ItemStruct,
        path_fields: &[String],
        body_fields: &Option<String>,
    ) -> (Vec<String>, Vec<String>) {
        let fields = if let syn::Fields::Named(fields) = &message.fields {
            fields
        } else {
            panic!("non named fields aren't supported");
        };

        let path_filter: HashSet<&str> = HashSet::from_iter(path_fields.iter().map(|s| s.as_ref()));
        let (path, mut body): (Vec<_>, Vec<_>) = fields
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

        if let Some(body_fields) = body_fields {
            if body_fields != "*" {
                body = body.into_iter().filter(|f| f == body_fields).collect()
            }
        }

        (path, body)
    }

    pub fn filter_fields(&self, filter: &[String]) -> syn::Fields {
        let filter: HashSet<&str> = HashSet::from_iter(filter.iter().map(|x| x.as_ref()));
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

    pub fn path_name(&self) -> Option<Ident> {
        if !self.path.is_empty() {
            Some(quote::format_ident!("{}Path", self.method_name))
        } else {
            None
        }
    }

    pub fn generate_path(&self) -> Option<syn::ItemStruct> {
        self.path_name().map(|name| {
            let mut path = self.message.clone();
            path.ident = name;
            path.fields = self.filter_fields(&self.path);
            path
        })
    }

    pub fn body_name(&self) -> Option<Ident> {
        if !self.body.is_empty() {
            Some(quote::format_ident!("{}Body", self.method_name))
        } else {
            None
        }
    }

    pub fn generate_body(&self) -> Option<syn::ItemStruct> {
        self.body_name().map(|name| {
            let mut body = self.message.clone();
            body.ident = name;
            body.fields = self.filter_fields(&self.body);
            body
        })
    }

    pub fn generate_new_request(&self) -> TokenStream {
        let name = &self.message.ident;
        let path_fields = self
            .path
            .iter()
            .map(|f| quote::format_ident!("{}", f))
            .map(|f| {
                quote::quote!(
                    #f: path.#f,
                )
            });
        let body_fields = self
            .body
            .iter()
            .map(|f| quote::format_ident!("{}", f))
            .map(|f| {
                quote::quote!(
                    #f: body.#f,
                )
            });
        quote::quote!(
            #name {
                #(#path_fields)*
                #(#body_fields)*
            }
        )
    }

    pub fn generate_fn_arg(&self) -> TokenStream {
        let path = self
            .path_name()
            .map(|name| quote::quote!(path: Path<#name>,));
        let body = self
            .body_name()
            .map(|name| quote::quote!(body: Json<#name>,));
        quote::quote!(#path #body)
    }

    pub fn generate_into_inner(&self) -> TokenStream {
        let path = self
            .path_name()
            .map(|_| quote::quote!(let path = path.into_inner();));
        let body = self
            .body_name()
            .map(|_| quote::quote!(let body = body.into_inner();));
        quote::quote!(#path #body)
    }
}
