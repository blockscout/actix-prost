use std::{collections::HashMap, env, fs, path::PathBuf};

use proc_macro2::TokenStream;
use prost_build::Service;
use prost_reflect::{DescriptorPool, DynamicMessage, MessageDescriptor};
use quote::quote;
use syn::{Fields, ItemStruct, Type};

pub struct ExtraFieldOptions {
    pub name: String,
    pub ty: String,
}

pub struct ConvertOptions {
    pub ty: Option<String>,
    pub val_override: Option<String>,
}

#[derive(Default)]
struct ConvertFields {
    fields: Vec<(String, ConvertOptions)>,
    extra: Vec<ExtraFieldOptions>,
}

impl From<(&DescriptorPool, &MessageDescriptor)> for ConvertFields {
    fn from((descriptors, message): (&DescriptorPool, &MessageDescriptor)) -> Self {
        let message_extension = descriptors
            .get_message_by_name("google.protobuf.MessageOptions")
            .unwrap()
            .extensions()
            .find(|ext| ext.name() == "extra_fields")
            .unwrap();

        let fields_extension = descriptors
            .get_message_by_name("google.protobuf.FieldOptions")
            .unwrap()
            .extensions()
            .find(|ext| ext.name() == "convert")
            .unwrap();

        let options = message.options();
        let extra = options
            .get_extension(&message_extension)
            .as_list()
            .unwrap()
            .iter()
            .map(|v| {
                let m = v.as_message().unwrap();
                ExtraFieldOptions::from(m)
            })
            .collect();

        let fields = message
            .fields()
            .map(|f| {
                let options = f.options();
                let ext_val = options.get_extension(&fields_extension);
                let ext_val = ext_val.as_message().unwrap();

                let convert_options = ConvertOptions::from(ext_val);

                (String::from(f.name()), convert_options)
            })
            .collect();
        Self { fields, extra }
    }
}

impl From<&DynamicMessage> for ConvertOptions {
    fn from(value: &DynamicMessage) -> Self {
        Self {
            ty: get_string_field(value, "type"),
            val_override: get_string_field(value, "override"),
        }
    }
}

impl From<&DynamicMessage> for ExtraFieldOptions {
    fn from(value: &DynamicMessage) -> Self {
        Self {
            name: get_string_field(value, "name").unwrap(),
            ty: get_string_field(value, "type").unwrap(),
        }
    }
}

#[derive(Default)]
pub struct ConversionsGenerator {}

impl ConversionsGenerator {
    fn prepare_fields<'a, I>(
        fields: I,
        convert_fields: &ConvertFields,
        convert_prefix: &TokenStream,
    ) -> Vec<(TokenStream, TokenStream)>
    where
        I: Iterator<Item = &'a syn::Field>,
    {
        let fields: Vec<_> = fields
            .map(|f| {
                let name = f.ident.clone().unwrap();
                let vis = &f.vis;

                let field_override = convert_fields
                    .fields
                    .iter()
                    .find(|(n, _)| n.eq(&name.to_string()));
                let (ty, conv) = match field_override {
                    Some((_, ConvertOptions { ty: Some(ty), .. })) => {
                        let ty = syn::parse_str::<Type>(ty).unwrap();
                        (
                            quote!(#ty),
                            quote! { #convert_prefix::convert(from.#name)? },
                        )
                    }
                    _ => {
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

        fields
    }

    pub fn create_conversions(
        &self,
        service: &Service,
        messages: &HashMap<String, syn::ItemStruct>,
    ) -> TokenStream {
        let path =
            PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR environment variable not set"))
                .join("file_descriptor_set.bin");
        let buf = fs::read(path).unwrap();
        let descriptors = DescriptorPool::decode(&*buf).unwrap();

        let methods = &service.methods;

        let mut res = Vec::new();
        for method in methods.iter() {
            let message_in = descriptors
                .get_message_by_name(&method.input_proto_type)
                .unwrap();

            // TODO
            let _message_out = descriptors
                .get_message_by_name(&method.output_proto_type)
                .unwrap();

            let rust_struct = messages.get(&method.input_type).unwrap().clone();

            let convert_fields = ConvertFields::from((&descriptors, &message_in));

            res.push(Self::create_convert_struct(rust_struct, convert_fields));
        }
        quote!(
            #(#res)*
        )
    }

    fn create_convert_struct(input: ItemStruct, convert_fields: ConvertFields) -> TokenStream {
        let struct_name = input.ident.clone();
        let new_struct_name = quote::format_ident!("{}Internal", struct_name);

        let fields = match input.fields {
            Fields::Named(named) => named.named,
            _ => unimplemented!(),
        };

        let convert = quote!(actix_prost::convert::Convert);

        let fields = Self::prepare_fields(fields.iter(), &convert_fields, &convert);
        let field_types = fields.iter().map(|(t, _)| t);
        let field_conversions = fields.iter().map(|(_, c)| c);

        let extra_field_types =
            convert_fields
                .extra
                .iter()
                .map(|ExtraFieldOptions { name, ty }| {
                    let name = quote::format_ident!("{}", name);
                    let ty = syn::parse_str::<Type>(ty).unwrap();
                    quote! {
                        pub #name: Option<#ty>
                    }
                });

        let extra_field_conversions =
            convert_fields
                .extra
                .iter()
                .map(|ExtraFieldOptions { name, .. }| {
                    let name = quote::format_ident!("{}", name);
                    quote! {
                        #name: None
                    }
                });

        let expanded = quote::quote!(
            #[derive(Debug)]
            pub struct #new_struct_name {
                #(#field_types,)*
                #(#extra_field_types,)*
            }

            impl #convert<#struct_name> for #new_struct_name {
                fn convert(from: #struct_name) -> anyhow::Result<Self> {
                    Ok(Self {
                        #(#field_conversions,)*
                        #(#extra_field_conversions,)*
                    })
                }
            }
        );
        expanded
    }
}

fn get_string_field(m: &DynamicMessage, name: &str) -> Option<String> {
    let f = m.get_field_by_name(name)?.as_str().unwrap().to_string();
    if f.is_empty() {
        None
    } else {
        Some(f)
    }
}
