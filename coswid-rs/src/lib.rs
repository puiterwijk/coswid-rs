mod error;
pub use error::{Error, Result};
use std::collections::HashMap;
use std::convert::TryFrom;

mod manual_serde;

use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde_cbor_map::Deserialize_CBOR_Map;
use serde_repr::{Deserialize_repr, Serialize_repr};

/*
Parsed CoSWID:
Ok(
    Map([
        (
            Integer(Integer(true, 15)),
            Text("en-US")
        ),
        (
            Integer(Integer(true, 0)),
            Text("50609997-4498-4af0-8cab-2129479a4f1c")
        ),
        (
            Integer(Integer(true, 12)),
            Integer(Integer(true, 1))
        ),
        (
            Integer(Integer(true, 1)),
            Text("acme firmware")
        ),
        (
            Integer(Integer(true, 13)),
            Text("1.0.0")
        ),
        (
            Integer(Integer(true, 14)),
            Integer(Integer(true, 1))
        ),
        (
            Integer(Integer(true, 2)),
            Map([
                (
                    Integer(Integer(true, 31)),
                    Text("Acme Software")
                ),
                (
                    Integer(Integer(true, 32)),
                    Text("acme.com")
                ),
                (
                    Integer(Integer(true, 33)),
                    Array([Integer(Integer(true, 1)), Integer(Integer(true, 2))])
                )
            ])
        ),
        (
            Integer(Integer(true, 6)),
            Map([
                (
                    Integer(Integer(true, 59)),
                    Map([
                        (
                            Integer(Integer(true, 63)),
                            Text("0c0de2a7-26ff-47c4-99d9-981e48db0345")
                        ),
                        (
                            Integer(Integer(true, 64)),
                            Text("2021-01-19T17:31:40.316+01:00")
                        ),
                        (
                            Integer(Integer(true, 65)),
                            Integer(Integer(true, 1))
                        ),
                        (
                            Integer(Integer(true, 67)),
                            Bytes(Bytes([227, 238, 117, 61, 126, 76, 219, 127]))
                        ),
                        (
                            Integer(Integer(true, 85)),
                            Map([
                                    (
                                        Integer(Integer(true, 86)),
                                        Text("acme")
                                    ),
                                    (
                                        Integer(Integer(true, 88)),
                                        Text("acme")
                                    )
                            ])
                        ),
                        (
                            Integer(Integer(true, 60)),
                            Map([
                                (
                                    Integer(Integer(true, 61)),
                                    Text("9ec4f5f9-8be3-45bd-a172-b46bbdb2d8c8")
                                ),
                                (
                                    Integer(Integer(true, 71)),
                                    Map([
                                        (
                                            Integer(Integer(true, 72)),
                                            Integer(Integer(true, 0))
                                        )
                                    ])
                                ),
                                (
                                    Integer(Integer(true, 74)),
                                    Integer(Integer(true, 2048))
                                ),
                                (
                                    Integer(Integer(true, 97)),
                                    Array([
                                        Map([
                                            (
                                                Integer(Integer(true, 98)),
                                                Integer(Integer(true, 1))
                                            ),
                                            (
                                                Integer(Integer(true, 100)),
                                                Bytes(Bytes([39, 40, 19, 49, 45, 138, 231, 205, 186, 12, 203, 104, 176, 121, 215, 214, 5, 131, 240, 126, 70, 173, 236, 119, 185, 58, 126, 66, 20, 20, 188, 127, 66, 126, 141, 28, 93, 59, 208, 31, 205, 123, 86, 11, 124, 20, 43, 246, 68, 60, 158, 167, 57, 57, 210, 54, 229, 137, 32, 95, 62, 59, 42, 93]))
                                            )
                                        ])
                                    ])
                                ),
                                (
                                    Integer(Integer(true, 80)),
                                    Text("some storageId")
                                ),
                                (
                                    Integer(Integer(true, 75)),
                                    Map([
                                        (
                                            Integer(Integer(true, 78)),
                                            Bytes(Bytes([181, 255, 71, 5, 84, 155, 60, 171, 30, 7, 215, 68, 109, 106, 50, 78, 207, 149, 148, 155, 28, 152, 44, 86, 70, 174, 133, 173, 119, 210, 34, 236, 84, 109, 222, 181, 194, 205, 53, 176, 247, 136, 103, 138, 198, 12, 237, 119, 213, 189, 63, 202, 108, 131, 29, 49, 99, 144, 132, 238, 0, 72, 248, 242, 6, 34, 1, 17, 194, 225, 121, 225, 174, 199, 2, 88, 184, 71, 255, 18, 180, 222, 24, 11, 212, 96, 146, 249, 8, 222, 159, 254, 242, 94, 232, 181, 245, 36, 227, 73, 59, 176, 62, 150, 253, 93, 90, 84, 230, 14, 196, 35, 160, 195, 223, 159, 125, 125, 9, 22, 11, 254, 173, 135, 4, 78, 80, 83, 255, 251, 20, 46, 182, 188, 78, 197, 116, 230, 235, 48, 56, 187, 186, 62, 195, 109, 126, 47, 115, 87, 99, 65, 86, 0, 113, 35, 111, 65, 60, 1, 55, 78, 234, 158, 225, 101, 78, 211, 248, 161, 9, 93, 88, 239, 204, 6, 184, 94, 22, 251, 38, 86, 122, 192, 101, 44, 243, 146, 110, 7, 215, 37, 100, 86, 1, 84, 14, 166, 70, 165, 63, 110, 225, 9, 54, 140, 188, 99, 18, 179, 124, 58, 234, 18, 201, 176, 204, 223, 118, 118, 197, 204, 114, 72, 129, 37, 86, 27, 180, 99, 151, 132, 85, 12, 252, 34, 133, 181, 171, 167, 211, 12, 47, 113, 119, 49, 97, 109, 175, 188, 19, 167, 56, 233, 60, 79, 59, 181, 139, 46, 237, 81, 179, 35, 25, 183, 238, 19, 59, 230, 240, 6, 245, 77, 11, 228, 105, 200, 180, 173, 73, 40, 91, 156, 15, 188, 238, 45, 118, 202, 84, 192, 247, 96, 71, 221, 192, 9, 57, 209, 21, 21, 169, 57, 229, 34, 84, 87, 193, 89, 244, 47, 101, 25, 158, 121, 152, 152, 13, 162, 201, 167, 38, 152, 179, 190, 175, 25, 82, 54, 137, 159, 17, 233, 185, 218, 2, 93, 204, 227, 98, 91, 3, 119, 232, 54, 174, 119, 165, 194, 139, 217, 253, 143, 159, 109, 41, 103, 220, 160, 40, 140, 224, 129, 241, 253, 53, 28, 35, 29, 243, 0, 149, 79, 69, 98, 69, 70, 52, 97, 10, 123, 220, 232, 145, 24, 75, 184, 2, 69, 7, 129, 116, 78, 82, 12, 191, 56, 85, 181, 14, 206, 170, 217, 133, 109, 197, 43, 7, 234, 245, 192, 32, 28, 205, 197, 57, 76, 174, 211, 13, 226, 221, 28, 185, 218, 86, 150, 67, 44, 115, 130, 2, 239, 212, 159, 196, 217, 122, 167, 75, 176, 164, 50, 69, 70, 181, 20, 91, 157, 197, 170, 186, 167, 0, 230, 79, 31, 237, 108, 143, 141, 197, 43, 46, 174, 172, 218, 197, 110, 171, 116, 67, 113, 16, 0, 90, 28, 208, 117, 144, 168, 94, 153, 162, 175, 249, 66, 225, 97, 4, 178, 116, 184, 188, 75, 55, 83, 234, 154, 95, 21, 175, 48, 180, 253, 214, 213, 67, 144, 208, 40, 41, 199, 247, 212, 253, 35, 13, 34, 238, 85, 232, 11, 10, 68, 64, 250, 182, 121, 249, 22, 80, 173, 149, 160, 120, 29, 235, 227, 107, 191, 2, 205, 168, 113, 15, 122, 42, 48, 133, 196, 75, 131, 169, 36, 44, 167, 57, 86, 184, 27, 204, 112, 184, 133, 113, 53, 254, 221, 140, 47, 61, 95, 140, 188, 66, 114, 13, 78, 43, 129, 91, 96, 159, 252, 187, 62, 249, 236, 30, 163, 60, 108, 20, 78, 154, 101, 72, 69, 197, 48, 141, 53, 172, 57, 158, 1, 100, 231, 47, 97, 38, 86, 98, 80, 229, 101, 70, 209, 85, 121, 111, 212, 184, 129, 239, 122, 94, 36, 237, 208, 228, 217, 246, 60, 172, 185, 168, 44, 43, 22, 38, 255, 229, 32, 79, 173, 238, 74, 219, 51, 102, 221, 84, 237, 149, 208, 74, 6, 82, 210, 67, 181, 133, 174, 118, 227, 182, 135, 229, 98, 234, 12, 67, 200, 87, 182, 72, 84, 220, 197, 150, 222, 23, 188, 217, 110, 174, 117, 142, 38, 36, 113, 178, 180, 101, 35, 29, 126, 72, 54, 200, 212, 168, 21, 114, 96, 13, 15, 219, 145, 168, 117, 218, 168, 152, 190, 104, 43, 213, 80, 86, 188, 105, 60, 192, 219, 86, 43, 83, 226, 99, 254, 39, 108, 159, 248, 86, 220, 189, 151, 114, 226, 158, 196, 33, 190, 219, 152, 133, 58, 79, 239, 67, 120, 26, 130, 37, 98, 190, 101, 28, 12, 11, 140, 52, 234, 191, 14, 68, 248, 195, 17, 193, 123, 220, 126, 168, 181, 25, 30, 242, 115, 82, 70, 51, 128, 18, 11, 206, 79, 22, 157, 0, 224, 203, 218, 135, 221, 40, 36, 240, 47, 155, 3, 174, 156, 214, 184, 221, 86, 76, 101, 177, 205, 142, 61, 92, 239, 145, 19, 144, 219, 107, 149, 54, 248, 46, 34, 231, 208, 180, 42, 22, 1, 104, 185, 180, 243, 213, 5, 221, 25, 178, 253, 154, 52, 161, 9, 43, 180, 188, 211, 153, 175, 223, 42, 234, 5, 71, 210, 132, 211, 106, 188, 104, 254, 113, 213, 209, 197, 115, 254, 103, 96, 80, 51, 112, 251, 161, 86, 217, 29, 26, 152, 215, 253, 41, 192, 183, 208, 42, 160, 213, 3, 12, 110, 131, 219, 158, 112, 98, 226, 32, 23, 26, 185, 109, 99, 165, 200, 151, 66, 18, 227, 24, 106, 212, 148, 17, 253, 182, 71, 112, 162, 74, 22, 138, 241, 195, 69, 255, 202, 150, 218, 161, 198, 208, 59, 18, 97, 81, 255, 254, 39, 43, 138, 135, 72, 215, 243, 245, 220, 174, 104, 184, 125, 130, 233, 141, 8, 53, 49, 69, 140, 90, 101, 139, 252, 129, 51, 145, 233, 244, 251, 72, 56, 187, 226, 25, 240, 62, 179, 72, 13, 77, 251, 32, 161, 195, 25, 237, 70, 245, 59, 71, 148, 212, 82, 204, 231, 250, 214, 251, 40, 12, 34, 89, 230, 155, 175, 231, 51, 94, 116, 13, 156, 122, 63, 208, 146, 209, 45, 59, 165, 195, 237, 127, 91, 224, 244, 35, 13, 65, 199, 168, 78, 156, 163, 10, 33, 156, 183, 238, 229, 90, 242, 96, 246, 39, 183, 111, 76, 111, 103, 56, 227, 155, 228, 139, 227, 34, 108, 163, 173, 143, 103, 179, 49, 47, 32, 167, 93, 112, 50, 141, 79, 209, 187, 237, 54, 42, 37, 172, 167, 32, 122, 92, 186, 140, 114, 172, 55, 157, 219, 52, 125, 43, 232, 124, 5, 239, 180, 114, 114, 169, 238, 8, 58, 62, 147, 119, 196, 3, 180, 182, 218, 252, 106, 80, 224, 49, 204, 74, 125, 222, 30, 18, 169, 134, 10, 36, 30, 212, 219, 30, 65, 188, 156, 8, 135, 113, 114, 83, 64, 178, 248, 196, 4, 164, 164, 116, 217, 123, 170, 110, 148, 55, 65, 76, 46, 111, 58, 20, 197, 22, 132, 84, 49, 100, 98, 25, 89, 139, 179, 234, 196, 219, 46, 143, 202, 169, 218, 90, 155, 175, 45, 70, 72, 146, 158, 70, 128, 65, 149, 118, 29, 87, 34, 49, 123, 234, 56, 43, 204, 69, 240, 59, 116, 245, 156, 211, 2, 7, 176, 73, 255, 199, 226, 164, 15, 206, 72, 55, 127, 89, 221, 186, 24, 45, 19, 198, 65, 28, 160, 18, 62, 141, 190, 86, 86, 61, 136, 210, 114, 100, 52, 187, 4, 163, 7, 128, 93, 196, 6, 172, 190, 59, 37, 19, 123, 18, 143, 151, 125, 249, 222, 6, 39, 169, 143, 226, 135, 161, 129, 208, 154, 80, 52, 223, 227, 164, 141, 114, 129, 86, 4, 3, 28, 49, 51, 248, 239, 216, 154, 29, 183, 102, 98, 36, 130, 119, 157, 151, 184, 238, 29, 152, 146, 178, 8, 24, 240, 186, 229, 56, 24, 39, 21, 241, 164, 78, 59, 163, 46, 30, 212, 68, 145, 107, 201, 234, 197, 124, 255, 197, 249, 223, 92, 245, 245, 11, 10, 123, 74, 232, 83, 54, 12, 225, 61, 253, 205, 35, 147, 149, 46, 112, 181, 237, 35, 200, 213, 191, 108, 202, 183, 204, 220, 233, 27, 58, 225, 91, 12, 85, 100, 198, 99, 241, 37, 46, 18, 2, 249, 136, 111, 16, 108, 19, 242, 109, 38, 235, 198, 105, 13, 254, 197, 162, 225, 218, 242, 187, 253, 180, 80, 163, 81, 163, 26, 147, 44, 191, 87, 177, 75, 72, 146, 156, 60, 157, 213, 13, 234, 146, 156, 148, 3, 219, 20, 80, 53, 171, 193, 82, 158, 216, 41, 153, 235, 39, 175, 107, 150, 7, 73, 13, 214, 154, 10, 130, 51, 212, 39, 136, 36, 163, 93, 82, 168, 64, 101, 117, 192, 21, 8, 219, 235, 104, 41, 28, 195, 175, 225, 106, 201, 142, 196, 237, 8, 180, 208, 84, 150, 136, 241, 143, 138, 249, 198, 150, 26, 127, 47, 193, 133, 212, 216, 184, 33, 132, 11, 71, 227, 188, 139, 166, 235, 248, 175, 125, 124, 213, 17, 99, 225, 86, 163, 72, 58, 75, 181, 184, 151, 76, 210, 123, 80, 154, 219, 77, 218, 218, 190, 234, 99, 49, 160, 218, 67, 41, 216, 189, 6, 165, 119, 71, 92, 134, 139, 203, 245, 84, 52, 244, 99, 221, 76, 252, 178, 35, 90, 133, 107, 147, 215, 6, 204, 56, 161, 228, 41, 32, 162, 0, 34, 191, 146, 92, 190, 117, 60, 164, 45, 232, 92, 234, 66, 224, 47, 50, 85, 249, 197, 75, 83, 140, 115, 82, 68, 11, 132, 70, 135, 118, 242, 111, 163, 191, 213, 63, 227, 71, 129, 84, 219, 158, 18, 32, 128, 97, 103, 30, 147, 189, 125, 18, 180, 207, 22, 185, 153, 244, 21, 167, 231, 152, 203, 132, 193, 58, 59, 84, 143, 194, 194, 208, 43, 64, 65, 123, 135, 214, 253, 86, 107, 31, 218, 78, 225, 73, 79, 218, 15, 73, 57, 79, 25, 36, 69, 143, 106, 96, 198, 67, 68, 145, 74, 0, 250, 31, 113, 202, 13, 212, 254, 132, 189, 204, 179, 249, 251, 20, 255, 135, 3, 50, 188, 174, 93, 5, 169, 53, 235, 67, 139, 119, 37, 94, 167, 65, 25, 3, 105, 59, 201, 48, 46, 229, 216, 253, 164, 15, 159, 235, 4, 70, 93, 124, 30, 213, 41, 64, 181, 235, 228, 1, 209, 158, 236, 114, 144, 228, 111, 82, 27, 228, 185, 150, 94, 202, 90, 116, 66, 18, 156, 148, 154, 211, 211, 210, 86, 251, 118, 123, 59, 127, 148, 62, 45, 44, 238, 211, 11, 173, 225, 166, 75, 59, 2, 197, 208, 26, 93, 7, 105, 134, 199, 89, 41, 156, 254, 13, 139, 249, 162, 184, 153, 144, 104, 182, 45, 215, 45, 7, 70, 84, 10, 248, 48, 158, 79, 4, 43, 206, 244, 22, 134, 118, 213, 219, 132, 68, 56, 132, 186, 207, 224, 213, 209, 17, 232, 218, 65, 28, 131, 249, 126, 7, 200, 247, 215, 205, 210, 143, 70, 116, 0, 129, 114, 237, 69, 213, 116, 32, 81, 131, 140, 29, 57, 190, 87, 202, 232, 181, 27, 185, 205, 179, 171, 230, 149, 100, 48, 107, 200, 244, 70, 174, 70, 241, 16, 126, 97, 158, 54, 189, 17, 94, 208, 170, 26, 105, 8, 215, 21, 141, 172, 49, 58, 210, 240, 134, 182, 92, 127, 128, 145, 170, 45, 176, 228, 166, 97, 189, 69, 132, 39, 88, 141, 183, 191, 225, 122, 11, 30, 39, 198, 255, 54, 75, 150, 239, 229, 111, 46, 252, 103, 249, 91, 101, 98, 196, 12, 9, 39, 196, 1, 69, 25, 100, 68, 246, 190, 79, 134, 28, 96, 254, 34, 153, 173, 147, 181, 118, 166, 158, 142, 207, 218, 39, 91, 229, 118, 201, 20, 165, 135, 175, 194, 66, 234, 8, 23, 9, 43, 39, 225, 15, 64, 115, 158, 16, 186, 216, 229, 139, 16, 4, 143, 229, 3, 60, 144, 68, 195, 176, 212, 111, 83, 32, 16, 64, 219, 158, 7, 232, 190, 190, 119, 231, 13, 243, 185, 178, 86, 172, 115, 0, 55, 4, 121, 137, 190, 209, 34, 5, 86, 254, 84, 100, 179, 188, 24, 183, 196]))
                                        )
                                    ])
                                )
                            ])
                        )
                    ])
                )
            ])
        )
    ])
)
*/

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum VersionScheme {
    MultiPartNumeric = 1,
    MultiPartNumericSuffix = 2,
    AlphaNumeric = 3,
    Decimal = 4,
    Semver = 16384,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
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

#[derive(Debug)]
pub struct GlobalAttributes {
    any_attribute: HashMap<u32, ciborium::value::Value>,
}

impl GlobalAttributes {
    fn new() -> Self {
        GlobalAttributes {
            //lang: None,
            any_attribute: HashMap::new(),
        }
    }

    pub fn get_value(&self, key: GlobalAttributesKey) -> Option<&ciborium::value::Value> {
        unsafe {
            self.get_raw_value(key.into())
        }
    }

    pub unsafe fn get_raw_value(&self, key: u32) -> Option<&ciborium::value::Value> {
        return self.any_attribute.get(&key)
    }

    fn handles_key(&self, key: u32) -> bool {
        true
    }

    fn fill_value(&mut self, key: u32, value: ciborium::value::Value) {
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

// TODO
pub type HashEntry = String;

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

#[derive(Debug, Deserialize_CBOR_Map)]
pub struct EntityEntry {
    #[cbor_map_id(31)]
    entity_name: Text,
    #[cbor_map_id(32)]
    reg_id: Option<AnyURI>,
    #[cbor_map_id(33)]
    role: OneOrMany<EntityRole>,
    #[cbor_map_id(34)]
    thumbprint: Option<HashEntry>,
    #[cbor_map_unknown]
    global_attributes: GlobalAttributes,
    // * $$ entity-extension
}

#[derive(Debug, Deserialize_CBOR_Map)]
pub struct SoftwareMetaEntry {
    #[cbor_map_id(43)]
    activation_status: Option<Text>,
    #[cbor_map_id(44)]
    channel_type: Option<Text>,
    #[cbor_map_id(45)]
    colloquial_version: Option<Text>,
    #[cbor_map_id(46)]
    description: Option<Text>,
    #[cbor_map_id(47)]
    edition: Option<Text>,
    #[cbor_map_id(48)]
    entitlement_data_required: Option<bool>,
    #[cbor_map_id(49)]
    entitlement_key: Option<Text>,
    #[cbor_map_id(50)]
    generator: Option<Text>,
    #[cbor_map_id(51)]
    persistent_id: Option<Text>,
    #[cbor_map_id(52)]
    product: Option<Text>,
    #[cbor_map_id(53)]
    product_family: Option<Text>,
    #[cbor_map_id(54)]
    revision: Option<Text>,
    #[cbor_map_id(55)]
    summary: Option<Text>,
    #[cbor_map_id(56)]
    unspsc_code: Option<Text>,
    #[cbor_map_id(57)]
    unspsc_version: Option<Text>,
    #[cbor_map_unknown]
    global_attributes: GlobalAttributes,
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

#[derive(Debug, Deserialize_CBOR_Map)]
pub struct LinkEntry {
    #[cbor_map_id(37)]
    artifact: Option<Text>,
    #[cbor_map_id(38)]
    href: AnyURI,
    #[cbor_map_id(10)]
    media: Option<Text>,
    #[cbor_map_id(39)]
    ownership: Option<LinkOwnership>,
    #[cbor_map_id(40)]
    rel: LinkRel,
    #[cbor_map_id(41)]
    media_type: Option<Text>,
    #[cbor_map_id(42)]
    link_use: Option<LinkUse>,
}

#[derive(Debug)]
pub struct ResourceCollection {

}

#[derive(Debug, Deserialize_CBOR_Map)]
pub struct PayloadEntry {
    #[cbor_map_fields]
    resource_collection: ResourceCollection,
    #[cbor_map_unknown]
    global_attributes: GlobalAttributes,
}

// TODO
type Time = String;

#[derive(Debug, Deserialize_CBOR_Map)]
pub struct EvidenceEntry {
    #[cbor_map_fields]
    resource_collection: ResourceCollection,
    #[cbor_map_id(35)]
    date: Time,
    #[cbor_map_id(36)]
    device_id: Text,
    #[cbor_map_unknown]
    global_attributes: GlobalAttributes,
}

#[derive(Debug, Deserialize_CBOR_Map)]
pub struct CoSWIDTag {
    #[cbor_map_id(0)]
    tag_id: String,
    #[cbor_map_id(12)]
    tag_version: i32,
    #[cbor_map_id(8)]
    corpus: Option<bool>,
    #[cbor_map_id(9)]
    patch: Option<bool>,
    #[cbor_map_id(11)]
    supplemental: Option<bool>,
    #[cbor_map_id(1)]
    software_name: Text,
    #[cbor_map_id(13)]
    software_version: Option<Text>,
    #[cbor_map_id(14)]
    version_scheme: Option<VersionScheme>,
    #[cbor_map_id(10)]
    media: Option<Text>,
    #[cbor_map_id(5)]
    software_meta: Option<OneOrMany<SoftwareMetaEntry>>,
    #[cbor_map_id(2)]
    entity: OneOrMany<EntityEntry>,
    #[cbor_map_id(4)]
    link: Option<OneOrMany<LinkEntry>>,
    #[cbor_map_id(6)]
    payload: Option<PayloadEntry>,
    #[cbor_map_id(3)]
    evidence: Option<EvidenceEntry>,
    #[cbor_map_unknown]
    global_attributes: GlobalAttributes,
    // coswid-extension
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
