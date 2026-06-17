use binrw::binrw;

use crate::{
    options::ParsingOptions,
    types::{FString, TArray, TOption},
};

#[binrw]
#[brw(import(options: ParsingOptions))]
#[derive(Debug)]
pub struct FText {
    flags: u32,
    // #[br(dbg)]
    #[brw(args(options))]
    history: FTextHistory,
}

#[binrw]
#[brw(import(options: ParsingOptions))]
#[derive(Debug)]
pub enum FTextHistory {
    // #[brw(magic = -1i8)]
    // #[br(pre_assert(!options.culture_invariant_stability))]
    // Empty(),
    #[brw(magic = -1i8)]
    // #[br(pre_assert(options.culture_invariant_stability))]
    None(#[br(args(options))] FTextHistoryNone),
    #[brw(magic = 0i8)]
    Base(FString, FString, FString),
    // #[brw(magic = 1i8)] NamedFormat(Box<FText>, TArray<(FString, FormatArgumentValue)>),
    // #[brw(magic = 2i8)] OrderedFormat(Box<FText>, TArray<FormatArgumentValue>),
    #[brw(magic = 3i8)]
    ArgumentFormat(
        #[brw(args(options))] Box<FText>,
        #[brw(args(options))] TArray<ArgumentFormatEntry>,
    ),
    #[brw(magic = 4i8)]
    AsNumber(
        #[brw(args(options))] Box<FormatArgumentValue>,
        #[brw(args(options))] TOption<NumberFormattingOptions>,
        FString,
    ),
    // #[brw(magic = 5i8)] AsPercent(FormatArgumentValue, TOption<NumberFormattingOptions>, FString),
    // #[brw(magic = 6i8)] AsCurrency(FString, FormatArgumentValue, TOption<NumberFormattingOptions>, FString),
    // #[brw(magic = 7i8)] AsDate(FDateTime, DateTimeStyle, #[brw(if(options.ftext_history_date_timezone))] FString, FString),
    // #[brw(magic = 8i8)] AsTime(FDateTime, EDateTimeStyle, FString, FString),
    // #[brw(magic = 9i8)] AsDateTime(FDateTime, DateTimeStyle, DateTimeStyle, FString, FString),
    // #[brw(magic = 10i8)] Transform(Box<FText>, TransformType),
    #[brw(magic = 11i8)]
    StringTableEntry(FString, FString),
    // #[brw(magic = 12i8)] TextGenerator(),
    // #[brw(magic = 13i8)] RawText(),
}

#[binrw]
#[br(import(options: ParsingOptions))]
#[derive(Debug)]
pub enum FTextHistoryNone {
    #[br(pre_assert(!options.culture_invariant_stability))]
    Old(),
    #[br(pre_assert(options.culture_invariant_stability))]
    New(TOption<FString>),
}

#[binrw]
#[brw(import(options: ParsingOptions))]
#[derive(Debug)]
pub struct ArgumentFormatEntry(FString, #[brw(args(options))] FormatArgumentValue);

#[binrw]
#[brw(import(options: ParsingOptions))]
#[derive(Debug)]
#[rustfmt::skip]
pub enum FormatArgumentValue {
    #[brw(magic = 0i8)] Int(#[br(args(options))] FormatArgumentValueInt),
    #[brw(magic = 1i8)] UInt(#[br(args(options))] FormatArgumentValueUInt),
    #[brw(magic = 2i8)] Float(f32),
    #[brw(magic = 3i8)] Double(f64),
    #[brw(magic = 4i8)] Text(#[brw(args(options))] FText),
    #[brw(magic = 5i8)] UI(i32),
}

#[binrw]
#[br(import(options: ParsingOptions))]
#[derive(Debug)]
#[rustfmt::skip]
pub enum FormatArgumentValueInt {
    #[br(pre_assert(!options.text_64bit_support))] Int32(i32),
    #[br(pre_assert(options.text_64bit_support))] Int64(i64),
}

#[binrw]
#[br(import(options: ParsingOptions))]
#[derive(Debug)]
#[rustfmt::skip]
pub enum FormatArgumentValueUInt {
    #[br(pre_assert(!options.text_64bit_support))] UInt32(u32),
    #[br(pre_assert(options.text_64bit_support))] UInt64(u64),
}

#[binrw]
#[brw(import(options: ParsingOptions))]
#[derive(Debug)]
pub struct NumberFormattingOptions {
    #[brw(if(options.include_always_sign))]
    always_sign: i32,
    use_grouping: i32,
    roudning_mode: RoundingMode,
    minimum_integral_digits: i32,
    maximum_integral_digits: i32,
    minimum_fractional_digits: i32,
    maximum_fractional_digits: i32,
}

#[binrw]
#[brw(repr(i8))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RoundingMode {
    HalfToEven,
    HalfFromZero,
    HalfToZero,
    FromZero,
    ToZero,
    ToNegativeInfinity,
    ToPositiveInfinity,
}
