use binrw::BinRead;

use crate::{
    error::PropertyTagError,
    types::{CollectionProperties, PropertyTag},
};

impl PropertyTag {
    fn bool_value(&self) -> Result<bool, PropertyTagError> {
        match self {
            Self::Incomplete {
                extra: CollectionProperties::Bool { value },
                ..
            } => Ok(*value != 0),
            Self::Complete { flags, .. } => Ok(flags.bool_true()),
            Self::Incomplete { .. } => Err(PropertyTagError::Unsupported(
                "bool_value".into(),
                format!("{self:?}"),
            )),
        }
    }
}

#[derive(BinRead, Debug, PartialEq)]
#[br(import(t: &PropertyTag))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FBoolProperty(#[br(try_calc = t.bool_value())] pub bool);
