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
    pub const fn from_u32(a: u32, b: u32, c: u32, d: u32) -> Self {
        Self { a, b, c, d }
    }

    #[inline]
    pub const fn from_u128(value: u128) -> Self {
        let [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] = value.to_le_bytes();
        Self {
            a: u32::from_le_bytes([a, b, c, d]),
            b: u32::from_le_bytes([e, f, g, h]),
            c: u32::from_le_bytes([i, j, k, l]),
            d: u32::from_le_bytes([m, n, o, p]),
        }
    }

    #[inline]
    pub const fn to_u128(&self) -> u128 {
        let [a, b, c, d] = self.a.to_le_bytes();
        let [e, f, g, h] = self.b.to_le_bytes();
        let [i, j, k, l] = self.c.to_le_bytes();
        let [m, n, o, p] = self.d.to_le_bytes();
        u128::from_le_bytes([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p])
    }

    #[inline]
    pub fn invalid() -> Self {
        Self::from_u32(0, 0, 0, 0)
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
