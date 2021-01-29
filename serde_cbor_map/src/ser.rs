use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Fields, GenericArgument, Path, PathArguments,
    Type, TypePath,
};

use crate::{utils::list_to_tuple, CatchallType};

pub(crate) fn impl_derive_serialize_cbor_map(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

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
