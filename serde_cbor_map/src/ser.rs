use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Fields,
};

use crate::{CatchallType, parser_helper};

pub(crate) fn impl_derive_serialize_cbor_map(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("Serialize_CBOR_Map applied to invalid struct type"),
        },
        _ => panic!("Serialize_CBOR_Map applied to invalid type"),
    };

    let quotes_list = fields
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref().expect("No identifier");

            let (attr_key, catchall_type) = parser_helper::get_field_matcher_and_catchall_type(field);

            let attr_counter = if catchall_type.is_some() {
                quote! { + self.#ident.num_items() }
            } else {
                /* A field that's not catchall, has 1 value */
                quote! { + 1 }
            };

            let attr_serializer = match catchall_type {
                None => {
                    quote! {
                        map.serialize_entry(&#attr_key, &self.#ident)?;
                    }
                }
                Some(CatchallType::Fields) => {
                    // TODO
                    panic!("CatchallType::Fields implementation");
                }
                Some(CatchallType::Unknown) => {
                    quote! {
                        for (k, v) in self.#ident.iter() {
                            map.serialize_entry(k, v)?;
                        }
                    }
                }
            };

            (attr_counter, attr_serializer)
        })
        .collect::<Vec<(_, _)>>();
    let (attr_counters, attr_serializers) = crate::utils::list_to_tuple_2(quotes_list);

    let res = TokenStream::from(quote! {
        impl serde::Serialize for #ident {
            fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::SerializeMap;

                let count = 0
                    #(#attr_counters)*
                ;

                let mut map = serializer.serialize_map(Some(count))?;
                #(#attr_serializers)*
                map.end()
            }
        }
    });
    if ident.to_string() == "CoSWIDTag" {
        println!("Tokenstream: {}", res);
    }
    res
}
