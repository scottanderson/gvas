use binrw::binrw;

use crate::{
    options::ParsingOptions,
    types::{
        FDateTime, FGuid, FIntPoint, FLinearColor, FQuat, FRotator, FTimespan, FVector, FVector2D,
        PropertyType, TaggedProperties,
    },
};

mod structs {
    use binrw::binrw;

    use crate::types::{FString, TArray};

    #[binrw]
    #[derive(Debug)]
    pub struct GameplayTagContainer(TArray<FString>);
}

pub use structs::*;

#[binrw]
#[br(import(options: ParsingOptions, t: &PropertyType, struct_type: &str))]
#[bw(import(options: ParsingOptions))]
#[derive(Debug)]
pub enum StructProperty {
    #[br(pre_assert(struct_type == "DateTime"))]
    DateTime(FDateTime),
    #[br(pre_assert(struct_type == "GameplayTagContainer"))]
    GameplayTagContainer(GameplayTagContainer),
    #[br(pre_assert(struct_type == "Guid"))]
    Guid(FGuid),
    #[br(pre_assert(struct_type == "IntPoint"))]
    IntPoint(FIntPoint),
    #[br(pre_assert(struct_type == "LinearColor"))]
    LinearColor(FLinearColor),
    #[br(pre_assert(struct_type == "Quat"))]
    Quat(#[br(args(options))] FQuat),
    #[br(pre_assert(struct_type == "Rotator"))]
    Rotator(#[br(args(options))] FRotator),
    #[br(pre_assert(struct_type == "Timespan"))]
    Timespan(FTimespan),
    #[br(pre_assert(struct_type == "Vector"))]
    Vector(#[br(args(options))] FVector),
    #[br(pre_assert(struct_type == "Vector2D"))]
    Vector2D(FVector2D),
    Custom(#[brw(args(options))] TaggedProperties),
    Unknown(#[br(count = t.size())] Vec<u8>),
}
