//! Serializable Unreal Engine structs.

mod e_editor_object_version;
mod e_property_tag_flags;
mod e_ue5_release_stream_object_version;
mod e_unreal_engine_object_ue4_version;
mod e_unreal_engine_object_ue5_version;
mod f_array_property;
mod f_bool_property;
mod f_byte_property;
mod f_custom_version_container;
mod f_date_time;
mod f_enum_property;
mod f_guid;
mod f_map_property;
mod f_package_file_version;
mod f_property;
mod f_property_tag;
mod f_property_type_name;
mod f_save_game_header;
mod f_set_property;
mod f_soft_object_property;
mod f_string;
mod f_struct_property;
mod f_text;
mod f_text_property;
mod t_array;
mod t_optional;
mod u_save_game;

use binrw::binrw;

use crate::format::SerializationFormat;
pub use crate::types::{
    e_editor_object_version::EEditorObjectVersion,
    e_property_tag_flags::EPropertyTagFlags,
    e_ue5_release_stream_object_version::EUE5ReleaseStreamObjectVersion,
    e_unreal_engine_object_ue4_version::EUnrealEngineObjectUE4Version,
    e_unreal_engine_object_ue5_version::EUnrealEngineObjectUE5Version,
    f_array_property::FArrayProperty,
    f_bool_property::FBoolProperty,
    f_byte_property::FByteProperty,
    f_custom_version_container::{CustomVersion, FCustomVersionArray, FCustomVersionContainer},
    f_enum_property::FEnumProperty,
    f_guid::FGuid,
    f_map_property::{FMapProperty, MapEntry},
    f_package_file_version::FPackageFileVersion,
    f_property::FProperty,
    f_property_tag::{
        CollectionProperties, FPropertyTag, PropertyTag, PropertyTagIncompleteGuid,
        TaggedProperties, TaggedProperty,
    },
    f_property_type_name::FPropertyTypeName,
    f_save_game_header::{FSaveGameHeader, SaveGameFileVersion},
    f_set_property::FSetProperty,
    f_soft_object_property::FSoftObjectProperty,
    f_string::FString,
    f_struct_property::FStructProperty,
    f_text::FText,
    f_text_property::FTextProperty,
    t_array::TArray,
    t_optional::TOptional,
    u_save_game::USaveGame,
};

pub const NAME_NONE: &str = "None";

pub const NAME_ARRAY_PROPERTY: &str = "ArrayProperty";
pub const NAME_BOOL_PROPERTY: &str = "BoolProperty";
pub const NAME_BYTE_PROPERTY: &str = "ByteProperty";
pub const NAME_DELEGATE_PROPERTY: &str = "DelegateProperty";
pub const NAME_DOUBLE_PROPERTY: &str = "DoubleProperty";
pub const NAME_ENUM_PROPERTY: &str = "EnumProperty";
pub const NAME_FIELD_PATH_PROPERTY: &str = "FieldPathProperty";
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

pub const PATH__SCRIPT__CORE_U_OBJECT: &str = "/Script/CoreUObject";

pub const NAME_DATE_TIME: &str = "DateTime";
pub const NAME_GAMEPLAY_TAG_CONTAINER: &str = "GameplayTagContainer";
pub const NAME_GUID: &str = "Guid";
pub const NAME_INT_POINT: &str = "IntPoint";
pub const NAME_LINEAR_COLOR: &str = "LinearColor";
pub const NAME_QUAT: &str = "Quat";
pub const NAME_ROTATOR: &str = "Rotator";
pub const NAME_TIMESPAN: &str = "Timespan";
pub const NAME_VECTOR: &str = "Vector";
pub const NAME_VECTOR2D: &str = "Vector2D";

// #[binrw] #[derive(Debug)] pub struct OptionalProperty(...);

macro_rules! ue_struct {
    ($name:ident, LWC, $($field:ident),+) => {
        #[binrw]
        #[br(import(format: &SerializationFormat))]
        #[derive(Debug, PartialEq)]
        pub enum $name {
            #[br(pre_assert(!format.large_world_coordinates()))]
            F {
                $($field: f32,)+
            },
            #[br(pre_assert( format.large_world_coordinates()))]
            D {
                $($field: f64,)+
            },
        }
    };
    ($name:ident, $($ty:ty),+) => {
        #[binrw]
        #[derive(Debug, PartialEq)]
        pub struct $name(
            $(pub $ty),+
        );
    };
    ($name:ident, $($field_name:ident : $ty:ty),+) => {
        #[binrw]
        #[derive(Debug, PartialEq)]
        pub struct $name {
            $(pub $field_name: $ty),+
        }
    };
}

// Engine structs
ue_struct!(FCustomVersion, key: FGuid, value: u32);
ue_struct!(FEngineVersion, major: u16, minor: u16, patch: u16, change_list: u32, branch: FString);

// Properties
ue_struct!(FDelegateProperty, object: FString, function_name: FString);
ue_struct!(FDoubleProperty, f64);
ue_struct!(FFloatProperty, f32);
ue_struct!(FFieldPathProperty, path: TArray<FString>, resolved_owner: FString);
ue_struct!(FInt16Property, i16);
ue_struct!(FInt64Property, i64);
ue_struct!(FInt8Property, i8);
ue_struct!(FIntProperty, i32);
ue_struct!(FMulticastInlineDelegateProperty, TArray<FDelegateProperty>);
ue_struct!(FMulticastSparseDelegateProperty, TArray<FDelegateProperty>);
ue_struct!(FNameProperty, FString);
ue_struct!(FObjectProperty, FString);
ue_struct!(FStrProperty, FString);
ue_struct!(FUInt16Property, u16);
ue_struct!(FUInt32Property, u32);
ue_struct!(FUInt64Property, u64);

// StructProperty structs
ue_struct!(FDateTime, ticks: i64);
ue_struct!(FGameplayTag, FString);
ue_struct!(FGameplayTagContainer, TArray<FGameplayTag>);
ue_struct!(FIntPoint, x: i32, y: i32);
ue_struct!(FLinearColor, r: f32, g: f32, b: f32, a: f32);
ue_struct!(FQuat, LWC, x, y, z, w);
ue_struct!(FRotator, LWC, pitch, yaw, roll);
ue_struct!(FTimespan, ticks: i64);
ue_struct!(FVector, LWC, x, y, z);
ue_struct!(FVector2D, x: f64, y: f64);
