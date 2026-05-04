use binrw::BinRead;
use std::{
    fs::File,
    io::{Cursor, Read, Seek},
    path::Path,
};

use crate::types::SaveGameFile;

mod enums;
mod types;

fn read_save_game_file<P: AsRef<Path> + std::fmt::Debug>(
    path: P,
) -> binrw::BinResult<SaveGameFile> {
    // Open
    let mut file = File::open(&path)?;

    // Read
    let mut buf = Vec::new();
    let _len = file.read_to_end(&mut buf)?;

    // Parse
    let mut cursor = Cursor::new(buf);
    let result = SaveGameFile::read(&mut cursor)?;

    let pos = cursor.stream_position()?;
    println!("0x{:04x}", pos);

    // Success
    Ok(result)
}

fn main() {
    let path = "/home/scott/git/gvas/resources/test/complete_property_tag.sav";
    let save_game = read_save_game_file(path).expect(path);
    println!("{:#?}", save_game);
}
