use crate::types::{
    FBoolProperty, FCustomVersion, FCustomVersionContainer, FDelegateProperty, FEngineVersion,
    FFloatProperty, FGuid, FIntProperty, FMulticastInlineDelegateProperty,
    FPackageFileVersion::UE5,
    FProperty::{Bool, Float, Int, MulticastInlineDelegate, Str, Struct},
    FSaveGameHeader, FStrProperty, FString,
    FStructProperty::{Custom, Vector2D},
    FVector2D, SaveGameFileVersion, TArray, TaggedProperties, TaggedProperty, USaveGame,
};

const DELEGATE_PREFIX: &str = "/Game/DefaultMap.DefaultMap:PersistentLevel.";

pub(crate) fn expected() -> USaveGame {
    USaveGame {
        header: FSaveGameHeader {
            save_game_file_version: SaveGameFileVersion::PackageFileSummaryVersionChange as u32,
            package_file_version: UE5 {
                package_file_version: 522,
                package_file_version_ue5: 1009,
            },
            engine_version: FEngineVersion {
                major: 5,
                minor: 3,
                patch: 2,
                change_list: 29314046,
                branch: FString::from("++UE5+Release-5.3"),
            },
            custom_versions: Some(FCustomVersionContainer {
                // custom_version_format: 3,
                custom_versions: TArray::from([
                    FCustomVersion {
                        key: FGuid::from_u32(0x9C54D522, 0xA8264FBE, 0x94210746, 0x61B482D0),
                        value: 44,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x62915CA3, 0x1C8E4BF7, 0xA30E12C7, 0xC8219DF7),
                        value: 32,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xCC400D24, 0xE0E94E7B, 0x9BF9A283, 0xDCC0C027),
                        value: 0,
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
                        value: 20,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x6631380F, 0x2D4D43E0, 0x8009CF27, 0x6956A95A),
                        value: 0,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x12F88B9F, 0x88754AFC, 0xA67CD90C, 0x383ABD29),
                        value: 47,
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
                        value: 111,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x8DBC2C5B, 0x54A743E0, 0xA768FCBB, 0x7DA29060),
                        value: 2,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x5B4C06B7, 0x24634AF8, 0x805BBF70, 0xCDF5D0DD),
                        value: 10,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xE7086368, 0x6B234C58, 0x84391B70, 0x16265E91),
                        value: 11,
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
                        key: FGuid::from_u32(0x697DD581, 0xE64F41AB, 0xAA4A51EC, 0xBEB7B628),
                        value: 118,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xD89B5E42, 0x24BD4D46, 0x8412ACA8, 0xDF641779),
                        value: 47,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x59DA5D52, 0x12324948, 0xB8785978, 0x70B8E98B),
                        value: 8,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x26075A32, 0x730F4708, 0x88E98C32, 0xF1599D05),
                        value: 0,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x6F0ED827, 0xA6094895, 0x9C91998D, 0x90180EA4),
                        value: 2,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x30D58BE3, 0x95EA4282, 0xA6E3B159, 0xD8EBB06A),
                        value: 1,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x717F9EE7, 0xE9B0493A, 0x88B39132, 0x1B388107),
                        value: 17,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x68C409FC, 0x70954986, 0x8963ACD2, 0xC4865183),
                        value: 3,
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
                        key: FGuid::from_u32(0x95A4F03E, 0x7E0B49E4, 0xBA43D356, 0x94FF87D9),
                        value: 7,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xB6E31B1C, 0xD29F11EC, 0x857E9F85, 0x6F9970E2),
                        value: 1,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x4A56EB40, 0x10F511DC, 0x92D3347E, 0xB2C96AE7),
                        value: 3,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x8417998A, 0xBBC043EC, 0x81B3D119, 0x072D2722),
                        value: 19,
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
                        value: 5,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x804E3F75, 0x70884B49, 0xA4D68C06, 0x3C7EB6DC),
                        value: 5,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x1ED048F4, 0x2F2E4C68, 0x89D053A4, 0xF18F102D),
                        value: 1,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xFB680AF2, 0x59EF4BA3, 0xBAA819B5, 0x73C8443D),
                        value: 2,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x9950B70E, 0xB41A4E17, 0xBBCCFA0D, 0x57817FD6),
                        value: 1,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x5E1714CD, 0x484E2951, 0x707A89A7, 0x9302AB78),
                        value: 3,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x0925477B, 0x763D4001, 0x9D91D673, 0x0B75B411),
                        value: 1,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x4288211B, 0x454816C6, 0x1A7667B2, 0x507A2A00),
                        value: 1,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xDC49959B, 0x53C04DE7, 0x9156EA88, 0x5E7C5D39),
                        value: 2,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xA7820CFB, 0x20A74359, 0x8C542C14, 0x9623CF50),
                        value: 27,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x82E77C4E, 0x332343A5, 0xB46B13C5, 0x97310DF3),
                        value: 0,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xE21E1CAA, 0xAF47425E, 0x89BF6AD4, 0x4C44A8BB),
                        value: 0,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x134A157E, 0xD5E249A3, 0x8D4E843C, 0x98FE9E31),
                        value: 2,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xFCF57AFA, 0x50764283, 0xB9A9E658, 0xFFA02D32),
                        value: 79,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x11310AED, 0x2E554D61, 0xAF679AA3, 0xC5A1082C),
                        value: 17,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xF6DFBB78, 0xBB50A0E4, 0x4018B84D, 0x60CBAF23),
                        value: 2,
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
                        key: FGuid::from_u32(0x92738C43, 0x29884D9C, 0x9A3D9BBE, 0x6EFF9FC0),
                        value: 1,
                    },
                ]),
            }),
            save_game_class_name: FString::from(
                "/Game/_Blueprints/BP_SettingsSave.BP_SettingsSave_C",
            ),
        },
        properties: TaggedProperties::from([
            (
                FString::from("SettingsChanged"),
                TaggedProperty {
                    array_index: 0,
                    has_binary_or_native_serialize: false,
                    has_property_extensions: false,
                    property: MulticastInlineDelegate(FMulticastInlineDelegateProperty(
                        TArray::from([
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_WaterGauge_C_2147482315",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Plow_C_2147482312",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Plow_Row_Single_C_2147482309",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Plow_Row_3_C_2147482305",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Plow_5Row_C_2147482301",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Plow_Row_5_C_2147482297",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Plant_C_2147482293",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Plant_Row_C_2147482286",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Plant_Row3_C_2147482280",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Plant_Row5_C_2147482274",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Cultivate_C_2147482268",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Cultivate_Row_C_2147482265",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Cultivate_Row3_C_2147482261",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Cultivate_Row5_C_2147482257",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_PlasticRow_C_2147482253",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Purchase_C_2147482249",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Purchase_1x10_C_2147482242",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Purchase_3Row_C_2147482235",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Purchase_5Row_C_2147482228",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Purchase_10x10_C_2147482221",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Modify_C_2147482214",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Row_C_2147482198",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Row3_C_2147482181",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Harvest_C_2147482164",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Harvest_Row_C_2147482161",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Harvest_Row_3_C_2147482157",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Harvest_Row_5_C_2147482153",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Harvest_Row_C_2147482149",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_AutomatedActionControl_C_2147482145",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_RemovePlaceable_C_2147482142",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_SeedSilo_C_2147482139",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_TractorBarn_C_2147482132",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Sell_C_2147482125",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_FuelStorageTank_C_2147482118",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_ChickenRun_C_2147482115",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_MovePlaceable_C_2147482112",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Beehive_C_2147482109",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_SetPHTool_Row_C_2147482106",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_BiodieselRefinery_C_2147482089",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_OilPress_C_2147482086",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_FlourMill_C_2147482083",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_LargeChickenCoop_C_2147482080",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_CropSign_C_2147482077",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Mulch_C_2147482070",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Mulch_Row_C_2147482054",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Mulch_Row3_C_2147482037",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Warehouse_C_2147482020",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_HarvestSilo_C_2147482013",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_Stockpile_C_2147482008",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_ActionTool_CompostStation_C_2147482001",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged_Event"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!("{}BP_Renders_C_1", DELEGATE_PREFIX)),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_PlayerPawn_C_2147482331",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("UpdatedSavedSettings"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147478921",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147478905",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147478890",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147478875",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147478860",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147478303",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147478288",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147478273",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147478258",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147478243",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147478228",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147478141",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147478126",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147478111",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147478096",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147477750",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147477735",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147477720",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147477705",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147477690",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147477675",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147477660",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147477645",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147477189",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                            FDelegateProperty {
                                object: FString::from(format!(
                                    "{}BP_AutomatedTool_C_2147477162",
                                    DELEGATE_PREFIX
                                )),
                                function_name: FString::from("SettingsChanged"),
                            },
                        ]),
                    )),
                    property_guid: FGuid::default(),
                },
            ),
            (
                FString::from("AudioSettings"),
                TaggedProperty {
                    array_index: 0,
                    has_binary_or_native_serialize: false,
                    has_property_extensions: false,
                    property: Struct(Custom(
                        FString::from("GameAudioSettings"),
                        FString(None),
                        FGuid::default(),
                        TaggedProperties::from([
                            (
                                FString::from("MasterLevel"),
                                TaggedProperty {
                                    array_index: 0,
                                    has_binary_or_native_serialize: false,
                                    has_property_extensions: false,
                                    property: Float(FFloatProperty(0.20348908)),
                                    property_guid: FGuid::default(),
                                },
                            ),
                            (
                                FString::from("MusicLevel"),
                                TaggedProperty {
                                    array_index: 0,
                                    has_binary_or_native_serialize: false,
                                    has_property_extensions: false,
                                    property: Float(FFloatProperty(0.1511635)),
                                    property_guid: FGuid::default(),
                                },
                            ),
                            (
                                FString::from("SFXLevel"),
                                TaggedProperty {
                                    array_index: 0,
                                    has_binary_or_native_serialize: false,
                                    has_property_extensions: false,
                                    property: Float(FFloatProperty(0.5436054)),
                                    property_guid: FGuid::default(),
                                },
                            ),
                        ]),
                    )),
                    property_guid: FGuid::default(),
                },
            ),
            (
                FString::from("GameSettings"),
                TaggedProperty {
                    array_index: 0,
                    has_binary_or_native_serialize: false,
                    has_property_extensions: false,
                    property: Struct(Custom(
                        FString::from("GameSettings"),
                        FString(None),
                        FGuid::default(),
                        TaggedProperties::from([
                            (
                                FString::from("CurrentSaveSlot"),
                                TaggedProperty {
                                    array_index: 0,
                                    has_binary_or_native_serialize: false,
                                    has_property_extensions: false,
                                    property: Str(FStrProperty(FString::from("SAVE2"))),
                                    property_guid: FGuid::default(),
                                },
                            ),
                            (
                                FString::from("LoadTutorial"),
                                TaggedProperty {
                                    array_index: 0,
                                    has_binary_or_native_serialize: false,
                                    has_property_extensions: false,
                                    property: Bool(FBoolProperty(false)),
                                    property_guid: FGuid::default(),
                                },
                            ),
                            (
                                FString::from("DisplayNewOrders"),
                                TaggedProperty {
                                    array_index: 0,
                                    has_binary_or_native_serialize: false,
                                    has_property_extensions: false,
                                    property: Bool(FBoolProperty(false)),
                                    property_guid: FGuid::default(),
                                },
                            ),
                            (
                                FString::from("EscapeExitsTool"),
                                TaggedProperty {
                                    array_index: 0,
                                    has_binary_or_native_serialize: false,
                                    has_property_extensions: false,
                                    property: Bool(FBoolProperty(false)),
                                    property_guid: FGuid::default(),
                                },
                            ),
                            (
                                FString::from("UseDarkMode"),
                                TaggedProperty {
                                    array_index: 0,
                                    has_binary_or_native_serialize: false,
                                    has_property_extensions: false,
                                    property: Bool(FBoolProperty(true)),
                                    property_guid: FGuid::default(),
                                },
                            ),
                            (
                                FString::from("AnimateDayCycle"),
                                TaggedProperty {
                                    array_index: 0,
                                    has_binary_or_native_serialize: false,
                                    has_property_extensions: false,
                                    property: Bool(FBoolProperty(false)),
                                    property_guid: FGuid::default(),
                                },
                            ),
                            (
                                FString::from("EnableTractorCollision"),
                                TaggedProperty {
                                    array_index: 0,
                                    has_binary_or_native_serialize: false,
                                    has_property_extensions: false,
                                    property: Bool(FBoolProperty(false)),
                                    property_guid: FGuid::default(),
                                },
                            ),
                            (
                                FString::from("ShowInventory"),
                                TaggedProperty {
                                    array_index: 0,
                                    has_binary_or_native_serialize: false,
                                    has_property_extensions: false,
                                    property: Bool(FBoolProperty(true)),
                                    property_guid: FGuid::default(),
                                },
                            ),
                            (
                                FString::from("CameraAngle"),
                                TaggedProperty {
                                    array_index: 0,
                                    has_binary_or_native_serialize: false,
                                    has_property_extensions: false,
                                    property: Struct(Vector2D(FVector2D {
                                        x: 30.574748247861862,
                                        y: 60.42525175213814,
                                    })),
                                    property_guid: FGuid::default(),
                                },
                            ),
                        ]),
                    )),
                    property_guid: FGuid::default(),
                },
            ),
            (
                FString::from("HighScore"),
                TaggedProperty {
                    array_index: 0,
                    has_binary_or_native_serialize: false,
                    has_property_extensions: false,
                    property: Int(FIntProperty(2649)),
                    property_guid: FGuid::default(),
                },
            ),
        ]),
    }
}

