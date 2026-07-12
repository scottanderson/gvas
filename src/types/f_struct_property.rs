use binrw::binrw;

use crate::{
    format::SerializationFormat,
    types::{
        FDateTime, FGameplayTagContainer, FGuid, FIntPoint, FLinearColor, FQuat, FRotator, FString,
        FTimespan, FVector, FVector2D, NAME_DATE_TIME, NAME_GAMEPLAY_TAG_CONTAINER, NAME_GUID,
        NAME_INT_POINT, NAME_LINEAR_COLOR, NAME_QUAT, NAME_ROTATOR, NAME_TIMESPAN, NAME_VECTOR,
        NAME_VECTOR2D, PATH__SCRIPT__CORE_U_OBJECT, PropertyTag, TaggedProperties,
    },
};

#[binrw]
#[br(import(format: &SerializationFormat, t: &PropertyTag, struct_type: &str, class_name: Option<&str>, guid: FGuid))]
#[bw(import(format: &SerializationFormat))]
#[derive(Debug, PartialEq)]
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
            Self::DateTime(..) => Some(NAME_DATE_TIME),
            Self::GameplayTagContainer(..) => Some(NAME_GAMEPLAY_TAG_CONTAINER),
            Self::Guid(..) => Some(NAME_GUID),
            Self::IntPoint(..) => Some(NAME_INT_POINT),
            Self::LinearColor(..) => Some(NAME_LINEAR_COLOR),
            Self::Quat(..) => Some(NAME_QUAT),
            Self::Rotator(..) => Some(NAME_ROTATOR),
            Self::Timespan(..) => Some(NAME_TIMESPAN),
            Self::Vector(..) => Some(NAME_VECTOR),
            Self::Vector2D(..) => Some(NAME_VECTOR2D),
            Self::Unknown(struct_type, _c, _g, _) | Self::Custom(struct_type, _c, _g, _) => {
                struct_type.as_deref()
            }
        })
    }

    #[inline]
    pub fn struct_class(&self) -> FString {
        match self {
            Self::DateTime(..)
            | Self::GameplayTagContainer(..)
            | Self::Guid(..)
            | Self::IntPoint(..)
            | Self::LinearColor(..)
            | Self::Quat(..)
            | Self::Rotator(..)
            | Self::Timespan(..)
            | Self::Vector(..)
            | Self::Vector2D(..) => FString::from(PATH__SCRIPT__CORE_U_OBJECT),
            Self::Unknown(_t, struct_class, _g, _) | Self::Custom(_t, struct_class, _g, _) => {
                struct_class.clone()
            }
        }
    }

    #[inline]
    pub fn struct_guid(&self) -> FGuid {
        match self {
            Self::Unknown(_t, _c, guid, _) | Self::Custom(_t, _c, guid, _) => *guid,
            _ => FGuid::default(),
        }
    }
}
