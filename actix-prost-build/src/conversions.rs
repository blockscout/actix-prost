use std::{
    collections::{BTreeMap, HashMap},
    env, fs,
    io::Error,
    path::PathBuf,
    rc::Rc,
};

use crate::helpers::extract_type_from_option;
use proc_macro2::{Ident, TokenStream};
use prost_build::Service;
use prost_reflect::{
    Cardinality, DescriptorPool, DynamicMessage, ExtensionDescriptor, FieldDescriptor, Kind,
    MessageDescriptor,
};
use quote::quote;
use syn::{punctuated::Punctuated, Expr, Field, Fields, Lit, Meta, MetaNameValue, Token, Type};

#[derive(Debug)]
pub struct ExtraFieldOptions {
    pub name: String,
    pub ty: String,
}
#[derive(Debug)]
pub struct ConvertFieldOptions {
    pub field: FieldDescriptor,
    pub ty: Option<String>,
    pub val_override: Option<String>,
    pub required: bool,
}

#[derive(Default, Debug)]
struct ConvertOptions {
    fields: BTreeMap<String, ConvertFieldOptions>,
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
                let convert_options = ConvertFieldOptions::from((&f, &fields_extension));

                (String::from(f.name()), convert_options)
            })
            .collect();
        Ok(Self { fields, extra })
    }
}

impl From<(&FieldDescriptor, &ExtensionDescriptor)> for ConvertFieldOptions {
    fn from((f, ext): (&FieldDescriptor, &ExtensionDescriptor)) -> Self {
        let options = f.options();
        let ext_val = options.get_extension(ext);
        let ext_val = ext_val.as_message().unwrap();

        Self {
            field: f.clone(),
            ty: get_string_field(ext_val, "type"),
            val_override: get_string_field(ext_val, "override"),
            required: match ext_val.get_field_by_name("required") {
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
        // At this point the file_descriptor_set.bin should be already generated
        let fds_path =
            PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR environment variable not set"))
                .join("file_descriptor_set.bin");
        let buf = fs::read(fds_path)?;

        let descriptors = DescriptorPool::decode(&*buf).unwrap();

        Ok(Self {
            descriptors,
            convert_prefix: quote!(convert_trait::TryConvert),
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

        let (extra_field_types, mut extra_field_conversions) =
            self.prepare_extra_fields(m_type, &convert_options);
        // Filter out extra_fields for Internal -> Proto conversions
        extra_field_conversions.retain(|v| v.is_some());

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

                let from = match field_conversions.len() + extra_field_conversions.len() {
                    0 => quote!(_from),
                    _ => quote!(from),
                };
                quote!(
                    impl #convert<#from_struct_ident> for #to_struct_ident {
                        fn try_convert(#from: #from_struct_ident) -> Result<Self, String> {
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
        *entry |= 1 << m_type as i32;

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
                let convert_field = convert_options.fields.get(&name.to_string());

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
        self.try_process_option(m_type, f, convert_field, res)
            .or(self.try_process_map(m_type, f, convert_field, res))
    }

    fn try_process_option(
        &mut self,
        m_type: MessageType,
        f: &Field,
        convert_field: Option<&ConvertFieldOptions>,
        res: &mut Vec<TokenStream>,
    ) -> Option<ProcessedType> {
        let name = f.ident.as_ref().unwrap();

        match extract_type_from_option(&f.ty) {
            Some(Type::Path(ty)) => {
                let ty = ty.path.segments.first()?;
                let rust_struct_name = self.messages.get(&ty.ident.to_string())?.ident.clone();
                let new_struct_name =
                    self.build_internal_nested_struct(m_type, &rust_struct_name, res);
                let convert = &self.convert_prefix;
                let (ty, conversion) = match convert_field {
                    Some(ConvertFieldOptions { required: true, .. }) => {
                        let require_message = format!("field {} is required", name);
                        (
                            quote!(#new_struct_name),
                            quote!(#convert::try_convert(from.#name.ok_or(#require_message)?)?),
                        )
                    }
                    _ => (
                        quote!(::core::option::Option<#new_struct_name>),
                        quote!(#convert::try_convert(from.#name)?),
                    ),
                };
                Some((ty, conversion))
            }
            _ => None,
        }
    }

    fn try_process_map(
        &mut self,
        m_type: MessageType,
        f: &Field,
        convert_field: Option<&ConvertFieldOptions>,
        res: &mut Vec<TokenStream>,
    ) -> Option<ProcessedType> {
        let name = f.ident.as_ref().unwrap();

        let field_desc = convert_field.map(|cf| &cf.field)?;
        let map_type = match (field_desc.cardinality(), field_desc.kind()) {
            (Cardinality::Repeated, Kind::Message(m)) => Some(m),
            _ => None,
        }?;
        // Map keys can only be of scalar types, so we search for nested messages only in values
        let map_value_type = match map_type.map_entry_value_field().kind() {
            Kind::Message(m) => Some(m),
            _ => None,
        }?;
        let map_key_rust_type = match map_type.map_entry_key_field().kind() {
            Kind::String => quote!(::prost::alloc::string::String),
            Kind::Int32 => quote!(i32),
            Kind::Int64 => quote!(i64),
            Kind::Uint32 => quote!(u32),
            Kind::Uint64 => quote!(u64),
            Kind::Sint32 => quote!(i32),
            Kind::Sint64 => quote!(i64),
            Kind::Fixed32 => quote!(u32),
            Kind::Fixed64 => quote!(u64),
            Kind::Sfixed32 => quote!(i32),
            Kind::Sfixed64 => quote!(i64),
            Kind::Bool => quote!(bool),
            _ => panic!("Map key type not supported"),
        };
        // TODO: Proto name might not be the same as Rust struct name
        let rust_struct_name = self.messages.get(map_value_type.name())?.ident.clone();

        let new_struct_name = self.build_internal_nested_struct(m_type, &rust_struct_name, res);

        let convert = &self.convert_prefix;
        let map_collection = if let Type::Path(p) = &f.ty {
            match p.path.segments.iter().find(|s| s.ident == "HashMap") {
                Some(_) => quote!(::std::collections::HashMap),
                None => quote!(::std::collections::BTreeMap),
            }
        } else {
            panic!("Type of map field is not a path")
        };
        let ty = quote!(#map_collection<#map_key_rust_type, #new_struct_name>);
        let conversion = quote!(#convert::try_convert(from.#name)?);
        Some((ty, conversion))
    }

    fn build_internal_nested_struct(
        &mut self,
        m_type: MessageType,
        nested_struct_name: &Ident,
        res: &mut Vec<TokenStream>,
    ) -> Ident {
        // TODO: could incorrectly detect messages with same name in different packages
        let message = self
            .descriptors
            .all_messages()
            .find(|m| *nested_struct_name == m.name())
            .unwrap();

        self.create_convert_struct(m_type, &message, &nested_struct_name.to_string(), res)
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
                    (quote!(#ty), quote!(#convert::try_convert(from.#name)?))
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
    ) -> (Vec<TokenStream>, Vec<Option<TokenStream>>) {
        convert_options
            .extra
            .iter()
            .map(|ExtraFieldOptions { name, ty }| {
                let name = quote::format_ident!("{}", name);
                let ty = syn::parse_str::<Type>(ty).unwrap();
                let conv = match m_type {
                    MessageType::Input => Some(quote!(#name: None)),
                    MessageType::Output => None,
                };

                (quote!(pub #name: Option<#ty>), conv)
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
