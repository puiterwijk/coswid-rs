use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, GenericArgument, Path, PathArguments,
    Type, TypePath,
};

use crate::{parser_helper, CatchallType};

pub(crate) fn impl_derive_deserialize_cbor_map(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("Deserialize_CBOR_Map applied to invalid struct type"),
        },
        _ => panic!("Deserialize_CBOR_Map applied to invalid type"),
    };

    // Build the components based on attributes
    let quotes_list = fields.iter().map(|field| {
        let ident = field.ident.as_ref().expect("No identifier");
        let (non_option_ty, is_optional) = match &field.ty {
            Type::Path(TypePath {
                qself: None,
                path:
                    Path {
                        leading_colon: _,
                        segments: seg,
                    },
            }) => {
                let last_seg = &seg.last().expect("No last segment");
                match &last_seg.ident.to_string()[..] {
                    "Option" => match &last_seg.arguments {
                        PathArguments::AngleBracketed(args) => {
                            match &args.args.first().expect("No argument to Option") {
                                GenericArgument::Type(ty) => match ty {
                                    Type::Path(tp) => (Type::Path(tp.clone()), true),
                                    _ => panic!("Non-path Option type"),
                                },
                                _ => panic!("Non-type Option argument"),
                            }
                        }
                        _ => panic!("Non-bracketed option"),
                    },
                    _ => (field.ty.clone(), false),
                }
            }
            _ => panic!("Unsupported type encontered"),
        };

        let (matcher, catchall_type) = parser_helper::get_field_matcher_and_catchall_type(field);

        let attr_placeholder = if catchall_type.is_some() {
            quote! {
                let mut #ident: #non_option_ty = #non_option_ty::new();
            }
        } else {
            quote! {
                let mut #ident: core::option::Option<#non_option_ty> = None;
            }
        };

        let attr_matcher = if catchall_type.is_some() {
            quote! {}
        } else {
            quote! {
                #matcher => {
                    if #ident.is_some() {
                        return Err(serde::de::Error::duplicate_field(stringify!(#ident)));
                    }
                    #ident = Some(map.next_value()?);
                }
            }
        };

        let catchall_attr_matcher = match catchall_type {
            None => quote! {},
            Some(CatchallType::Fields) => {
                panic!("TODO: CatchallType::Fields implementation");
            }
            Some(CatchallType::Unknown) =>
                quote! {
                    if #ident.handles_key(catchall) {
                        #ident.fill_value(catchall, map.next_value()?);
                        continue;
                    }
                }
            };

        let attr_installer = if is_optional || catchall_type.is_some() {
            quote! {
                #ident,
            }
        } else {
            quote! {
                #ident: #ident.ok_or_else(|| serde::de::Error::missing_field(stringify!(#ident)))?,
            }
        };

        (attr_placeholder, attr_matcher, catchall_attr_matcher, attr_installer)
    })
    .collect::<Vec<(_, _, _, _)>>();
    let (attr_placeholders, attr_matchers, catchall_attr_matchers, attr_installers) =
        crate::utils::list_to_tuple_4(quotes_list);

    let mut attr_matchers = attr_matchers;
    // Add the catchall entry
    attr_matchers.push(quote! {
        catchall => {
            #(#catchall_attr_matchers)*

            // If none of the catchall-ers parsed it, it's unknown
            return Err(serde::de::Error::custom(format!("Field found with unknown field ID: {}", catchall)));
        },
    });
    let attr_matchers = attr_matchers;

    let res = TokenStream::from(quote! {
        impl<'de> serde::Deserialize<'de> for #ident {
            fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct OurVisitor;

                impl<'de> serde::de::Visitor<'de> for OurVisitor {
                    type Value = #ident;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        write!(formatter, "a map")
                    }

                    fn visit_map<V>(self, mut map: V) -> core::result::Result<#ident, V::Error>
                    where
                        V: serde::de::MapAccess<'de>,
                    {
                        #(#attr_placeholders)*

                        while let Some(_cbor_map_key) = map.next_key::<u32>()? {
                            match _cbor_map_key {
                                #(#attr_matchers)*
                            }
                        }

                        Ok(#ident {
                            #(#attr_installers)*
                        })
                    }
                }

                deserializer.deserialize_map(OurVisitor)
            }
        }
    });

    #[cfg(feature = "print_tokenstreams")]
    println!("Deserialization tokenstream for {}: {}", ident, res);

    res
}
