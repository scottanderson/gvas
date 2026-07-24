use crate::types::{
    FBoolProperty, FCustomVersion, FCustomVersionContainer, FDelegateProperty, FEngineVersion,
    FFloatProperty, FGuid, FIntProperty, FMulticastInlineDelegateProperty, FPackageFileVersion,
    FProperty, FSaveGameHeader, FStrProperty, FString, FStructProperty, FVector2D, TArray,
    TaggedProperties, TaggedProperty, USaveGame,
};

const DELEGATE_PREFIX: &str = "/Game/DefaultMap.DefaultMap:PersistentLevel.";

pub(crate) fn expected() -> USaveGame {
    USaveGame {
        header: FSaveGameHeader {
            package_file_version: FPackageFileVersion::UE5 {
                file_version_ue4: 522,
                file_version_ue5: 1009,
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
            TaggedProperty {
                property_name: FString::from("SettingsChanged"),
                array_index: 0,
                has_binary_or_native_serialize: false,
                has_property_extensions: false,
                property: FProperty::from(FMulticastInlineDelegateProperty::from(TArray::from([
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
                ]))),
                property_guid: FGuid::default(),
            },
            TaggedProperty {
                property_name: FString::from("AudioSettings"),
                array_index: 0,
                has_binary_or_native_serialize: false,
                has_property_extensions: false,
                property: FProperty::from(FStructProperty::Custom {
                    struct_type: FString::from("GameAudioSettings"),
                    class_name: FString::null(),
                    struct_guid: FGuid::default(),
                    properties: TaggedProperties::from([
                        TaggedProperty {
                            property_name: FString::from("MasterLevel"),
                            array_index: 0,
                            has_binary_or_native_serialize: false,
                            has_property_extensions: false,
                            property: FProperty::from(FFloatProperty::from(0.20348908)),
                            property_guid: FGuid::default(),
                        },
                        TaggedProperty {
                            property_name: FString::from("MusicLevel"),
                            array_index: 0,
                            has_binary_or_native_serialize: false,
                            has_property_extensions: false,
                            property: FProperty::from(FFloatProperty::from(0.1511635)),
                            property_guid: FGuid::default(),
                        },
                        TaggedProperty {
                            property_name: FString::from("SFXLevel"),
                            array_index: 0,
                            has_binary_or_native_serialize: false,
                            has_property_extensions: false,
                            property: FProperty::from(FFloatProperty::from(0.5436054)),
                            property_guid: FGuid::default(),
                        },
                    ]),
                }),
                property_guid: FGuid::default(),
            },
            TaggedProperty {
                property_name: FString::from("GameSettings"),
                array_index: 0,
                has_binary_or_native_serialize: false,
                has_property_extensions: false,
                property: FProperty::from(FStructProperty::Custom {
                    struct_type: FString::from("GameSettings"),
                    class_name: FString::null(),
                    struct_guid: FGuid::default(),
                    properties: TaggedProperties::from([
                        TaggedProperty {
                            property_name: FString::from("CurrentSaveSlot"),
                            array_index: 0,
                            has_binary_or_native_serialize: false,
                            has_property_extensions: false,
                            property: FProperty::from(FStrProperty::from(FString::from("SAVE2"))),
                            property_guid: FGuid::default(),
                        },
                        TaggedProperty {
                            property_name: FString::from("LoadTutorial"),
                            array_index: 0,
                            has_binary_or_native_serialize: false,
                            has_property_extensions: false,
                            property: FProperty::from(FBoolProperty::from(false)),
                            property_guid: FGuid::default(),
                        },
                        TaggedProperty {
                            property_name: FString::from("DisplayNewOrders"),
                            array_index: 0,
                            has_binary_or_native_serialize: false,
                            has_property_extensions: false,
                            property: FProperty::from(FBoolProperty::from(false)),
                            property_guid: FGuid::default(),
                        },
                        TaggedProperty {
                            property_name: FString::from("EscapeExitsTool"),
                            array_index: 0,
                            has_binary_or_native_serialize: false,
                            has_property_extensions: false,
                            property: FProperty::from(FBoolProperty::from(false)),
                            property_guid: FGuid::default(),
                        },
                        TaggedProperty {
                            property_name: FString::from("UseDarkMode"),
                            array_index: 0,
                            has_binary_or_native_serialize: false,
                            has_property_extensions: false,
                            property: FProperty::from(FBoolProperty::from(true)),
                            property_guid: FGuid::default(),
                        },
                        TaggedProperty {
                            property_name: FString::from("AnimateDayCycle"),
                            array_index: 0,
                            has_binary_or_native_serialize: false,
                            has_property_extensions: false,
                            property: FProperty::from(FBoolProperty::from(false)),
                            property_guid: FGuid::default(),
                        },
                        TaggedProperty {
                            property_name: FString::from("EnableTractorCollision"),
                            array_index: 0,
                            has_binary_or_native_serialize: false,
                            has_property_extensions: false,
                            property: FProperty::from(FBoolProperty::from(false)),
                            property_guid: FGuid::default(),
                        },
                        TaggedProperty {
                            property_name: FString::from("ShowInventory"),
                            array_index: 0,
                            has_binary_or_native_serialize: false,
                            has_property_extensions: false,
                            property: FProperty::from(FBoolProperty::from(true)),
                            property_guid: FGuid::default(),
                        },
                        TaggedProperty {
                            property_name: FString::from("CameraAngle"),
                            array_index: 0,
                            has_binary_or_native_serialize: false,
                            has_property_extensions: false,
                            property: FProperty::from(FStructProperty::Vector2D(FVector2D {
                                x: 30.574748247861862,
                                y: 60.42525175213814,
                            })),
                            property_guid: FGuid::default(),
                        },
                    ]),
                }),
                property_guid: FGuid::default(),
            },
            TaggedProperty {
                property_name: FString::from("HighScore"),
                array_index: 0,
                has_binary_or_native_serialize: false,
                has_property_extensions: false,
                property: FProperty::from(FIntProperty::from(2649)),
                property_guid: FGuid::default(),
            },
        ]),
    }
}

