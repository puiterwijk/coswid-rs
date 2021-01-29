use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Fields, GenericArgument, Path, PathArguments,
    Type, TypePath,
};

use crate::{utils::list_to_tuple, CatchallType};

pub(crate) fn impl_derive_serialize_cbor_map(input: TokenStream) -> TokenStream {
    todo!();
}
