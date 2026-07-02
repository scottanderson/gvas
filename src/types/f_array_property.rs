use binrw::binrw;

use crate::{
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
    fn array_struct_type_name(&self) -> Result<&str, binrw::Error> {
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
            } => Some(type_name.as_str()),
            _ => None,
        }
        .ok_or_else(|| binrw::Error::AssertFail {
            pos: 0,
            message: format!("array_struct_type_name({self:?})"),
        })
    }

    #[inline]
    fn array_struct_guid(&self) -> Result<FGuid, binrw::Error> {
        match self {
            Self::Some {
                property_tag:
                    PropertyTag::Incomplete {
                        extra: CollectionProperties::Struct { guid, .. },
                        ..
                    },
                ..
            } => Some(*guid),
            _ => None,
        }
        .ok_or_else(|| binrw::Error::AssertFail {
            pos: 0,
            message: format!("array_struct_guid({self:?})"),
        })
    }
}

#[binrw]
#[br(import(format: SerializationFormat, t: &PropertyTag, inner_type: &FString))]
#[bw(import(format: SerializationFormat))]
#[derive(Debug, PartialEq)]
pub enum FArrayProperty {
    #[br(pre_assert(inner_type == NAME_BOOL_PROPERTY))]
    Bool(#[br(count = t.size())] Vec<u8>),

    #[br(pre_assert(inner_type == NAME_BYTE_PROPERTY))]
    Byte(#[br(count = t.size())] Vec<u8>),

    #[br(pre_assert(inner_type == NAME_ENUM_PROPERTY))]
    Enum(
        #[br(calc = t.enum_type()?)]
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

    #[br(pre_assert(inner_type == NAME_STRUCT_PROPERTY && format.property_tag_complete_type_name))]
    #[bw(assert(format.property_tag_complete_type_name))]
    Struct {
        #[br(temp)]
        #[br(calc = t.array_struct_type_binrw()?)]
        #[bw(ignore)]
        meta: (&str, &str, FGuid),

        #[br(calc = meta.0.into())]
        #[bw(ignore)]
        field_name: FString,

        #[br(calc = meta.1.into())]
        #[bw(ignore)]
        type_name: FString,

        #[br(calc = meta.2)]
        #[bw(ignore)]
        struct_guid: FGuid,

        #[br(args(format, t, meta.0, Some(meta.1), meta.2,))]
        #[bw(args(format))]
        values: TArray<FStructProperty>,
    },

    #[br(pre_assert(inner_type == NAME_STRUCT_PROPERTY && !format.property_tag_complete_type_name))]
    #[bw(assert(!format.property_tag_complete_type_name))]
    TaggedStruct {
        #[br(temp)]
        #[bw(try_calc(u32::try_from(values.len())))]
        count: u32,

        #[br(args(format))]
        struct_tag: FPropertyTag,

        #[br(temp)]
        #[br(calc = struct_tag.array_struct_type_name()?)]
        #[bw(ignore)]
        type_name: &str,

        #[br(temp)]
        #[br(calc = struct_tag.array_struct_guid()?)]
        #[bw(ignore)]
        guid: FGuid,

        #[br(count = count)]
        #[br(args { inner: (format, t, type_name, None, guid, ) })]
        #[bw(args(format))]
        values: Vec<FStructProperty>,
    },

    #[br(pre_assert(inner_type == NAME_TEXT_PROPERTY))]
    Text(#[brw(args(format))] TArray<FTextProperty>),

    #[br(pre_assert(false, "ArrayProperty<{}> not yet implemented", inner_type))]
    Unknown(
        #[br(calc = t.clone())]
        #[bw(ignore)]
        PropertyTag,
        #[br(count = t.size())] Vec<u8>,
    ),
}
