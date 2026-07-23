use std::io::Cursor;

use binrw::{BinRead, BinWrite, binrw};

use crate::{
    error::binrw_custom,
    format::SerializationFormat,
    types::{
        CollectionProperties, FFloatProperty, FGuid, FIntProperty, FNameProperty, FObjectProperty,
        FPropertyTag, FPropertyTypeName, FSoftObjectProperty, FStrProperty, FString,
        FStructProperty, FTextProperty, NAME_BOOL_PROPERTY, NAME_BYTE_PROPERTY, NAME_ENUM_PROPERTY,
        NAME_FLOAT_PROPERTY, NAME_INT_PROPERTY, NAME_NAME_PROPERTY, NAME_OBJECT_PROPERTY,
        NAME_SOFT_OBJECT_PROPERTY, NAME_STR_PROPERTY, NAME_STRUCT_PROPERTY, NAME_TEXT_PROPERTY,
        PropertyTag, PropertyTagIncompleteGuid, TArray,
    },
};

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
#[cfg_attr(feature = "serde", serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FArrayProperty {
    /// An array of BoolProperty values.
    #[br(pre_assert(inner_type == NAME_BOOL_PROPERTY))]
    Bools {
        #[br(count = t.size())]
        bools: Vec<u8>,
    },

    /// An array of ByteProperty values.
    #[br(pre_assert(inner_type == NAME_BYTE_PROPERTY))]
    Bytes {
        #[cfg_attr(feature = "serde", serde_as(as = "serde_with::hex::Hex"))]
        #[br(count = t.size())]
        bytes: Vec<u8>,
    },

    /// An array of EnumProperty values.
    #[br(pre_assert(inner_type == NAME_ENUM_PROPERTY))]
    Enums {
        #[br(try_calc = t.enum_type())]
        #[bw(ignore)]
        type_name: FPropertyTypeName,
        enums: TArray<FString>,
    },

    /// An array of FloatProperty values.
    #[br(pre_assert(inner_type == NAME_FLOAT_PROPERTY))]
    Floats { floats: TArray<FFloatProperty> },

    /// An array of IntProperty values.
    #[br(pre_assert(inner_type == NAME_INT_PROPERTY))]
    Ints { ints: TArray<FIntProperty> },

    /// An array of NameProperty values.
    #[br(pre_assert(inner_type == NAME_NAME_PROPERTY))]
    Names { names: TArray<FNameProperty> },

    /// An array of ObjectProperty values.
    #[br(pre_assert(inner_type == NAME_OBJECT_PROPERTY))]
    Objects { objects: TArray<FObjectProperty> },

    /// An array of SoftObjectProperty values.
    #[br(pre_assert(inner_type == NAME_SOFT_OBJECT_PROPERTY))]
    SoftObjects {
        #[br(args(format))]
        soft_objects: TArray<FSoftObjectProperty>,
    },

    /// An array of StrProperty values.
    #[br(pre_assert(inner_type == NAME_STR_PROPERTY))]
    Strs { strings: TArray<FStrProperty> },

    /// An array of StructProperty values with complete types.
    #[br(pre_assert(inner_type == NAME_STRUCT_PROPERTY && format.property_tag_complete_type_name()))]
    #[bw(assert(format.property_tag_complete_type_name()))]
    Structs {
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

    /// An array of StructProperty values.
    #[br(pre_assert(inner_type == NAME_STRUCT_PROPERTY && !format.property_tag_complete_type_name()))]
    #[bw(assert(!format.property_tag_complete_type_name()))]
    TaggedStructs(
        #[br(args(format, t))]
        #[bw(args(format))]
        ArrayPropertyTaggedStructs,
    ),

    /// An array of TextProperty values.
    #[br(pre_assert(inner_type == NAME_TEXT_PROPERTY))]
    Texts {
        #[brw(args(format))]
        texts: TArray<FTextProperty>,
    },
}

impl FArrayProperty {
    pub(crate) fn element_property_type_name(&self) -> &str {
        match self {
            Self::Bools { .. } => NAME_BOOL_PROPERTY,
            Self::Bytes { .. } => NAME_BYTE_PROPERTY,
            Self::Enums { .. } => NAME_ENUM_PROPERTY,
            Self::Floats { .. } => NAME_FLOAT_PROPERTY,
            Self::Ints { .. } => NAME_INT_PROPERTY,
            Self::Names { .. } => NAME_NAME_PROPERTY,
            Self::Objects { .. } => NAME_OBJECT_PROPERTY,
            Self::SoftObjects { .. } => NAME_SOFT_OBJECT_PROPERTY,
            Self::Strs { .. } => NAME_STR_PROPERTY,
            Self::Structs { .. } => NAME_STRUCT_PROPERTY,
            Self::TaggedStructs(..) => NAME_STRUCT_PROPERTY,
            Self::Texts { .. } => NAME_TEXT_PROPERTY,
        }
    }
}

