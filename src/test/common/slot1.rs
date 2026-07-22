use crate::types::{
    CollectionProperties, FArrayProperty, FByteProperty, FCustomVersion, FCustomVersionContainer,
    FDateTime, FDoubleProperty, FEngineVersion, FFloatProperty, FGuid, FInt8Property,
    FInt16Property, FInt64Property, FIntProperty, FPackageFileVersion, FProperty, FPropertyTag,
    FSaveGameHeader, FStrProperty, FString, FStructProperty, FUInt16Property, FUInt32Property,
    FUInt64Property, PropertyTag, PropertyTagIncompleteGuid, TArray, TaggedProperties,
    TaggedProperty, USaveGame,
};

#[allow(clippy::approx_constant)]
pub(crate) fn expected() -> USaveGame {
    USaveGame {
        header: FSaveGameHeader {
            package_file_version: FPackageFileVersion::UE4 { file_version: 522 },
            engine_version: FEngineVersion {
                major: 4,
                minor: 27,
                patch: 2,
                change_list: 18319896,
                branch: FString::from("++UE4+Release-4.27"),
            },
            custom_versions: Some(FCustomVersionContainer {
                custom_versions: TArray::from([
                    FCustomVersion {
                        key: FGuid::from_u32(0x9C54D522, 0xA8264FBE, 0x94210746, 0x61B482D0),
                        value: 43,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xB0D832E4, 0x1F894F0D, 0xACCF7EB7, 0x36FD4AA2),
                        value: 10,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xE1C64328, 0xA22C4D53, 0xA36C8E86, 0x6417BD8C),
                        value: 0,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x375EC13C, 0x06E448FB, 0xB50084F0, 0x262A717E),
                        value: 4,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xE4B068ED, 0xF49442E9, 0xA231DA0B, 0x2E46BB41),
                        value: 40,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xCFFC743F, 0x43B04480, 0x939114DF, 0x171D2073),
                        value: 37,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xB02B49B5, 0xBB2044E9, 0xA30432B7, 0x52E40360),
                        value: 3,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xA4E4105C, 0x59A149B5, 0xA7C540C4, 0x547EDFEE),
                        value: 0,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x39C831C9, 0x5AE647DC, 0x9A449C17, 0x3E1C8E7C),
                        value: 0,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x78F01B33, 0xEBEA4F98, 0xB9B484EA, 0xCCB95AA2),
                        value: 14,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x6631380F, 0x2D4D43E0, 0x8009CF27, 0x6956A95A),
                        value: 0,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x12F88B9F, 0x88754AFC, 0xA67CD90C, 0x383ABD29),
                        value: 45,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x7B5AE74C, 0xD2704C10, 0xA9585798, 0x0B212A5A),
                        value: 13,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xD7296918, 0x1DD64BDD, 0x9DE264A8, 0x3CC13884),
                        value: 3,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xC2A15278, 0xBFE74AFE, 0x6C1790FF, 0x531DF755),
                        value: 1,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x6EACA3D4, 0x40EC4CC1, 0xB7868BED, 0x09428FC5),
                        value: 3,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x29E575DD, 0xE0A34627, 0x9D10D276, 0x232CDCEA),
                        value: 17,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xAF43A65D, 0x7FD34947, 0x98733E8E, 0xD9C1BB05),
                        value: 15,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x6B266CEC, 0x1EC74B8F, 0xA30BE4D9, 0x0942FC07),
                        value: 1,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x0DF73D61, 0xA23F47EA, 0xB72789E9, 0x0C41499A),
                        value: 1,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x601D1886, 0xAC644F84, 0xAA16D3DE, 0x0DEAC7D6),
                        value: 47,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xE7086368, 0x6B234C58, 0x84391B70, 0x16265E91),
                        value: 1,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x9DFFBCD6, 0x494F0158, 0xE2211282, 0x3C92A888),
                        value: 10,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xF2AED0AC, 0x9AFE416F, 0x8664AA7F, 0xFA26D6FC),
                        value: 1,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x174F1F0B, 0xB4C645A5, 0xB13F2EE8, 0xD0FB917D),
                        value: 10,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x35F94A83, 0xE258406C, 0xA31809F5, 0x9610247C),
                        value: 41,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xB68FC16E, 0x8B1B42E2, 0xB453215C, 0x058844FE),
                        value: 1,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xB2E18506, 0x4273CFC2, 0xA54EF4BB, 0x758BBA07),
                        value: 1,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x64F58936, 0xFD1B42BA, 0xBA967289, 0xD5D0FA4E),
                        value: 1,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x6F0ED827, 0xA6094895, 0x9C91998D, 0x90180EA4),
                        value: 2,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x717F9EE7, 0xE9B0493A, 0x88B39132, 0x1B388107),
                        value: 8,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x54683250, 0x809948AF, 0x8BC89896, 0xFBADF9B7),
                        value: 0,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x8E7DDCB3, 0x80DA47BB, 0x9FD346A2, 0x93984DF6),
                        value: 1,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xCB8AB0CD, 0xE78C4BDE, 0xA8621393, 0x14E9EF62),
                        value: 0,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xAB965196, 0x45D808FC, 0xB7D7228D, 0x78AD569E),
                        value: 1,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x9950B70E, 0xB41A4E17, 0xBBCCFA0D, 0x57817FD6),
                        value: 1,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xFB680AF2, 0x59EF4BA3, 0xBAA819B5, 0x73C8443D),
                        value: 2,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xAFE08691, 0x3A0D4952, 0xB673673B, 0x7CF22D1E),
                        value: 2,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x2EB5FDBD, 0x01AC4D10, 0x8136F38F, 0x3393A5DA),
                        value: 5,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x509D354F, 0xF6E6492F, 0xA74985B2, 0x073C631C),
                        value: 0,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xA462B7EA, 0xF4994E3A, 0x99C1EC1F, 0x8224E1B2),
                        value: 4,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x430C4D19, 0x71544970, 0x87699B69, 0xDF90B0E5),
                        value: 15,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xAAFE32BD, 0x53954C14, 0xB66A5E25, 0x1032D1DD),
                        value: 1,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x23AFE18E, 0x4CE14E58, 0x8D61C252, 0xB953BEB7),
                        value: 11,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x4A56EB40, 0x10F511DC, 0x92D3347E, 0xB2C96AE7),
                        value: 2,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xD78A4A00, 0xE8584697, 0xBAA819B5, 0x487D46B4),
                        value: 18,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x5579F886, 0x933A4C1F, 0x83BA087B, 0x6361B92F),
                        value: 2,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x612FBE52, 0xDA53400B, 0x910D4F91, 0x9FB1857C),
                        value: 1,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xA4237A36, 0xCAEA41C9, 0x8FA218F8, 0x58681BF3),
                        value: 4,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x804E3F75, 0x70884B49, 0xA4D68C06, 0x3C7EB6DC),
                        value: 5,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x76A52329, 0x092345B5, 0x98AED841, 0xCF2F6AD8),
                        value: 5,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x5FBC6907, 0x55C840AE, 0x8E67F184, 0x5EFFF13F),
                        value: 1,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xFCF57AFA, 0x50764283, 0xB9A9E658, 0xFFA02D32),
                        value: 68,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x24BB7AF3, 0x56464F83, 0x1F2F2DC2, 0x49AD96FF),
                        value: 5,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x11310AED, 0x2E554D61, 0xAF679AA3, 0xC5A1082C),
                        value: 17,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x82E77C4E, 0x332343A5, 0xB46B13C5, 0x97310DF3),
                        value: 0,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xFB26E412, 0x1F154B4D, 0x9372550A, 0x961D2F70),
                        value: 3,
                    },
                ]),
            }),
            save_game_class_name: FString::from("/Script/UE4SaveFile.TestSaveGame"),
        },
        properties: TaggedProperties::from([
            TaggedProperty {
                property_name: FString::from("u8_test"),
                array_index: 0,
                has_binary_or_native_serialize: false,
                has_property_extensions: false,
                property: FProperty::from(FByteProperty::Byte(129)),
                property_guid: FGuid::default(),
            },
            TaggedProperty {
                property_name: FString::from("i8_test"),
                array_index: 0,
                has_binary_or_native_serialize: false,
                has_property_extensions: false,
                property: FProperty::from(FInt8Property::from(-123)),
                property_guid: FGuid::default(),
            },
            TaggedProperty {
                property_name: FString::from("ushort_test"),
                array_index: 0,
                has_binary_or_native_serialize: false,
                has_property_extensions: false,
                property: FProperty::from(FUInt16Property::from(65530)),
                property_guid: FGuid::default(),
            },
            TaggedProperty {
                property_name: FString::from("short_test"),
                array_index: 0,
                has_binary_or_native_serialize: false,
                has_property_extensions: false,
                property: FProperty::from(FInt16Property::from(-32764)),
                property_guid: FGuid::default(),
            },
            TaggedProperty {
                property_name: FString::from("uint32_test"),
                array_index: 0,
                has_binary_or_native_serialize: false,
                has_property_extensions: false,
                property: FProperty::from(FUInt32Property::from(4294967294)),
                property_guid: FGuid::default(),
            },
            TaggedProperty {
                property_name: FString::from("int32_test"),
                array_index: 0,
                has_binary_or_native_serialize: false,
                has_property_extensions: false,
                property: FProperty::from(FIntProperty::from(-2147483647)),
                property_guid: FGuid::default(),
            },
            TaggedProperty {
                property_name: FString::from("ulong_test"),
                array_index: 0,
                has_binary_or_native_serialize: false,
                has_property_extensions: false,
                property: FProperty::from(FUInt64Property::from(18446744073709551614)),
                property_guid: FGuid::default(),
            },
            TaggedProperty {
                property_name: FString::from("long_test"),
                array_index: 0,
                has_binary_or_native_serialize: false,
                has_property_extensions: false,
                property: FProperty::from(FInt64Property::from(-9223372036854775807)),
                property_guid: FGuid::default(),
            },
            TaggedProperty {
                property_name: FString::from("f_property"),
                array_index: 0,
                has_binary_or_native_serialize: false,
                has_property_extensions: false,
                property: FProperty::from(FFloatProperty::from(3.14159)),
                property_guid: FGuid::default(),
            },
            TaggedProperty {
                property_name: FString::from("d_property"),
                array_index: 0,
                has_binary_or_native_serialize: false,
                has_property_extensions: false,
                property: FProperty::from(FDoubleProperty::from(3.14159265358979)),
                property_guid: FGuid::default(),
            },
            TaggedProperty {
                property_name: FString::from("str_property"),
                array_index: 0,
                has_binary_or_native_serialize: false,
                has_property_extensions: false,
                property: FProperty::from(FStrProperty::from(FString::from("Hello world"))),
                property_guid: FGuid::default(),
            },
            TaggedProperty {
                property_name: FString::from("struct_property"),
                array_index: 0,
                has_binary_or_native_serialize: false,
                has_property_extensions: false,
                property: FProperty::from(FStructProperty::Custom {
                    struct_type: FString::from("CustomStruct"),
                    class_name: FString(None),
                    struct_guid: FGuid::default(),
                    properties: TaggedProperties::from([TaggedProperty {
                        property_name: FString::from("test_field"),
                        array_index: 0,
                        has_binary_or_native_serialize: false,
                        has_property_extensions: false,
                        property: FProperty::from(FUInt64Property::from(12345)),
                        property_guid: FGuid::default(),
                    }]),
                }),
                property_guid: FGuid::default(),
            },
            TaggedProperty {
                property_name: FString::from("date_time_property"),
                array_index: 0,
                has_binary_or_native_serialize: false,
                has_property_extensions: false,
                property: FProperty::from(FStructProperty::DateTime(FDateTime {
                    ticks: 637864237380020000,
                })),
                property_guid: FGuid::default(),
            },
            TaggedProperty {
                property_name: FString::from("array_of_structs"),
                array_index: 0,
                has_binary_or_native_serialize: false,
                has_property_extensions: false,
                property: FProperty::from(FArrayProperty::TaggedStructs {
                    struct_tag: FPropertyTag::Some {
                        name: FString::from("array_of_structs"),
                        property_tag: PropertyTag::Incomplete {
                            property_type: FString::from("StructProperty"),
                            size: 120,
                            array_index: 0,
                            extra: CollectionProperties::Struct {
                                type_name: FString::from("CustomStruct"),
                                struct_guid: FGuid::default(),
                            },
                            maybe_property_guid: PropertyTagIncompleteGuid::default(),
                        },
                    },
                    values: vec![
                        FStructProperty::Custom {
                            struct_type: FString::from("CustomStruct"),
                            class_name: FString(None),
                            struct_guid: FGuid::default(),
                            properties: TaggedProperties::from([TaggedProperty {
                                property_name: FString::from("test_field"),
                                array_index: 0,
                                has_binary_or_native_serialize: false,
                                has_property_extensions: false,
                                property: FProperty::from(FUInt64Property::from(10)),
                                property_guid: FGuid::default(),
                            }]),
                        },
                        FStructProperty::Custom {
                            struct_type: FString::from("CustomStruct"),
                            class_name: FString(None),
                            struct_guid: FGuid::default(),
                            properties: TaggedProperties::from([TaggedProperty {
                                property_name: FString::from("test_field"),
                                array_index: 0,
                                has_binary_or_native_serialize: false,
                                has_property_extensions: false,
                                property: FProperty::from(FUInt64Property::from(10)),
                                property_guid: FGuid::default(),
                            }]),
                        },
                    ],
                }),
                property_guid: FGuid::default(),
            },
            TaggedProperty {
                property_name: FString::from("array_of_ints"),
                array_index: 0,
                has_binary_or_native_serialize: false,
                has_property_extensions: false,
                property: FProperty::from(FArrayProperty::Ints {
                    ints: TArray::from([
                        FIntProperty::from(12),
                        FIntProperty::from(12),
                        FIntProperty::from(12),
                        FIntProperty::from(12),
                        FIntProperty::from(12),
                    ]),
                }),
                property_guid: FGuid::default(),
            },
            TaggedProperty {
                property_name: FString::from("array_of_strings"),
                array_index: 0,
                has_binary_or_native_serialize: false,
                has_property_extensions: false,
                property: FProperty::from(FArrayProperty::Strs {
                    strings: TArray::from([
                        FStrProperty::from(FString::from("Hello world from array")),
                        FStrProperty::from(FString::from("Hello world from array")),
                        FStrProperty::from(FString::from("Hello world from array")),
                    ]),
                }),
                property_guid: FGuid::default(),
            },
        ]),
    }
}

