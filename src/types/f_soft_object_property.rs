use binrw::binrw;

use crate::{format::SerializationFormat, types::FString};

#[binrw]
#[br(import(format: &SerializationFormat))]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FSoftObjectProperty {
    #[br(pre_assert(!format.fsoftobjectpath_remove_asset_path_fnames()))]
    Old(FString, FString),
    #[br(pre_assert(format.fsoftobjectpath_remove_asset_path_fnames()))]
    New(FString, FString, FString),
}
