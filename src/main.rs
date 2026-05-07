use binrw::BinRead;
use std::{
    fs::{self, File},
    io::{Cursor, Read},
    path::Path,
};

use crate::error::Result;
use crate::types::SaveGameFile;

mod error;
mod options;
mod properties;
mod types;
mod versions;

fn read_save_game_file<P: AsRef<Path> + std::fmt::Debug>(path: P) -> Result<SaveGameFile> {
    // Open
    let mut file = File::open(&path)?;

    // Read
    let mut buf = Vec::new();
    let _len = file.read_to_end(&mut buf)?;

    // Parse
    let mut cursor = Cursor::new(buf);
    let result = SaveGameFile::read(&mut cursor)?;

    // Success
    Ok(result)
}

fn visit_dirs<P: AsRef<Path>>(dir: P) -> Result<()> {
    let dir = dir.as_ref();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // visit_dirs(&path)?;
            } else {
                println!("{}", &path.display());
                let save_game = read_save_game_file(&path)?;
                print!("{:#?}", save_game);
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    visit_dirs("/home/scott/git/gvas/resources/test/")
}
