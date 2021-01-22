use std::fmt;

use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize, Serialize,
};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    SoftwareMetaEntry,
};

#[derive(Debug, Copy, Clone, Serialize_repr, Deserialize_repr, Hash, PartialEq, Eq)]
#[repr(u32)]
#[non_exhaustive]
enum TagIndex {
    Media = 10,
    Artifact = 37,
    Href = 38,
    Ownership = 39,
    Rel = 40,
    MediaType = 41,
    Use = 42,
}

impl<'de> Deserialize<'de> for SoftwareMetaEntry {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SoftwareMetaEntryVisitor;

        impl<'de> Visitor<'de> for SoftwareMetaEntryVisitor {
            type Value = SoftwareMetaEntry;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a map")
            }

            fn visit_map<V>(self, mut map: V) -> Result<SoftwareMetaEntry, V::Error>
            where
                V: MapAccess<'de>,
            {
                todo!();
            }
        }

        deserializer.deserialize_map(SoftwareMetaEntryVisitor)
    }
}
