use binrw::binrw;

use crate::types::{FCustomVersionContainer, FEngineVersion, FPackageFileVersion, FString};

#[binrw]
#[brw(repr = u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SaveGameFileVersion {
    InitialVersion = 1,
    // serializing custom versions into the savegame data to handle that type of versioning
    AddedCustomVersions = 2,
    // added a new UE5 version number to FPackageFileSummary
    PackageFileSummaryVersionChange = 3,
}

#[binrw]
#[brw(little, magic = b"GVAS")]
#[derive(Debug, PartialEq)]
pub struct FSaveGameHeader {
    pub save_game_file_version: u32, //SaveGameFileVersion,

    #[br(args(save_game_file_version))]
    pub package_file_version: FPackageFileVersion,

    pub engine_version: FEngineVersion,

    #[br(if(save_game_file_version >= SaveGameFileVersion::AddedCustomVersions as u32))]
    #[bw(if(*save_game_file_version >= SaveGameFileVersion::AddedCustomVersions as u32))]
    pub custom_versions: Option<FCustomVersionContainer>,

    pub save_game_class_name: FString,
}

#[cfg(test)]
mod test {
    use std::io::{Read, Seek};
    use std::{fs::File, io::Cursor};

    use binrw::{BinRead, BinWrite};

    use crate::{error::Result, test::common::ALL_TEST_PATHS, types::FSaveGameHeader};

    #[test]
    fn test_save_game_header() -> Result<()> {
        for path in ALL_TEST_PATHS {
            // Open
            let mut file = File::open(&path)?;

            // Read
            let mut buf = Vec::new();
            let _len = file.read_to_end(&mut buf)?;

            // Parse
            let mut cursor = Cursor::new(buf);
            let result = FSaveGameHeader::read(&mut cursor)?;

            // Write
            let len = cursor.stream_position()? as usize;
            let buf2 = vec![0u8; len];
            let mut cursor2 = Cursor::new(buf2);
            FSaveGameHeader::write(&result, &mut cursor2)?;

            // Compare
            let buf = cursor.into_inner();
            let buf2 = cursor2.into_inner();
            assert_eq!(&buf[..buf2.len()], &buf2[..]);
        }

        // Success
        Ok(())
    }
}
