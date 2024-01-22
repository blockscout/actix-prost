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
    pub required: bool,
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
            required: match value.get_field_by_name("required") {
                Some(v) => v.as_bool().unwrap(),
                None => false,
            },
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
    convert_prefix: TokenStream,
}

type ProcessedType = (TokenStream, TokenStream);

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
            convert_prefix: quote!(convert_trait::Convert),
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

        let (field_types, field_conversions) =
            self.prepare_fields(fields.iter(), &convert_options, res);

        let (extra_field_types, extra_field_conversions) =
            self.prepare_extra_fields(&convert_options);

        let struct_ident = &rust_struct.ident;
        let new_struct_ident = quote::format_ident!("{}Internal", struct_ident);
        let convert = &self.convert_prefix;
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
        res: &mut Vec<TokenStream>,
    ) -> (Vec<TokenStream>, Vec<TokenStream>)
    where
        I: Iterator<Item = &'a syn::Field>,
    {
        fields
            .map(|f| {
                let name = f.ident.clone().unwrap();
                let vis = &f.vis;
                let convert_field = convert_options
                    .fields
                    .iter()
                    .find(|(n, _)| n.eq(&name.to_string()))
                    .map(|(_, v)| v);

                // 1. Check if the field contains a nested message
                // 2. Check if the field is an enum
                // 3. Use the default conversion
                let (ty, conv) = self
                    .process_internal_struct(f, convert_field, res)
                    .or_else(|| Self::process_enum(f))
                    .unwrap_or_else(|| self.process_default(f, convert_field));

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

    fn process_internal_struct(
        &self,
        f: &Field,
        convert_field: Option<&ConvertFieldOptions>,
        res: &mut Vec<TokenStream>,
    ) -> Option<ProcessedType> {
        let name = f.ident.as_ref().unwrap();
        let convert = &self.convert_prefix;

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
        internal_struct.map(|s| {
            let ident = &s.ident;
            // TODO: could incorrectly detect messages with same name in different packages
            let message = self
                .descriptors
                .all_messages()
                .find(|m| *ident == m.name())
                .unwrap();
            let new_struct_name = self.create_convert_struct(&message, &ident.to_string(), res);

            match convert_field {
                Some(ConvertFieldOptions { required: true, .. }) => {
                    let require_message = format!("field {} is required", name);
                    (
                        quote!(#new_struct_name),
                        quote!(#convert::convert(from.#name.ok_or(#require_message)?)?),
                    )
                }
                _ => (
                    quote!(::core::option::Option<#new_struct_name>),
                    quote!(#convert::convert(from.#name)?),
                ),
            }
        })
    }

    fn process_enum(f: &Field) -> Option<ProcessedType> {
        let name = f.ident.as_ref().unwrap();

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
                        let enum_ident = lit.parse::<Ident>().ok();

                        return Some((
                            quote!(#enum_ident),
                            quote!(#enum_ident::try_from(from.#name)?),
                        ));
                    }
                }
            };

            None
        })
    }

    fn process_default(
        &self,
        f: &Field,
        convert_field: Option<&ConvertFieldOptions>,
    ) -> ProcessedType {
        let name = f.ident.as_ref().unwrap();
        let convert = &self.convert_prefix;

        let get_default_type = || {
            let ty = &f.ty;
            quote!(#ty)
        };

        match convert_field {
            Some(ConvertFieldOptions {
                ty, val_override, ..
            }) => match (ty, val_override) {
                (Some(ty), Some(val_override)) => {
                    let ty = syn::parse_str::<Type>(&ty).unwrap();
                    let val_override = syn::parse_str::<Expr>(&val_override).unwrap();
                    (quote!(#ty), quote!(#val_override))
                }
                (Some(ty), None) => {
                    let ty = syn::parse_str::<Type>(&ty).unwrap();
                    (quote!(#ty), quote!(#convert::convert(from.#name)?))
                }
                (None, Some(val_override)) => {
                    let val_override = syn::parse_str::<Expr>(&val_override).unwrap();
                    (get_default_type(), quote!(#val_override))
                }
                (None, None) => (get_default_type(), quote!(from.#name)),
            },
            None => (get_default_type(), quote!(from.#name)),
        }
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
}

fn get_string_field(m: &DynamicMessage, name: &str) -> Option<String> {
    let f = m.get_field_by_name(name)?.as_str().unwrap().to_string();
    if f.is_empty() {
        None
    } else {
        Some(f)
    }
}
