use binrw::binrw;

use crate::{
    format::SerializationFormat,
    types::{
        FDateTime, FGameplayTagContainer, FGuid, FIntPoint, FLinearColor, FQuat, FRotator, FString,
        FTimespan, FVector, FVector2D, NAME_DATE_TIME, NAME_GAMEPLAY_TAG_CONTAINER, NAME_GUID,
        NAME_INT_POINT, NAME_LINEAR_COLOR, NAME_QUAT, NAME_ROTATOR, NAME_TIMESPAN, NAME_VECTOR,
        NAME_VECTOR2D, PATH__SCRIPT__CORE_U_OBJECT, PropertyType, TaggedProperties,
    },
};

#[binrw]
#[br(import(format: SerializationFormat, t: &PropertyType, struct_type: &str, class_name: Option<&str>, guid: FGuid))]
#[bw(import(format: SerializationFormat))]
#[derive(Debug)]
pub enum FStructProperty {
    #[br(pre_assert(struct_type == NAME_DATE_TIME))]
    DateTime(FDateTime),
    #[br(pre_assert(struct_type == NAME_GAMEPLAY_TAG_CONTAINER))]
    GameplayTagContainer(FGameplayTagContainer),
    #[br(pre_assert(struct_type == NAME_GUID))]
    Guid(FGuid),
    #[br(pre_assert(struct_type == NAME_INT_POINT))]
    IntPoint(FIntPoint),
    #[br(pre_assert(struct_type == NAME_LINEAR_COLOR))]
    LinearColor(FLinearColor),
    #[br(pre_assert(struct_type == NAME_QUAT))]
    Quat(#[br(args(format))] FQuat),
    #[br(pre_assert(struct_type == NAME_ROTATOR))]
    Rotator(#[br(args(format))] FRotator),
    #[br(pre_assert(struct_type == NAME_TIMESPAN))]
    Timespan(FTimespan),
    #[br(pre_assert(struct_type == NAME_VECTOR))]
    Vector(#[br(args(format))] FVector),
    #[br(pre_assert(struct_type == NAME_VECTOR2D))]
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
            FStructProperty::DateTime(..) => Some(NAME_DATE_TIME),
            FStructProperty::GameplayTagContainer(..) => Some(NAME_GAMEPLAY_TAG_CONTAINER),
            FStructProperty::Guid(..) => Some(NAME_GUID),
            FStructProperty::IntPoint(..) => Some(NAME_INT_POINT),
            FStructProperty::LinearColor(..) => Some(NAME_LINEAR_COLOR),
            FStructProperty::Quat(..) => Some(NAME_QUAT),
            FStructProperty::Rotator(..) => Some(NAME_ROTATOR),
            FStructProperty::Timespan(..) => Some(NAME_TIMESPAN),
            FStructProperty::Vector(..) => Some(NAME_VECTOR),
            FStructProperty::Vector2D(..) => Some(NAME_VECTOR2D),
            FStructProperty::Unknown(struct_type, _c, _g, _)
            | FStructProperty::Custom(struct_type, _c, _g, _) => struct_type.as_deref(),
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
            | FStructProperty::Vector2D(..) => FString::from(PATH__SCRIPT__CORE_U_OBJECT),
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
