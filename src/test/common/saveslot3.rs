use std::collections::HashMap;

use crate::types::{
    CollectionProperties, FCustomVersion, FCustomVersionContainer, FDateTime, FEngineVersion,
    FFloatProperty, FGuid, FIntProperty,
    FMapProperty::Known,
    FNameProperty, FObjectProperty,
    FPackageFileVersion::UE4,
    FProperty::{Float, Int, Map, Name, Object, Str, Struct, Unknown},
    FSaveGameHeader, FStrProperty, FString,
    FStructProperty::{Custom, DateTime},
    MapEntry,
    PropertyTag::Incomplete,
    SaveGameFileVersion, TArray, TaggedProperties, TaggedProperty, USaveGame,
};

pub(crate) fn hints() -> HashMap<String, String> {
    HashMap::from([
        (
            "MinersManualKnownObjects.SetProperty.StructProperty".to_string(),
            "Struct".to_string(),
        ),
        (
            "GameplayDatabase.MapProperty.Value.StructProperty".to_string(),
            "Struct".to_string(),
        ),
        (
            "PlayerAttributes.MapProperty.Key.StructProperty".to_string(),
            "Struct".to_string(),
        ),
    ])
}

pub(crate) fn expected() -> USaveGame {
    USaveGame {
        header: FSaveGameHeader {
            save_game_file_version: SaveGameFileVersion::AddedCustomVersions as u32,
            package_file_version: UE4 {
                package_file_version: 522,
            },
            engine_version: FEngineVersion {
                major: 4,
                minor: 27,
                patch: 2,
                change_list: 18319896,
                branch: FString::from("++UE4+Release-4.27"),
            },
            custom_versions: Some(FCustomVersionContainer {
                // custom_version_format: 3,
                custom_versions: TArray::from([
                    FCustomVersion {
                        key: FGuid::from_u32(0xFCF57AFA, 0x50764283, 0xB9A9E658, 0xFFA02D32),
                        value: 68,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xFB26E412, 0x1F154B4D, 0x9372550A, 0x961D2F70),
                        value: 3,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xA7820CFB, 0x20A74359, 0x8C542C14, 0x9623CF50),
                        value: 6,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x82E77C4E, 0x332343A5, 0xB46B13C5, 0x97310DF3),
                        value: 0,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x11310AED, 0x2E554D61, 0xAF679AA3, 0xC5A1082C),
                        value: 17,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x24BB7AF3, 0x56464F83, 0x1F2F2DC2, 0x49AD96FF),
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
                        key: FGuid::from_u32(0xA462B7EA, 0xF4994E3A, 0x99C1EC1F, 0x8224E1B2),
                        value: 4,
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
                        key: FGuid::from_u32(0xFB680AF2, 0x59EF4BA3, 0xBAA819B5, 0x73C8443D),
                        value: 2,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x9950B70E, 0xB41A4E17, 0xBBCCFA0D, 0x57817FD6),
                        value: 1,
                    },
                ]),
            }),
            save_game_class_name: FString::from("/Script/CD.CDSave_GameState"),
        },
        properties: TaggedProperties::from([
            (
                FString::from("LastSaveTime"),
                TaggedProperty {
                    array_index: 0,
                    extensions: false,
                    guid: FGuid::default(),
                    native: false,
                    property: Struct(DateTime(FDateTime {
                        ticks: 638160761644140000,
                    })),
                },
            ),
            (
                FString::from("PlayerClass"),
                TaggedProperty {
                    array_index: 0,
                    extensions: false,
                    guid: FGuid::default(),
                    native: false,
                    property: Object(FObjectProperty(FString::from(
                        "/Game/Character/Player/Blueprints/BP_Soldier.BP_Soldier_C",
                    ))),
                },
            ),
            (
                FString::from("Version"),
                TaggedProperty {
                    array_index: 0,
                    extensions: false,
                    guid: FGuid::default(),
                    native: false,
                    property: Int(FIntProperty(3)),
                },
            ),
            (
                FString::from("GameplayDatabase"),
                TaggedProperty {
                    array_index: 0,
                    extensions: false,
                    guid: FGuid::default(),
                    native: false,
                    property: Map(Known {
                        allocation_flags: 0,
                        key_type: Incomplete {
                            property_type: FString::from("NameProperty"),
                            size: 0,
                            array_index: 0,
                            extra: CollectionProperties::None,
                            guid: FGuid::default(),
                        },
                        value_type: Incomplete {
                            property_type: FString::from("StructProperty"),
                            size: 0,
                            array_index: 0,
                            extra: CollectionProperties::None,
                            guid: FGuid::default(),
                        },
                        properties: TArray::from([
                            MapEntry {
                                key: Name(FNameProperty(FString::from(
                                    "unlock.welcomescreen.seen",
                                ))),
                                value: Struct(Custom(
                                    FString::from(""),
                                    FString(None),
                                    FGuid::default(),
                                    TaggedProperties::from([
                                        (
                                            FString::from("AsFloat"),
                                            TaggedProperty {
                                                array_index: 0,
                                                extensions: false,
                                                guid: FGuid::default(),
                                                native: false,
                                                property: Float(FFloatProperty(0.0)),
                                            },
                                        ),
                                        (
                                            FString::from("AsString"),
                                            TaggedProperty {
                                                array_index: 0,
                                                extensions: false,
                                                guid: FGuid::default(),
                                                native: false,
                                                property: Str(FStrProperty(FString(None))),
                                            },
                                        ),
                                    ]),
                                )),
                            },
                            MapEntry {
                                key: Name(FNameProperty(FString::from("game.tutorial.finished"))),
                                value: Struct(Custom(
                                    FString::from(""),
                                    FString(None),
                                    FGuid::default(),
                                    TaggedProperties::from([
                                        (
                                            FString::from("AsFloat"),
                                            TaggedProperty {
                                                array_index: 0,
                                                extensions: false,
                                                guid: FGuid::default(),
                                                native: false,
                                                property: Float(FFloatProperty(1.0)),
                                            },
                                        ),
                                        (
                                            FString::from("AsString"),
                                            TaggedProperty {
                                                array_index: 0,
                                                extensions: false,
                                                guid: FGuid::default(),
                                                native: false,
                                                property: Str(FStrProperty(FString(None))),
                                            },
                                        ),
                                    ]),
                                )),
                            },
                            MapEntry {
                                key: Name(FNameProperty(FString::from("game.tutorial.skipped"))),
                                value: Struct(Custom(
                                    FString::from(""),
                                    FString(None),
                                    FGuid::default(),
                                    TaggedProperties::from([
                                        (
                                            FString::from("AsFloat"),
                                            TaggedProperty {
                                                array_index: 0,
                                                extensions: false,
                                                guid: FGuid::default(),
                                                native: false,
                                                property: Float(FFloatProperty(1.0)),
                                            },
                                        ),
                                        (
                                            FString::from("AsString"),
                                            TaggedProperty {
                                                array_index: 0,
                                                extensions: false,
                                                guid: FGuid::default(),
                                                native: false,
                                                property: Str(FStrProperty(FString(None))),
                                            },
                                        ),
                                    ]),
                                )),
                            },
                            MapEntry {
                                key: Name(FNameProperty(FString::from(
                                    "dialogs.messages.seen.Rumiko.0.50",
                                ))),
                                value: Struct(Custom(
                                    FString::from(""),
                                    FString(None),
                                    FGuid::default(),
                                    TaggedProperties::from([
                                        (
                                            FString::from("AsFloat"),
                                            TaggedProperty {
                                                array_index: 0,
                                                extensions: false,
                                                guid: FGuid::default(),
                                                native: false,
                                                property: Float(FFloatProperty(1.0)),
                                            },
                                        ),
                                        (
                                            FString::from("AsString"),
                                            TaggedProperty {
                                                array_index: 0,
                                                extensions: false,
                                                guid: FGuid::default(),
                                                native: false,
                                                property: Str(FStrProperty(FString(None))),
                                            },
                                        ),
                                    ]),
                                )),
                            },
                            MapEntry {
                                key: Name(FNameProperty(FString::from("codex.Rumiko"))),
                                value: Struct(Custom(
                                    FString::from(""),
                                    FString(None),
                                    FGuid::default(),
                                    TaggedProperties::from([
                                        (
                                            FString::from("AsFloat"),
                                            TaggedProperty {
                                                array_index: 0,
                                                extensions: false,
                                                guid: FGuid::default(),
                                                native: false,
                                                property: Float(FFloatProperty(1.0)),
                                            },
                                        ),
                                        (
                                            FString::from("AsString"),
                                            TaggedProperty {
                                                array_index: 0,
                                                extensions: false,
                                                guid: FGuid::default(),
                                                native: false,
                                                property: Str(FStrProperty(FString(None))),
                                            },
                                        ),
                                    ]),
                                )),
                            },
                        ]),
                    }),
                },
            ),
            (
                FString::from("PlayerAttributes"),
                TaggedProperty {
                    array_index: 0,
                    extensions: false,
                    guid: FGuid::default(),
                    native: false,
                    property: Map(Known {
                        allocation_flags: 0,
                        key_type: Incomplete {
                            property_type: FString::from("StructProperty"),
                            size: 0,
                            array_index: 0,
                            extra: CollectionProperties::None,
                            guid: FGuid::default(),
                        },
                        value_type: Incomplete {
                            property_type: FString::from("FloatProperty"),
                            size: 0,
                            array_index: 0,
                            extra: CollectionProperties::None,
                            guid: FGuid::default(),
                        },
                        properties: TArray::from([
                            MapEntry {
                                key: Struct(Custom(
                                    FString::from(""),
                                    FString(None),
                                    FGuid::default(),
                                    TaggedProperties::from([
                                        (
                                            FString::from("AttributeName"),
                                            TaggedProperty {
                                                array_index: 0,
                                                extensions: false,
                                                guid: FGuid::default(),
                                                native: false,
                                                property: Str(FStrProperty(FString::from(
                                                    "Currency_Blueprints",
                                                ))),
                                            },
                                        ),
                                        (
                                            FString::from("Attribute"),
                                            TaggedProperty {
                                                array_index: 0,
                                                extensions: false,
                                                guid: FGuid::default(),
                                                native: false,
                                                property: Unknown(
                                                    Incomplete {
                                                        property_type: FString::from(
                                                            "FieldPathProperty",
                                                        ),
                                                        size: 64,
                                                        array_index: 0,
                                                        extra: CollectionProperties::None,
                                                        guid: FGuid::default(),
                                                    },
                                                    vec![
                                                        1, 0, 0, 0, 20, 0, 0, 0, 67, 117, 114, 114,
                                                        101, 110, 99, 121, 95, 66, 108, 117, 101,
                                                        112, 114, 105, 110, 116, 115, 0, 32, 0, 0,
                                                        0, 47, 83, 99, 114, 105, 112, 116, 47, 67,
                                                        68, 46, 67, 68, 80, 108, 97, 121, 101, 114,
                                                        65, 116, 116, 114, 105, 98, 117, 116, 101,
                                                        83, 101, 116, 0,
                                                    ],
                                                ),
                                            },
                                        ),
                                        (
                                            FString::from("AttributeOwner"),
                                            TaggedProperty {
                                                array_index: 0,
                                                extensions: false,
                                                guid: FGuid::default(),
                                                native: false,
                                                property: Object(FObjectProperty(FString::from(
                                                    "None",
                                                ))),
                                            },
                                        ),
                                    ]),
                                )),
                                value: Float(FFloatProperty(0.0)),
                            },
                            MapEntry {
                                key: Struct(Custom(
                                    FString::from(""),
                                    FString(None),
                                    FGuid::default(),
                                    TaggedProperties::from([
                                        (
                                            FString::from("AttributeName"),
                                            TaggedProperty {
                                                array_index: 0,
                                                extensions: false,
                                                guid: FGuid::default(),
                                                native: false,
                                                property: Str(FStrProperty(FString::from(
                                                    "Currency_Electrum",
                                                ))),
                                            },
                                        ),
                                        (
                                            FString::from("Attribute"),
                                            TaggedProperty {
                                                array_index: 0,
                                                extensions: false,
                                                guid: FGuid::default(),
                                                native: false,
                                                property: Unknown(
                                                    Incomplete {
                                                        property_type: FString::from(
                                                            "FieldPathProperty",
                                                        ),
                                                        size: 62,
                                                        array_index: 0,
                                                        extra: CollectionProperties::None,
                                                        guid: FGuid::default(),
                                                    },
                                                    vec![
                                                        1, 0, 0, 0, 18, 0, 0, 0, 67, 117, 114, 114,
                                                        101, 110, 99, 121, 95, 69, 108, 101, 99,
                                                        116, 114, 117, 109, 0, 32, 0, 0, 0, 47, 83,
                                                        99, 114, 105, 112, 116, 47, 67, 68, 46, 67,
                                                        68, 80, 108, 97, 121, 101, 114, 65, 116,
                                                        116, 114, 105, 98, 117, 116, 101, 83, 101,
                                                        116, 0,
                                                    ],
                                                ),
                                            },
                                        ),
                                        (
                                            FString::from("AttributeOwner"),
                                            TaggedProperty {
                                                array_index: 0,
                                                extensions: false,
                                                guid: FGuid::default(),
                                                native: false,
                                                property: Object(FObjectProperty(FString::from(
                                                    "None",
                                                ))),
                                            },
                                        ),
                                    ]),
                                )),
                                value: Float(FFloatProperty(0.0)),
                            },
                        ]),
                    }),
                },
            ),
            (
                FString::from("SecondaryWeaponClass"),
                TaggedProperty {
                    array_index: 0,
                    extensions: false,
                    guid: FGuid::default(),
                    native: false,
                    property: Object(FObjectProperty(FString::from(
                        "/Game/Weapons/RocketLauncher/Blueprints/BP_RocketLauncher.BP_RocketLauncher_C",
                    ))),
                },
            ),
        ]),
    }
}

