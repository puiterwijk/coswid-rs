use std::fmt;

use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize, Serialize,
};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    CoSWIDTag, EntityEntry, GlobalAttributes, LinkEntry, OneOrMany, PayloadOrEvidence,
    SoftwareMetaEntry, VersionScheme,
    wire::reusable::{set_if_empty, require_field},
};

#[derive(Debug, Copy, Clone, Serialize_repr, Deserialize_repr, Hash, PartialEq, Eq)]
#[repr(u32)]
#[non_exhaustive]
enum TagIndex {
    TagID = 0,
    SoftwareName = 1,
    Entity = 2,
    Evidence = 3,
    Link = 4,
    SoftwareMeta = 5,
    Payload = 6,
    Corpus = 8,
    Patch = 9,
    Media = 10,
    Supplemental = 11,
    TagVersion = 12,
    SoftwareVersion = 13,
    VersionScheme = 14,

    // global-attributes
    Lang = 15,
    Directory = 16,
    File = 17,
    Process = 18,
    Resource = 19,
    Size = 20,
    FileVersion = 21,
    Key = 22,
    Location = 23,
    FsName = 24,
    Root = 25,
    PathElements = 26,
    ProcessName = 27,
    Pid = 28,
    Type = 29,
    EntityName = 31,
    RegId = 32,
    Role = 33,
    Thumbprint = 34,
    Date = 35,
    DeviceID = 36,
    Artifact = 37,
    Href = 38,
    Ownership = 39,
    Rel = 40,
    MediaType = 41,
    Use = 42,
    ActivationStatus = 43,
    ChannelType = 44,
    ColloquiaVersion = 45,
    Description = 46,
    Edition = 47,
    EntitlementDataRequired = 48,
    EntitlementKey = 49,
    Generator = 50,
    PersistentId = 51,
    Product = 52,
    ProductFamily = 53,
    Revision = 54,
    Summary = 55,
    UnspscCode = 56,
    UnspscVersion = 57,
}

impl<'de> Deserialize<'de> for CoSWIDTag {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CoSWIDTagVisitor;

        impl<'de> Visitor<'de> for CoSWIDTagVisitor {
            type Value = CoSWIDTag;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a map")
            }

            fn visit_map<V>(self, mut map: V) -> Result<CoSWIDTag, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut tag_id: Option<String> = None;
                let mut tag_version: Option<i32> = None;
                let mut corpus: Option<bool> = None;
                let mut patch: Option<bool> = None;
                let mut supplemental: Option<bool> = None;
                let mut software_name: Option<String> = None;
                let mut software_version: Option<String> = None;
                let mut version_scheme: Option<VersionScheme> = None;
                let mut media: Option<String> = None;
                let mut software_meta: Option<OneOrMany<SoftwareMetaEntry>> = None;
                let mut entity: Option<OneOrMany<EntityEntry>> = None;
                let mut entity: Option<OneOrMany<EntityEntry>> = None;
                let mut link: Option<OneOrMany<LinkEntry>> = None;
                let mut payload_or_evidence: Option<PayloadOrEvidence> = None;
                let mut global_attributes = GlobalAttributes::new();

                while let Some(key) = map.next_key::<TagIndex>()? {
                    match key {
                        TagIndex::TagID => {
                            set_if_empty!(map, tag_id);
                        }
                        TagIndex::TagVersion => {
                            set_if_empty!(map, tag_version);
                        }
                        TagIndex::Corpus => {
                            set_if_empty!(map, corpus);
                        }
                        TagIndex::Patch => {
                            set_if_empty!(map, patch);
                        }
                        TagIndex::Supplemental => {
                            set_if_empty!(map, supplemental);
                        }
                        TagIndex::SoftwareName => {
                            set_if_empty!(map, software_name);
                        }
                        TagIndex::SoftwareVersion => {
                            set_if_empty!(map, software_version);
                        }
                        TagIndex::VersionScheme => {
                            set_if_empty!(map, version_scheme);
                        }
                        TagIndex::Media => {
                            set_if_empty!(map, media);
                        }
                        software_meta => {
                            //set_if_empty!(map, software_meta);
                        }
                        TagIndex::Entity => {
                            set_if_empty!(map, entity);
                        }
                        TagIndex::Link => {
                            set_if_empty!(map, link);
                        }
                        TagIndex::Payload => {
                            if payload_or_evidence.is_some() {
                                return Err(de::Error::duplicate_field("payload-or-evidence"));
                            }
                            //payload_or_evidence = Some(PayloadOrEvidence::Payload(map.next_value()?));
                        }
                        TagIndex::Evidence => {
                            if payload_or_evidence.is_some() {
                                return Err(de::Error::duplicate_field("payload-or-evidence"));
                            }
                            //payload_or_evidence = Some(PayloadOrEvidence::Evidence(map.next_value()?));
                        }

                        // Global Attributes
                        TagIndex::Lang => {
                            global_attributes.lang = Some(map.next_value()?);
                        }
                        key => {
                            let value = map.next_value::<ciborium::value::Value>()?;
                            global_attributes.fill_value(key as u32, value);
                        }
                    }
                }

                Ok(CoSWIDTag {
                    tag_id: require_field!(tag_id),
                    tag_version: require_field!(tag_version)?,
                    corpus,
                    patch,
                    supplemental,
                    software_name: require_field!(software_name)?,
                    software_version,
                    version_scheme,
                    media,
                    software_meta,
                    entity: require_field!(entity)?,
                    link,
                    payload_or_evidence,
                    global_attributes: global_attributes.get_none_or_some(),
                })
            }
        }

        deserializer.deserialize_map(CoSWIDTagVisitor)
    }
}
