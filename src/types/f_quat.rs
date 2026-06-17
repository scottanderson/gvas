use binrw::binrw;

use crate::options::ParsingOptions;

#[binrw]
#[br(import(options: ParsingOptions))]
#[derive(Debug)]
pub enum FQuat {
    #[br(pre_assert(!options.large_world_coordinates))]
    QuatF { x: f32, y: f32, z: f32, w: f32 },
    #[br(pre_assert(options.large_world_coordinates))]
    QuatD { x: f64, y: f64, z: f64, w: f64 },
}
