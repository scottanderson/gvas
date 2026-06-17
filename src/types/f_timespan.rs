use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct FTimespan {
    ticks: i64,
}
