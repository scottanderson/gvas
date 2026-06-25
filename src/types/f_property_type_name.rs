use binrw::binrw;

use crate::types::{FGuid, FString, NAME_ARRAY_PROPERTY, NAME_NONE, NAME_STRUCT_PROPERTY, TArray};

#[binrw]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FPropertyTypeName {
    pub name: FString,
    pub children: TArray<Self>,
}

impl FPropertyTypeName {
    #[inline]
    pub fn with_children(
        name: impl Into<FString>,
        children: impl IntoIterator<Item = Self>,
    ) -> Self {
        Self {
            name: name.into(),
            children: TArray(children.into_iter().collect()),
        }
    }

    pub fn for_struct(
        struct_type: impl Into<FString>,
        class: impl Into<FString>,
        guid: FGuid,
    ) -> Self {
        let has_guid = guid.is_valid();
        let mut children = Vec::with_capacity(1 + has_guid as usize);
        children.push(Self::with_children(struct_type, [Self::from(class)]));
        if has_guid {
            children.push(Self::from(guid.to_string()))
        }
        Self::with_children(NAME_STRUCT_PROPERTY, children)
    }

    pub fn struct_guid(&self) -> Option<FGuid> {
        match self.name.as_deref().unwrap_or_default() {
            NAME_STRUCT_PROPERTY => match self.children.len() {
                1 => None,
                2 => {
                    let second = self.children.get(1)?;
                    let guid = second.name.as_deref()?;
                    std::str::FromStr::from_str(guid).ok()
                }
                _ => todo!("struct_guid({self:?})"),
            },
            _ => None,
        }
    }
}

impl<T: Into<FString>> From<T> for FPropertyTypeName {
    #[inline]
    fn from(name: T) -> Self {
        Self {
            name: name.into(),
            children: TArray::empty(),
        }
    }
}

impl std::fmt::Display for FPropertyTypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name.as_deref().unwrap_or(NAME_NONE);
        write!(f, "{}", name)?;
        if !self.children.is_empty() {
            write!(f, "<")?;
            for (i, child) in self.children.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", child)?;
            }
            write!(f, ">")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use binrw::{BinRead, BinWrite};

    use crate::error::Result;

    use super::*;

    const STRUCT_PROPERTY_DATA: [u8; 0x4B] = [
        0x0F, 0x00, 0x00, 0x00, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x50, 0x72, 0x6F, 0x70, 0x65,
        0x72, 0x74, 0x79, 0x00, 0x01, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x44, 0x79, 0x6E,
        0x61, 0x6D, 0x69, 0x63, 0x53, 0x61, 0x76, 0x65, 0x44, 0x61, 0x74, 0x61, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x2F, 0x53, 0x63, 0x72, 0x69, 0x70, 0x74, 0x2F, 0x44,
        0x79, 0x6E, 0x61, 0x6D, 0x69, 0x63, 0x53, 0x61, 0x76, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    #[test]
    fn structproperty() -> Result<()> {
        let expected = FPropertyTypeName::with_children(
            NAME_STRUCT_PROPERTY,
            [FPropertyTypeName::with_children(
                "DynamicSaveData",
                [FPropertyTypeName::from("/Script/DynamicSave")],
            )],
        );

        let mut reader = Cursor::new(Vec::from(STRUCT_PROPERTY_DATA));
        let read: FPropertyTypeName = BinRead::read_le(&mut reader)?;
        assert_eq!(read, expected);

        let mut writer = Cursor::new(Vec::with_capacity(STRUCT_PROPERTY_DATA.len()));
        BinWrite::write_le(&expected, &mut writer)?;
        let writer = writer.into_inner();
        assert_eq!(writer, STRUCT_PROPERTY_DATA);

        Ok(())
    }
}
