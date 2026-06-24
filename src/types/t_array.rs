use binrw::{BinRead, BinWrite};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TArray<T>(pub Vec<T>);

impl<T> TArray<T> {
    pub fn empty() -> Self {
        Self(Vec::new())
    }
}

impl<T: BinRead> BinRead for TArray<T>
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
        Ok(TArray(items))
    }
}

impl<T: BinWrite> BinWrite for TArray<T>
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
        let count = self.len();
        let count = u32::try_from(count).map_err(|e| binrw::Error::Custom {
            pos,
            err: Box::new(e),
        })?;
        count.write_options(writer, endian, ())?;
        for item in self.iter() {
            item.write_options(writer, endian, args)?;
        }
        Ok(())
    }
}

impl<T> std::ops::Deref for TArray<T> {
    type Target = Vec<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for TArray<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const N: usize> From<[T; N]> for TArray<T> {
    #[inline]
    fn from(value: [T; N]) -> Self {
        TArray(Vec::from(value))
    }
}

impl<T> From<Vec<T>> for TArray<T> {
    #[inline]
    fn from(value: Vec<T>) -> Self {
        TArray(value)
    }
}
