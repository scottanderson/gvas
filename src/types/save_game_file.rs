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
    #[bw(calc(ParsingOptions::from(header)))]
    options: ParsingOptions,

    #[brw(if(options.property_tag_complete_type_name))]
    #[br(temp, assert(spacer == 0))]
    #[bw(calc(0))]
    spacer: u8,

    #[brw(args(options))]
    pub properties: TaggedProperties,

    #[br(temp, assert(footer == 0))]
    #[bw(calc(0))]
    footer: u32,
}
