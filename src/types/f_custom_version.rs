use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct FCustomVersion {
    pub key: u128,
    pub value: u32,
}
