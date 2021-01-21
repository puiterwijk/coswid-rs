use std::fmt;

use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Serialize,
};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{AnyURI, EntityEntry, EntityRole, GlobalAttributes, HashEntry, OneOrMany};

#[derive(Debug, Copy, Clone, Serialize_repr, Deserialize_repr, Hash, PartialEq, Eq)]
#[repr(u32)]
#[non_exhaustive]
enum TagIndex {
    EntityName = 31,
    RegId = 32,
    Role = 33,
    ThumbPrint = 34,

    // global-attributes
    Lang = 15,
}

impl<'de> Deserialize<'de> for EntityEntry {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct EntityEntryVisitor;

        impl<'de> Visitor<'de> for EntityEntryVisitor {
            type Value = EntityEntry;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "an map")
            }

            fn visit_map<V>(self, mut map: V) -> Result<EntityEntry, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut entity_name: Option<String> = None;
                let mut reg_id: Option<AnyURI> = None;
                let mut role: Option<OneOrMany<EntityRole>> = None;
                let mut thumbprint: Option<HashEntry> = None;

                let mut global_attributes = GlobalAttributes::new();

                while let Some(key) = map.next_key::<TagIndex>()? {
                    match key {
                        TagIndex::EntityName => {
                            entity_name = Some(map.next_value()?);
                        }
                        TagIndex::RegId => {
                            reg_id = Some(map.next_value()?);
                        }
                        TagIndex::Role => {
                            role = Some(map.next_value()?);
                        }
                        TagIndex::ThumbPrint => {
                            thumbprint = Some(map.next_value()?);
                        }
                        TagIndex::Lang => {
                            global_attributes.lang = Some(map.next_value()?);
                        }
                    }
                }

                Ok(EntityEntry {
                    entity_name: entity_name
                        .ok_or_else(|| de::Error::missing_field("entity_name"))?,
                    reg_id,
                    role: role.ok_or_else(|| de::Error::missing_field("role"))?,
                    thumbprint,
                    global_attributes: global_attributes.get_none_or_some(),
                })
            }
        }

        deserializer.deserialize_any(EntityEntryVisitor)
    }
}
