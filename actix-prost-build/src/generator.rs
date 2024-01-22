use crate::{config::HttpRule, conversions::ConversionsGenerator, method::Method, Config};
use proc_macro2::TokenStream;
use prost_build::{Service, ServiceGenerator};
use quote::quote;
use std::{collections::HashMap, fs::File, path::Path, rc::Rc};
use syn::Item;

pub struct ActixGenerator {
    messages: Rc<HashMap<String, syn::ItemStruct>>,
    config: Config,
    conversions_gen: Option<ConversionsGenerator>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("could not open file {0}")]
    File(#[from] std::io::Error),
    #[error("could not parse the config {0}")]
    Parse(#[from] serde_yaml::Error),
}

impl ActixGenerator {
    pub fn new(path: impl AsRef<Path>) -> Result<ActixGenerator, Error> {
        let file = File::open(path)?;
        let config: Config = serde_yaml::from_reader(file)?;
        let conversions_gen = ConversionsGenerator::new().ok();
        Ok(ActixGenerator {
            messages: Default::default(),
            config,
            conversions_gen,
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
        // let messages = self.messages.borrow_mut();
        self.messages = Rc::new(
            file.items
                .into_iter()
                .filter_map(|item| match item {
                    Item::Struct(message) => Some(message),
                    _ => None,
                })
                .map(|message| (message.ident.to_string(), message))
                .collect(),
        );
        if let Some(ref mut conversions_gen) = self.conversions_gen {
            conversions_gen.messages = Rc::clone(&self.messages);
        }
    }

    fn token_stream_to_code(&self, tokens: TokenStream) -> String {
        let ast: syn::File = syn::parse2(tokens).expect("not a valid tokenstream");
        prettyplease::unparse(&ast)
    }

    pub fn create_conversions(&mut self, service: &Service) -> Option<TokenStream> {
        self.conversions_gen
            .as_mut()
            .map(|g| g.create_conversions(service))
    }
}

impl ServiceGenerator for ActixGenerator {
    fn generate(&mut self, service: Service, buf: &mut String) {
        self.parse_messages(buf);
        let router = self.router(&service);
        let conversions = self.create_conversions(&service);

        buf.push_str(&self.token_stream_to_code(router));
        if let Some(conversions) = conversions {
            buf.push_str(&self.token_stream_to_code(conversions));
        }
    }
}
