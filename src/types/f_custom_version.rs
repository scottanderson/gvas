use binrw::binrw;

#[binrw]
#[derive(Debug)]
pub struct FCustomVersion {
    pub key: u128,
    pub value: u32,
}

// pub type FCustomVersionArray = Vec<FCustomVersion>;

#[binrw]
#[br(import(custom_version_format: i32))]
#[derive(Debug, Default)]
pub struct FCustomVersionContainer {
    #[bw(try_calc(i32::try_from(custom_versions.len())))]
    custom_version_length: i32,

    #[br(count = custom_version_length)]
    custom_versions: Vec<FCustomVersion>,
}

impl FCustomVersionContainer {
    pub fn get(&self, version: u128) -> u32 {
        self.custom_versions
            .iter()
            .find(|v| v.key == version)
            .map(|v| v.value)
            .unwrap_or(0)
    }
}
