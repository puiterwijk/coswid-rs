use proc_macro::TokenStream;

mod de;
mod ser;
mod utils;

enum CatchallType {
    Fields,
    Unknown,
}

#[proc_macro_derive(Serialize_CBOR_Map, attributes(cbor_map_id, cbor_map_unknown))]
pub fn derive_serialize_cbor_map(input: TokenStream) -> TokenStream {
    ser::impl_derive_serialize_cbor_map(input)
}

#[proc_macro_derive(Deserialize_CBOR_Map, attributes(cbor_map_id, cbor_map_unknown))]
pub fn derive_deserialize_cbor_map(input: TokenStream) -> TokenStream {
    de::impl_derive_deserialize_cbor_map(input)
}
