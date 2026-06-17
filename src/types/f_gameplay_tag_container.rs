use binrw::binrw;

use crate::types::{FString, TArray};

#[binrw]
#[derive(Debug)]
pub struct FGameplayTagContainer(TArray<FString>);
