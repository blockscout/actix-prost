use crate::{config::HttpRule, method::Method, Config};
use proc_macro2::TokenStream;
use prost_build::{Service, ServiceGenerator};
use prost_reflect::{Cardinality, DescriptorPool, DynamicMessage, MessageDescriptor};
use quote::quote;
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    path::{Path, PathBuf},
};
use syn::{Fields, Item, ItemStruct, Type};

pub struct ActixGenerator {
    messages: HashMap<String, syn::ItemStruct>,
    config: Config,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("could not open file {0}")]
    File(#[from] std::io::Error),
    #[error("could not parse the config {0}")]
    Parse(#[from] serde_yaml::Error),
}

#[derive(Default)]
struct ConvertFields {
    fields: Vec<(String, String)>,
    extra: Vec<(String, String)>,
}

impl ActixGenerator {
    pub fn new(path: impl AsRef<Path>) -> Result<ActixGenerator, Error> {
        let file = File::open(path)?;
        let config: Config = serde_yaml::from_reader(file)?;
        Ok(ActixGenerator {
            messages: Default::default(),
            config,
        })
    }

    fn map_methods_with_rules<'a, 'b>(
        &'a self,
        service: &'b Service,
    ) -> Vec<(&'b prost_build::Method, &'a HttpRule)> {
        let map: HashMap<String, &HttpRule> = self
            .config
            .http
            .rules
            .iter()
            .map(|r| (r.selector.clone(), r))
            .collect();
        service
            .methods
            .iter()
            .filter_map(|m| {
                map.get(&format!(
                    "{}.{}.{}",
                    service.package, service.proto_name, m.proto_name
                ))
                .map(|r| (m, *r))
            })
            .collect()
    }

    fn create_convert_struct(input: ItemStruct, convert_fields: ConvertFields) -> TokenStream {
        let struct_name = input.ident.clone();
        let new_struct_name = quote::format_ident!("{}Internal", struct_name);

        let fields = match input.fields {
            Fields::Named(named) => named.named,
            _ => unimplemented!(),
        };
        let convert = quote!(actix_prost::convert::Convert);
        let fields: Vec<_> = fields
            .iter()
            .map(|f| {
                let name = f.ident.clone().unwrap();
                let vis = &f.vis;

                let field_override = convert_fields
                    .fields
                    .iter()
                    .find(|(n, _)| n.eq(&name.to_string()));
                let (ty, conv) = match field_override {
                    Some((_, ty)) => {
                        let ty = syn::parse_str::<Type>(ty).unwrap();
                        (quote!(#ty), quote! { #convert::convert(from.#name)? })
                    }
                    None => {
                        let ty = &f.ty;
                        (quote!(#ty), quote! { from.#name })
                    }
                };
                (
                    quote! {
                        #vis #name: #ty
                    },
                    quote! {
                        #name: #conv
                    },
                )
            })
            .collect();

        let field_types = fields.iter().map(|(t, _)| t);
        let field_conversions = fields.iter().map(|(_, c)| c);

        let expanded = quote::quote!(
            #[derive(Debug)]
            pub struct #new_struct_name {
                #(#field_types,)*
            }

            impl #convert<#struct_name> for #new_struct_name {
                fn convert(from: #struct_name) -> anyhow::Result<Self> {
                    Ok(Self {
                        #(#field_conversions,)*
                    })
                }
            }
        );
        TokenStream::from(expanded)
    }

    fn extract_extra_fields(
        descriptors: &DescriptorPool,
        m: &MessageDescriptor,
    ) -> Vec<(String, String)> {
        let message_extension = descriptors
            .get_message_by_name("google.protobuf.MessageOptions")
            .unwrap()
            .extensions()
            .find(|ext| ext.name() == "extra_fields")
            .unwrap();

        let options = m.options();
        options
            .get_extension(&message_extension)
            .as_list()
            .unwrap()
            .iter()
            .map(|v| {
                let m = v.as_message().unwrap();
                let name = Self::get_string_field(m, "name");
                let ty = Self::get_string_field(m, "type");
                (name, ty)
            })
            .collect::<Vec<_>>()
    }

    fn get_string_field(m: &DynamicMessage, name: &str) -> String {
        m.get_field_by_name(name)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
    }

    fn create_conversions(&self, service: &Service) -> TokenStream {
        let path =
            PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR environment variable not set"))
                .join("file_descriptor_set.bin");
        let buf = fs::read(path).unwrap();
        let descriptors = DescriptorPool::decode(&*buf).unwrap();

        let fields_extension = descriptors
            .get_message_by_name("google.protobuf.FieldOptions")
            .unwrap()
            .extensions()
            .find(|ext| ext.name() == "convert")
            .unwrap();

        let methods = &service.methods;

        let mut res = Vec::new();
        for method in methods.iter() {
            let message_in = descriptors
                .get_message_by_name(&method.input_proto_type)
                .unwrap();

            // TODO
            let _extra_fields = Self::extract_extra_fields(&descriptors, &message_in);

            // TODO
            let _message_out = descriptors
                .get_message_by_name(&method.output_proto_type)
                .unwrap();

            let rust_struct = self.messages.get(&method.input_type).unwrap().clone();

            let mut convert_fields: ConvertFields = Default::default();
            convert_fields.fields = message_in
                .fields()
                .filter_map(|f| {
                    let options = f.options();
                    let ext_val = options.get_extension(&fields_extension);
                    let ext_val = ext_val.as_message().unwrap();

                    let ty = ext_val.get_field_by_name("type")?;
                    let ty = ty.as_str()?;

                    if !ty.is_empty() {
                        Some((String::from(f.name()), String::from(ty)))
                    } else {
                        None
                    }
                })
                .collect();

            res.push(Self::create_convert_struct(rust_struct, convert_fields));
        }
        quote!(
            #(#res)*
        )
    }

    fn router(&self, service: &Service) -> TokenStream {
        let service_name = crate::string::naive_snake_case(&service.name);

        let name = quote::format_ident!("route_{}", service_name);
        let mod_name = quote::format_ident!("{}_actix", service_name);

        let tonic_mod_name = quote::format_ident!("{}_server", service_name);
        let trait_name = quote::format_ident!("{}", service.name);
        let full_trait = quote::quote!(super::#tonic_mod_name::#trait_name);

        let methods_with_config = self.map_methods_with_rules(service);

        let methods: Vec<_> = methods_with_config
            .into_iter()
            .map(|(method, config)| {
                Method::new(
                    method.clone(),
                    self.messages.get(&method.input_type).unwrap().clone(),
                    self.messages.get(&method.output_type).unwrap().clone(),
                    config.clone(),
                    trait_name.clone(),
                )
            })
            .collect();

        if methods.is_empty() {
            return quote!();
        }
        let request_structs = methods.iter().map(|m| m.request().generate_structs());
        let fns = methods.iter().map(|m| m.generate_route());
        let configs = methods.iter().map(|m| m.generate_config());
        quote!(
            pub mod #mod_name {
                #![allow(unused_variables, dead_code, missing_docs)]

                use super::*;
                use #full_trait;
                use std::sync::Arc;

                #(#request_structs)*

                #(#fns)*

                pub fn #name(
                    config: &mut ::actix_web::web::ServiceConfig,
                    service: Arc<dyn #trait_name + Send + Sync + 'static>,
                ) {
                    config.app_data(::actix_web::web::Data::from(service));
                    #(#configs)*
                }
            }
        )
    }

    fn parse_messages(&mut self, buf: &mut str) {
        let file: syn::File = syn::parse_str(buf).unwrap();
        self.messages.extend(
            file.items
                .into_iter()
                .filter_map(|item| match item {
                    Item::Struct(message) => Some(message),
                    _ => None,
                })
                .map(|message| (message.ident.to_string(), message)),
        );
    }

    fn token_stream_to_code(&self, tokens: TokenStream) -> String {
        let ast: syn::File = syn::parse2(tokens).expect("not a valid tokenstream");
        let code = prettyplease::unparse(&ast);
        code
    }
}

impl ServiceGenerator for ActixGenerator {
    fn generate(&mut self, service: Service, buf: &mut String) {
        self.parse_messages(buf);
        let router = self.router(&service);
        let res = self.create_conversions(&service);

        buf.push_str(&self.token_stream_to_code(router));
        buf.push_str(&self.token_stream_to_code(res));
    }
}
