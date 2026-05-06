use binrw::BinRead;
use std::{
    fs::{self, File},
    io::{self, Cursor, Read},
    path::Path,
};

use crate::types::SaveGameFile;

mod options;
mod types;
mod versions;

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

    // Success
    Ok(result)
}

fn visit_dirs<P: AsRef<Path>>(dir: P) -> io::Result<()> {
    let dir = dir.as_ref();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path)?;
            } else {
                // cb(&entry);
                print!("{}", &path.display());
                match read_save_game_file(&path) {
                    Ok(save_game) => print!("{:#?}", save_game),
                    Err(e) => {
                        print!("{}", e);
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    visit_dirs("/home/scott/git/gvas/resources/test/")
}
