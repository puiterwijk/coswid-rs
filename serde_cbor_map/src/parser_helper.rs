use proc_macro::{TokenStream, TokenTree};
use syn::{Attribute, Field};

use crate::CatchallType;

pub(crate) fn get_field_matcher_and_catchall_type(field: &Field) -> (u32, Option<CatchallType>) {
    let cbor_map_id_attrs = field
        .attrs
        .iter()
        .filter(|attr| {
            let ident = attr.path.segments.first().unwrap().ident.to_string();
            ident == "cbor_map_id" || ident == "cbor_map_unknown" || ident == "cbor_map_fields"
        })
        .collect::<Vec<&Attribute>>();
    if cbor_map_id_attrs.len() == 0 {
        panic!(format!(
            "No cbor_map_id, cbor_map_fields or cbor_map_unknown found on field {}",
            field.ident.as_ref().unwrap()
        ));
    }
    if cbor_map_id_attrs.len() > 1 {
        panic!(format!("Multiple cbor_map_id, cbor_map_fields or cbor_map_unknown attributes found on field {}", field.ident.as_ref().unwrap()));
    }
    let cbor_map_id_attr = cbor_map_id_attrs.first().unwrap();

    let catchall_type = match &cbor_map_id_attr
        .path
        .segments
        .first()
        .unwrap()
        .ident
        .to_string()[..]
    {
        "cbor_map_id" => None,
        "cbor_map_fields" => Some(CatchallType::Fields),
        "cbor_map_unknown" => Some(CatchallType::Unknown),
        _ => panic!("Impossible"),
    };
    let matcher = if catchall_type.is_some() {
        // This can be any value, it's ignored anyway
        42
    } else {
        let token_stream = cbor_map_id_attr.tokens.clone();
        let token_stream = TokenStream::from(token_stream);
        let token = token_stream.into_iter().next().expect(&format!(
            "No cbor_map_id value on field {}",
            field.ident.as_ref().unwrap().to_string()
        ));
        let token = match token {
            TokenTree::Group(group) => group,
            _ => panic!("Invalid token matched"),
        }
        .stream()
        .into_iter()
        .next()
        .expect(&format!(
            "Nothing found in parenthesis group for field {}",
            field.ident.as_ref().unwrap().to_string()
        ));
        let token = match token {
            TokenTree::Literal(literal) => literal.to_string(),
            _ => panic!(format!(
                "Non-literal cbor_map_id value for field {}: {}",
                field.ident.as_ref().unwrap().to_string(),
                token.to_string()
            )),
        };
        token.parse::<u32>().expect(&format!(
            "Non-integer cbor_map_id value for field {}: {}",
            field.ident.as_ref().unwrap().to_string(),
            token.to_string()
        ))
    };

    (matcher, catchall_type)
}
