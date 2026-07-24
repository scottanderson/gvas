use crate::{
    format::SerializationFormat,
    types::{EPropertyTagFlags, FProperty, FPropertyTypeName, PropertyTag, TArray},
};
use binrw::binrw;

#[binrw]
#[br(import(format: &SerializationFormat, t: &PropertyTag))]
#[bw(import(format: &SerializationFormat))]
#[derive(Debug, PartialEq)]
pub enum FMapProperty {
    #[br(pre_assert(!t.flags().is_some_and(EPropertyTagFlags::has_binary_or_native_serialize)))]
    Known {
        allocation_flags: u32,

        #[br(try_calc = t.map_key_type())]
        #[bw(ignore)]
        key_type: FPropertyTypeName,

        #[br(try_calc = t.map_value_type())]
        #[bw(ignore)]
        value_type: FPropertyTypeName,

        #[br(args(format, &key_type, &value_type))]
        #[bw(args(format))]
        properties: TArray<MapEntry>,
    },
    Unknown {
        #[br(try_calc = t.map_key_type())]
        #[bw(ignore)]
        key_type: FPropertyTypeName,

        #[br(try_calc = t.map_value_type())]
        #[bw(ignore)]
        value_type: FPropertyTypeName,

        #[br(count = t.size())]
        data: Vec<u8>,
    },
}

impl FMapProperty {
    pub const fn key_type(&self) -> &FPropertyTypeName {
        match self {
            Self::Known { key_type, .. } => key_type,
            Self::Unknown { key_type, .. } => key_type,
        }
    }

    pub const fn value_type(&self) -> &FPropertyTypeName {
        match self {
            Self::Known { value_type, .. } => value_type,
            Self::Unknown { value_type, .. } => value_type,
        }
    }
}

#[binrw]
#[br(import(format: &SerializationFormat, key_type: &FPropertyTypeName, value_type: &FPropertyTypeName))]
#[bw(import(format: &SerializationFormat))]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MapEntry {
    #[br(args(format, &key_type.as_tag(format, 0)))]
    #[bw(args(format))]
    pub key: FProperty,

    #[br(args(format, &value_type.as_tag(format, 0)))]
    #[bw(args(format))]
    pub value: FProperty,
}

#[cfg(feature = "serde")]
mod serde_impl {
    use std::collections::HashSet;

    use serde::{
        Deserialize, Serialize, Serializer,
        ser::{Error as _, SerializeMap, SerializeSeq},
    };
    use serde_with::serde_as;

    use super::{FMapProperty, MapEntry};
    use crate::serde::is_default;
    use crate::types::{
        FIntProperty, FNameProperty, FProperty, FPropertyTypeName, FStrProperty, FString,
        NAME_INT_PROPERTY, NAME_NAME_PROPERTY, NAME_STR_PROPERTY, TArray,
    };

    #[derive(Clone, Copy)]
    enum CompactKind {
        StrStr,
        StrInt,
        StrProperty,
        NameInt,
        NameProperty,
    }

