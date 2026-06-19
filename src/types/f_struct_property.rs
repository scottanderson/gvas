use binrw::binrw;

use crate::{
    format::SerializationFormat,
    types::{
        FDateTime, FGameplayTagContainer, FGuid, FIntPoint, FLinearColor, FQuat, FRotator,
        FTimespan, FVector, FVector2D, PropertyType, TaggedProperties,
    },
};

#[binrw]
#[br(import(format: SerializationFormat, t: &PropertyType, struct_type: &str))]
#[bw(import(format: SerializationFormat))]
#[derive(Debug)]
pub enum FStructProperty {
    #[br(pre_assert(struct_type == "DateTime"))]
    DateTime(FDateTime),
    #[br(pre_assert(struct_type == "GameplayTagContainer"))]
    GameplayTagContainer(FGameplayTagContainer),
    #[br(pre_assert(struct_type == "Guid"))]
    Guid(FGuid),
    #[br(pre_assert(struct_type == "IntPoint"))]
    IntPoint(FIntPoint),
    #[br(pre_assert(struct_type == "LinearColor"))]
    LinearColor(FLinearColor),
    #[br(pre_assert(struct_type == "Quat"))]
    Quat(#[br(args(format))] FQuat),
    #[br(pre_assert(struct_type == "Rotator"))]
    Rotator(#[br(args(format))] FRotator),
    #[br(pre_assert(struct_type == "Timespan"))]
    Timespan(FTimespan),
    #[br(pre_assert(struct_type == "Vector"))]
    Vector(#[br(args(format))] FVector),
    #[br(pre_assert(struct_type == "Vector2D"))]
    Vector2D(FVector2D),
    Custom(#[brw(args(format))] TaggedProperties),
    Unknown(#[br(count = t.size())] Vec<u8>),
}
