use std::fmt::Display;

use binrw::binrw;

#[binrw]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct FGuid {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl FGuid {
    #[inline]
    pub fn new(a: u32, b: u32, c: u32, d: u32) -> Self {
        FGuid { a, b, c, d }
    }

    #[inline]
    pub fn invalid() -> Self {
        Self::new(0, 0, 0, 0)
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        (self.a | self.b | self.c | self.d) != 0
    }
}

impl Display for FGuid {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
