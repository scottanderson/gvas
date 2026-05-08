#![allow(unused)]
use binrw::binrw;

mod array_property;
mod property;
mod struct_property;

pub use array_property::*;
pub use property::*;
pub use struct_property::*;

use crate::{
    options::ParsingOptions,
    types::{CollectionProperties, FString, PropertyType},
};

pub const NAME_NONE: &str = "None";

pub const NAME_ARRAY_PROPERTY: &str = "ArrayProperty";
pub const NAME_BOOL_PROPERTY: &str = "BoolProperty";
pub const NAME_BYTE_PROPERTY: &str = "ByteProperty";
pub const NAME_DELEGATE_PROPERTY: &str = "DelegateProperty";
pub const NAME_DOUBLE_PROPERTY: &str = "DoubleProperty";
pub const NAME_ENUM_PROPERTY: &str = "EnumProperty";
pub const NAME_FLOAT_PROPERTY: &str = "FloatProperty";
pub const NAME_INT16_PROPERTY: &str = "Int16Property";
pub const NAME_INT64_PROPERTY: &str = "Int64Property";
pub const NAME_INT8_PROPERTY: &str = "Int8Property";
pub const NAME_INT_PROPERTY: &str = "IntProperty";
pub const NAME_MAP_PROPERTY: &str = "MapProperty";
pub const NAME_MULTICAST_INLINE_DELGATE_PROPERTY: &str = "MulticastInlineDelegateProperty";
pub const NAME_MULTICAST_SPARSE_DELGATE_PROPERTY: &str = "MulticastSparseDelegateProperty";
pub const NAME_NAME_PROPERTY: &str = "NameProperty";
pub const NAME_OBJECT_PROPERTY: &str = "ObjectProperty";
pub const NAME_OPTION_PROPERTY: &str = "OptionProperty";
pub const NAME_SET_PROPERTY: &str = "SetProperty";
pub const NAME_SOFT_OBJECT_PROPERTY: &str = "SoftObjectProperty";
pub const NAME_STRUCT_PROPERTY: &str = "StructProperty";
pub const NAME_STR_PROPERTY: &str = "StrProperty";
pub const NAME_TEXT_PROPERTY: &str = "TextProperty";
pub const NAME_UINT16_PROPERTY: &str = "UInt16Property";
pub const NAME_UINT32_PROPERTY: &str = "UInt32Property";
pub const NAME_UINT64_PROPERTY: &str = "UInt64Property";

// #[binrw] #[derive(Debug)] pub struct BoolProperty();
// #[binrw] #[derive(Debug)] pub struct ByteProperty();

#[binrw]
#[derive(Debug)]
pub struct DelegateProperty {
    pub object: FString,
    pub function_name: FString,
}

#[binrw]
#[derive(Debug)]
pub struct DoubleProperty(pub f64);

#[binrw]
#[derive(Debug)]
pub struct EnumProperty(pub FString);

#[binrw]
#[derive(Debug)]
pub struct FloatProperty(pub f32);

#[binrw]
#[derive(Debug)]
pub struct Int16Property(pub i16);

#[binrw]
#[derive(Debug)]
pub struct Int64Property(pub i64);

#[binrw]
#[derive(Debug)]
pub struct Int8Property(pub i8);

#[binrw]
#[derive(Debug)]
pub struct IntProperty(pub i32);

// #[binrw] #[derive(Debug)] pub struct MapProperty();
// #[binrw] #[derive(Debug)] pub struct MulticastInlineDelegateProperty();
// #[binrw] #[derive(Debug)] pub struct MulticastSparseDelegateProperty();

#[binrw]
#[derive(Debug)]
pub struct NameProperty(pub FString);

#[binrw]
#[derive(Debug)]
pub struct ObjectProperty(pub FString);

// #[binrw] #[derive(Debug)] pub struct OptionProperty();
// #[binrw] #[derive(Debug)] pub struct SetProperty();
// #[binrw] #[derive(Debug)] pub struct SoftObjectProperty();

#[binrw]
#[derive(Debug)]
pub struct StrProperty(pub FString);

// #[binrw] #[derive(Debug)] pub struct TextProperty();

#[binrw]
#[derive(Debug)]
pub struct UInt16Property(u16);

#[binrw]
#[derive(Debug)]
pub struct UInt32Property(u32);

#[binrw]
#[derive(Debug)]
pub struct UInt64Property(u64);
