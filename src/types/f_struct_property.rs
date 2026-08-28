use std::io::{Read, Seek};

use binrw::{BinRead, BinWrite, VecArgs};

use crate::{
    format::SerializationFormat,
    hints::{HintMap, Path},
    types::{
        FDateTime, FGameplayTagContainer, FGuid, FIntPoint, FLinearColor, FQuat, FRotator, FString,
        FTimespan, FVector, FVector2D, NAME_DATE_TIME, NAME_GAMEPLAY_TAG_CONTAINER, NAME_GUID,
        NAME_INT_POINT, NAME_LINEAR_COLOR, NAME_QUAT, NAME_ROTATOR, NAME_TIMESPAN, NAME_VECTOR,
        NAME_VECTOR2D, PATH__SCRIPT__CORE_U_OBJECT, TaggedProperties,
    },
};

#[cfg(feature = "serde")]
use crate::serde::is_default;

// #[binrw]
// #[br(import(format: &SerializationFormat, size: Option<u32>, struct_type: &FString, class_name: Option<&str>, guid: FGuid, hint_map: &HintMap, path: Path))]
// #[bw(import(format: &SerializationFormat))]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FStructProperty {
    DateTime(FDateTime),
    GameplayTagContainer(FGameplayTagContainer),
    Guid(FGuid),
    IntPoint(FIntPoint),
    LinearColor(FLinearColor),
    Quat(FQuat),
    Rotator(FRotator),
    Timespan(FTimespan),
    Vector(FVector),
    Vector2D(FVector2D),
    Custom {
        #[cfg_attr(
            feature = "serde",
            serde(default = "FString::null", skip_serializing_if = "FString::is_null")
        )]
        struct_type: FString,
        #[cfg_attr(
            feature = "serde",
            serde(default = "FString::null", skip_serializing_if = "FString::is_null")
        )]
        class_name: FString,
        #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "is_default"))]
        struct_guid: FGuid,
        properties: TaggedProperties,
    },
    Unknown {
        struct_type: FString,
        class_name: FString,
        struct_guid: FGuid,
        data: Vec<u8>,
    },
}

type FStructPropertyArgs<'a> = (
    &'a SerializationFormat,
    Option<u32>,
    &'a FString,
    Option<&'a str>,
    FGuid,
    &'a HintMap,
    Path,
);

impl FStructProperty {
    fn read_custom<R: Read + Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        (format, size, struct_type, class_name, guid, hint_map, path): FStructPropertyArgs<'_>,
    ) -> binrw::BinResult<Self> {
        let struct_type = struct_type.clone();
        let class_name: FString = class_name.into();
        let struct_guid = guid;

        Ok(
            match TaggedProperties::read_options(reader, endian, (format, hint_map, path.clone())) {
                Ok(properties) => Self::Custom {
                    struct_type,
                    class_name,
                    struct_guid,
                    properties,
                },
                Err(_) => Self::Unknown {
                    struct_type,
                    class_name,
                    struct_guid,
                    data: Vec::read_options(
                        reader,
                        endian,
                        VecArgs {
                            count: size.unwrap_or(0) as usize,
                            inner: (),
                        },
                    )?,
                },
            },
        )
    }
}

impl BinRead for FStructProperty {
    type Args<'a> = FStructPropertyArgs<'a>;

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        (format, size, struct_type, class_name, guid, hint_map, path): Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        Ok(match struct_type {
            x if x == NAME_DATE_TIME => {
                Self::DateTime(FDateTime::read_options(reader, endian, ())?)
            }
            x if x == NAME_GAMEPLAY_TAG_CONTAINER => {
                Self::GameplayTagContainer(FGameplayTagContainer::read_options(reader, endian, ())?)
            }
            x if x == NAME_GUID => Self::Guid(FGuid::read_options(reader, endian, ())?),
            x if x == NAME_INT_POINT => {
                Self::IntPoint(FIntPoint::read_options(reader, endian, ())?)
            }
            x if x == NAME_LINEAR_COLOR => {
                Self::LinearColor(FLinearColor::read_options(reader, endian, ())?)
            }
            x if x == NAME_QUAT => Self::Quat(FQuat::read_options(reader, endian, (format,))?),
            x if x == NAME_ROTATOR => {
                Self::Rotator(FRotator::read_options(reader, endian, (format,))?)
            }
            x if x == NAME_TIMESPAN => Self::Timespan(FTimespan::read_options(reader, endian, ())?),
            x if x == NAME_VECTOR => {
                Self::Vector(FVector::read_options(reader, endian, (format,))?)
            }
            x if x == NAME_VECTOR2D => Self::Vector2D(FVector2D::read_options(reader, endian, ())?),
            _ => {
                if let Some(hint) = hint_map.get(&path.to_string()) {
                    Self::read_options(
                        reader,
                        endian,
                        (
                            format,
                            size,
                            &hint.clone().into(),
                            class_name,
                            guid,
                            hint_map,
                            Path::recursing(),
                        ),
                    )?
                } else {
                    Self::read_custom(
                        reader,
                        endian,
                        (format, size, struct_type, class_name, guid, hint_map, path),
                    )?
                }
            }
        })
    }
}

impl BinWrite for FStructProperty {
    type Args<'a> = (&'a SerializationFormat,);

    fn write_options<W: std::io::prelude::Write + Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        (format,): Self::Args<'_>,
    ) -> binrw::prelude::BinResult<()> {
        match self {
            FStructProperty::DateTime(e) => e.write_options(writer, endian, ()),
            FStructProperty::GameplayTagContainer(e) => e.write_options(writer, endian, ()),
            FStructProperty::Guid(e) => e.write_options(writer, endian, ()),
            FStructProperty::IntPoint(e) => e.write_options(writer, endian, ()),
            FStructProperty::LinearColor(e) => e.write_options(writer, endian, ()),
            FStructProperty::Quat(e) => e.write_options(writer, endian, ()),
            FStructProperty::Rotator(e) => e.write_options(writer, endian, ()),
            FStructProperty::Timespan(e) => e.write_options(writer, endian, ()),
            FStructProperty::Vector(e) => e.write_options(writer, endian, ()),
            FStructProperty::Vector2D(e) => e.write_options(writer, endian, ()),
            FStructProperty::Custom { properties, .. } => {
                properties.write_options(writer, endian, (format,))
            }
            FStructProperty::Unknown { data, .. } => data.write_options(writer, endian, ()),
        }
    }
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
            Self::Custom { struct_type, .. } => struct_type.as_deref(),
            Self::Unknown { struct_type, .. } => struct_type.as_deref(),
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
            Self::Custom { class_name, .. } => class_name.clone(),
            Self::Unknown { class_name, .. } => class_name.clone(),
        }
    }

    #[inline]
    pub fn struct_guid(&self) -> FGuid {
        match self {
            Self::Unknown { struct_guid, .. } | Self::Custom { struct_guid, .. } => *struct_guid,
            _ => FGuid::default(),
        }
    }
}
