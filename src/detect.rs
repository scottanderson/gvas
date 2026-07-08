//! Automatically detect file contents.
//!
//! # Examples
//!
//! ```
//! use binrw::BinRead;
//! use gvas::{detect::AutoDetectFile, error::{Error, Result}};
//! use std::{assert_matches, fs::File, io::{Cursor, Read, Seek}};
//!
//! // Open
//! let mut file = File::open("resources/test/regression_01.bin")?;
//!
//! // Read
//! let mut buf = Vec::new();
//! let size = file.read_to_end(&mut buf)?;
//!
//! // Parse
//! let mut cursor = Cursor::new(buf);
//! let result = AutoDetectFile::read(&mut cursor)?;
//!
//! // Compare
//! assert_eq!(size, cursor.stream_position()? as usize);
//! assert_matches!(result, AutoDetectFile::GVAS(_));
//! # Ok::<(), Error>(())
//! ```

use binrw::binrw;

#[cfg(feature = "palworld")]
use crate::palworld::PalworldSaveGame;
use crate::types::USaveGame;

#[cfg(not(feature = "palworld"))]
#[binrw]
#[br(little)]
#[derive(Debug, PartialEq)]
pub enum AutoDetectFile {
    GVAS(USaveGame),
}

#[cfg(feature = "palworld")]
#[binrw]
#[br(little)]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AutoDetectFile {
    GVAS(USaveGame),
    Palworld(PalworldSaveGame),
}

#[cfg(test)]
mod test {
    use std::{
        assert_matches,
        fs::File,
        io::{Cursor, Read, Seek},
    };

    use binrw::BinRead;

    use crate::{
        detect::AutoDetectFile, error::Result, test::common::REGRESSION_01_PATH, types::USaveGame,
    };

    #[test]
    fn gvas() -> Result<()> {
        // Open
        let mut file = File::open(REGRESSION_01_PATH)?;

        // Read
        let mut buf = Vec::new();
        let size = file.read_to_end(&mut buf)?;

        // Parse
        let mut cursor = Cursor::new(buf);
        let result = AutoDetectFile::read(&mut cursor)?;

        // Compare
        let pos = cursor.stream_position()?;
        let pos = usize::try_from(pos)?;
        assert_eq!(size, pos);
        assert_matches!(result, AutoDetectFile::GVAS(USaveGame { .. }));
        Ok(())
    }
}
