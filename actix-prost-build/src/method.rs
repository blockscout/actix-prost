use crate::{config::HttpRule, Request};
use proc_macro2::{Ident, TokenStream};

pub struct Method {
    name: Ident,
    method: prost_build::Method,
    config: HttpRule,
    trait_name: Ident,
    request: Request,
}

impl Method {
    pub fn new(
        method: prost_build::Method,
        request: syn::ItemStruct,
        config: HttpRule,
        trait_name: Ident,
    ) -> Method {
        let request = Request::new(
            request,
            quote::format_ident!("{}", method.proto_name),
            &config,
        );
        Method {
            name: quote::format_ident!("{}", method.name),
            method,
            config,
            trait_name,
            request,
        }
    }

    pub fn method_name(&self) -> Ident {
        quote::format_ident!("call_{}", self.name)
    }

    pub fn generate_config(&self) -> TokenStream {
        let path = self.config.pattern.path();
        let method_type = quote::format_ident!("{}", self.config.pattern.method());
        let name = self.method_name();
        quote::quote!(
            config.route(#path, ::actix_web::web::#method_type().to(#name));
        )
    }

    pub fn generate_route(&self) -> TokenStream {
        let method_name = self.method_name();
        let name = &self.name;
        let trait_name = &self.trait_name;
        let response_type = quote::format_ident!("{}", self.method.input_type);
        let request_init = self.request.generate_new_request();
        let args = self.request.generate_fn_args();
        let into_inners = self.request.generate_into_inners();
        quote::quote!(
            async fn #method_name(
                service: ::actix_web::web::Data<dyn #trait_name + Sync + Send + 'static>,
                #args
            ) -> Result<::actix_web::web::Json<#response_type>, ::actix_web::Error> {
                #into_inners
                let request = #request_init;
                Ok(
                    ::actix_web::web::Json(
                        service.
                            #name(request.into_request())
                            .await
                            .map_err(::actix_prost::map_tonic_error)?
                            .into_inner()
                    ),
                )
            }
        )
    }

    pub fn request(&self) -> &Request {
        &self.request
    }
}
