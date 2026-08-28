use binrw::binrw;

use crate::{
    hints::{HintMap, Path},
    types::{FSaveGameHeader, TaggedProperties},
};

#[binrw]
#[brw(little)]
#[br(import(hint_map: HintMap))]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct USaveGame {
    pub header: FSaveGameHeader,

    #[brw(if(header.serialization_format().property_tag_complete_type_name()))]
    #[br(temp, assert(spacer == 0))]
    #[bw(calc(0))]
    spacer: u8,

    #[br(args(&header.serialization_format(), &hint_map, Path::root()))]
    #[bw(args(&header.serialization_format()))]
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
        hints::HintMap,
        test::common::{
            ASSERT_FAILED_PATH, COMPLETE_PROPERTY_TAG_PATH, COMPONENT8_PATH, DELEGATE_PATH,
            ENUM_ARRAY_PATH, FEATURES_01_PATH, MEDIEVAL_DYNASTY_PATH, OPTIONS_PATH,
            PACKAGE_VERSION_524_PATH, PACKAGE_VERSION_525_PATH, PROFILE_0_PATH, REGRESSION_01_PATH,
            RO_64BIT_FAV_PATH, SAVESLOT_03_PATH, SLOT1_PATH, SLOT2_PATH, SLOT3_PATH,
            STRING_TABLE_ENTRY, TAGCONTAINER_PATH, TEXT_PROPERTY_NOARRAY, TRANSFORM_PATH,
            VECTOR2D_PATH, delegate, features, options,
            profile0::PROFILE_0_JSON,
            regression::REGRESSION_01_JSON,
            saveslot3::{self, SAVESLOT_03_JSON},
            slot1::{self, SLOT1_JSON},
            tagcontainer::TAGCONTAINER_JSON,
            vector2d::{self, VECTOR2D_JSON},
        },
        types::USaveGame,
    };

    #[derive(Default)]
    struct TestParams<'a> {
        expected_fn: Option<fn() -> USaveGame>,
        expected_json: Option<&'a str>,
        hints: Option<HintMap>,
    }

    fn test_save_game(
        path: &str,
        TestParams {
            expected_fn,
            expected_json,
            hints,
        }: TestParams,
    ) -> Result<USaveGame> {
        // Open
        let mut file = File::open(path)?;

        // Read
        let mut buf = Vec::new();
        let len = file.read_to_end(&mut buf)?;

        // Parse
        let mut cursor = Cursor::new(buf);
        let hints = hints.unwrap_or_default();
        let result = USaveGame::read_options(&mut cursor, binrw::Endian::Little, (hints,))?;
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

            // Convert from JSON
            let result2 = serde_json::from_str(&json)?;
            assert!(result == result2);

            // Compare to expected JSON
            if let Some(expected_json) = expected_json {
                pretty_assertions::assert_eq!(expected_json, json);
            }
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
        ($test_name:ident, $path:ident) => {
            #[test]
            fn $test_name() -> Result<()> {
                test_save_game($path, Default::default())?;
                Ok(())
            }
        };
        ($test_name:ident, $path:ident, $params:expr) => {
            #[test]
            fn $test_name() -> Result<()> {
                test_save_game($path, $params)?;
                Ok(())
            }
        };
    }

    save_game_test!(assert_failed, ASSERT_FAILED_PATH);
    save_game_test!(complete_property_tag, COMPLETE_PROPERTY_TAG_PATH);
    save_game_test!(component8, COMPONENT8_PATH);
    save_game_test!(
        delegate,
        DELEGATE_PATH,
        TestParams {
            expected_fn: Some(delegate::expected),
            ..Default::default()
        }
    );
    save_game_test!(enum_array, ENUM_ARRAY_PATH);
    save_game_test!(
        features_01,
        FEATURES_01_PATH,
        TestParams {
            // expected_fn: Some(features::expected),
            hints: Some(features::hints()),
            ..Default::default()
        }
    );
    save_game_test!(medieval_dynasty, MEDIEVAL_DYNASTY_PATH);
    save_game_test!(
        options,
        OPTIONS_PATH,
        TestParams {
            expected_fn: Some(options::expected),
            ..Default::default()
        }
    );
    save_game_test!(package_version_524, PACKAGE_VERSION_524_PATH);
    save_game_test!(package_version_525, PACKAGE_VERSION_525_PATH);
    save_game_test!(
        profile_0,
        PROFILE_0_PATH,
        TestParams {
            expected_json: Some(PROFILE_0_JSON),
            ..Default::default()
        }
    );
    save_game_test!(
        regression_01,
        REGRESSION_01_PATH,
        TestParams {
            expected_json: Some(REGRESSION_01_JSON),
            ..Default::default()
        }
    );
    save_game_test!(ro_64bit_fav, RO_64BIT_FAV_PATH);
    save_game_test!(
        saveslot_03,
        SAVESLOT_03_PATH,
        TestParams {
            expected_fn: Some(saveslot3::expected),
            expected_json: Some(SAVESLOT_03_JSON),
            ..Default::default()
        }
    );
    save_game_test!(
        slot1,
        SLOT1_PATH,
        TestParams {
            expected_fn: Some(slot1::expected),
            expected_json: Some(SLOT1_JSON),
            ..Default::default()
        }
    );
    save_game_test!(slot2, SLOT2_PATH);
    save_game_test!(slot3, SLOT3_PATH);
    save_game_test!(string_table_entry, STRING_TABLE_ENTRY);
    save_game_test!(
        tagcontainer,
        TAGCONTAINER_PATH,
        TestParams {
            expected_json: Some(TAGCONTAINER_JSON),
            ..Default::default()
        }
    );
    save_game_test!(text_property_noarray, TEXT_PROPERTY_NOARRAY);
    save_game_test!(transform, TRANSFORM_PATH);
    save_game_test!(
        vector2d,
        VECTOR2D_PATH,
        TestParams {
            expected_fn: Some(vector2d::expected),
            expected_json: Some(VECTOR2D_JSON),
            ..Default::default()
        }
    );
}
