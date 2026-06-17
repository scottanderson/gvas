use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct FLinearColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}
