use binrw::binrw;

use crate::options::ParsingOptions;

#[binrw]
#[br(import(options: ParsingOptions))]
#[derive(Debug)]
pub enum FVector {
    #[br(pre_assert(!options.large_world_coordinates))]
    VectorF { x: f32, y: f32, z: f32 },
    #[br(pre_assert(options.large_world_coordinates))]
    VectorD { x: f64, y: f64, z: f64 },
}
