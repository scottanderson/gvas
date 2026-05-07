mod property;
mod str_property;

pub const NAME_ARRAY_PROPERTY: &str = "ArrayProperty";
pub const NAME_BOOL_PROPERTY: &str = "BoolProperty";
pub const NAME_BYTE_PROPERTY: &str = "ByteProperty";
pub const NAME_ENUM_PROPERTY: &str = "EnumProperty";
pub const NAME_MAP_PROPERTY: &str = "MapProperty";
pub const NAME_OPTION_PROPERTY: &str = "OptionProperty";
pub const NAME_SET_PROPERTY: &str = "SetProperty";
pub const NAME_STRUCT_PROPERTY: &str = "StructProperty";

pub use property::*;
pub use str_property::*;
