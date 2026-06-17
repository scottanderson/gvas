use binrw::binrw;

use crate::{
    options::ParsingOptions,
    types::{FDateTime, FGuid, PropertyType, TaggedProperties},
};

mod structs {
    use binrw::binrw;

    use crate::{
        options::ParsingOptions,
        types::{FString, TArray},
    };

    #[binrw]
    #[derive(Debug)]
    pub struct GameplayTagContainer(TArray<FString>);

    #[binrw]
    #[derive(Debug)]
    pub struct IntPoint(i32, i32);

    #[binrw]
    #[derive(Debug)]
    pub struct LinearColor(f32, f32, f32, f32);

    #[binrw]
    #[br(import(options: ParsingOptions))]
    #[derive(Debug)]
    pub enum Quat {
        #[br(pre_assert(!options.large_world_coordinates))]
        QuatF(f32, f32, f32, f32),
        #[br(pre_assert(options.large_world_coordinates))]
        QuatD(f64, f64, f64, f64),
    }

    #[binrw]
    #[derive(Debug)]
    pub struct Timespan(f64);

    #[binrw]
    #[derive(Debug)]
    pub struct Vector2D(f64, f64);

    #[binrw]
    #[br(import(options: ParsingOptions))]
    #[derive(Debug)]
    pub enum Rotator {
        #[br(pre_assert(!options.large_world_coordinates))]
        RotatorF(f32, f32, f32),
        #[br(pre_assert(options.large_world_coordinates))]
        RotatorD(f64, f64, f64),
    }

    #[binrw]
    #[br(import(options: ParsingOptions))]
    #[derive(Debug)]
    pub enum Vector {
        #[br(pre_assert(!options.large_world_coordinates))]
        VectorF(f32, f32, f32),
        #[br(pre_assert(options.large_world_coordinates))]
        VectorD(f64, f64, f64),
    }
}

pub use structs::*;

#[binrw]
#[br(import(options: ParsingOptions, t: &PropertyType, struct_type: &str))]
#[bw(import(options: ParsingOptions))]
#[derive(Debug)]
pub enum StructProperty {
    #[br(pre_assert(struct_type == "DateTime"))]
    DateTime(FDateTime),
    #[br(pre_assert(struct_type == "GameplayTagContainer"))]
    GameplayTagContainer(GameplayTagContainer),
    #[br(pre_assert(struct_type == "Guid"))]
    Guid(FGuid),
    #[br(pre_assert(struct_type == "IntPoint"))]
    IntPoint(IntPoint),
    #[br(pre_assert(struct_type == "LinearColor"))]
    LinearColor(LinearColor),
    #[br(pre_assert(struct_type == "Quat"))]
    Quat(#[br(args(options))] Quat),
    #[br(pre_assert(struct_type == "Rotator"))]
    Rotator(#[br(args(options))] Rotator),
    #[br(pre_assert(struct_type == "Timespan"))]
    Timespan(Timespan),
    #[br(pre_assert(struct_type == "Vector"))]
    Vector(#[br(args(options))] Vector),
    #[br(pre_assert(struct_type == "Vector2D"))]
    Vector2D(Vector2D),
    Custom(#[brw(args(options))] TaggedProperties),
    Unknown(#[br(count = t.size())] Vec<u8>),
}
