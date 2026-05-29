use std::num::TryFromIntError;

use binrw::{BinRead, BinWrite};

#[derive(Clone, Debug)]
pub struct StaticArray<T>(pub Vec<T>);

impl<T: BinRead> BinRead for StaticArray<T>
where
    for<'a> T::Args<'a>: Copy,
{
    type Args<'a> = T::Args<'a>;

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let count = u32::read_options(reader, endian, ())?;
        let mut items = Vec::with_capacity(count as usize);
        for _ in 0..count {
            items.push(T::read_options(reader, endian, args)?);
        }
        Ok(StaticArray(items))
    }
}

impl<T: BinWrite> BinWrite for StaticArray<T>
where
    for<'a> T::Args<'a>: Copy,
{
    type Args<'a> = T::Args<'a>;

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        let pos = writer.stream_position()?;
        let count = self.0.len();
        let count = u32::try_from(count).map_err(|e| binrw::Error::Custom {
            pos,
            err: Box::new(e),
        })?;
        count.write_options(writer, endian, ())?;
        for item in &self.0 {
            item.write_options(writer, endian, args)?;
        }
        Ok(())
    }
}

impl<T> std::ops::Deref for StaticArray<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for StaticArray<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
