use binrw::binrw;

use crate::types::{FSaveGameHeader, TaggedProperties};

#[binrw]
#[brw(little)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct USaveGame {
    pub header: FSaveGameHeader,

    #[brw(if(header.serialization_format().property_tag_complete_type_name()))]
    #[br(temp, assert(spacer == 0))]
    #[bw(calc(0))]
    spacer: u8,

    #[brw(args(&header.serialization_format()))]
    pub properties: TaggedProperties,

    #[br(temp, assert(footer == 0))]
    #[bw(calc(0))]
    footer: u32,
}

#[cfg(test)]
mod test {
    use std::{
        fs::File,
        io::{Cursor, Read, Seek},
    };

    use binrw::{BinRead, BinWrite};

    use crate::{
        error::Result,
        test::common::{
            ASSERT_FAILED_PATH, COMPLETE_PROPERTY_TAG_PATH, COMPONENT8_PATH, DELEGATE_PATH,
            ENUM_ARRAY_PATH, FEATURES_01_PATH, MEDIEVAL_DYNASTY_PATH, OPTIONS_PATH,
            PACKAGE_VERSION_524_PATH, PACKAGE_VERSION_525_PATH, PROFILE_0_PATH, REGRESSION_01_PATH,
            RO_64BIT_FAV_PATH, SAVESLOT_03_PATH, SLOT1_PATH, SLOT2_PATH, SLOT3_PATH,
            STRING_TABLE_ENTRY, TAGCONTAINER_PATH, TEXT_PROPERTY_NOARRAY, TRANSFORM_PATH,
            VECTOR2D_PATH, delegate, options,
            profile0::PROFILE_0_JSON,
            regression::REGRESSION_01_JSON,
            saveslot3::{self, SAVESLOT_03_JSON},
            slot1::{self, SLOT1_JSON},
            tagcontainer::TAGCONTAINER_JSON,
            vector2d::{self, VECTOR2D_JSON},
        },
        types::USaveGame,
    };

    fn test_save_game(
        path: &str,
        expected_fn: Option<fn() -> USaveGame>,
        expected_json: Option<&str>,
    ) -> Result<USaveGame> {
        // Open
        let mut file = File::open(path)?;

        // Read
        let mut buf = Vec::new();
        let len = file.read_to_end(&mut buf)?;

        // Parse
        let mut cursor = Cursor::new(buf);
        let result = USaveGame::read(&mut cursor)?;
        assert_eq!(len as u64, cursor.stream_position()?);

        // Write
        let buf2 = vec![0u8; len];
        let mut cursor2 = Cursor::new(buf2);
        USaveGame::write(&result, &mut cursor2)?;

        // Compare to original bytes
        let buf = cursor.into_inner();
        let buf2 = cursor2.into_inner();
        assert!(buf == buf2);

        // Compare to expected value
        if let Some(expected_fn) = expected_fn {
            let expected = expected_fn();
            assert_eq!(result, expected);
        }

        #[cfg(feature = "serde")]
        {
            // Convert to JSON
            let json = serde_json::to_string_pretty(&result)?;

            // Compare to expected JSON
            if let Some(expected_json) = expected_json {
                pretty_assertions::assert_eq!(expected_json, json);
            }

            // Convert from JSON
            let result2 = serde_json::from_str(&json)?;
            assert!(result == result2);
        }
        #[cfg(not(feature = "serde"))]
        {
            // Suppress unused variable warning
            let _ = expected_json;
        }

        // Success
        Ok(result)
    }

    macro_rules! save_game_test {
        ($test_name:ident, $path:expr) => {
            #[test]
            fn $test_name() -> Result<()> {
                test_save_game($path, None, None)?;
                Ok(())
            }
        };

        ($test_name:ident, $path:expr, $expected:expr) => {
            #[test]
            fn $test_name() -> Result<()> {
                test_save_game($path, Some($expected), None)?;
                Ok(())
            }
        };

        ($test_name:ident, $path:expr, None, $json:expr) => {
            #[test]
            fn $test_name() -> Result<()> {
                test_save_game($path, None, Some($json))?;
                Ok(())
            }
        };

        ($test_name:ident, $path:expr, $expected:expr, $json:expr) => {
            #[test]
            fn $test_name() -> Result<()> {
                test_save_game($path, Some($expected), Some($json))?;
                Ok(())
            }
        };
    }

    save_game_test!(assert_failed, ASSERT_FAILED_PATH);
    save_game_test!(complete_property_tag, COMPLETE_PROPERTY_TAG_PATH);
    save_game_test!(component8, COMPONENT8_PATH);
    save_game_test!(delegate, DELEGATE_PATH, delegate::expected);
    save_game_test!(enum_array, ENUM_ARRAY_PATH);
    save_game_test!(features_01, FEATURES_01_PATH);
    save_game_test!(medieval_dynasty, MEDIEVAL_DYNASTY_PATH);
    save_game_test!(options, OPTIONS_PATH, options::expected);
    save_game_test!(package_version_524, PACKAGE_VERSION_524_PATH);
    save_game_test!(package_version_525, PACKAGE_VERSION_525_PATH);
    save_game_test!(profile_0, PROFILE_0_PATH, None, PROFILE_0_JSON);
    save_game_test!(regression_01, REGRESSION_01_PATH, None, REGRESSION_01_JSON);
    save_game_test!(ro_64bit_fav, RO_64BIT_FAV_PATH);
    save_game_test!(
        saveslot_03,
        SAVESLOT_03_PATH,
        saveslot3::expected,
        SAVESLOT_03_JSON
    );
    save_game_test!(slot1, SLOT1_PATH, slot1::expected, SLOT1_JSON);
    save_game_test!(slot2, SLOT2_PATH);
    save_game_test!(slot3, SLOT3_PATH);
    save_game_test!(string_table_entry, STRING_TABLE_ENTRY);
    save_game_test!(tagcontainer, TAGCONTAINER_PATH, None, TAGCONTAINER_JSON);
    save_game_test!(text_property_noarray, TEXT_PROPERTY_NOARRAY);
    save_game_test!(transform, TRANSFORM_PATH);
    save_game_test!(vector2d, VECTOR2D_PATH, vector2d::expected, VECTOR2D_JSON);
}
