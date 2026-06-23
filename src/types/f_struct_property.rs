use binrw::binrw;

use crate::{
    format::SerializationFormat,
    types::{
        FDateTime, FGameplayTagContainer, FGuid, FIntPoint, FLinearColor, FQuat, FRotator, FString,
        FTimespan, FVector, FVector2D, PropertyType, TaggedProperties,
    },
};

#[binrw]
#[br(import(format: SerializationFormat, t: &PropertyType, struct_type: &str, class_name: Option<&str>, guid: FGuid))]
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
    Custom(
        #[br(calc = struct_type.into())]
        #[bw(ignore)]
        FString,
        #[br(calc = class_name.into())]
        #[bw(ignore)]
        FString,
        #[br(calc = guid)]
        #[bw(ignore)]
        FGuid,
        #[brw(args(format))] TaggedProperties,
    ),
    Unknown(
        #[br(calc = struct_type.into())]
        #[bw(ignore)]
        FString,
        #[br(calc = class_name.into())]
        #[bw(ignore)]
        FString,
        #[br(calc = guid)]
        #[bw(ignore)]
        FGuid,
        #[br(count = t.size())] Vec<u8>,
    ),
}

impl FStructProperty {
    #[inline]
    pub fn struct_type(&self) -> FString {
        FString::from(match self {
            FStructProperty::DateTime(..) => Some("DateTime"),
            FStructProperty::GameplayTagContainer(..) => Some("GameplayTagContainer"),
            FStructProperty::Guid(..) => Some("Guid"),
            FStructProperty::IntPoint(..) => Some("IntPoint"),
            FStructProperty::LinearColor(..) => Some("LinearColor"),
            FStructProperty::Quat(..) => Some("Quat"),
            FStructProperty::Rotator(..) => Some("Rotator"),
            FStructProperty::Timespan(..) => Some("Timespan"),
            FStructProperty::Vector(..) => Some("Vector"),
            FStructProperty::Vector2D(..) => Some("Vector2D"),
            FStructProperty::Unknown(struct_type, _c, _g, _)
            | FStructProperty::Custom(struct_type, _c, _g, _) => struct_type.0.as_deref(),
        })
    }

    #[inline]
    pub fn struct_class(&self) -> FString {
        match self {
            FStructProperty::DateTime(..)
            | FStructProperty::GameplayTagContainer(..)
            | FStructProperty::Guid(..)
            | FStructProperty::IntPoint(..)
            | FStructProperty::LinearColor(..)
            | FStructProperty::Quat(..)
            | FStructProperty::Rotator(..)
            | FStructProperty::Timespan(..)
            | FStructProperty::Vector(..)
            | FStructProperty::Vector2D(..) => FString::from("/Script/CoreUObject"),
            FStructProperty::Unknown(_t, struct_class, _g, _)
            | FStructProperty::Custom(_t, struct_class, _g, _) => struct_class.clone(),
        }
    }

    #[inline]
    pub fn struct_guid(&self) -> FGuid {
        match self {
            FStructProperty::Unknown(_t, _c, guid, _)
            | FStructProperty::Custom(_t, _c, guid, _) => *guid,
            _ => FGuid::invalid(),
        }
    }
}
