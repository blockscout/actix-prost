use proc_macro2::{Ident, TokenStream};
use prost_build::{Method, Service, ServiceGenerator};
use serde::Deserialize;
use std::{collections::HashMap, fs::File, path::Path};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Pattern {
    Get(String),
    Put(String),
    Post(String),
    Delete(String),
    Patch(String),
    // Custom(String, String) // TODO
}

impl Pattern {
    fn path(&self) -> &String {
        match self {
            Self::Get(p) => p,
            Self::Put(p) => p,
            Self::Post(p) => p,
            Self::Delete(p) => p,
            Self::Patch(p) => p,
        }
    }

    fn method(&self) -> &str {
        match self {
            Self::Get(p) => "get",
            Self::Put(p) => "put",
            Self::Post(p) => "post",
            Self::Delete(p) => "delete",
            Self::Patch(p) => "patch",
        }
    }
}

#[derive(Debug, Deserialize)]
struct HttpRule {
    selector: String,
    #[serde(flatten)]
    pattern: Pattern,
    body: Option<String>,
    response_body: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Http {
    rules: Vec<HttpRule>,
}

#[derive(Debug, Deserialize)]
struct Config {
    http: Http,
}

pub struct ActixGenerator {
    config: Config,
}

fn naive_snake_case(name: &str) -> String {
    let mut s = String::new();
    let mut it = name.chars().peekable();

    while let Some(x) = it.next() {
        s.push(x.to_ascii_lowercase());
        if let Some(y) = it.peek() {
            if y.is_uppercase() {
                s.push('_');
            }
        }
    }

    s
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
        Ok(ActixGenerator { config })
    }

    fn method(
        &self,
        method: &Method,
        config: &HttpRule,
        full_trait: &Ident,
    ) -> Option<(syn::ItemFn, TokenStream)> {
        let fields: Vec<String> = config
            .pattern
            .path()
            .split('{')
            .skip(1)
            .filter_map(|q| q.split('}').next())
            .map(|x| x.to_owned())
            .collect();

        let name = quote::format_ident!("call_{}", naive_snake_case(&method.name));
        let path = config.pattern.path();
        let method_type = quote::format_ident!("{}", config.pattern.method());
        let method_config = quote::quote!(
            config.route(#path, web::#method_type().to(#name));
        );
        let request_type = quote::format_ident!("{}", method.input_type);
        let response_type = quote::format_ident!("{}", method.input_type);
        let method_fn = if fields.is_empty() {
            syn::parse_quote!(
                async fn #name(
                    service: Data<dyn #full_trait>,
                    body: Json<super::#request_type>,
                ) -> Result<Json<super::#response_type>, Error> {
                    Err(actix_web::error::ErrorNotImplemented(""))
                }
            )
        } else {
            let fields = fields
                .into_iter()
                .map(|field| quote::format_ident!("{}", field))
                .map(|field| {
                    quote::quote!(
                        body.#field = path.#field;
                    )
                });
            syn::parse_quote!(
                async fn #name(
                    service: Data<dyn #full_trait>,
                    path: Path<super::#request_type>,
                    mut body: Json<super::#request_type>,
                ) -> Result<Json<super::#response_type>, Error> {
                    #(#fields)*
                    dbg!(&body);
                    Err(actix_web::error::ErrorNotImplemented(""))
                }
            )
        };
        Some((method_fn, method_config))
    }

    fn map_methods_with_rules<'a, 'b>(
        &'a self,
        service: &'b Service,
    ) -> Vec<(&'b Method, &'a HttpRule)> {
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
        dbg!(&service);
        let name = quote::format_ident!("route_{}", naive_snake_case(&service.name));
        let mod_name = quote::format_ident!("{}_actix", naive_snake_case(&service.name));
        let tonic_mod_name = quote::format_ident!("{}_server", naive_snake_case(&service.name));
        let trait_name = quote::format_ident!("{}", service.name);
        let full_trait = quote::quote!(super::#tonic_mod_name::#trait_name);
        let methods_with_config = self.map_methods_with_rules(service);
        let (fns, configs): (Vec<_>, Vec<_>) = methods_with_config
            .into_iter()
            .filter_map(|(method, config)| self.method(method, config, &trait_name))
            .unzip();
        quote::quote!(
            pub mod #mod_name {
                #![allow(unused_variables, dead_code, missing_docs)]

                use #full_trait;
                use actix_web::{web::{self, Json, ServiceConfig, Data, Path}, error::Error};
                use std::sync::Arc;

                #(#fns)*

                pub fn #name(
                    config: &mut ServiceConfig,
                    service: Arc<dyn #trait_name>,
                ) {
                    config.app_data(Data::from(service));
                    #(#configs)*
                }
            }
        )
    }
}

impl ServiceGenerator for ActixGenerator {
    fn generate(&mut self, service: Service, buf: &mut String) {
        let router = self.router(&service);

        let ast: syn::File = syn::parse2(router).expect("not a valid tokenstream");
        let code = prettyplease::unparse(&ast);
        buf.push_str(&code);
    }
}
