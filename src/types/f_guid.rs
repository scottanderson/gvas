use std::{fmt::Display, str::FromStr};

use binrw::binrw;

use crate::error::ParseGuidError;

/// Enumerates known GUID formats.
#[derive(Clone, Copy, Debug)]
pub enum EGuidFormats {
    /// 32 digits.
    ///
    /// For example: "00000000000000000000000000000000"
    Digits,

    /// 32 digits in lowercase
    ///
    /// For example: "0123abc456def789abcd123ef4a5b6c7"
    DigitsLower,

    /// 32 digits separated by hyphens.
    ///
    /// For example: 00000000-0000-0000-0000-000000000000
    DigitsWithHyphens,

    /// 32 digits separated by hyphens, in lowercase as described by RFC 4122.
    ///
    /// For example: bd048ce3-358b-46c5-8cee-627c719418f8
    DigitsWithHyphensLower,

    /// 32 digits separated by hyphens and enclosed in braces.
    ///
    /// For example: {00000000-0000-0000-0000-000000000000}
    DigitsWithHyphensInBraces,

    /// 32 digits separated by hyphens and enclosed in parentheses.
    ///
    /// For example: (00000000-0000-0000-0000-000000000000)
    DigitsWithHyphensInParentheses,

    /// Comma-separated hexadecimal values enclosed in braces.
    ///
    /// For example: {0x00000000,0x0000,0x0000,{0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00}}
    HexValuesInBraces,

    /// This format is currently used by the FUniqueObjectGuid class.
    ///
    /// For example: 00000000-00000000-00000000-00000000
    UniqueObjectGuid,

    /// Base64 characters with dashes and underscores instead of pluses and slashes (respectively)
    ///
    /// For example: AQsMCQ0PAAUKCgQEBAgADQ
    Short,

    /// Base-36 encoded, compatible with case-insensitive OS file systems (such as Windows).
    ///
    /// For example: 1DPF6ARFCM4XH5RMWPU8TGR0J
    Base36Encoded,
}

#[binrw]
#[derive(Copy, Clone, Default, Hash, PartialEq, Eq)]
pub struct FGuid {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl std::fmt::Debug for FGuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_valid() {
            write!(
                f,
                "FGuid::from_u32(0x{:08X}, 0x{:08X}, 0x{:08X}, 0x{:08X})",
                self.a, self.b, self.c, self.d
            )
        } else {
            write!(f, "FGuid::default()")
        }
    }
}

impl FGuid {
    #[inline]
    pub const fn from_u32(a: u32, b: u32, c: u32, d: u32) -> Self {
        Self { a, b, c, d }
    }

    #[inline]
    pub const fn from_u32s(value: [u32; 4]) -> Self {
        let [a, b, c, d] = value;
        Self::from_u32(a, b, c, d)
    }

    #[inline]
    pub const fn from_u8(value: [u8; 16]) -> Self {
        let [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] = value;
        let a = u32::from_le_bytes([a, b, c, d]);
        let b = u32::from_le_bytes([e, f, g, h]);
        let c = u32::from_le_bytes([i, j, k, l]);
        let d = u32::from_le_bytes([m, n, o, p]);
        Self::from_u32(a, b, c, d)
    }

    #[inline]
    pub const fn from_u128(value: u128) -> Self {
        Self::from_u8(value.to_le_bytes())
    }

    #[inline]
    pub const fn to_u8(self) -> [u8; 16] {
        let [a, b, c, d] = self.a.to_le_bytes();
        let [e, f, g, h] = self.b.to_le_bytes();
        let [i, j, k, l] = self.c.to_le_bytes();
        let [m, n, o, p] = self.d.to_le_bytes();
        [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]
    }

    #[inline]
    pub const fn to_u32(self) -> [u32; 4] {
        [self.a, self.b, self.c, self.d]
    }

    #[inline]
    pub const fn to_u128(self) -> u128 {
        u128::from_le_bytes(self.to_u8())
    }

    #[inline]
    pub const fn is_valid(&self) -> bool {
        (self.a | self.b | self.c | self.d) != 0
    }

