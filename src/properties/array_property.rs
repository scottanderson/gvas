use binrw::binrw;

use crate::{
    options::ParsingOptions,
    properties::{
        CollectionProperties, EnumProperty, FString, FloatProperty, IntProperty,
        NAME_BOOL_PROPERTY, NAME_BYTE_PROPERTY, NAME_ENUM_PROPERTY, NAME_FLOAT_PROPERTY,
        NAME_INT_PROPERTY, NAME_NAME_PROPERTY, NAME_OBJECT_PROPERTY, NAME_SOFT_OBJECT_PROPERTY,
        NAME_STR_PROPERTY, NAME_STRUCT_PROPERTY, NAME_TEXT_PROPERTY, NameProperty, ObjectProperty,
        PropertyType, SoftObjectProperty, StrProperty, StructProperty, TextProperty,
    },
    types::{FPropertyTag, StaticArray},
};

impl FPropertyTag {
    #[inline]
    fn array_struct_type_name(&self) -> Option<String> {
        let FPropertyTag::Some { property_type, .. } = &self else {
            return None;
        };
        let PropertyType::Incomplete { extra, .. } = property_type else {
            return None;
        };
        let CollectionProperties::Struct { type_name, .. } = extra else {
            return None;
        };
        let FString(Some(type_name)) = type_name else {
            return None;
        };
        Some(type_name.to_owned())
    }
}

#[binrw]
#[br(import(options: ParsingOptions, t: &PropertyType, inner_type: &str))]
#[bw(import(options: ParsingOptions))]
#[derive(Debug)]
pub enum ArrayProperty {
    #[br(pre_assert(inner_type == NAME_BOOL_PROPERTY))]
    Bool(#[br(count = t.size())] Vec<u8>),

    #[br(pre_assert(inner_type == NAME_BYTE_PROPERTY))]
    Byte(#[br(count = t.size())] Vec<u8>),

    #[br(pre_assert(inner_type == NAME_ENUM_PROPERTY))]
    Enum(StaticArray<EnumProperty>),

    #[br(pre_assert(inner_type == NAME_FLOAT_PROPERTY))]
    Float(StaticArray<FloatProperty>),

    #[br(pre_assert(inner_type == NAME_INT_PROPERTY))]
    Int(StaticArray<IntProperty>),

    #[br(pre_assert(inner_type == NAME_NAME_PROPERTY))]
    Name(StaticArray<NameProperty>),

    #[br(pre_assert(inner_type == NAME_OBJECT_PROPERTY))]
    Object(StaticArray<ObjectProperty>),

    #[br(pre_assert(inner_type == NAME_SOFT_OBJECT_PROPERTY))]
    SoftObject(#[br(args(options))] StaticArray<SoftObjectProperty>),

    #[br(pre_assert(inner_type == NAME_STR_PROPERTY))]
    Str(StaticArray<StrProperty>),

    #[br(pre_assert(inner_type == NAME_STRUCT_PROPERTY))]
    Struct {
        #[br(temp)]
        #[bw(try_calc(u32::try_from(values.len())))]
        count: u32,

        #[br(temp, if(!options.property_tag_complete_type_name))]
        #[br(args(options))]
        #[bw(calc(None))]
        struct_tag: Option<FPropertyTag>,

        #[br(temp)]
        #[br(calc = struct_tag
            .and_then(|t| t.array_struct_type_name())
            .unwrap_or_else(|| t.array_struct_type_name().expect("array_struct_type_type").to_string()))]
        #[bw(ignore)]
        type_name: String,

        #[br(count = count)]
        #[br(args { inner: (options, &type_name,) })]
        #[bw(args(options))]
        values: Vec<StructProperty>,
    },

    #[br(pre_assert(inner_type == NAME_TEXT_PROPERTY))]
    Text(#[brw(args(options))] StaticArray<TextProperty>),

    #[br(pre_assert(false, "ArrayProperty<{}> not yet implemented", inner_type))]
    Unknown(#[br(count = t.size())] Vec<u8>),
}
