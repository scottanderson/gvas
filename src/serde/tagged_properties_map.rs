use std::collections::HashSet;

use ::serde::{
    Deserialize, Deserializer, Serialize, Serializer, de,
    ser::{Error as _, SerializeMap, SerializeSeq},
};

use crate::types::{FGuid, FProperty, FString, TaggedProperty};

#[inline]
fn is_false(value: &bool) -> bool {
    !value
}

#[inline]
fn is_default_ref<T>(value: &&T) -> bool
where
    T: Default + PartialEq,
{
    *value == &T::default()
}

/// The JSON representation of a tagged property.
///
/// `property_name` is represented by the enclosing map key, and
/// `array_index` is represented by the value's position in its array.
#[derive(Serialize)]
struct TaggedPropertyValueRef<'a> {
    #[serde(skip_serializing_if = "is_false")]
    has_binary_or_native_serialize: bool,

    #[serde(skip_serializing_if = "is_false")]
    has_property_extensions: bool,

    #[serde(skip_serializing_if = "is_default_ref")]
    property_guid: &'a FGuid,

    #[serde(flatten)]
    property: &'a FProperty,
}

impl<'a> From<&'a TaggedProperty> for TaggedPropertyValueRef<'a> {
    fn from(value: &'a TaggedProperty) -> Self {
        Self {
            has_binary_or_native_serialize: value.has_binary_or_native_serialize,
            has_property_extensions: value.has_property_extensions,
            property_guid: &value.property_guid,
            property: &value.property,
        }
    }
}

/// The deserialized JSON representation of one tagged property.
///
/// `property_name` and `array_index` are included only to detect and reject
/// attempts to specify values which must instead be derived from the map key
/// and array position.
#[derive(Deserialize)]
struct TaggedPropertyValue {
    #[serde(default)]
    property_name: Option<de::IgnoredAny>,

    #[serde(default)]
    array_index: Option<de::IgnoredAny>,

    #[serde(default)]
    has_binary_or_native_serialize: bool,

    #[serde(default)]
    has_property_extensions: bool,

    #[serde(default)]
    property_guid: FGuid,

    #[serde(flatten)]
    property: FProperty,
}

impl TaggedPropertyValue {
    fn into_tagged_property<E>(
        self,
        property_name: FString,
        array_index: u32,
    ) -> Result<TaggedProperty, E>
    where
        E: de::Error,
    {
        if self.property_name.is_some() {
            return Err(E::custom(
                "`property_name` is represented by the map key \
                 and must not be present",
            ));
        }

        if self.array_index.is_some() {
            return Err(E::custom(
                "`array_index` is represented by the array position \
                 and must not be present",
            ));
        }

        Ok(TaggedProperty {
            property_name,
            array_index,
            has_binary_or_native_serialize: self.has_binary_or_native_serialize,
            has_property_extensions: self.has_property_extensions,
            property_guid: self.property_guid,
            property: self.property,
        })
    }
}

struct PropertySequence<'a>(&'a [TaggedProperty]);

impl Serialize for PropertySequence<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut sequence = serializer.serialize_seq(Some(self.0.len()))?;

        for property in self.0 {
            let value = TaggedPropertyValueRef::from(property);
            sequence.serialize_element(&value)?;
        }

        sequence.end()
    }
}

pub(crate) fn serialize<S>(properties: &[TaggedProperty], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(None)?;
    let mut seen_names = HashSet::new();
    let mut group_start = 0;

    while group_start < properties.len() {
        let name = properties[group_start]
            .property_name
            .as_deref()
            .ok_or_else(|| S::Error::custom("tagged property names cannot be null"))?;

        if !seen_names.insert(name.to_owned()) {
            return Err(S::Error::custom(format_args!(
                "tagged property {name:?} is not stored contiguously"
            )));
        }

        let mut group_end = group_start + 1;

        while group_end < properties.len()
            && properties[group_end].property_name.as_deref() == Some(name)
        {
            group_end += 1;
        }

        let group = &properties[group_start..group_end];

        for (index, property) in group.iter().enumerate() {
            let expected = u32::try_from(index).map_err(S::Error::custom)?;

            if property.array_index != expected {
                return Err(S::Error::custom(format_args!(
                    "tagged property {name:?} has array_index {}, \
                     expected {expected}",
                    property.array_index
                )));
            }
        }

        if let [property] = group {
            let value = TaggedPropertyValueRef::from(property);
            map.serialize_entry(name, &value)?;
        } else {
            map.serialize_entry(name, &PropertySequence(group))?;
        }

        group_start = group_end;
    }

    map.end()
}

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Vec<TaggedProperty>, D::Error>
where
    D: Deserializer<'de>,
{
    struct TaggedPropertiesVisitor;

    impl<'de> de::Visitor<'de> for TaggedPropertiesVisitor {
        type Value = Vec<TaggedProperty>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a map from property names to properties or arrays of properties")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: de::MapAccess<'de>,
        {
            let mut result = Vec::new();
            let mut seen_names = HashSet::new();

            while let Some(name) = map.next_key::<String>()? {
                if !seen_names.insert(name.clone()) {
                    return Err(de::Error::custom(format_args!(
                        "duplicate tagged property name {name:?}"
                    )));
                }

                /// Accepts a TaggedPropertyValue or an array of the same
                #[allow(clippy::large_enum_variant, reason = "small variant is uncommon")]
                #[derive(Deserialize)]
                #[serde(untagged)]
                enum PropertyValues {
                    One(TaggedPropertyValue),
                    Many(Vec<TaggedPropertyValue>),
                }

                let values = map.next_value::<PropertyValues>()?;

                match values {
                    PropertyValues::One(value) => {
                        result.push(value.into_tagged_property(FString::from(name), 0)?);
                    }

                    PropertyValues::Many(values) => {
                        if values.is_empty() {
                            return Err(de::Error::custom(format_args!(
                                "tagged property {name:?} must contain \
                                 at least one value"
                            )));
                        }

                        result.reserve(values.len());

                        for (index, value) in values.into_iter().enumerate() {
                            let property_name = FString::from(name.as_str());
                            let array_index = u32::try_from(index).map_err(de::Error::custom)?;

                            result.push(value.into_tagged_property(property_name, array_index)?);
                        }
                    }
                }
            }

            Ok(result)
        }
    }

    deserializer.deserialize_map(TaggedPropertiesVisitor)
}
