use crate::types::{
    FCustomVersion, FCustomVersionContainer, FEngineVersion, FFloatProperty, FGuid,
    FPackageFileVersion, FProperty, FSaveGameHeader, FString, SaveGameFileVersion, TArray,
    TaggedProperties, TaggedProperty, USaveGame,
};

pub(crate) fn expected() -> USaveGame {
    USaveGame {
        header: FSaveGameHeader {
            save_game_file_version: SaveGameFileVersion::AddedCustomVersions as u32,
            package_file_version: FPackageFileVersion::UE4 {
                package_file_version: 518,
            },
            engine_version: FEngineVersion {
                major: 4,
                minor: 25,
                patch: 3,
                change_list: 13942748,
                branch: FString::from("++UE4+Release-4.25"),
            },
            custom_versions: Some(FCustomVersionContainer {
                // custom_version_format: 3,
                custom_versions: TArray::from([
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
                        value: 2,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x5FBC6907, 0x55C840AE, 0x8E67F184, 0x5EFFF13F),
                        value: 1,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xFB26E412, 0x1F154B4D, 0x9372550A, 0x961D2F70),
                        value: 3,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xFCF57AFA, 0x50764283, 0xB9A9E658, 0xFFA02D32),
                        value: 61,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x9C54D522, 0xA8264FBE, 0x94210746, 0x61B482D0),
                        value: 30,
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
                        value: 38,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xCFFC743F, 0x43B04480, 0x939114DF, 0x171D2073),
                        value: 37,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xB02B49B5, 0xBB2044E9, 0xA30432B7, 0x52E40360),
                        value: 2,
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
                        value: 4,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x6631380F, 0x2D4D43E0, 0x8009CF27, 0x6956A95A),
                        value: 0,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x12F88B9F, 0x88754AFC, 0xA67CD90C, 0x383ABD29),
                        value: 43,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x7B5AE74C, 0xD2704C10, 0xA9585798, 0x0B212A5A),
                        value: 12,
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
                        value: 7,
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
                        value: 31,
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
                        value: 37,
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
                        key: FGuid::from_u32(0x54683250, 0x809948AF, 0x8BC89896, 0xFBADF9B7),
                        value: 0,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x430C4D19, 0x71544970, 0x87699B69, 0xDF90B0E5),
                        value: 14,
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
                        key: FGuid::from_u32(0x717F9EE7, 0xE9B0493A, 0x88B39132, 0x1B388107),
                        value: 6,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x4A56EB40, 0x10F511DC, 0x92D3347E, 0xB2C96AE7),
                        value: 2,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0xD78A4A00, 0xE8584697, 0xBAA819B5, 0x487D46B4),
                        value: 17,
                    },
                    FCustomVersion {
                        key: FGuid::from_u32(0x5579F886, 0x933A4C1F, 0x83BA087B, 0x6361B92F),
                        value: 1,
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
                    FCustomVersion {
                        key: FGuid::from_u32(0xAB965196, 0x45D808FC, 0xB7D7228D, 0x78AD569E),
                        value: 1,
                    },
                ]),
            }),
            save_game_class_name: FString::from("/Game/UI/BP_SaveOptions.BP_SaveOptions_C"),
        },
        properties: TaggedProperties::from([
            (
                FString::from("Slider1"),
                TaggedProperty {
                    array_index: 0,
                    has_binary_or_native_serialize: false,
                    has_property_extensions: false,
                    property: FProperty::from(FFloatProperty(0.16610672)),
                    property_guid: FGuid::default(),
                },
            ),
            (
                FString::from("Slider2"),
                TaggedProperty {
                    array_index: 0,
                    has_binary_or_native_serialize: false,
                    has_property_extensions: false,
                    property: FProperty::from(FFloatProperty(0.28251615)),
                    property_guid: FGuid::default(),
                },
            ),
        ]),
    }
}