    impl CompactKind {
        const fn field_name(self) -> &'static str {
            match self {
                Self::StrStr => "str_strs",
                Self::StrInt => "str_ints",
                Self::StrProperty => "str_props",
                Self::NameInt => "name_ints",
                Self::NameProperty => "name_props",
            }
        }
    }

    struct CompactEntries<'a> {
        entries: &'a [MapEntry],
        kind: CompactKind,
    }

    impl Serialize for CompactEntries<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut map = serializer.serialize_map(Some(self.entries.len()))?;

            for entry in self.entries {
                match (self.kind, &entry.key, &entry.value) {
                    (
                        CompactKind::StrStr,
                        FProperty::Str {
                            value: FStrProperty(FString(Some(key))),
                        },
                        FProperty::Str {
                            value: FStrProperty(FString(Some(value))),
                        },
                    ) => map.serialize_entry(key, value)?,

                    (
                        CompactKind::StrInt,
                        FProperty::Str {
                            value: FStrProperty(FString(Some(key))),
                        },
                        FProperty::Int {
                            value: FIntProperty(value),
                        },
                    ) => map.serialize_entry(key, value)?,

                    (
                        CompactKind::StrProperty,
                        FProperty::Str {
                            value: FStrProperty(FString(Some(key))),
                        },
                        value,
                    ) => map.serialize_entry(key, value)?,

                    (
                        CompactKind::NameInt,
                        FProperty::Name {
                            value: FNameProperty(FString(Some(key))),
                        },
                        FProperty::Int {
                            value: FIntProperty(value),
                        },
                    ) => map.serialize_entry(key, value)?,

                    (
                        CompactKind::NameProperty,
                        FProperty::Name {
                            value: FNameProperty(FString(Some(key))),
                        },
                        value,
                    ) => map.serialize_entry(key, value)?,

                    _ => {
                        return Err(S::Error::custom(
                            "map entries do not match their declared key/value types",
                        ));
                    }
                }
            }

            map.end()
        }
    }

    struct PropertyTypeRef<'a>(&'a FPropertyTypeName);

    impl Serialize for PropertyTypeRef<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if round_trips(self.0) {
                self.0.name().serialize(serializer)
            } else {
                self.0.serialize(serializer)
            }
        }
    }

    struct GenericEntriesRef<'a>(&'a [MapEntry]);

    impl Serialize for GenericEntriesRef<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut sequence = serializer.serialize_seq(Some(self.0.len()))?;

            for entry in self.0 {
                sequence.serialize_element(&(&entry.key, &entry.value))?;
            }

            sequence.end()
        }
    }

    #[derive(Serialize)]
    struct GenericKnown<'a> {
        #[serde(skip_serializing_if = "is_default")]
        allocation_flags: u32,

        key_type: PropertyTypeRef<'a>,
        value_type: PropertyTypeRef<'a>,

        #[serde(rename = "value")]
        properties: GenericEntriesRef<'a>,
    }

    #[derive(Serialize)]
    struct GenericUnknown<'a> {
        key_type: PropertyTypeRef<'a>,
        value_type: PropertyTypeRef<'a>,
        data: &'a [u8],
    }

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum PropertyTypeRepresentation {
        Storage(FPropertyTypeName),
        Name(String),
    }

    impl PropertyTypeRepresentation {
        fn into_property_type<E>(self) -> Result<FPropertyTypeName, E>
        where
            E: serde::de::Error,
        {
            match self {
                Self::Storage(value) => Ok(value),
                Self::Name(value) => property_type(value),
            }
        }
    }

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum GenericEntries {
        Pairs(Vec<(FProperty, FProperty)>),
        Storage(TArray<MapEntry>),
    }

    impl GenericEntries {
        fn into_entries(self) -> TArray<MapEntry> {
            match self {
                Self::Pairs(values) => TArray(
                    values
                        .into_iter()
                        .map(|(key, value)| MapEntry { key, value })
                        .collect(),
                ),
                Self::Storage(values) => values,
            }
        }
    }

    #[serde_as]
    #[derive(Deserialize)]
    #[serde(bound(deserialize = "V: Deserialize<'de>"))]
    struct OrderedMap<V>(#[serde_as(as = "serde_with::Map<_, _>")] Vec<(String, V)>);

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum MapPropertyRepresentation {
        StrStrs {
            #[serde(default)]
            allocation_flags: u32,
            str_strs: OrderedMap<String>,
        },

        StrInts {
            #[serde(default)]
            allocation_flags: u32,
            str_ints: OrderedMap<i32>,
        },

        StrProperties {
            #[serde(default)]
            allocation_flags: u32,
            value_type: String,
            str_props: OrderedMap<FProperty>,
        },

        NameInts {
            #[serde(default)]
            allocation_flags: u32,
            name_ints: OrderedMap<i32>,
        },

        NameProperties {
            #[serde(default)]
            allocation_flags: u32,
            value_type: String,
            name_props: OrderedMap<FProperty>,
        },

        GenericKnown {
            #[serde(default)]
            allocation_flags: u32,
            key_type: PropertyTypeRepresentation,
            value_type: PropertyTypeRepresentation,

            #[serde(rename = "value")]
            properties: GenericEntries,
        },

        GenericUnknown {
            key_type: PropertyTypeRepresentation,
            value_type: PropertyTypeRepresentation,
            data: Vec<u8>,
        },
    }

    fn round_trips(value: &FPropertyTypeName) -> bool {
        FPropertyTypeName::from_name(value.name()).as_ref() == Some(value)
    }

    const fn string_property(value: String) -> FProperty {
        FProperty::Str {
            value: FStrProperty(FString(Some(value))),
        }
    }

    const fn name_property(value: String) -> FProperty {
        FProperty::Name {
            value: FNameProperty(FString(Some(value))),
        }
    }

    const fn int_property(value: i32) -> FProperty {
        FProperty::Int {
            value: FIntProperty(value),
        }
    }

    const fn property(value: FProperty) -> FProperty {
        value
    }

    fn entries<V>(
        values: OrderedMap<V>,
        key_fn: fn(String) -> FProperty,
        val_fn: fn(V) -> FProperty,
    ) -> TArray<MapEntry> {
        TArray(
            values
                .0
                .into_iter()
                .map(|(key_value, map_value)| MapEntry {
                    key: key_fn(key_value),
                    value: val_fn(map_value),
                })
                .collect(),
        )
    }

    fn property_type<E>(value: String) -> Result<FPropertyTypeName, E>
    where
        E: serde::de::Error,
    {
        FPropertyTypeName::from_name(value.as_str())
            .ok_or_else(|| E::custom(format!("unsupported map property type {value:?}")))
    }

    fn compact_kind(
        key_type: &FPropertyTypeName,
        value_type: &FPropertyTypeName,
    ) -> Option<CompactKind> {
        match (key_type.name(), value_type.name()) {
            (NAME_STR_PROPERTY, NAME_STR_PROPERTY) => Some(CompactKind::StrStr),
            (NAME_STR_PROPERTY, NAME_INT_PROPERTY) => Some(CompactKind::StrInt),
            (NAME_STR_PROPERTY, _) if round_trips(value_type) => Some(CompactKind::StrProperty),
            (NAME_NAME_PROPERTY, NAME_INT_PROPERTY) => Some(CompactKind::NameInt),
            (NAME_NAME_PROPERTY, _) if round_trips(value_type) => Some(CompactKind::NameProperty),
            _ => None,
        }
    }

    fn compact_key(entry: &MapEntry, kind: CompactKind) -> Option<&str> {
        match (kind, &entry.key) {
            (
                CompactKind::StrStr | CompactKind::StrInt | CompactKind::StrProperty,
                FProperty::Str {
                    value: FStrProperty(FString(Some(key))),
                },
            ) => Some(key),

            (
                CompactKind::NameInt | CompactKind::NameProperty,
                FProperty::Name {
                    value: FNameProperty(FString(Some(key))),
                },
            ) => Some(key),

            _ => None,
        }
    }

    const fn compact_value_matches(entry: &MapEntry, kind: CompactKind) -> bool {
        match kind {
            CompactKind::StrStr => matches!(
                entry.value,
                FProperty::Str {
                    value: FStrProperty(FString(Some(_))),
                }
            ),

            CompactKind::StrInt | CompactKind::NameInt => {
                matches!(entry.value, FProperty::Int { .. })
            }

            CompactKind::StrProperty | CompactKind::NameProperty => true,
        }
    }

    /// Compact JSON objects cannot faithfully represent null keys, duplicate
    /// keys, or entries that disagree with their declared key/value types.
    fn can_serialize_compactly(entries: &[MapEntry], kind: CompactKind) -> bool {
        let mut keys = HashSet::with_capacity(entries.len());

        entries.iter().all(|entry| {
            if let Some(key) = compact_key(entry, kind) {
                compact_value_matches(entry, kind) && keys.insert(key)
            } else {
                false
            }
        })
    }

    impl Serialize for FMapProperty {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Known {
                    allocation_flags,
                    key_type,
                    value_type,
                    properties,
                } => {
                    let entries = properties.as_slice();

                    let Some(kind) = compact_kind(key_type, value_type)
                        .filter(|kind| can_serialize_compactly(entries, *kind))
                    else {
                        return GenericKnown {
                            allocation_flags: *allocation_flags,
                            key_type: PropertyTypeRef(key_type),
                            value_type: PropertyTypeRef(value_type),
                            properties: GenericEntriesRef(entries),
                        }
                        .serialize(serializer);
                    };

                    let has_allocation_flags = *allocation_flags != 0;
                    let has_value_type =
                        matches!(kind, CompactKind::StrProperty | CompactKind::NameProperty);
                    let field_count =
                        usize::from(has_allocation_flags) + usize::from(has_value_type) + 1;
                    let mut map = serializer.serialize_map(Some(field_count))?;

                    if has_allocation_flags {
                        map.serialize_entry("allocation_flags", allocation_flags)?;
                    }

                    if has_value_type {
                        map.serialize_entry("value_type", value_type.name())?;
                    }

                    map.serialize_entry(kind.field_name(), &CompactEntries { entries, kind })?;

                    map.end()
                }

                Self::Unknown {
                    key_type,
                    value_type,
                    data,
                } => GenericUnknown {
                    key_type: PropertyTypeRef(key_type),
                    value_type: PropertyTypeRef(value_type),
                    data,
                }
                .serialize(serializer),
            }
        }
    }

    impl<'de> Deserialize<'de> for FMapProperty {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let representation = MapPropertyRepresentation::deserialize(deserializer)?;

            match representation {
                MapPropertyRepresentation::StrStrs {
                    allocation_flags,
                    str_strs,
                } => Ok(Self::Known {
                    allocation_flags,
                    key_type: FPropertyTypeName::Str,
                    value_type: FPropertyTypeName::Str,
                    properties: entries(str_strs, string_property, string_property),
                }),

                MapPropertyRepresentation::StrInts {
                    allocation_flags,
                    str_ints,
                } => Ok(Self::Known {
                    allocation_flags,
                    key_type: FPropertyTypeName::Str,
                    value_type: FPropertyTypeName::Int,
                    properties: entries(str_ints, string_property, int_property),
                }),

                MapPropertyRepresentation::StrProperties {
                    allocation_flags,
                    value_type,
                    str_props,
                } => Ok(Self::Known {
                    allocation_flags,
                    key_type: FPropertyTypeName::Str,
                    value_type: property_type::<D::Error>(value_type)?,
                    properties: entries(str_props, string_property, property),
                }),

                MapPropertyRepresentation::NameInts {
                    allocation_flags,
                    name_ints,
                } => Ok(Self::Known {
                    allocation_flags,
                    key_type: FPropertyTypeName::Name,
                    value_type: FPropertyTypeName::Int,
                    properties: entries(name_ints, name_property, int_property),
                }),

                MapPropertyRepresentation::NameProperties {
                    allocation_flags,
                    value_type,
                    name_props,
                } => Ok(Self::Known {
                    allocation_flags,
                    key_type: FPropertyTypeName::Name,
                    value_type: property_type::<D::Error>(value_type)?,
                    properties: entries(name_props, name_property, property),
                }),

                MapPropertyRepresentation::GenericKnown {
                    allocation_flags,
                    key_type,
                    value_type,
                    properties,
                } => Ok(Self::Known {
                    allocation_flags,
                    key_type: key_type.into_property_type::<D::Error>()?,
                    value_type: value_type.into_property_type::<D::Error>()?,
                    properties: properties.into_entries(),
                }),

                MapPropertyRepresentation::GenericUnknown {
                    key_type,
                    value_type,
                    data,
                } => Ok(Self::Unknown {
                    key_type: key_type.into_property_type::<D::Error>()?,
                    value_type: value_type.into_property_type::<D::Error>()?,
                    data,
                }),
            }
        }
    }
}