pub(crate) const VECTOR2D_JSON: &str = r#"{
  "header": {
    "file_version_ue4": 522,
    "file_version_ue5": 1009,
    "engine_version": {
      "major": 5,
      "minor": 3,
      "patch": 2,
      "change_list": 29314046,
      "branch": "++UE5+Release-5.3"
    },
    "custom_versions": {
      "9c54d522-a826-4fbe-9421-074661b482d0": 44,
      "62915ca3-1c8e-4bf7-a30e-12c7c8219df7": 32,
      "cc400d24-e0e9-4e7b-9bf9-a283dcc0c027": 0,
      "b0d832e4-1f89-4f0d-accf-7eb736fd4aa2": 10,
      "e1c64328-a22c-4d53-a36c-8e866417bd8c": 0,
      "375ec13c-06e4-48fb-b500-84f0262a717e": 4,
      "e4b068ed-f494-42e9-a231-da0b2e46bb41": 40,
      "cffc743f-43b0-4480-9391-14df171d2073": 37,
      "b02b49b5-bb20-44e9-a304-32b752e40360": 3,
      "a4e4105c-59a1-49b5-a7c5-40c4547edfee": 0,
      "39c831c9-5ae6-47dc-9a44-9c173e1c8e7c": 0,
      "78f01b33-ebea-4f98-b9b4-84eaccb95aa2": 20,
      "6631380f-2d4d-43e0-8009-cf276956a95a": 0,
      "12f88b9f-8875-4afc-a67c-d90c383abd29": 47,
      "7b5ae74c-d270-4c10-a958-57980b212a5a": 13,
      "d7296918-1dd6-4bdd-9de2-64a83cc13884": 3,
      "c2a15278-bfe7-4afe-6c17-90ff531df755": 1,
      "6eaca3d4-40ec-4cc1-b786-8bed09428fc5": 3,
      "29e575dd-e0a3-4627-9d10-d276232cdcea": 17,
      "af43a65d-7fd3-4947-9873-3e8ed9c1bb05": 15,
      "6b266cec-1ec7-4b8f-a30b-e4d90942fc07": 1,
      "0df73d61-a23f-47ea-b727-89e90c41499a": 1,
      "601d1886-ac64-4f84-aa16-d3de0deac7d6": 111,
      "8dbc2c5b-54a7-43e0-a768-fcbb7da29060": 2,
      "5b4c06b7-2463-4af8-805b-bf70cdf5d0dd": 10,
      "e7086368-6b23-4c58-8439-1b7016265e91": 11,
      "9dffbcd6-494f-0158-e221-12823c92a888": 10,
      "f2aed0ac-9afe-416f-8664-aa7ffa26d6fc": 1,
      "174f1f0b-b4c6-45a5-b13f-2ee8d0fb917d": 10,
      "35f94a83-e258-406c-a318-09f59610247c": 41,
      "b68fc16e-8b1b-42e2-b453-215c058844fe": 1,
      "b2e18506-4273-cfc2-a54e-f4bb758bba07": 1,
      "64f58936-fd1b-42ba-ba96-7289d5d0fa4e": 1,
      "697dd581-e64f-41ab-aa4a-51ecbeb7b628": 118,
      "d89b5e42-24bd-4d46-8412-aca8df641779": 47,
      "59da5d52-1232-4948-b878-597870b8e98b": 8,
      "26075a32-730f-4708-88e9-8c32f1599d05": 0,
      "6f0ed827-a609-4895-9c91-998d90180ea4": 2,
      "30d58be3-95ea-4282-a6e3-b159d8ebb06a": 1,
      "717f9ee7-e9b0-493a-88b3-91321b388107": 17,
      "68c409fc-7095-4986-8963-acd2c4865183": 3,
      "430c4d19-7154-4970-8769-9b69df90b0e5": 15,
      "aafe32bd-5395-4c14-b66a-5e251032d1dd": 1,
      "23afe18e-4ce1-4e58-8d61-c252b953beb7": 11,
      "a462b7ea-f499-4e3a-99c1-ec1f8224e1b2": 4,
      "2eb5fdbd-01ac-4d10-8136-f38f3393a5da": 5,
      "509d354f-f6e6-492f-a749-85b2073c631c": 0,
      "95a4f03e-7e0b-49e4-ba43-d35694ff87d9": 7,
      "b6e31b1c-d29f-11ec-857e-9f856f9970e2": 1,
      "4a56eb40-10f5-11dc-92d3-347eb2c96ae7": 3,
      "8417998a-bbc0-43ec-81b3-d119072d2722": 19,
      "d78a4a00-e858-4697-baa8-19b5487d46b4": 18,
      "5579f886-933a-4c1f-83ba-087b6361b92f": 2,
      "612fbe52-da53-400b-910d-4f919fb1857c": 1,
      "a4237a36-caea-41c9-8fa2-18f858681bf3": 5,
      "804e3f75-7088-4b49-a4d6-8c063c7eb6dc": 5,
      "1ed048f4-2f2e-4c68-89d0-53a4f18f102d": 1,
      "fb680af2-59ef-4ba3-baa8-19b573c8443d": 2,
      "9950b70e-b41a-4e17-bbcc-fa0d57817fd6": 1,
      "5e1714cd-484e-2951-707a-89a79302ab78": 3,
      "0925477b-763d-4001-9d91-d6730b75b411": 1,
      "4288211b-4548-16c6-1a76-67b2507a2a00": 1,
      "dc49959b-53c0-4de7-9156-ea885e7c5d39": 2,
      "a7820cfb-20a7-4359-8c54-2c149623cf50": 27,
      "82e77c4e-3323-43a5-b46b-13c597310df3": 0,
      "e21e1caa-af47-425e-89bf-6ad44c44a8bb": 0,
      "134a157e-d5e2-49a3-8d4e-843c98fe9e31": 2,
      "fcf57afa-5076-4283-b9a9-e658ffa02d32": 79,
      "11310aed-2e55-4d61-af67-9aa3c5a1082c": 17,
      "f6dfbb78-bb50-a0e4-4018-b84d60cbaf23": 2,
      "24bb7af3-5646-4f83-1f2f-2dc249ad96ff": 5,
      "76a52329-0923-45b5-98ae-d841cf2f6ad8": 5,
      "5fbc6907-55c8-40ae-8e67-f1845efff13f": 1,
      "92738c43-2988-4d9c-9a3d-9bbe6eff9fc0": 1
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
      "Custom": {
        "struct_type": "GameAudioSettings",
        "properties": {
          "MasterLevel": {
            "type": "FloatProperty",
            "value": 0.20348908
          },
          "MusicLevel": {
            "type": "FloatProperty",
            "value": 0.1511635
          },
          "SFXLevel": {
            "type": "FloatProperty",
            "value": 0.5436054
          }
        }
      }
    },
    "GameSettings": {
      "type": "StructProperty",
      "Custom": {
        "struct_type": "GameSettings",
        "properties": {
          "CurrentSaveSlot": {
            "type": "StrProperty",
            "value": "SAVE2"
          },
          "LoadTutorial": {
            "type": "BoolProperty",
            "value": false
          },
          "DisplayNewOrders": {
            "type": "BoolProperty",
            "value": false
          },
          "EscapeExitsTool": {
            "type": "BoolProperty",
            "value": false
          },
          "UseDarkMode": {
            "type": "BoolProperty",
            "value": true
          },
          "AnimateDayCycle": {
            "type": "BoolProperty",
            "value": false
          },
          "EnableTractorCollision": {
            "type": "BoolProperty",
            "value": false
          },
          "ShowInventory": {
            "type": "BoolProperty",
            "value": true
          },
          "CameraAngle": {
            "type": "StructProperty",
            "Vector2D": {
              "x": 30.574748247861862,
              "y": 60.42525175213814
            }
          }
        }
      }
    },
    "HighScore": {
      "type": "IntProperty",
      "value": 2649
    }
  }
}"#;
