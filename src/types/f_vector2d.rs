use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct FVector2D {
    x: f64,
    y: f64,
}
