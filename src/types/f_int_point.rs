use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct FIntPoint {
    x: i32,
    y: i32,
}
