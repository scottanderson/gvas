use binrw::binrw;

use crate::{
    error::PropertyTagError,
    format::SerializationFormat,
    types::{
        CollectionProperties, FFloatProperty, FGuid, FIntProperty, FNameProperty, FObjectProperty,
        FPropertyTag, FPropertyTypeName, FSoftObjectProperty, FStrProperty, FString,
        FStructProperty, FTextProperty, NAME_BOOL_PROPERTY, NAME_BYTE_PROPERTY, NAME_ENUM_PROPERTY,
        NAME_FLOAT_PROPERTY, NAME_INT_PROPERTY, NAME_NAME_PROPERTY, NAME_OBJECT_PROPERTY,
        NAME_SOFT_OBJECT_PROPERTY, NAME_STR_PROPERTY, NAME_STRUCT_PROPERTY, NAME_TEXT_PROPERTY,
        PropertyTag, TArray,
    },
};

impl FPropertyTag {
    #[inline]
    fn array_struct_type_name(&self) -> Result<&str, PropertyTagError> {
        match self {
            Self::Some {
                property_tag:
                    PropertyTag::Incomplete {
                        extra:
                            CollectionProperties::Struct {
                                type_name: FString(Some(type_name)),
                                ..
                            },
                        ..
                    },
                ..
            } => Ok(type_name.as_ref()),
            _ => Err(PropertyTagError::Unsupported(
                "array_struct_type_name".into(),
                format!("{self:?}"),
            )),
        }
    }

    #[inline]
    fn array_struct_guid(&self) -> Result<FGuid, PropertyTagError> {
        match self {
            Self::Some {
                property_tag:
                    PropertyTag::Incomplete {
                        extra: CollectionProperties::Struct { struct_guid, .. },
                        ..
                    },
                ..
            } => Ok(*struct_guid),
            _ => Err(PropertyTagError::Unsupported(
                "array_struct_guid".into(),
                format!("{self:?}"),
            )),
        }
    }

    #[inline]
    fn as_some(&self) -> Result<&PropertyTag, PropertyTagError> {
        match self {
            Self::Some { property_tag, .. } => Ok(property_tag),
            Self::None => Err(PropertyTagError::Unsupported(
                "as_some".into(),
                format!("{self:?}"),
            )),
        }
    }
}

impl PropertyTag {
    #[inline]
    fn array_struct_suggested_size(&self, property_count: u32) -> Option<u32> {
        if property_count > 0
            && let size = self.size()
            && size >= 4
        {
            Some((size - 4) / property_count)
        } else {
            None
        }
    }
}

#[binrw]
#[br(import(format: &SerializationFormat, t: &PropertyTag, inner_type: &str))]
#[bw(import(format: &SerializationFormat))]
#[derive(Debug, PartialEq)]
pub enum FArrayProperty {
    #[br(pre_assert(inner_type == NAME_BOOL_PROPERTY))]
    Bool(#[br(count = t.size())] Vec<u8>),

    #[br(pre_assert(inner_type == NAME_BYTE_PROPERTY))]
    Byte(#[br(count = t.size())] Vec<u8>),

    #[br(pre_assert(inner_type == NAME_ENUM_PROPERTY))]
    Enum(
        #[br(try_calc = t.enum_type())]
        #[bw(ignore)]
        FPropertyTypeName,
        TArray<FString>,
    ),

    #[br(pre_assert(inner_type == NAME_FLOAT_PROPERTY))]
    Float(TArray<FFloatProperty>),

    #[br(pre_assert(inner_type == NAME_INT_PROPERTY))]
    Int(TArray<FIntProperty>),

    #[br(pre_assert(inner_type == NAME_NAME_PROPERTY))]
    Name(TArray<FNameProperty>),

    #[br(pre_assert(inner_type == NAME_OBJECT_PROPERTY))]
    Object(TArray<FObjectProperty>),

    #[br(pre_assert(inner_type == NAME_SOFT_OBJECT_PROPERTY))]
    SoftObject(#[br(args(format))] TArray<FSoftObjectProperty>),

    #[br(pre_assert(inner_type == NAME_STR_PROPERTY))]
    Str(TArray<FStrProperty>),

    #[br(pre_assert(inner_type == NAME_STRUCT_PROPERTY && format.property_tag_complete_type_name()))]
    #[bw(assert(format.property_tag_complete_type_name()))]
    Struct {
        #[br(temp)]
        #[br(try_calc = t.array_struct_type())]
        #[bw(ignore)]
        meta: (&str, &str, FGuid),

        #[br(calc = meta.0.into())]
        #[bw(ignore)]
        type_name: Box<str>,

        #[br(calc = meta.1.into())]
        #[bw(ignore)]
        class_name: Box<str>,

        #[br(calc = meta.2)]
        #[bw(ignore)]
        struct_guid: FGuid,

        #[br(temp, restore_position)]
        #[bw(ignore)]
        property_count: u32,

        #[br(args(format, t.array_struct_suggested_size(property_count), &type_name, Some(&class_name), struct_guid,))]
        #[bw(args(format))]
        values: TArray<FStructProperty>,
    },

    #[br(pre_assert(inner_type == NAME_STRUCT_PROPERTY && !format.property_tag_complete_type_name()))]
    #[bw(assert(!format.property_tag_complete_type_name()))]
    TaggedStruct {
        #[br(temp)]
        #[bw(try_calc(u32::try_from(values.len())))]
        count: u32,

        #[brw(args(format))]
        struct_tag: FPropertyTag,

        #[br(temp)]
        #[br(try_calc = struct_tag.as_some())]
        #[bw(ignore)]
        struct_t: &PropertyTag,

        #[br(temp)]
        #[br(try_calc = struct_tag.array_struct_type_name())]
        #[bw(ignore)]
        type_name: &str,

        #[br(temp)]
        #[br(try_calc = struct_tag.array_struct_guid())]
        #[bw(ignore)]
        struct_guid: FGuid,

        #[br(temp, restore_position)]
        #[bw(ignore)]
        property_count: u32,

        #[br(temp, calc = {
            if property_count > 0 && let size = t.size() && size >= 4 {
                Some((size - 4) / property_count)
            } else {
                None
            }
        })]
        #[bw(ignore)]
        size: Option<u32>,

        #[br(count = count)]
        #[br(args { inner: (format, t.array_struct_suggested_size(property_count), type_name, None, struct_guid, ) })]
        #[bw(args(format))]
        values: Vec<FStructProperty>,
    },

    #[br(pre_assert(inner_type == NAME_TEXT_PROPERTY))]
    Text(#[brw(args(format))] TArray<FTextProperty>),
}

impl FArrayProperty {
    pub(crate) fn element_property_type_name(&self) -> &str {
        match self {
            Self::Bool(..) => NAME_BOOL_PROPERTY,
            Self::Byte(..) => NAME_BYTE_PROPERTY,
            Self::Enum(..) => NAME_ENUM_PROPERTY,
            Self::Float(..) => NAME_FLOAT_PROPERTY,
            Self::Int(..) => NAME_INT_PROPERTY,
            Self::Name(..) => NAME_NAME_PROPERTY,
            Self::Object(..) => NAME_OBJECT_PROPERTY,
            Self::SoftObject(..) => NAME_SOFT_OBJECT_PROPERTY,
            Self::Str(..) => NAME_STR_PROPERTY,
            Self::Struct { .. } => NAME_STRUCT_PROPERTY,
            Self::TaggedStruct { .. } => NAME_STRUCT_PROPERTY,
            Self::Text(..) => NAME_TEXT_PROPERTY,
        }
    }
}
