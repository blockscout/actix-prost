use crate::{config::HttpRule, Request, Response};
use proc_macro2::{Ident, TokenStream};

pub struct Method {
    name: Ident,
    // method: prost_build::Method,
    config: HttpRule,
    trait_name: Ident,
    request: Request,
    response: Response,
}

impl Method {
    pub fn new(
        method: prost_build::Method,
        request_message: syn::ItemStruct,
        response_message: syn::ItemStruct,
        config: HttpRule,
        trait_name: Ident,
    ) -> Method {
        let method_name = quote::format_ident!("{}", method.proto_name);
        // TODO: accept all fields by reference
        let request = Request::new(request_message, method_name.clone(), &config);
        let response = Response::new(response_message, method_name, config.response_body.clone());
        Method {
            name: quote::format_ident!("{}", method.name),
            // method,
            config,
            trait_name,
            request,
            response,
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
        let request_init = self.request.generate_new_request();
        let extractors = self.request.generate_extractors();
        let (payload, payload_convert) = if self.request.has_sub(self.request.body()) {
            (
                Some(quote::quote!(payload: ::actix_web::web::Payload,)),
                Some(quote::quote!(let mut payload = payload.into_inner();)),
            )
        } else {
            (None, None)
        };
        let response_type = self.response.name();
        let response_convert = self.response.generate_convert();
        quote::quote!(
            async fn #method_name(
                service: ::actix_web::web::Data<dyn #trait_name + Sync + Send + 'static>,
                http_request: ::actix_web::HttpRequest,
                #payload
            ) -> Result<::actix_web::web::Json<#response_type>, ::actix_web::Error> {
                #payload_convert
                #extractors
                let request = #request_init;
                let request = ::actix_prost::new_request(request, &http_request);
                let response = service
                    .#name(request)
                    .await
                    .map_err(::actix_prost::map_tonic_error)?;
                let response = response.into_inner();
                #response_convert
                Ok(::actix_web::web::Json(response))
            }
        )
    }

    pub fn request(&self) -> &Request {
        &self.request
    }

    pub fn response(&self) -> &Response {
        &self.response
    }
}
