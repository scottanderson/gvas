mod e_property_tag_flags;
mod f_array_property;
mod f_bool_property;
mod f_custom_version_container;
mod f_date_time;
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

use binrw::binrw;

use crate::options::ParsingOptions;
pub use crate::types::{
    e_property_tag_flags::EPropertyTagFlags,
    f_array_property::FArrayProperty,
    f_bool_property::FBoolProperty,
    f_custom_version_container::FCustomVersionContainer,
    f_guid::FGuid,
    f_map_property::FMapProperty,
    f_package_file_version::FPackageFileVersion,
    f_property::FProperty,
    f_property_tag::{CollectionProperties, FPropertyTag, PropertyType, TaggedProperties},
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

// #[binrw] #[derive(Debug)] pub struct ByteProperty(...);

// #[binrw] #[derive(Debug)] pub struct OptionalProperty(...);

macro_rules! ue_struct {
    ($name:ident, $($ty:ty),+) => {
        #[binrw]
        #[derive(Debug)]
        pub struct $name(
            $(pub $ty),+
        );
    };
    ($name:ident, $($field_name:ident : $ty:ty),+) => {
        #[binrw]
        #[derive(Debug)]
        pub struct $name {
            $(pub $field_name: $ty),+
        }
    };
}

macro_rules! lwc_enum {
    ($name:ident, $($field:ident),+ $(,)?) => {
        #[binrw]
        #[br(import(options: ParsingOptions))]
        #[derive(Debug)]
        pub enum $name {
            #[br(pre_assert(!options.large_world_coordinates))]
            F {
                $($field: f32,)+
            },
            #[br(pre_assert( options.large_world_coordinates))]
            D {
                $($field: f64,)+
            },
        }
    };
}

// Newtype numbers
ue_struct!(FDoubleProperty, f64);
ue_struct!(FFloatProperty, f32);
ue_struct!(FInt16Property, i16);
ue_struct!(FInt64Property, i64);
ue_struct!(FInt8Property, i8);
ue_struct!(FIntProperty, i32);
ue_struct!(FUInt16Property, u16);
ue_struct!(FUInt32Property, u32);
ue_struct!(FUInt64Property, u64);

// Newtype strings
ue_struct!(FEnumProperty, FString);
ue_struct!(FGameplayTag, FString);
ue_struct!(FNameProperty, FString);
ue_struct!(FObjectProperty, FString);
ue_struct!(FStrProperty, FString);

// Newtype arrays
ue_struct!(FGameplayTagContainer, TArray<FGameplayTag>);
ue_struct!(FMulticastInlineDelegateProperty, TArray<FDelegateProperty>);
ue_struct!(FMulticastSparseDelegateProperty, TArray<FDelegateProperty>);

// Structs
ue_struct!(FCustomVersion, key: u128, value: u32);
ue_struct!(FDateTime, ticks: i64);
ue_struct!(FDelegateProperty, object: FString, function_name: FString);
ue_struct!(FEngineVersion, major: u16, minor: u16, patch: u16, change_list: u32, branch: FString);
ue_struct!(FIntPoint, x: i32, y: i32);
ue_struct!(FLinearColor, r: f32, g: f32, b: f32, a: f32);
ue_struct!(FTimespan, ticks: i64);
ue_struct!(FVector2D, x: f64, y: f64);

// LWC-aware enums
lwc_enum!(FQuat, x, y, z, w);
lwc_enum!(FRotator, pitch, yaw, roll);
lwc_enum!(FVector, x, y, z);

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct SaveGameFile {
    pub header: FSaveGameHeader,

    #[br(temp, calc(ParsingOptions::from(&header)))]
    #[bw(calc(ParsingOptions::from(header)))]
    options: ParsingOptions,

    #[brw(if(options.property_tag_complete_type_name))]
    #[br(temp, assert(spacer == 0))]
    #[bw(calc(0))]
    spacer: u8,

    #[brw(args(options))]
    pub properties: TaggedProperties,

    #[br(temp, assert(footer == 0))]
    #[bw(calc(0))]
    footer: u32,
}

#[cfg(test)]
mod test {
    use std::io::{Read, Seek};
    use std::{fs::File, io::Cursor, path::Path};

    use binrw::BinRead;

    use crate::error::Result;
    use crate::types::SaveGameFile;

    fn test_save_game_file<P: AsRef<Path> + std::fmt::Debug>(path: P) -> Result<()> {
        // Open
        let mut file = File::open(&path)?;

        // Read
        let mut buf = Vec::new();
        let len = file.read_to_end(&mut buf)?;

        // Parse
        let mut cursor = Cursor::new(buf);
        let _result = SaveGameFile::read(&mut cursor)?;
        assert_eq!(len as u64, cursor.stream_position()?);

        // TODO: Write
        // let mut buf2 = vec![0u8; len];
        // let mut cursor2 = Cursor::new(buf2);
        // SaveGameFile::write(&result, &mut cursor2)?;

        // TODO: Compare
        // let buf = cursor.into_inner();
        // let buf2 = cursor2.into_inner();
        // assert!(buf == buf2);

        // Success
        Ok(())
    }

    #[test]
    fn test_regression_01_bin() -> Result<()> {
        test_save_game_file("tests/resources/regression_01.bin")
    }

    #[test]
    fn test_options_sav() -> Result<()> {
        test_save_game_file("tests/resources/Options.sav")
    }

    #[test]
    fn test_enum_array_sav() -> Result<()> {
        test_save_game_file("tests/resources/enum_array.sav")
    }

    #[test]
    fn test_component8_sav() -> Result<()> {
        test_save_game_file("tests/resources/component8.sav")
    }

    #[test]
    fn test_complete_property_tag_sav() -> Result<()> {
        test_save_game_file("tests/resources/complete_property_tag.sav")
    }

    #[test]
    fn test_slot2_sav() -> Result<()> {
        test_save_game_file("tests/resources/Slot2.sav")
    }

    #[test]
    fn test_features_01_bin() -> Result<()> {
        test_save_game_file("tests/resources/features_01.bin")
    }

    #[test]
    fn test_profile_0_sav() -> Result<()> {
        test_save_game_file("tests/resources/Profile_0.sav")
    }

    #[test]
    fn test_save_slot_03_sav() -> Result<()> {
        test_save_game_file("tests/resources/SaveSlot_03.sav")
    }

    #[test]
    fn test_package_version_524_sav() -> Result<()> {
        test_save_game_file("tests/resources/package_version_524.sav")
    }

    #[test]
    fn test_vector2d_sav() -> Result<()> {
        test_save_game_file("tests/resources/vector2d.sav")
    }

    #[test]
    fn test_delegate_sav() -> Result<()> {
        test_save_game_file("tests/resources/Delegate.sav")
    }

    #[test]
    fn test_slot3_sav() -> Result<()> {
        test_save_game_file("tests/resources/Slot3.sav")
    }

    #[test]
    fn test_assert_failed_sav() -> Result<()> {
        test_save_game_file("tests/resources/assert_failed.sav")
    }

    #[test]
    fn test_slot1_sav() -> Result<()> {
        test_save_game_file("tests/resources/Slot1.sav")
    }

    #[test]
    fn test_tagcontainer_sav() -> Result<()> {
        test_save_game_file("tests/resources/tagcontainer.sav")
    }

    #[test]
    fn test_string_table_entry_sav() -> Result<()> {
        test_save_game_file("tests/resources/string_table_entry.sav")
    }

    #[test]
    fn test_package_version_525_sav() -> Result<()> {
        test_save_game_file("tests/resources/package_version_525.sav")
    }

    #[test]
    fn test_ro_64bit_fav_sav() -> Result<()> {
        test_save_game_file("tests/resources/ro_64bit_fav.sav")
    }

    #[test]
    fn test_transform_sav() -> Result<()> {
        test_save_game_file("tests/resources/transform.sav")
    }

    #[test]
    fn test_text_property_noarray_bin() -> Result<()> {
        test_save_game_file("tests/resources/text_property_noarray.bin")
    }
}
