use binrw::binrw;

use crate::{
    properties::NAME_NONE,
    types::{FString, TArray},
};

#[binrw]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FPropertyTypeName {
    pub name: FString,
    pub children: TArray<FPropertyTypeName>,
}

impl From<FString> for FPropertyTypeName {
    fn from(name: FString) -> Self {
        Self {
            name,
            children: TArray(vec![]),
        }
    }
}

impl std::fmt::Display for FPropertyTypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name.0.as_deref().unwrap_or(NAME_NONE);
        write!(f, "{}", name)?;
        if !self.children.is_empty() {
            write!(f, "<")?;
            for (i, child) in self.children.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", child)?;
            }
            write!(f, ">")?;
        }
        Ok(())
    }
}
