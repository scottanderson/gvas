use serde::{Deserializer, Serializer, de};

use crate::types::{FCustomVersion, FCustomVersionArray, FGuid};

pub fn serialize<S: Serializer>(
    custom_versions: &FCustomVersionArray,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.collect_map(
        custom_versions
            .iter()
            .map(|FCustomVersion { key, value }| (key, value)),
    )
}

pub fn deserialize<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<FCustomVersionArray, D::Error> {
    struct VisitorType;

    impl<'de> de::Visitor<'de> for VisitorType {
        type Value = FCustomVersionArray;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a map of custom version GUIDs to version numbers")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: de::MapAccess<'de>,
        {
            let mut custom_versions = Vec::with_capacity(map.size_hint().unwrap_or_default());

            while let Some((key, value)) = map.next_entry::<FGuid, u32>()? {
                custom_versions.push(FCustomVersion { key, value });
            }

            Ok(custom_versions.into())
        }
    }

    deserializer.deserialize_map(VisitorType)
}
