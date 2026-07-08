use binrw::BinRead;

use crate::types::{CollectionProperties, PropertyTag};

impl PropertyTag {
    fn bool_value(&self) -> binrw::BinResult<bool> {
        match self {
            Self::Incomplete {
                extra: CollectionProperties::Bool { value },
                ..
            } => Ok(*value != 0),
            Self::Complete { flags, .. } => Ok(flags.bool_true()),
            Self::Incomplete { .. } => Err(binrw::Error::AssertFail {
                pos: 0,
                message: "BoolProperty type not found".to_string(),
            }),
        }
    }
}

#[derive(BinRead, Debug, PartialEq)]
#[br(import(t: &PropertyTag))]
pub struct FBoolProperty(#[br(calc = t.bool_value()?)] pub bool);
