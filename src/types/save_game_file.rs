use binrw::binrw;

use crate::{
    options::ParsingOptions,
    types::{FSaveGameHeader, TaggedProperties},
};

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct SaveGameFile {
    pub header: FSaveGameHeader,

    #[br(temp, calc(ParsingOptions::from(&header)))]
    #[bw(ignore)]
    options: ParsingOptions,

    #[br(args(options))]
    pub properties: TaggedProperties,

    #[br(temp, assert(footer == 0))]
    #[bw(calc(0))]
    footer: u32,
}
