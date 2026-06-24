use binrw::{BinRead, BinWrite};

#[derive(Debug)]
pub struct TOptional<T>(pub Option<T>);

impl<T: BinRead + std::fmt::Debug> BinRead for TOptional<T>
where
    for<'a> T::Args<'a>: Copy,
{
    type Args<'a> = T::Args<'a>;

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let pos = reader.stream_position()?;
        let count = u32::read_options(reader, endian, ())?;
        let value = match count {
            0 => None,
            1 => Some(T::read_options(reader, endian, args)?),
            n => Err(binrw::Error::Custom {
                pos,
                err: Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("TOption expected 0 or 1 elements, got {n}"),
                )),
            })?,
        };
        Ok(TOptional(value))
    }
}

impl<T: BinWrite> BinWrite for TOptional<T>
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
        match self.as_ref() {
            None => {
                u32::write_options(&0, writer, endian, ())?;
            }
            Some(value) => {
                u32::write_options(&1, writer, endian, ())?;
                T::write_options(value, writer, endian, args)?;
            }
        }
        Ok(())
    }
}

impl<T> std::ops::Deref for TOptional<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for TOptional<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