    pub fn format(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        format: EGuidFormats,
    ) -> std::fmt::Result {
        match format {
            EGuidFormats::DigitsWithHyphens => write!(
                f,
                "{:08X}-{:04X}-{:04X}-{:04X}-{:04X}{:08X}",
                self.a,
                self.b >> 16,
                self.b & 0xFFFF,
                self.c >> 16,
                self.c & 0xFFFF,
                self.d
            ),
            EGuidFormats::DigitsWithHyphensLower => write!(
                f,
                "{:08x}-{:04x}-{:04x}-{:04x}-{:04x}{:08x}",
                self.a,
                self.b >> 16,
                self.b & 0xFFFF,
                self.c >> 16,
                self.c & 0xFFFF,
                self.d
            ),
            EGuidFormats::DigitsWithHyphensInBraces => write!(
                f,
                "{{{:08X}-{:04X}-{:04X}-{:04X}-{:04X}{:08X}}}",
                self.a,
                self.b >> 16,
                self.b & 0xFFFF,
                self.c >> 16,
                self.c & 0xFFFF,
                self.d
            ),
            EGuidFormats::DigitsWithHyphensInParentheses => write!(
                f,
                "({:08X}-{:04X}-{:04X}-{:04X}-{:04X}{:08X})",
                self.a,
                self.b >> 16,
                self.b & 0xFFFF,
                self.c >> 16,
                self.c & 0xFFFF,
                self.d
            ),
            EGuidFormats::HexValuesInBraces => write!(
                f,
                "{{0x{:08X},0x{:04X},0x{:04X},{{0x{:02X},0x{:02X},0x{:02X},0x{:02X},0x{:02X},0x{:02X},0x{:02X},0x{:02X}}}}}",
                self.a,
                self.b >> 16,
                self.b & 0xFFFF,
                self.c >> 24,
                (self.c >> 16) & 0xFF,
                (self.c >> 8) & 0xFF,
                self.c & 0xFF,
                self.d >> 24,
                (self.d >> 16) & 0xFF,
                (self.d >> 8) & 0xFF,
                self.d & 0xFF
            ),
            EGuidFormats::UniqueObjectGuid => write!(
                f,
                "{:08X}-{:08X}-{:08X}-{:08X}",
                self.a, self.b, self.c, self.d
            ),
            // EGuidFormats::Short => write!(f,"{}", base64_url::encode(&self.to_u8())),
            // EGuidFormats::Base36Encoded => todo!(),
            EGuidFormats::DigitsLower => {
                write!(
                    f,
                    "{:08x}{:08x}{:08x}{:08x}",
                    self.a, self.b, self.c, self.d
                )
            }
            EGuidFormats::Digits => {
                write!(
                    f,
                    "{:08X}{:08X}{:08X}{:08X}",
                    self.a, self.b, self.c, self.d
                )
            }
            _ => todo!("{format:?}"),
        }
    }
}

impl Display for FGuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format(f, EGuidFormats::DigitsWithHyphensLower)
    }
}

macro_rules! from {
    ($src:ty, $dst:ty, $con:path) => {
        impl From<$src> for $dst {
            fn from(value: $src) -> Self {
                $con(value)
            }
        }
    };
}

from!(u128, FGuid, Self::from_u128);
from!([u32; 4], FGuid, Self::from_u32s);
from!([u8; 16], FGuid, Self::from_u8);

from!(FGuid, u128, FGuid::to_u128);
from!(FGuid, [u32; 4], FGuid::to_u32);
from!(FGuid, [u8; 16], FGuid::to_u8);

impl FromStr for FGuid {
    type Err = ParseGuidError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace('-', "");
        let s = s.trim();
        let s = s.strip_prefix('{').unwrap_or(s);
        let s = s.strip_suffix('}').unwrap_or(s);
        let s = s.strip_prefix('(').unwrap_or(s);
        let s = s.strip_suffix(')').unwrap_or(s);

        if s == "0" {
            return Ok(Self::default());
        }

        let length = s.len();
        if length != 32 {
            return Err(ParseGuidError::InvalidLength(length));
        }

        let a = u32::from_str_radix(&s[0..8], 16)?;
        let b_hi = u32::from_str_radix(&s[8..12], 16)?;
        let b_lo = u32::from_str_radix(&s[12..16], 16)?;
        let c_hi = u32::from_str_radix(&s[16..20], 16)?;
        let c_lo = u32::from_str_radix(&s[20..24], 16)?;
        let d = u32::from_str_radix(&s[24..32], 16)?;

        let b = (b_hi << 16) | b_lo;
        let c = (c_hi << 16) | c_lo;

        Ok(Self::from_u32(a, b, c, d))
    }
}

#[cfg(test)]
mod test {
    use crate::error::Result;

    use super::*;

    #[test]
    fn display() -> Result<()> {
        let expected = "e4b068ed-f494-42e9-a231-da0b2e46bb41";
        for guid in [
            FGuid::from_u32(0xE4B068ED, 0xF49442E9, 0xA231DA0B, 0x2E46BB41),
            FGuid::from_u128(0x2E46BB41A231DA0BF49442E9E4B068ED),
            FGuid::from_str(expected)?,
        ] {
            assert_eq!(format!("{guid}"), expected);
        }
        Ok(())
    }
}
