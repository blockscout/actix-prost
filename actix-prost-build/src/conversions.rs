use std::{collections::HashMap, env, fs, path::PathBuf, rc::Rc};

use crate::helpers::extract_type_from_option;
use proc_macro2::{Ident, TokenStream};
use prost_build::Service;
use prost_reflect::{DescriptorPool, DynamicMessage, MessageDescriptor};
use quote::quote;
use syn::{punctuated::Punctuated, Expr, Field, Fields, Lit, Meta, MetaNameValue, Token, Type};

#[derive(Debug)]
pub struct ExtraFieldOptions {
    pub name: String,
    pub ty: String,
}
#[derive(Debug)]
pub struct ConvertFieldOptions {
    pub ty: Option<String>,
    pub val_override: Option<String>,
}

#[derive(Default, Debug)]
struct ConvertOptions {
    fields: Vec<(String, ConvertFieldOptions)>,
    extra: Vec<ExtraFieldOptions>,
}

impl TryFrom<(&DescriptorPool, &MessageDescriptor)> for ConvertOptions {
    type Error = String;

    fn try_from(
        (descriptors, message): (&DescriptorPool, &MessageDescriptor),
    ) -> Result<Self, Self::Error> {
        let message_extension = descriptors
            .get_message_by_name("google.protobuf.MessageOptions")
            .ok_or("MessageOptions not found")?
            .extensions()
            .find(|ext| ext.name() == "extra_fields")
            .unwrap();

        let fields_extension = descriptors
            .get_message_by_name("google.protobuf.FieldOptions")
            .ok_or("FieldOptions not found")?
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

                let convert_options = ConvertFieldOptions::from(ext_val);

                (String::from(f.name()), convert_options)
            })
            .collect();
        Ok(Self { fields, extra })
    }
}

impl From<&DynamicMessage> for ConvertFieldOptions {
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
pub struct ConversionsGenerator {
    pub messages: Rc<HashMap<String, syn::ItemStruct>>,
    descriptors: DescriptorPool,
}

impl ConversionsGenerator {
    pub fn new() -> Self {
        let path =
            PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR environment variable not set"))
                .join("file_descriptor_set.bin");
        let buf = fs::read(path).unwrap();
        let descriptors = DescriptorPool::decode(&*buf).unwrap();

        Self {
            messages: Default::default(),
            descriptors,
        }
    }

    pub fn create_conversions(&self, service: &Service) -> TokenStream {
        let methods = &service.methods;

        let mut res = vec![];
        for method in methods.iter() {
            let message_in = self
                .descriptors
                .get_message_by_name(&method.input_proto_type)
                .unwrap();

            // TODO
            let _message_out = self
                .descriptors
                .get_message_by_name(&method.output_proto_type)
                .unwrap();

            self.create_convert_struct(&message_in, &method.input_type, &mut res);
        }

        quote!(
            #(#res)*
        )
    }

    fn create_convert_struct(
        &self,
        message: &MessageDescriptor,
        struct_name: &String,
        res: &mut Vec<TokenStream>,
    ) -> Ident {
        let rust_struct = self.messages.get(struct_name).unwrap().clone();

        let fields = match rust_struct.fields {
            Fields::Named(named) => named.named,
            _ => unimplemented!(),
        };

        let convert_options = ConvertOptions::try_from((&self.descriptors, message)).unwrap();

        let convert = quote!(convert_trait::Convert);

        let (field_types, field_conversions) =
            self.prepare_fields(fields.iter(), &convert_options, &convert, res);

        let (extra_field_types, extra_field_conversions) =
            self.prepare_extra_fields(&convert_options);

        let struct_ident = &rust_struct.ident;
        let new_struct_ident = quote::format_ident!("{}Internal", struct_ident);
        let expanded = quote::quote!(
            #[derive(Debug)]
            pub struct #new_struct_ident {
                #(#field_types,)*
                #(#extra_field_types,)*
            }

            impl #convert<#struct_ident> for #new_struct_ident {
                fn convert(from: #struct_ident) -> Result<Self, String> {
                    Ok(Self {
                        #(#field_conversions,)*
                        #(#extra_field_conversions,)*
                    })
                }
            }
        );
        res.push(expanded);
        new_struct_ident
    }

    fn prepare_fields<'a, I>(
        &self,
        fields: I,
        convert_options: &ConvertOptions,
        convert_prefix: &TokenStream,
        res: &mut Vec<TokenStream>,
    ) -> (Vec<TokenStream>, Vec<TokenStream>)
    where
        I: Iterator<Item = &'a syn::Field>,
    {
        fields
            .map(|f| {
                let name = f.ident.clone().unwrap();
                let vis = &f.vis;

                // Check if the field contains a nested message
                let internal_struct = match extract_type_from_option(&f.ty) {
                    Some(Type::Path(ty)) => ty
                        .path
                        .segments
                        .first()
                        .and_then(|ty| self.messages.get(&ty.ident.to_string())),
                    _ => None,
                };

                // Process the nested message
                if let Some(s) = internal_struct {
                    let ident = &s.ident;
                    let message = self
                        .descriptors
                        .all_messages()
                        .find(|m| *ident == m.name())
                        .unwrap();
                    // let message = self.descriptors.get_message_by_name(ident).unwrap();
                    let new_struct_name =
                        self.create_convert_struct(&message, &ident.to_string(), res);

                    return (
                        quote! {
                            #vis #name: ::core::option::Option<#new_struct_name>
                        },
                        quote! {
                            #name: #convert_prefix::convert(from.#name)?
                        },
                    );
                }

                // Handle enums
                if let Some(enum_ident) = Self::is_enum(f) {
                    return (
                        quote! {
                            #vis #name: #enum_ident
                        },
                        quote! {
                            #name: #enum_ident::try_from(from.#name)?
                        },
                    );
                };

                let convert_field = convert_options
                    .fields
                    .iter()
                    .find(|(n, _)| n.eq(&name.to_string()));

                let (ty, conv) = match convert_field {
                    Some((_, ConvertFieldOptions { ty: Some(ty), .. })) => {
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
            .unzip()
    }

    fn prepare_extra_fields(
        &self,
        convert_options: &ConvertOptions,
    ) -> (Vec<TokenStream>, Vec<TokenStream>) {
        convert_options
            .extra
            .iter()
            .map(|ExtraFieldOptions { name, ty }| {
                let name = quote::format_ident!("{}", name);
                let ty = syn::parse_str::<Type>(ty).unwrap();
                (
                    quote! {
                        pub #name: Option<#ty>
                    },
                    quote! {
                        #name: None
                    },
                )
            })
            .unzip()
    }

    fn is_enum(f: &Field) -> Option<Ident> {
        f.attrs.iter().find_map(|a| {
            if !a.path().is_ident("prost") {
                return None;
            }

            if let Meta::List(list) = &a.meta {
                let meta_list = list
                    .parse_args_with(Punctuated::<MetaNameValue, Token![,]>::parse_terminated)
                    .ok()?;
                let enum_part = meta_list.iter().find(|m| m.path.is_ident("enumeration"))?;

                if let Expr::Lit(expr) = &enum_part.value {
                    if let Lit::Str(lit) = &expr.lit {
                        return lit.parse::<Ident>().ok();
                    }
                }
            };

            None
        })
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
