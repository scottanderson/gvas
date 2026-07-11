use binrw::binrw;

use crate::types::{FSaveGameHeader, TaggedProperties};

#[binrw]
#[brw(little)]
#[derive(Debug, PartialEq)]
pub struct USaveGame {
    pub header: FSaveGameHeader,

    #[brw(if(header.serialization_format().property_tag_complete_type_name()))]
    #[br(temp, assert(spacer == 0))]
    #[bw(calc(0))]
    spacer: u8,

    #[brw(args(header.serialization_format()))]
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
            ALL_TEST_PATHS, DELEGATE_PATH, OPTIONS_PATH, SAVESLOT_03_PATH, SLOT1_PATH,
            VECTOR2D_PATH, delegate, options, saveslot3, slot1, vector2d,
        },
        types::USaveGame,
    };

    #[test]
    fn test_save_games() -> Result<()> {
        for path in ALL_TEST_PATHS {
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

            // Compare
            let buf = cursor.into_inner();
            let buf2 = cursor2.into_inner();
            assert!(buf == buf2);

            assert_eq!(
                result,
                match path {
                    DELEGATE_PATH => delegate::expected(),
                    OPTIONS_PATH => options::expected(),
                    SAVESLOT_03_PATH => saveslot3::expected(),
                    SLOT1_PATH => slot1::expected(),
                    VECTOR2D_PATH => vector2d::expected(),
                    _ => continue,
                }
            );
        }

        // Success
        Ok(())
    }
}