pub(crate) const SAVESLOT_03_JSON: &str = r#"{
  "header": {
    "type": "Version2",
    "package_file_version": 522,
    "engine_version": {
      "major": 4,
      "minor": 27,
      "patch": 2,
      "change_list": 18319896,
      "branch": "++UE4+Release-4.27"
    },
    "custom_version_format": 3,
    "custom_versions": {
      "FA7AF5FC-8342-7650-58E6-A9B9322DA0FF": 68,
      "12E426FB-4D4B-151F-0A55-7293702F1D96": 3,
      "FB0C82A7-5943-A720-142C-548C50CF2396": 6,
      "4E7CE782-A543-2333-C513-6BB4F30D3197": 0,
      "ED0A3111-614D-552E-A39A-67AF2C08A1C5": 17,
      "F37ABB24-834F-4656-C22D-2F1FFF96AD49": 5,
      "2923A576-B545-2309-41D8-AE98D86A2FCF": 5,
      "0769BC5F-AE40-C855-84F1-678E3FF1FF5E": 1,
      "22D5549C-BE4F-26A8-4607-2194D082B461": 43,
      "E432D8B0-0D4F-891F-B77E-CFACA24AFD36": 10,
      "2843C6E1-534D-2CA2-868E-6CA38CBD1764": 0,
      "3CC15E37-FB48-E406-F084-00B57E712A26": 4,
      "ED68B0E4-E942-94F4-0BDA-31A241BB462E": 40,
      "3F74FCCF-8044-B043-DF14-919373201D17": 37,
      "B5492BB0-E944-20BB-B732-04A36003E452": 3,
      "5C10E4A4-B549-A159-C440-C5A7EEDF7E54": 0,
      "C931C839-DC47-E65A-179C-449A7C8E1C3E": 0,
      "331BF078-984F-EAEB-EA84-B4B9A25AB9CC": 14,
      "0F383166-E043-4D2D-27CF-09805AA95669": 0,
      "9F8BF812-FC4A-7588-0CD9-7CA629BD3A38": 45,
      "4CE75A7B-104C-70D2-9857-58A95A2A210B": 13,
      "186929D7-DD4B-D61D-A864-E29D8438C13C": 3,
      "7852A1C2-FE4A-E7BF-FF90-176C55F71D53": 1,
      "D4A3AC6E-C14C-EC40-ED8B-86B7C58F4209": 3,
      "DD75E529-2746-A3E0-76D2-109DEADC2C23": 17,
      "5DA643AF-4749-D37F-8E3E-739805BBC1D9": 15,
      "EC6C266B-8F4B-C71E-D9E4-0BA307FC4209": 1,
      "613DF70D-EA47-3FA2-E989-27B79A49410C": 1,
      "86181D60-844F-64AC-DED3-16AAD6C7EA0D": 47,
      "686308E7-584C-236B-701B-3984915E2616": 1,
      "D6BCFF9D-5801-4F49-8212-21E288A8923C": 10,
      "ACD0AEF2-6F41-FE9A-7FAA-6486FCD626FA": 1,
      "0B1F4F17-A545-C6B4-E82E-3FB17D91FBD0": 10,
      "834AF935-6C40-58E2-F509-18A37C241096": 41,
      "6EC18FB6-E242-1B8B-5C21-53B4FE448805": 1,
      "0685E1B2-C2CF-7342-BBF4-4EA507BA8B75": 1,
      "3689F564-BA42-1BFD-8972-96BA4EFAD0D5": 1,
      "27D80E6F-9548-09A6-8D99-919CA40E1890": 2,
      "E79E7F71-3A49-B0E9-3291-B3880781381B": 8,
      "50326854-AF48-9980-9698-C88BB7F9ADFB": 0,
      "194D0C43-7049-5471-699B-6987E5B090DF": 15,
      "BD32FEAA-144C-9553-255E-6AB6DDD13210": 1,
      "8EE1AF23-584E-E14C-52C2-618DB7BE53B9": 11,
      "EAB762A4-3A4E-99F4-1FEC-C199B2E12482": 4,
      "BDFDB52E-104D-AC01-8FF3-3681DAA59333": 5,
      "4F359D50-2F49-E6F6-B285-49A71C633C07": 0,
      "40EB564A-DC11-F510-7E34-D392E76AC9B2": 2,
      "004A8AD7-9746-58E8-B519-A8BAB4467D48": 18,
      "86F87955-1F4C-3A93-7B08-BA832FB96163": 2,
      "52BE2F61-0B40-53DA-914F-0D917C85B19F": 1,
      "367A23A4-C941-EACA-F818-A28FF31B6858": 4,
      "753F4E80-494B-8870-068C-D6A4DCB67E3C": 5,
      "F20A68FB-A34B-EF59-B519-A8BA3D44C873": 2,
      "0EB75099-174E-1AB4-0DFA-CCBBD67F8157": 1
    },
    "save_game_class_name": "/Script/CD.CDSave_GameState"
  },
  "properties": {
    "LastSaveTime": {
      "type": "StructProperty",
      "type_name": "DateTime",
      "DateTime": {
        "ticks": 638160761644140000
      }
    },
    "PlayerClass": {
      "type": "ObjectProperty",
      "value": "/Game/Character/Player/Blueprints/BP_Soldier.BP_Soldier_C"
    },
    "Version": {
      "type": "IntProperty",
      "value": 3
    },
    "GameplayDatabase": {
      "type": "MapProperty",
      "value_type": "StructProperty",
      "name_props": {
        "unlock.welcomescreen.seen": {
          "type": "StructPropertyValue",
          "CustomStruct": {
            "AsFloat": [
              {
                "type": "FloatProperty",
                "value": 0.0
              }
            ],
            "AsString": [
              {
                "type": "StrProperty"
              }
            ]
          }
        },
        "game.tutorial.finished": {
          "type": "StructPropertyValue",
          "CustomStruct": {
            "AsFloat": [
              {
                "type": "FloatProperty",
                "value": 1.0
              }
            ],
            "AsString": [
              {
                "type": "StrProperty"
              }
            ]
          }
        },
        "game.tutorial.skipped": {
          "type": "StructPropertyValue",
          "CustomStruct": {
            "AsFloat": [
              {
                "type": "FloatProperty",
                "value": 1.0
              }
            ],
            "AsString": [
              {
                "type": "StrProperty"
              }
            ]
          }
        },
        "dialogs.messages.seen.Rumiko.0.50": {
          "type": "StructPropertyValue",
          "CustomStruct": {
            "AsFloat": [
              {
                "type": "FloatProperty",
                "value": 1.0
              }
            ],
            "AsString": [
              {
                "type": "StrProperty"
              }
            ]
          }
        },
        "codex.Rumiko": {
          "type": "StructPropertyValue",
          "CustomStruct": {
            "AsFloat": [
              {
                "type": "FloatProperty",
                "value": 1.0
              }
            ],
            "AsString": [
              {
                "type": "StrProperty"
              }
            ]
          }
        }
      }
    },
    "PlayerAttributes": {
      "type": "MapProperty",
      "key_type": "StructProperty",
      "value_type": "FloatProperty",
      "allocation_flags": 0,
      "value": [
        [
          {
            "type": "StructPropertyValue",
            "CustomStruct": {
              "AttributeName": [
                {
                  "type": "StrProperty",
                  "value": "Currency_Blueprints"
                }
              ],
              "Attribute": [
                {
                  "type": "FieldPathProperty",
                  "value": {
                    "path": [
                      "Currency_Blueprints"
                    ],
                    "resolved_owner": "/Script/CD.CDPlayerAttributeSet"
                  }
                }
              ],
              "AttributeOwner": [
                {
                  "type": "ObjectProperty",
                  "value": "None"
                }
              ]
            }
          },
          {
            "type": "FloatProperty",
            "value": 0.0
          }
        ],
        [
          {
            "type": "StructPropertyValue",
            "CustomStruct": {
              "AttributeName": [
                {
                  "type": "StrProperty",
                  "value": "Currency_Electrum"
                }
              ],
              "Attribute": [
                {
                  "type": "FieldPathProperty",
                  "value": {
                    "path": [
                      "Currency_Electrum"
                    ],
                    "resolved_owner": "/Script/CD.CDPlayerAttributeSet"
                  }
                }
              ],
              "AttributeOwner": [
                {
                  "type": "ObjectProperty",
                  "value": "None"
                }
              ]
            }
          },
          {
            "type": "FloatProperty",
            "value": 0.0
          }
        ]
      ]
    },
    "SecondaryWeaponClass": {
      "type": "ObjectProperty",
      "value": "/Game/Weapons/RocketLauncher/Blueprints/BP_RocketLauncher.BP_RocketLauncher_C"
    }
  }
}"#;