pub const SLOT1_JSON: &str = r#"{
  "header": {
    "file_version": 522,
    "engine_version": {
      "major": 4,
      "minor": 27,
      "patch": 2,
      "change_list": 18319896,
      "branch": "++UE4+Release-4.27"
    },
    "custom_versions": {
      "9c54d522-a826-4fbe-9421-074661b482d0": 43,
      "b0d832e4-1f89-4f0d-accf-7eb736fd4aa2": 10,
      "e1c64328-a22c-4d53-a36c-8e866417bd8c": 0,
      "375ec13c-06e4-48fb-b500-84f0262a717e": 4,
      "e4b068ed-f494-42e9-a231-da0b2e46bb41": 40,
      "cffc743f-43b0-4480-9391-14df171d2073": 37,
      "b02b49b5-bb20-44e9-a304-32b752e40360": 3,
      "a4e4105c-59a1-49b5-a7c5-40c4547edfee": 0,
      "39c831c9-5ae6-47dc-9a44-9c173e1c8e7c": 0,
      "78f01b33-ebea-4f98-b9b4-84eaccb95aa2": 14,
      "6631380f-2d4d-43e0-8009-cf276956a95a": 0,
      "12f88b9f-8875-4afc-a67c-d90c383abd29": 45,
      "7b5ae74c-d270-4c10-a958-57980b212a5a": 13,
      "d7296918-1dd6-4bdd-9de2-64a83cc13884": 3,
      "c2a15278-bfe7-4afe-6c17-90ff531df755": 1,
      "6eaca3d4-40ec-4cc1-b786-8bed09428fc5": 3,
      "29e575dd-e0a3-4627-9d10-d276232cdcea": 17,
      "af43a65d-7fd3-4947-9873-3e8ed9c1bb05": 15,
      "6b266cec-1ec7-4b8f-a30b-e4d90942fc07": 1,
      "0df73d61-a23f-47ea-b727-89e90c41499a": 1,
      "601d1886-ac64-4f84-aa16-d3de0deac7d6": 47,
      "e7086368-6b23-4c58-8439-1b7016265e91": 1,
      "9dffbcd6-494f-0158-e221-12823c92a888": 10,
      "f2aed0ac-9afe-416f-8664-aa7ffa26d6fc": 1,
      "174f1f0b-b4c6-45a5-b13f-2ee8d0fb917d": 10,
      "35f94a83-e258-406c-a318-09f59610247c": 41,
      "b68fc16e-8b1b-42e2-b453-215c058844fe": 1,
      "b2e18506-4273-cfc2-a54e-f4bb758bba07": 1,
      "64f58936-fd1b-42ba-ba96-7289d5d0fa4e": 1,
      "6f0ed827-a609-4895-9c91-998d90180ea4": 2,
      "717f9ee7-e9b0-493a-88b3-91321b388107": 8,
      "54683250-8099-48af-8bc8-9896fbadf9b7": 0,
      "8e7ddcb3-80da-47bb-9fd3-46a293984df6": 1,
      "cb8ab0cd-e78c-4bde-a862-139314e9ef62": 0,
      "ab965196-45d8-08fc-b7d7-228d78ad569e": 1,
      "9950b70e-b41a-4e17-bbcc-fa0d57817fd6": 1,
      "fb680af2-59ef-4ba3-baa8-19b573c8443d": 2,
      "afe08691-3a0d-4952-b673-673b7cf22d1e": 2,
      "2eb5fdbd-01ac-4d10-8136-f38f3393a5da": 5,
      "509d354f-f6e6-492f-a749-85b2073c631c": 0,
      "a462b7ea-f499-4e3a-99c1-ec1f8224e1b2": 4,
      "430c4d19-7154-4970-8769-9b69df90b0e5": 15,
      "aafe32bd-5395-4c14-b66a-5e251032d1dd": 1,
      "23afe18e-4ce1-4e58-8d61-c252b953beb7": 11,
      "4a56eb40-10f5-11dc-92d3-347eb2c96ae7": 2,
      "d78a4a00-e858-4697-baa8-19b5487d46b4": 18,
      "5579f886-933a-4c1f-83ba-087b6361b92f": 2,
      "612fbe52-da53-400b-910d-4f919fb1857c": 1,
      "a4237a36-caea-41c9-8fa2-18f858681bf3": 4,
      "804e3f75-7088-4b49-a4d6-8c063c7eb6dc": 5,
      "76a52329-0923-45b5-98ae-d841cf2f6ad8": 5,
      "5fbc6907-55c8-40ae-8e67-f1845efff13f": 1,
      "fcf57afa-5076-4283-b9a9-e658ffa02d32": 68,
      "24bb7af3-5646-4f83-1f2f-2dc249ad96ff": 5,
      "11310aed-2e55-4d61-af67-9aa3c5a1082c": 17,
      "82e77c4e-3323-43a5-b46b-13c597310df3": 0,
      "fb26e412-1f15-4b4d-9372-550a961d2f70": 3
    },
    "save_game_class_name": "/Script/UE4SaveFile.TestSaveGame"
  },
  "properties": {
    "u8_test": {
      "type": "ByteProperty",
      "Byte": 129
    },
    "i8_test": {
      "type": "Int8Property",
      "value": -123
    },
    "ushort_test": {
      "type": "UInt16Property",
      "value": 65530
    },
    "short_test": {
      "type": "Int16Property",
      "value": -32764
    },
    "uint32_test": {
      "type": "UInt32Property",
      "value": 4294967294
    },
    "int32_test": {
      "type": "IntProperty",
      "value": -2147483647
    },
    "ulong_test": {
      "type": "UInt64Property",
      "value": 18446744073709551614
    },
    "long_test": {
      "type": "Int64Property",
      "value": -9223372036854775807
    },
    "f_property": {
      "type": "FloatProperty",
      "value": 3.14159
    },
    "d_property": {
      "type": "DoubleProperty",
      "value": 3.14159265358979
    },
    "str_property": {
      "type": "StrProperty",
      "value": "Hello world"
    },
    "struct_property": {
      "type": "StructProperty",
      "Custom": {
        "struct_type": "CustomStruct",
        "properties": {
          "test_field": {
            "type": "UInt64Property",
            "value": 12345
          }
        }
      }
    },
    "date_time_property": {
      "type": "StructProperty",
      "DateTime": "2022-04-24 19:02:18.002 UTC"
    },
    "array_of_structs": {
      "type": "ArrayProperty",
      "field_name": "array_of_structs",
      "type_name": "CustomStruct",
      "structs": [
        {
          "Custom": {
            "struct_type": "CustomStruct",
            "properties": {
              "test_field": {
                "type": "UInt64Property",
                "value": 10
              }
            }
          }
        },
        {
          "Custom": {
            "struct_type": "CustomStruct",
            "properties": {
              "test_field": {
                "type": "UInt64Property",
                "value": 10
              }
            }
          }
        }
      ]
    },
    "array_of_ints": {
      "type": "ArrayProperty",
      "ints": [
        12,
        12,
        12,
        12,
        12
      ]
    },
    "array_of_strings": {
      "type": "ArrayProperty",
      "strings": [
        "Hello world from array",
        "Hello world from array",
        "Hello world from array"
      ]
    }
  }
}"#;
