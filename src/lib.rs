use std::collections::HashMap;

mod manual_serde;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_int_map::UnknownKeyHandler;
use serde_int_map_derive::{Deserialize_Int_Map, Serialize_Int_Map};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum VersionScheme {
    MultiPartNumeric = 1,
    MultiPartNumericSuffix = 2,
    AlphaNumeric = 3,
    Decimal = 4,
    Semver = 16384,
}

#[derive(
    Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive,
)]
#[repr(u32)]
pub enum GlobalAttributesKey {
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

#[derive(Debug, Default)]
pub struct GlobalAttributes {
    any_attribute: HashMap<u32, ciborium::value::Value>,
}

impl GlobalAttributes {
    pub fn get_value(&self, key: GlobalAttributesKey) -> Option<&ciborium::value::Value> {
        unsafe { self.get_raw_value(key.into()) }
    }

    pub unsafe fn get_raw_value(&self, key: u32) -> Option<&ciborium::value::Value> {
        return self.any_attribute.get(&key);
    }
}

impl UnknownKeyHandler for GlobalAttributes {
    type ValueType = ciborium::value::Value;

    fn new() -> Self {
        GlobalAttributes {
            //lang: None,
            any_attribute: HashMap::new(),
        }
    }

    fn num_items(&self) -> usize {
        self.any_attribute.len()
    }

    fn iter(&self) -> std::collections::hash_map::Iter<u32, ciborium::value::Value> {
        self.any_attribute.iter()
    }

    fn handles_key(&self, _key: u32) -> bool {
        true
    }

