use std::{assert_matches, str::FromStr};

use crate::{
    error::Result,
    types::{FGuid, ParseGuidError},
};

const GUID_0_BYTES: [u8; 16] = [0; 16];
const GUID_1_BYTES: [u8; 16] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const GUID_2_BYTES: [u8; 16] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
];
const GUID_3_BYTES: [u8; 16] = [
    0x38, 0x05, 0xEC, 0x1D, 0xD2, 0x5F, 0x45, 0xA9, 0xEC, 0x42, 0xAA, 0x65, 0x80, 0x41, 0x6A, 0xC5,
];

#[test]
fn test_guid_constructor() {
    for (value, expect) in [
        (GUID_0_BYTES, "00000000-0000-0000-0000-000000000000"),
        (GUID_1_BYTES, "00000001-0000-0000-0000-000000000000"),
        (GUID_2_BYTES, "03020100-0706-0504-0b0a-09080f0e0d0c"),
        (GUID_3_BYTES, "1dec0538-a945-5fd2-65aa-42ecc56a4180"),
    ] {
        assert_eq!(format!("{}", FGuid::from(value)), expect);
    }
}

#[test]
fn test_guid_from_u128() {
    for (value, from_u128) in [
        (GUID_0_BYTES, 0),
        (GUID_1_BYTES, 1),
        (GUID_2_BYTES, 0x0f0e_0d0c_0b0a_0908_0706_0504_0302_0100_u128),
        (GUID_3_BYTES, 0xc56a_4180_65aa_42ec_a945_5fd2_1dec_0538_u128),
    ] {
        let guid = FGuid::from(value);
        assert_eq!(u128::from(guid), from_u128);
        assert_eq!(guid, FGuid::from(from_u128));
    }
}

#[test]
fn test_guid_from_u32() {
    for (value, a, b, c, d) in [
        (GUID_0_BYTES, 0, 0, 0, 0),
        (GUID_1_BYTES, 1, 0, 0, 0),
        (GUID_2_BYTES, 0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c),
        (GUID_3_BYTES, 0x1dec0538, 0xa9455fd2, 0x65aa42ec, 0xc56a4180),
    ] {
        let guid = FGuid::from(value);
        assert_eq!(<[u32; 4]>::from(guid), [a, b, c, d]);
        assert_eq!(guid, FGuid::from([a, b, c, d]));
    }
}

#[test]
fn test_guid_from_str() -> Result<()> {
    let expected = FGuid::from_u32(0x3805EC1D, 0xD25F45A9, 0xEC42AA65, 0x80416AC5);
    for valid_guid in [
        "3805EC1D-D25F-45A9-EC42-AA6580416AC5",
        "{3805EC1D-D25F-45A9-EC42-AA6580416AC5}",
        "3805ec1d-d25f-45a9-ec42-aa6580416ac5",
        "{3805ec1d-d25f-45a9-ec42-aa6580416ac5}",
        "3805ec1dd25f45a9ec42aa6580416ac5",
        "{3805ec1dd25f45a9ec42aa6580416ac5}",
    ] {
        let guid = FGuid::from_str(valid_guid)?;
        assert_eq!(guid, expected);
    }
    Ok(())
}

const GUID_TOO_SHORT: &str = "3805ec1d-d25f-45a9-ec42-aa658041";
const GUID_NON_HEX: &str = "x805ec1d-h25f-45a9-ec42-aa6580416ac5";

#[test]
fn test_guid_from_str_invalid() {
    let err = FGuid::from_str(GUID_TOO_SHORT).unwrap_err();
    assert_matches!(err, ParseGuidError::InvalidLength(28),);
    let err = FGuid::from_str(GUID_NON_HEX).unwrap_err();
    assert_matches!(
        err,
        ParseGuidError::ParseIntError(e)
        if format!("{e:?}") == "ParseIntError { kind: InvalidDigit }");
}
