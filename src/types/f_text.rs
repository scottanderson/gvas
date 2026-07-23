use binrw::binrw;

use crate::{
    format::SerializationFormat,
    types::{FString, TArray, TOptional},
};

#[cfg(feature = "serde")]
use crate::serde::is_default;

#[binrw]
#[brw(import(format: &SerializationFormat))]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FText {
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "is_default"))]
    pub flags: u32,

    #[brw(args(format))]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub history: FTextHistory,
}

#[binrw]
#[brw(import(format: &SerializationFormat))]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FTextHistory {
    // #[brw(magic = -1i8)]
    // #[br(pre_assert(!format.culture_invariant_stability))]
    // Empty(),
    #[brw(magic = -1i8)]
    // #[br(pre_assert(format.culture_invariant_stability))]
    None(#[br(args(format))] FTextHistoryNone),
    #[brw(magic = 0i8)]
    Base(FString, FString, FString),
    // #[brw(magic = 1i8)] NamedFormat(Box<FText>, TArray<(FString, FormatArgumentValue)>),
    // #[brw(magic = 2i8)] OrderedFormat(Box<FText>, TArray<FormatArgumentValue>),
    #[brw(magic = 3i8)]
    ArgumentFormat(
        #[brw(args(format))] Box<FText>,
        #[brw(args(format))] TArray<ArgumentFormatEntry>,
    ),
    #[brw(magic = 4i8)]
    AsNumber(
        #[brw(args(format))] Box<FormatArgumentValue>,
        #[brw(args(format))] TOptional<NumberFormattingOptions>,
        FString,
    ),
    // #[brw(magic = 5i8)] AsPercent(FormatArgumentValue, TOptional<NumberFormattingOptions>, FString),
    // #[brw(magic = 6i8)] AsCurrency(FString, FormatArgumentValue, TOptional<NumberFormattingOptions>, FString),
    // #[brw(magic = 7i8)] AsDate(FDateTime, DateTimeStyle, #[brw(if(format.ftext_history_date_timezone))] FString, FString),
    // #[brw(magic = 8i8)] AsTime(FDateTime, EDateTimeStyle, FString, FString),
    // #[brw(magic = 9i8)] AsDateTime(FDateTime, DateTimeStyle, DateTimeStyle, FString, FString),
    // #[brw(magic = 10i8)] Transform(Box<FText>, TransformType),
    #[brw(magic = 11i8)]
    StringTableEntry(FString, FString),
    // #[brw(magic = 12i8)] TextGenerator(),
    // #[brw(magic = 13i8)] RawText(),
}

#[binrw]
#[br(import(format: &SerializationFormat))]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FTextHistoryNone {
    #[br(pre_assert(!format.culture_invariant_stability()))]
    Old(),
    #[br(pre_assert(format.culture_invariant_stability()))]
    New(TOptional<FString>),
}

#[binrw]
#[brw(import(format: &SerializationFormat))]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArgumentFormatEntry(FString, #[brw(args(format))] FormatArgumentValue);

#[binrw]
#[brw(import(format: &SerializationFormat))]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rustfmt::skip]
pub enum FormatArgumentValue {
    #[brw(magic = 0i8)] Int(#[br(args(format))] FormatArgumentValueInt),
    #[brw(magic = 1i8)] UInt(#[br(args(format))] FormatArgumentValueUInt),
    #[brw(magic = 2i8)] Float(f32),
    #[brw(magic = 3i8)] Double(f64),
    #[brw(magic = 4i8)] Text(#[brw(args(format))] FText),
    #[brw(magic = 5i8)] UI(i32),
}

#[binrw]
#[br(import(format: &SerializationFormat))]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rustfmt::skip]
pub enum FormatArgumentValueInt {
    #[br(pre_assert(!format.text_64bit_support()))] Int32(i32),
    #[br(pre_assert(format.text_64bit_support()))] Int64(i64),
}

#[binrw]
#[br(import(format: &SerializationFormat))]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rustfmt::skip]
pub enum FormatArgumentValueUInt {
    #[br(pre_assert(!format.text_64bit_support()))] UInt32(u32),
    #[br(pre_assert(format.text_64bit_support()))] UInt64(u64),
}

#[binrw]
#[brw(import(format: &SerializationFormat))]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NumberFormattingOptions {
    #[brw(if(format.include_always_sign()))]
    pub always_sign: i32,
    pub use_grouping: i32,
    pub roudning_mode: RoundingMode,
    pub minimum_integral_digits: i32,
    pub maximum_integral_digits: i32,
    pub minimum_fractional_digits: i32,
    pub maximum_fractional_digits: i32,
}

#[binrw]
#[brw(repr(i8))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RoundingMode {
    HalfToEven,
    HalfFromZero,
    HalfToZero,
    FromZero,
    ToZero,
    ToNegativeInfinity,
    ToPositiveInfinity,
}
