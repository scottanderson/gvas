use std::io::{Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{cursor_ext::CursorExt, error::Error};

use super::PropertyTrait;

pub struct StrProperty {
    pub value: String,
}

impl StrProperty {
    pub fn new(value: String) -> Self {
        StrProperty { value }
    }

    pub fn read(cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<Self, Error> {
        if include_header {
            let _length = cursor.read_u64::<LittleEndian>()?;
            cursor.read_exact(&mut [0u8; 1])?;
        }
        let value = cursor.read_string()?;
        Ok(StrProperty { value })
    }
}

impl PropertyTrait for StrProperty {
    fn write(&self, cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<(), Error> {
        if include_header {
            let property_length = self.value.len() + 1 + 4; // 1 is null-byte, 4 is string length field size
            cursor.write_i64::<LittleEndian>(property_length as i64)?;
        }
        
        cursor.write_string(&self.value)?;
        Ok(())
    }
}