    fn fill_value(&mut self, key: u32, value: Self::ValueType) {
        self.any_attribute.insert(key, value);
    }
}

#[derive(Debug)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

pub type Text = String;
pub type AnyURI = String;

// https://www.iana.org/assignments/named-information/named-information.xhtml
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum HashAlgorithm {
    Unknown = 0,
    Sha256 = 1,
    Sha256_128 = 2,
    Sha256_120 = 3,
    Sha256_96 = 4,
    Sha256_64 = 5,
    Sha256_32 = 6,
    Sha384 = 7,
    Sha512 = 8,
    Sha3_224 = 9,
    Sha3_256 = 10,
    Sha3_384 = 11,
    Sha3_512 = 12,
}

impl Default for HashAlgorithm {
    fn default() -> Self {
        HashAlgorithm::Unknown
    }
}

pub type HashEntry = (HashAlgorithm, Vec<u8>);

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum EntityRole {
    TagCreator = 1,
    SoftwareCreator = 2,
    Aggregator = 3,
    Distributor = 4,
    Licensor = 5,
    Maintainer = 6,
}

#[derive(Debug, Deserialize_Int_Map, Serialize_Int_Map)]
pub struct EntityEntry {
    #[int_map_id(31)]
    pub entity_name: Text,
    #[int_map_id(32)]
    pub reg_id: Option<AnyURI>,
    #[int_map_id(33)]
    pub role: OneOrMany<EntityRole>,
    #[int_map_id(34)]
    pub thumbprint: Option<HashEntry>,
    #[int_map_unknown]
    pub global_attributes: GlobalAttributes,
    // * $$ entity-extension
}

#[derive(Debug, Deserialize_Int_Map, Serialize_Int_Map)]
pub struct SoftwareMetaEntry {
    #[int_map_id(43)]
    pub activation_status: Option<Text>,
    #[int_map_id(44)]
    pub channel_type: Option<Text>,
    #[int_map_id(45)]
    pub colloquial_version: Option<Text>,
    #[int_map_id(46)]
    pub description: Option<Text>,
    #[int_map_id(47)]
    pub edition: Option<Text>,
    #[int_map_id(48)]
    pub entitlement_data_required: Option<bool>,
    #[int_map_id(49)]
    pub entitlement_key: Option<Text>,
    #[int_map_id(50)]
    pub generator: Option<Text>,
    #[int_map_id(51)]
    pub persistent_id: Option<Text>,
    #[int_map_id(52)]
    pub product: Option<Text>,
    #[int_map_id(53)]
    pub product_family: Option<Text>,
    #[int_map_id(54)]
    pub revision: Option<Text>,
    #[int_map_id(55)]
    pub summary: Option<Text>,
    #[int_map_id(56)]
    pub unspsc_code: Option<Text>,
    #[int_map_id(57)]
    pub unspsc_version: Option<Text>,
    #[int_map_unknown]
    pub global_attributes: GlobalAttributes,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum LinkOwnership {
    Shared = 1,
    Private = 2,
    Abandone = 3,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum LinkRel {
    Ancestor = 1,
    Component = 2,
    Feature = 3,
    InstallationMedia = 4,
    Parent = 6,
    Patches = 7,
    Requires = 8,
    SeeAlso = 9,
    Supersedes = 10,
    Supplemental = 11,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum LinkUse {
    Optional = 1,
    Required = 2,
    Recommended = 3,
}

#[derive(Debug, Deserialize_Int_Map, Serialize_Int_Map)]
pub struct LinkEntry {
    #[int_map_id(37)]
    pub artifact: Option<Text>,
    #[int_map_id(38)]
    pub href: AnyURI,
    #[int_map_id(10)]
    pub media: Option<Text>,
    #[int_map_id(39)]
    pub ownership: Option<LinkOwnership>,
    #[int_map_id(40)]
    pub rel: LinkRel,
    #[int_map_id(41)]
    pub media_type: Option<Text>,
    #[int_map_id(42)]
    pub link_use: Option<LinkUse>,
}

#[derive(Debug, Deserialize_Int_Map, Serialize_Int_Map)]
pub struct DirectoryEntry {
    // filesystem-item
    #[int_map_id(22)]
    pub key: Option<bool>,
    #[int_map_id(23)]
    pub location: Option<Text>,
    #[int_map_id(24)]
    pub fs_name: Text,
    #[int_map_id(25)]
    pub root: Text,
    // global-attributes
    #[int_map_unknown]
    pub global_attributes: GlobalAttributes,
}

#[derive(Debug, Deserialize_Int_Map, Serialize_Int_Map)]
pub struct FileEntry {
    // filesystem-item
    #[int_map_id(22)]
    pub key: Option<bool>,
    #[int_map_id(23)]
    pub location: Option<Text>,
    #[int_map_id(24)]
    pub fs_name: Text,
    #[int_map_id(25)]
    pub root: Text,
    // rest
    #[int_map_id(20)]
    pub size: Option<u128>,
    #[int_map_id(21)]
    pub file_version: Option<Text>,
    #[int_map_id(7)]
    pub hash: Option<HashEntry>,
}

#[derive(Debug, Deserialize_Int_Map, Serialize_Int_Map)]
pub struct ProcessEntry {
    #[int_map_id(27)]
    pub process_name: Text,
    #[int_map_id(28)]
    pub pid: u32,
    // global-attributes
    #[int_map_unknown]
    pub global_attributes: GlobalAttributes,
}

#[derive(Debug, Deserialize_Int_Map, Serialize_Int_Map)]
pub struct ResourceEntry {
    #[int_map_id(29)]
    pub type_: Text,
    // global-attributes
    #[int_map_unknown]
    pub global_attributes: GlobalAttributes,
}

#[derive(Debug, Deserialize_Int_Map, Serialize_Int_Map)]
pub struct PayloadEntry {
    // resource-collection
    #[int_map_id(16)]
    pub directory: Option<OneOrMany<DirectoryEntry>>,
    #[int_map_id(17)]
    pub file: Option<OneOrMany<FileEntry>>,
    #[int_map_id(18)]
    pub process: Option<OneOrMany<ProcessEntry>>,
    #[int_map_id(19)]
    pub resource: Option<OneOrMany<ResourceEntry>>,
    // rest
    #[int_map_unknown]
    pub global_attributes: GlobalAttributes,
}

// TODO
pub type Time = String;

#[derive(Debug, Deserialize_Int_Map, Serialize_Int_Map)]
pub struct EvidenceEntry {
    // resource-collection
    #[int_map_id(16)]
    pub directory: Option<OneOrMany<DirectoryEntry>>,
    #[int_map_id(17)]
    pub file: Option<OneOrMany<FileEntry>>,
    #[int_map_id(18)]
    pub process: Option<OneOrMany<ProcessEntry>>,
    #[int_map_id(19)]
    pub resource: Option<OneOrMany<ResourceEntry>>,
    // rest
    #[int_map_id(35)]
    pub date: Time,
    #[int_map_id(36)]
    pub device_id: Text,
    #[int_map_unknown]
    pub global_attributes: GlobalAttributes,
}

#[derive(Debug, Deserialize_Int_Map, Serialize_Int_Map)]
pub struct CoSWIDTag {
    #[int_map_id(0)]
    pub tag_id: String,
    #[int_map_id(12)]
    pub tag_version: i32,
    #[int_map_id(8)]
    pub corpus: Option<bool>,
    #[int_map_id(9)]
    pub patch: Option<bool>,
    #[int_map_id(11)]
    pub supplemental: Option<bool>,
    #[int_map_id(1)]
    pub software_name: Text,
    #[int_map_id(13)]
    pub software_version: Option<Text>,
    #[int_map_id(14)]
    pub version_scheme: Option<VersionScheme>,
    #[int_map_id(10)]
    pub media: Option<Text>,
    #[int_map_id(5)]
    pub software_meta: Option<OneOrMany<SoftwareMetaEntry>>,
    #[int_map_id(2)]
    pub entity: OneOrMany<EntityEntry>,
    #[int_map_id(4)]
    pub link: Option<OneOrMany<LinkEntry>>,
    #[int_map_id(6)]
    pub payload: Option<PayloadEntry>,
    #[int_map_id(3)]
    pub evidence: Option<EvidenceEntry>,
    #[int_map_unknown]
    pub global_attributes: GlobalAttributes,
    // coswid-extension
}