impl From<ArrayPropertyTaggedStructs> for FArrayProperty {
    fn from(value: ArrayPropertyTaggedStructs) -> Self {
        Self::TaggedStructs(value)
    }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArrayPropertyTaggedStructs {
    pub field_name: Box<str>,
    pub type_name: Box<str>,

    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "crate::serde::is_default")
    )]
    pub struct_guid: FGuid,

    #[cfg_attr(feature = "serde", serde(rename = "structs"))]
    pub values: Vec<FStructProperty>,
}

impl BinWrite for ArrayPropertyTaggedStructs {
    type Args<'a> = (&'a SerializationFormat,);

    fn write_options<W>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        (format,): Self::Args<'_>,
    ) -> binrw::BinResult<()>
    where
        W: std::io::Write + std::io::Seek,
    {
        let start = writer.stream_position()?;
        let count = u32::try_from(self.values.len()).map_err(binrw_custom(start))?;
        let mut buf = Cursor::new(Vec::new());
        for value in &self.values {
            value.write_options(&mut buf, endian, (format,))?;
        }
        let buf = buf.into_inner();
        let size = u32::try_from(buf.len()).map_err(binrw_custom(start))?;

        let struct_tag = FPropertyTag::Some {
            name: self.field_name.as_ref().into(),
            property_tag: PropertyTag::Incomplete {
                property_type: NAME_STRUCT_PROPERTY.into(),
                size,
                array_index: 0,
                extra: CollectionProperties::Struct {
                    type_name: self.type_name.as_ref().into(),
                    struct_guid: self.struct_guid,
                },
                maybe_property_guid: PropertyTagIncompleteGuid::default(),
            },
        };

        count.write_options(writer, endian, ())?;
        struct_tag.write_options(writer, endian, (format,))?;
        writer.write_all(&buf)?;

        Ok(())
    }
}

impl BinRead for ArrayPropertyTaggedStructs {
    type Args<'a> = (&'a SerializationFormat, &'a PropertyTag);

    fn read_options<R>(
        reader: &mut R,
        endian: binrw::Endian,
        (format, _outer_tag): Self::Args<'_>,
    ) -> binrw::BinResult<Self>
    where
        R: std::io::Read + std::io::Seek,
    {
        let position = reader.stream_position()?;
        let count = u32::read_options(reader, endian, ())?;
        let struct_tag = FPropertyTag::read_options(reader, endian, (format,))?;

        let FPropertyTag::Some {
            name: FString(Some(field_name)),
            property_tag:
                PropertyTag::Incomplete {
                    property_type: FString(Some(property_type)),
                    size,
                    array_index: 0,
                    extra:
                        CollectionProperties::Struct {
                            type_name,
                            struct_guid,
                        },
                    ..
                },
        } = struct_tag
        else {
            return Err(binrw::Error::AssertFail {
                pos: position,
                message: format!("invalid tagged-struct array tag: {struct_tag:?}"),
            });
        };

        if property_type != NAME_STRUCT_PROPERTY {
            return Err(binrw::Error::AssertFail {
                pos: position,
                message: format!("expected {NAME_STRUCT_PROPERTY}, found {property_type}"),
            });
        }

        let type_name = type_name
            .as_deref()
            .ok_or_else(|| binrw::Error::AssertFail {
                pos: position,
                message: "array struct type name cannot be null".into(),
            })?;

        let suggested_size = size.checked_div(count);

        let capacity = usize::try_from(count).map_err(binrw_custom(position))?;
        let mut values = Vec::with_capacity(capacity);

        for _ in 0..count {
            values.push(FStructProperty::read_options(
                reader,
                endian,
                (format, suggested_size, type_name, None, struct_guid),
            )?);
        }

        let field_name = Box::from(field_name);
        let type_name = Box::from(type_name);

        Ok(Self {
            field_name,
            type_name,
            struct_guid,
            values,
        })
    }
}
