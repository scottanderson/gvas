use std::fs::File;
use std::io::{Cursor, Read};

use binrw::BinRead;

use crate::types::{FPropertyTag, FSaveGameHeader, ParsingOptions};

mod object_verison;
mod types;

const LE: binrw::Endian = binrw::Endian::Little;

fn main() {
    let mut file = File::open("/home/scott/git/gvas/resources/test/complete_property_tag.sav")
        .expect("File::open");
    let mut buf = Vec::new();
    let _len = file.read_to_end(&mut buf).expect("read_to_end");

    let mut cursor = Cursor::new(buf);
    let save_game_header = FSaveGameHeader::read(&mut cursor).expect("FSaveGameHeader::read");
    println!("{:?}", save_game_header.save_game_file_version);
    println!("{:?}", save_game_header.package_file_version);
    println!("{:?}", save_game_header.engine_version);
    println!("{:?}", save_game_header.save_game_class_name);

    let options = ParsingOptions::from(save_game_header);
    println!("{:?}", options);

    let property =
        FPropertyTag::read_options(&mut cursor, LE, (options,)).expect("FPropertyTag::read");
    println!("{:?}", property);
}