pub const VECTOR2D_JSON: &str = r#"{
  "header": {
    "type": "Version3",
    "package_file_version": 522,
    "package_file_version_ue5": 1009,
    "engine_version": {
      "major": 5,
      "minor": 3,
      "patch": 2,
      "change_list": 29314046,
      "branch": "++UE5+Release-5.3"
    },
    "custom_version_format": 3,
    "custom_versions": {
      "22D5549C-BE4F-26A8-4607-2194D082B461": 44,
      "A35C9162-F74B-8E1C-C712-0EA3F79D21C8": 32,
      "240D40CC-7B4E-E9E0-83A2-F99B27C0C0DC": 0,
      "E432D8B0-0D4F-891F-B77E-CFACA24AFD36": 10,
      "2843C6E1-534D-2CA2-868E-6CA38CBD1764": 0,
      "3CC15E37-FB48-E406-F084-00B57E712A26": 4,
      "ED68B0E4-E942-94F4-0BDA-31A241BB462E": 40,
      "3F74FCCF-8044-B043-DF14-919373201D17": 37,
      "B5492BB0-E944-20BB-B732-04A36003E452": 3,
      "5C10E4A4-B549-A159-C440-C5A7EEDF7E54": 0,
      "C931C839-DC47-E65A-179C-449A7C8E1C3E": 0,
      "331BF078-984F-EAEB-EA84-B4B9A25AB9CC": 20,
      "0F383166-E043-4D2D-27CF-09805AA95669": 0,
      "9F8BF812-FC4A-7588-0CD9-7CA629BD3A38": 47,
      "4CE75A7B-104C-70D2-9857-58A95A2A210B": 13,
      "186929D7-DD4B-D61D-A864-E29D8438C13C": 3,
      "7852A1C2-FE4A-E7BF-FF90-176C55F71D53": 1,
      "D4A3AC6E-C14C-EC40-ED8B-86B7C58F4209": 3,
      "DD75E529-2746-A3E0-76D2-109DEADC2C23": 17,
      "5DA643AF-4749-D37F-8E3E-739805BBC1D9": 15,
      "EC6C266B-8F4B-C71E-D9E4-0BA307FC4209": 1,
      "613DF70D-EA47-3FA2-E989-27B79A49410C": 1,
      "86181D60-844F-64AC-DED3-16AAD6C7EA0D": 111,
      "5B2CBC8D-E043-A754-BBFC-68A76090A27D": 2,
      "B7064C5B-F84A-6324-70BF-5B80DDD0F5CD": 10,
      "686308E7-584C-236B-701B-3984915E2616": 11,
      "D6BCFF9D-5801-4F49-8212-21E288A8923C": 10,
      "ACD0AEF2-6F41-FE9A-7FAA-6486FCD626FA": 1,
      "0B1F4F17-A545-C6B4-E82E-3FB17D91FBD0": 10,
      "834AF935-6C40-58E2-F509-18A37C241096": 41,
      "6EC18FB6-E242-1B8B-5C21-53B4FE448805": 1,
      "0685E1B2-C2CF-7342-BBF4-4EA507BA8B75": 1,
      "3689F564-BA42-1BFD-8972-96BA4EFAD0D5": 1,
      "81D57D69-AB41-4FE6-EC51-4AAA28B6B7BE": 118,
      "425E9BD8-464D-BD24-A8AC-1284791764DF": 47,
      "525DDA59-4849-3212-7859-78B88BE9B870": 8,
      "325A0726-0847-0F73-328C-E988059D59F1": 0,
      "27D80E6F-9548-09A6-8D99-919CA40E1890": 2,
      "E38BD530-8242-EA95-59B1-E3A66AB0EBD8": 1,
      "E79E7F71-3A49-B0E9-3291-B3880781381B": 17,
      "FC09C468-8649-9570-D2AC-6389835186C4": 3,
      "194D0C43-7049-5471-699B-6987E5B090DF": 15,
      "BD32FEAA-144C-9553-255E-6AB6DDD13210": 1,
      "8EE1AF23-584E-E14C-52C2-618DB7BE53B9": 11,
      "EAB762A4-3A4E-99F4-1FEC-C199B2E12482": 4,
      "BDFDB52E-104D-AC01-8FF3-3681DAA59333": 5,
      "4F359D50-2F49-E6F6-B285-49A71C633C07": 0,
      "3EF0A495-E449-0B7E-56D3-43BAD987FF94": 7,
      "1C1BE3B6-EC11-9FD2-859F-7E85E270996F": 1,
      "40EB564A-DC11-F510-7E34-D392E76AC9B2": 3,
      "8A991784-EC43-C0BB-19D1-B38122272D07": 19,
      "004A8AD7-9746-58E8-B519-A8BAB4467D48": 18,
      "86F87955-1F4C-3A93-7B08-BA832FB96163": 2,
      "52BE2F61-0B40-53DA-914F-0D917C85B19F": 1,
      "367A23A4-C941-EACA-F818-A28FF31B6858": 5,
      "753F4E80-494B-8870-068C-D6A4DCB67E3C": 5,
      "F448D01E-684C-2E2F-A453-D0892D108FF1": 1,
      "F20A68FB-A34B-EF59-B519-A8BA3D44C873": 2,
      "0EB75099-174E-1AB4-0DFA-CCBBD67F8157": 1,
      "CD14175E-5129-4E48-A789-7A7078AB0293": 3,
      "7B472509-0140-3D76-73D6-919D11B4750B": 1,
      "1B218842-C616-4845-B267-761A002A7A50": 1,
      "9B9549DC-E74D-C053-88EA-5691395D7C5E": 2,
      "FB0C82A7-5943-A720-142C-548C50CF2396": 27,
      "4E7CE782-A543-2333-C513-6BB4F30D3197": 0,
      "AA1C1EE2-5E42-47AF-D46A-BF89BBA8444C": 0,
      "7E154A13-A349-E2D5-3C84-4E8D319EFE98": 2,
      "FA7AF5FC-8342-7650-58E6-A9B9322DA0FF": 79,
      "ED0A3111-614D-552E-A39A-67AF2C08A1C5": 17,
      "78BBDFF6-E4A0-50BB-4DB8-184023AFCB60": 2,
      "F37ABB24-834F-4656-C22D-2F1FFF96AD49": 5,
      "2923A576-B545-2309-41D8-AE98D86A2FCF": 5,
      "0769BC5F-AE40-C855-84F1-678E3FF1FF5E": 1,
      "438C7392-9C4D-8829-BE9B-3D9AC09FFF6E": 1
    },
    "save_game_class_name": "/Game/_Blueprints/BP_SettingsSave.BP_SettingsSave_C"
  },
  "properties": {
    "SettingsChanged": {
      "type": "MulticastInlineDelegateProperty",
      "value": {
        "delegates": [
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_WaterGauge_C_2147482315",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Plow_C_2147482312",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Plow_Row_Single_C_2147482309",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Plow_Row_3_C_2147482305",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Plow_5Row_C_2147482301",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Plow_Row_5_C_2147482297",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Plant_C_2147482293",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Plant_Row_C_2147482286",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Plant_Row3_C_2147482280",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Plant_Row5_C_2147482274",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Cultivate_C_2147482268",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Cultivate_Row_C_2147482265",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Cultivate_Row3_C_2147482261",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Cultivate_Row5_C_2147482257",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_PlasticRow_C_2147482253",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Purchase_C_2147482249",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Purchase_1x10_C_2147482242",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Purchase_3Row_C_2147482235",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Purchase_5Row_C_2147482228",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Purchase_10x10_C_2147482221",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Modify_C_2147482214",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Row_C_2147482198",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Row3_C_2147482181",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Harvest_C_2147482164",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Harvest_Row_C_2147482161",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Harvest_Row_3_C_2147482157",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Harvest_Row_5_C_2147482153",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Harvest_Row_C_2147482149",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_AutomatedActionControl_C_2147482145",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_RemovePlaceable_C_2147482142",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_SeedSilo_C_2147482139",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_TractorBarn_C_2147482132",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Sell_C_2147482125",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_FuelStorageTank_C_2147482118",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_ChickenRun_C_2147482115",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_MovePlaceable_C_2147482112",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Beehive_C_2147482109",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_SetPHTool_Row_C_2147482106",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_BiodieselRefinery_C_2147482089",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_OilPress_C_2147482086",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_FlourMill_C_2147482083",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_LargeChickenCoop_C_2147482080",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_CropSign_C_2147482077",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Mulch_C_2147482070",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Mulch_Row_C_2147482054",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Mulch_Row3_C_2147482037",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Warehouse_C_2147482020",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_HarvestSilo_C_2147482013",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_Stockpile_C_2147482008",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_ActionTool_CompostStation_C_2147482001",
            "function_name": "SettingsChanged_Event"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_Renders_C_1",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_PlayerPawn_C_2147482331",
            "function_name": "UpdatedSavedSettings"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478921",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478905",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478890",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478875",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478860",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478303",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478288",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478273",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478258",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478243",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478228",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478141",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478126",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478111",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147478096",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147477750",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147477735",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147477720",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147477705",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147477690",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147477675",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147477660",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147477645",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147477189",
            "function_name": "SettingsChanged"
          },
          {
            "object": "/Game/DefaultMap.DefaultMap:PersistentLevel.BP_AutomatedTool_C_2147477162",
            "function_name": "SettingsChanged"
          }
        ]
      }
    },
    "AudioSettings": {
      "type": "StructProperty",
      "type_name": "GameAudioSettings",
      "CustomStruct": {
        "MasterLevel": [
          {
            "type": "FloatProperty",
            "value": 0.20348908
          }
        ],
        "MusicLevel": [
          {
            "type": "FloatProperty",
            "value": 0.1511635
          }
        ],
        "SFXLevel": [
          {
            "type": "FloatProperty",
            "value": 0.5436054
          }
        ]
      }
    },
    "GameSettings": {
      "type": "StructProperty",
      "type_name": "GameSettings",
      "CustomStruct": {
        "CurrentSaveSlot": [
          {
            "type": "StrProperty",
            "value": "SAVE2"
          }
        ],
        "LoadTutorial": [
          {
            "type": "BoolProperty",
            "value": false
          }
        ],
        "DisplayNewOrders": [
          {
            "type": "BoolProperty",
            "value": false
          }
        ],
        "EscapeExitsTool": [
          {
            "type": "BoolProperty",
            "value": false
          }
        ],
        "UseDarkMode": [
          {
            "type": "BoolProperty",
            "value": true
          }
        ],
        "AnimateDayCycle": [
          {
            "type": "BoolProperty",
            "value": false
          }
        ],
        "EnableTractorCollision": [
          {
            "type": "BoolProperty",
            "value": false
          }
        ],
        "ShowInventory": [
          {
            "type": "BoolProperty",
            "value": true
          }
        ],
        "CameraAngle": [
          {
            "type": "StructProperty",
            "type_name": "Vector2D",
            "Vector2D": {
              "x": 30.574748247861862,
              "y": 60.42525175213814
            }
          }
        ]
      }
    },
    "HighScore": {
      "type": "IntProperty",
      "value": 2649
    }
  }
}"#;
