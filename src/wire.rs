use std::collections::HashMap;
use std::convert::TryFrom;

use serde_repr::{Serialize_repr, Deserialize_repr};

use crate::{
    CoSWID,
    Result,
    Error,
};

pub(crate) type CoSWIDWire = HashMap<RootTagIndex, ciborium::value::Value>;

#[derive(Debug, Copy, Clone, Serialize_repr, Deserialize_repr, Hash, PartialEq, Eq)]
#[repr(u32)]
#[non_exhaustive]
pub(crate) enum RootTagIndex {
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
    ColloquiaVersion =45,
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

fn try_get_value<'de, T: serde::Deserialize<'de>>(wire: &CoSWIDWire, index: RootTagIndex) -> Result<T> {
    wire
        .get(&index)
        .ok_or_else(|| Error::MissingField(format!("{:?}", index)))?
        .deserialized()
        .map_err(|e| Error::from_field(format!("{:?}", index), e))
}

fn try_get_optional_value<'de, T: serde::Deserialize<'de>>(wire: &CoSWIDWire, index: RootTagIndex) -> Result<Option<T>> {
    match try_get_value::<T>(wire, index) {
        Err(Error::MissingField(_)) => Ok(None),
        Err(e) => Err(e),
        Ok(val) => Ok(Some(val)),
    }
}

impl TryFrom<CoSWIDWire> for CoSWID {
    type Error = Error;

    fn try_from(wire: CoSWIDWire) -> Result<Self> {
        Ok(
            CoSWID {
                tag_id: try_get_value(&wire, RootTagIndex::TagID)?,
                tag_version: try_get_value(&wire, RootTagIndex::TagVersion)?,
                corpus: try_get_optional_value(&wire, RootTagIndex::Corpus)?,
                patch: try_get_optional_value(&wire, RootTagIndex::Patch)?,
                supplemental: try_get_optional_value(&wire, RootTagIndex::Supplemental)?,
                software_name: try_get_value(&wire, RootTagIndex::SoftwareName)?,
                software_version: try_get_optional_value(&wire, RootTagIndex::SoftwareVersion)?,
                version_scheme: try_get_optional_value(&wire, RootTagIndex::VersionScheme)?,
                media: try_get_optional_value(&wire, RootTagIndex::Media)?,
            }
        )
    }
}
