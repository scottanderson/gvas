use binrw::binrw;

use crate::{options::ParsingOptions, types::FString};

#[binrw]
#[br(import(options: ParsingOptions))]
#[derive(Debug)]
pub enum FSoftObjectProperty {
    #[br(pre_assert(!options.fsoftobjectpath_remove_asset_path_fnames))]
    Old(FString, FString),
    #[br(pre_assert(options.fsoftobjectpath_remove_asset_path_fnames))]
    New(FString, FString, FString),
}
