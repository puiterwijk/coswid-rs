use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Fields, GenericArgument, Path, PathArguments,
    Type, TypePath,
};

use crate::CatchallType;

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
            let attr_counter = quote! {};

            let attr_serializer = quote! {};

            (attr_counter, attr_serializer)
        })
        .collect::<Vec<(_, _)>>();
    let (attr_counters, attr_serializers) = crate::utils::list_to_tuple_2(quotes_list);

    let numfields = fields.len();

    let res = TokenStream::from(quote! {
        impl serde::Serialize for #ident {
            fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                todo!();
            }
        }
    });
    if ident.to_string() == "SomeStruct" {
        println!("Tokenstream: {}", res);
    }
    res
}
