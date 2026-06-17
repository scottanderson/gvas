use binrw::binrw;

use crate::options::ParsingOptions;

#[binrw]
#[br(import(options: ParsingOptions))]
#[derive(Debug)]
pub enum FRotator {
    #[br(pre_assert(!options.large_world_coordinates))]
    RotatorF { pitch: f32, yaw: f32, roll: f32 },
    #[br(pre_assert(options.large_world_coordinates))]
    RotatorD { pitch: f64, yaw: f64, roll: f64 },
}
