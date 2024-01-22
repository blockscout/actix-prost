use std::{collections::HashMap, env, fs, io::Error, path::PathBuf, rc::Rc};

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
    // Shared messages with ActixGenerator
    pub messages: Rc<HashMap<String, syn::ItemStruct>>,
    descriptors: DescriptorPool,
    // Prefix for the Convert trait (could be static?)
    convert_prefix: TokenStream,
    // Track already processed messages and their impls in a simple bitmap
    // to prevent duplicated code generation
    processed_messages: HashMap<String, i32>,
}

type ProcessedType = (TokenStream, TokenStream);

#[derive(Copy, Clone)]
enum MessageType {
    Input = 0,
    Output = 1,
}

impl ConversionsGenerator {
    pub fn new() -> Result<Self, Error> {
        let path =
            PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR environment variable not set"))
                .join("file_descriptor_set.bin");
        let buf = fs::read(path)?;

        let descriptors = DescriptorPool::decode(&*buf).unwrap();

        Ok(Self {
            descriptors,
            convert_prefix: quote!(convert_trait::Convert),
            ..Default::default()
        })
    }

    pub fn create_conversions(&mut self, service: &Service) -> TokenStream {
        let methods = &service.methods;

        let mut res = vec![];
        for method in methods.iter() {
            let message_in = self
                .descriptors
                .get_message_by_name(&method.input_proto_type)
                .unwrap();

            let message_out = self
                .descriptors
                .get_message_by_name(&method.output_proto_type)
                .unwrap();

            self.create_convert_struct(
                MessageType::Input,
                &message_in,
                &method.input_type,
                &mut res,
            );
            self.create_convert_struct(
                MessageType::Output,
                &message_out,
                &method.output_type,
                &mut res,
            );
        }

        quote!(
            #(#res)*
        )
    }

    fn create_convert_struct(
        &mut self,
        m_type: MessageType,
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
            self.prepare_fields(m_type, fields.iter(), &convert_options, res);

        let (extra_field_types, extra_field_conversions) =
            self.prepare_extra_fields(m_type, &convert_options);

        let struct_ident = &rust_struct.ident;
        let internal_struct_ident = quote::format_ident!("{}Internal", struct_ident);

        let (from_struct_ident, to_struct_ident) = match m_type {
            MessageType::Input => (struct_ident, &internal_struct_ident),
            MessageType::Output => (&internal_struct_ident, struct_ident),
        };

        let struct_desc = self.processed_messages.get(message.name());

        // Generate struct if it was not generated before
        let struct_def = match struct_desc {
            None => {
                quote!(
                    #[derive(Debug)]
                    pub struct #internal_struct_ident {
                        #(#field_types,)*
                        #(#extra_field_types,)*
                    }
                )
            }
            _ => quote!(),
        };

        // Generate impl if it was not generated before
        let struct_impl = match struct_desc.map(|s| s & (1 << m_type as i32) != 0) {
            Some(true) => quote!(),
            _ => {
                let convert = &self.convert_prefix;

                quote!(
                    impl #convert<#from_struct_ident> for #to_struct_ident {
                        fn convert(from: #from_struct_ident) -> Result<Self, String> {
                            Ok(Self {
                                #(#field_conversions,)*
                                #(#extra_field_conversions,)*
                            })
                        }
                    }
                )
            }
        };

        let expanded = quote!(
            #struct_def
            #struct_impl
        );

        let entry = self
            .processed_messages
            .entry(message.name().to_string())
            .or_insert(0);
        *entry += 1 << m_type as i32;

        res.push(expanded);

        internal_struct_ident
    }

    fn prepare_fields<'a, I>(
        &mut self,
        m_type: MessageType,
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
                    .process_internal_struct(m_type, f, convert_field, res)
                    .or_else(|| Self::process_enum(m_type, f))
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
        &mut self,
        m_type: MessageType,
        f: &Field,
        convert_field: Option<&ConvertFieldOptions>,
        res: &mut Vec<TokenStream>,
    ) -> Option<ProcessedType> {
        let name = f.ident.as_ref().unwrap();

        // Check if the field contains a nested message
        let internal_struct = match extract_type_from_option(&f.ty) {
            Some(Type::Path(ty)) => ty
                .path
                .segments
                .first()
                .and_then(|ty| self.messages.get(&ty.ident.to_string())),
            _ => None,
        }?;

        // Process the nested message
        let ident = &internal_struct.ident;
        // TODO: could incorrectly detect messages with same name in different packages
        let message = self
            .descriptors
            .all_messages()
            .find(|m| *ident == m.name())
            .unwrap();
        let new_struct_name = self.create_convert_struct(m_type, &message, &ident.to_string(), res);

        let convert = &self.convert_prefix;
        Some(match convert_field {
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
        })
    }

    fn process_enum(m_type: MessageType, f: &Field) -> Option<ProcessedType> {
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
                        let enum_ident = lit.parse::<syn::Path>().ok();
                        let conv = match m_type {
                            MessageType::Input => {
                                quote!(#enum_ident::try_from(from.#name)?)
                            }
                            MessageType::Output => {
                                quote!(from.#name.into())
                            }
                        };
                        return Some((quote!(#enum_ident), conv));
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
                    let ty = syn::parse_str::<Type>(ty).unwrap();
                    let val_override = syn::parse_str::<Expr>(val_override).unwrap();
                    (quote!(#ty), quote!(#val_override))
                }
                (Some(ty), None) => {
                    let ty = syn::parse_str::<Type>(ty).unwrap();
                    (quote!(#ty), quote!(#convert::convert(from.#name)?))
                }
                (None, Some(val_override)) => {
                    let val_override = syn::parse_str::<Expr>(val_override).unwrap();
                    (get_default_type(), quote!(#val_override))
                }
                (None, None) => (get_default_type(), quote!(from.#name)),
            },
            None => (get_default_type(), quote!(from.#name)),
        }
    }

    fn prepare_extra_fields(
        &self,
        m_type: MessageType,
        convert_options: &ConvertOptions,
    ) -> (Vec<TokenStream>, Vec<TokenStream>) {
        convert_options
            .extra
            .iter()
            .map(|ExtraFieldOptions { name, ty }| {
                let name = quote::format_ident!("{}", name);
                let ty = syn::parse_str::<Type>(ty).unwrap();
                let conv = match m_type {
                    MessageType::Input => {
                        quote!(#name: None)
                    }
                    MessageType::Output => {
                        quote!()
                    }
                };

                (
                    quote! {
                        pub #name: Option<#ty>
                    },
                    conv,
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
