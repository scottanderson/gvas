use binrw::binrw;

use crate::{options::ParsingOptions, types::TaggedProperties};

mod structs {
    use binrw::binrw;

    use crate::{options::ParsingOptions, types::FString};

    #[binrw]
    #[derive(Debug)]
    pub struct DateTime(u64);

    #[binrw]
    #[derive(Debug)]
    pub struct GameplayTagContainer {
        #[br(temp)]
        #[bw(try_calc(u32::try_from(tags.len())))]
        count: u32,
        #[br(count = count)]
        tags: Vec<FString>,
    }

    #[binrw]
    #[derive(Debug)]
    pub struct Guid(u128);

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
#[br(import(options: ParsingOptions, struct_type: &str))]
#[derive(Debug)]
pub enum StructProperty {
    #[br(pre_assert(struct_type == "DateTime"))]
    DateTime(u64),
    #[br(pre_assert(struct_type == "GameplayTagContainer"))]
    GameplayTagContainer(GameplayTagContainer),
    #[br(pre_assert(struct_type == "Guid"))]
    Guid(u128),
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
    Custom(#[br(args(options))] TaggedProperties),
}
