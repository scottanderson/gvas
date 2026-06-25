use binrw::BinRead;

use crate::types::{CollectionProperties, PropertyType};

impl PropertyType {
    fn bool_value(&self) -> binrw::BinResult<bool> {
        match self {
            Self::Incomplete {
                extra: CollectionProperties::Bool { value },
                ..
            } => Ok(*value != 0),
            Self::Complete { flags, .. } => Ok(flags.bool_true()),
            _ => Err(binrw::Error::AssertFail {
                pos: 0,
                message: "BoolProperty type not found".to_string(),
            }),
        }
    }
}

#[derive(BinRead, Debug)]
#[br(import(t: &PropertyType))]
pub struct FBoolProperty(#[br(calc = t.bool_value()?)] pub bool);
