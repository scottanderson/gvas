use binrw::binrw;

#[binrw]
#[brw(repr = u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EUnrealEngineObjectUE4Version {
    OldestLoadablePackage = 214,
}
