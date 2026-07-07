use std::{assert_matches, io::Cursor};

use binrw::{BinRead, BinWrite};

use crate::{
    error::Result,
    format::SerializationFormat,
    types::{
        CollectionProperties, FNameProperty, FProperty, NAME_NAME_PROPERTY, PropertyTag,
        PropertyTagIncompleteGuid,
    },
};

#[test]
fn name_property_with_array_index() -> Result<()> {
    let data = vec![
        0x0d, 0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74,
        0x79, 0x00, 0x1d, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00,
        0x51, 0x55, 0x39, 0x31, 0x5f, 0x49, 0x6e, 0x76, 0x65, 0x73, 0x74, 0x69, 0x67, 0x61, 0x74,
        0x65, 0x54, 0x6f, 0x77, 0x65, 0x72, 0x5f, 0x42, 0x32, 0x00,
    ];

    let format = SerializationFormat {
        ftext_history_date_timezone: false,
        property_tag_set_map_support: false,
        property_guid_in_property_tag: true,
        property_tag_complete_type_name: false,
        fsoftobjectpath_remove_asset_path_fnames: false,
        text_64bit_support: false,
        large_world_coordinates: false,
        include_always_sign: false,
        culture_invariant_stability: false,
    };

    // Convert the Vec<u8> to a NameProperty
    let mut cursor = Cursor::new(data);
    let tag = PropertyTag::read_le_args(&mut cursor, (format,))?;
    let prop = FProperty::read_le_args(&mut cursor, (format, &tag))?;

    // Compare the parsed value to its expected value
    assert_matches!(
        tag,
        PropertyTag::Incomplete {
            ref property_type,
            size: 29,
            array_index: 1,
            extra: CollectionProperties::None,
            guid: PropertyTagIncompleteGuid(None),
        } if property_type == NAME_NAME_PROPERTY
    );
    assert_eq!(
        FProperty::Name(FNameProperty("QU91_InvestigateTower_B2".into())),
        prop
    );

    // Convert the NameProperty back to a Vec<u8>
    let mut writer = Cursor::new(Vec::new());
    tag.write_le_args(&mut writer, (format,))?;
    prop.write_le_args(&mut writer, (format,))?;

    // Compare the two Vec<u8>s
    assert_eq!(cursor.get_ref(), writer.get_ref());

    Ok(())
}
