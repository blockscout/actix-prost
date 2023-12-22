use crate::config::HttpRule;
use proc_macro2::{Ident, TokenStream};
use std::{collections::HashSet, iter::FromIterator};
use syn::PathArguments;

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
            let found: HashSet<String> = HashSet::from_iter(path.into_iter());
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

        if path.len() + query.len() + body.len() != message.fields.len() {
            panic!("could not map all message fields to path, query and body parts")
        }

        (path, query, body)
    }

    pub fn filter_fields(&self, req: &RequestFields) -> syn::Fields {
        // Is called from `generate_struct` method. The method generates structs for the actix module
        // and those structs will be located inside `mod *_actix`. The problem with `super::` paths
        // occurs because proto structures are located in the main module. Thus, we need to add a one more
        // `super::` path segment for those paths to make them out of `mod *_actix`.
        fn update_type_super_path(ty: &mut syn::Type) {
            if let syn::Type::Path(type_path) = ty {
                let mut super_segment_data = None;
                for (i, segment) in type_path.path.segments.iter_mut().enumerate() {
                    if segment.ident.to_string().as_str() == "super" {
                        // We need to add only one additional `super` segment,
                        // thus we are looking only the first inclusion.
                        super_segment_data = Some((i, segment.clone()));
                        break;
                    }
                    // Update segment paths in the arguments, if there are any
                    match &mut segment.arguments {
                        PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                            args,
                            ..
                        }) => args.iter_mut().for_each(|arg| {
                            if let syn::GenericArgument::Type(ty) = arg {
                                update_type_super_path(ty)
                            }
                        }),
                        PathArguments::Parenthesized(syn::ParenthesizedGenericArguments {
                            inputs,
                            ..
                        }) => inputs.iter_mut().for_each(update_type_super_path),
                        PathArguments::None => {}
                    }
                }

                // Make the actual update. We cannot do that inside `for` cycle,
                // because `type_path.path.segments` are mutually borrowed to update arguments.
                if let Some((index, segment)) = super_segment_data {
                    type_path.path.segments.insert(index, segment)
                }
            }
        }

        let filter: HashSet<&str> = HashSet::from_iter(req.fields.iter().map(|x| x.as_ref()));
        let fields = self
            .message
            .fields
            .iter()
            .filter(|&field| filter.contains(field.ident.as_ref().unwrap().to_string().as_str()))
            .cloned()
            .map(|mut field| {
                update_type_super_path(&mut field.ty);
                field
            })
            .collect();
        let brace_token = if let syn::Fields::Named(named) = &self.message.fields {
            named.brace_token
        } else {
            panic!("not named fields not supported");
        };
        syn::Fields::Named(syn::FieldsNamed {
            brace_token,
            named: fields,
        })
    }

    pub fn path(&self) -> &RequestFields {
        &self.path
    }

    pub fn body(&self) -> &RequestFields {
        &self.body
    }

    pub fn query(&self) -> &RequestFields {
        &self.query
    }

    pub fn has_sub(&self, req: &RequestFields) -> bool {
        !req.fields.is_empty()
    }

    pub fn sub_name(&self, req: &RequestFields) -> Option<Ident> {
        if self.has_sub(req) {
            Some(quote::format_ident!("{}{}", self.method_name, req.name))
        } else {
            None
        }
    }

    fn generate_struct(
        &self,
        req: &RequestFields,
        attrs: Option<TokenStream>,
    ) -> Option<TokenStream> {
        self.sub_name(req).map(|name| {
            let mut generated = self.message.clone();
            generated.ident = name;
            if let Some(attrs) = attrs {
                generated
                    .attrs
                    .retain(|attr| attr.path != syn::parse_quote!(actix_prost_macros::serde));
                generated.attrs.push(syn::parse_quote!(#[actix_prost_macros::serde(#attrs)]));
            }
            generated.fields = self.filter_fields(req);
            quote::quote!(#generated)
        })
    }

    pub fn generate_structs(&self) -> TokenStream {
        let path = self.generate_struct(&self.path, Some(quote::quote!(rename_all = "snake_case")));
        let query = self.generate_struct(&self.query, None);
        let body = self.generate_struct(&self.body, None);
        quote::quote!(#path #query #body)
    }

    pub fn generate_fields_init(req: &RequestFields) -> impl Iterator<Item = TokenStream> + '_ {
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

    fn generate_extract(&self, req: &RequestFields) -> Option<TokenStream> {
        let field_name = quote::format_ident!("{}", req.name.to_lowercase());
        let extractor = quote::format_ident!("{}", req.name);
        self.sub_name(req)
            .map(|name| quote::quote!(
                let #field_name = <::actix_web::web::#extractor::<#name> as ::actix_web::FromRequest>::extract(&http_request)
                    .await
                    .map_err(|err| ::actix_prost::Error::from_actix(err, ::tonic::Code::InvalidArgument))?
                    .into_inner();
            ))
    }

    fn generate_from_request(&self, req: &RequestFields) -> Option<TokenStream> {
        let field_name = quote::format_ident!("{}", req.name.to_lowercase());
        let extractor = quote::format_ident!("{}", req.name);
        self.sub_name(req)
            .map(|name| quote::quote!(
                let #field_name = <::actix_web::web::#extractor::<#name> as ::actix_web::FromRequest>::from_request(&http_request, &mut payload)
                    .await
                    .map_err(|err| ::actix_prost::Error::from_actix(err, ::tonic::Code::InvalidArgument))?
                    .into_inner();
            ))
    }

    pub fn generate_extractors(&self) -> TokenStream {
        let path = self.generate_extract(&self.path);
        let query = self.generate_extract(&self.query);
        let body = self.generate_from_request(&self.body);
        quote::quote!(#path #query #body)
    }
}
