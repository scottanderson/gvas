use binrw::{BinRead, BinWrite, binrw};

use crate::{
    format::SerializationFormat,
    types::{
        CollectionProperties, FGuid, FString, NAME_ARRAY_PROPERTY, NAME_BOOL_PROPERTY,
        NAME_BYTE_PROPERTY, NAME_DELEGATE_PROPERTY, NAME_DOUBLE_PROPERTY, NAME_ENUM_PROPERTY,
        NAME_FLOAT_PROPERTY, NAME_INT_PROPERTY, NAME_INT8_PROPERTY, NAME_INT16_PROPERTY,
        NAME_INT64_PROPERTY, NAME_MAP_PROPERTY, NAME_MULTICAST_INLINE_DELGATE_PROPERTY,
        NAME_MULTICAST_SPARSE_DELGATE_PROPERTY, NAME_NAME_PROPERTY, NAME_OBJECT_PROPERTY,
        NAME_OPTIONAL_PROPERTY, NAME_SET_PROPERTY, NAME_SOFT_OBJECT_PROPERTY, NAME_STR_PROPERTY,
        NAME_STRUCT_PROPERTY, NAME_TEXT_PROPERTY, NAME_UINT16_PROPERTY, NAME_UINT32_PROPERTY,
        NAME_UINT64_PROPERTY, PropertyTag, TArray,
    },
};

#[binrw]
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct RawPropertyTypeName {
    name: FString,
    children: TArray<Self>,
}

impl RawPropertyTypeName {
    #[inline]
    pub(crate) fn from_name(name: impl Into<FString>) -> Self {
        Self {
            name: name.into(),
            children: TArray::empty(),
        }
    }

    #[inline]
    fn with_children(name: impl Into<FString>, children: impl IntoIterator<Item = Self>) -> Self {
        Self {
            name: name.into(),
            children: TArray(children.into_iter().collect()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FPropertyTypeName {
    Array(Box<Self>),
    Bool,
    Byte(Option<FString>),
    Delegate,
    Double,
    Enum {
        #[cfg_attr(
            feature = "serde",
            serde(default = "FString::null", skip_serializing_if = "FString::is_null")
        )]
        enum_class: FString,
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "crate::serde::is_default")
        )]
        class_path: Option<FString>,
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "crate::serde::is_default")
        )]
        inner_type: Option<Box<Self>>,
    },
    Float,
    Int,
    Int8,
    Int16,
    Int64,
    Map {
        key: Box<Self>,
        value: Box<Self>,
    },
    MulticastInlineDelegate,
    MulticastSparseDelegate,
    Name,
    Object,
    Optional(Box<Self>),
    Set(Box<Self>),
    SoftObject,
    Str,
    Struct,
    StructComplete {
        type_name: FString,
        #[cfg_attr(
            feature = "serde",
            serde(default = "FString::null", skip_serializing_if = "FString::is_null")
        )]
        class_name: FString,
        #[cfg_attr(
            feature = "serde",
            serde(default, skip_serializing_if = "crate::serde::is_default")
        )]
        struct_guid: FGuid,
    },
    Text,
    UInt16,
    UInt32,
    UInt64,
}

impl FPropertyTypeName {
    pub fn from_incomplete(name: &FString, extra: &CollectionProperties) -> Option<Self> {
        let result = match extra {
            CollectionProperties::Array { inner_type } => {
                let inner_type = Self::from_name(inner_type.clone())?;
                Self::Array(Box::new(inner_type))
            }
            CollectionProperties::Byte { enum_name } => Self::Byte(match enum_name.is_none() {
                true => None,
                false => Some(enum_name.clone()),
            }),
            CollectionProperties::Enum { enum_name } => Self::Enum {
                enum_class: enum_name.clone(),
                class_path: None,
                inner_type: None,
            },
            CollectionProperties::Map {
                inner_type,
                value_type,
            } => Self::Map {
                key: Box::new(Self::from_name(inner_type.clone())?),
                value: Box::new(Self::from_name(value_type.clone())?),
            },
            CollectionProperties::Option { inner_type } => {
                let inner_type = Self::from_name(inner_type.clone())?;
                Self::Optional(Box::new(inner_type))
            }
            CollectionProperties::Set { inner_type } => {
                Self::Set(Box::new(Self::from_name(inner_type.clone())?))
            }
            CollectionProperties::Struct {
                type_name,
                struct_guid,
            } => Self::StructComplete {
                type_name: type_name.clone(),
                class_name: FString(None),
                struct_guid: *struct_guid,
            },
            CollectionProperties::Bool { .. } | CollectionProperties::None => {
                Self::from_name(name.clone())?
            }
        };
        Some(result)
    }

    pub fn from_name(name: impl Into<FString>) -> Option<Self> {
        Self::from_raw(RawPropertyTypeName::from_name(name))
    }

    pub fn with_children(
        name: impl Into<FString>,
        children: impl IntoIterator<Item = Self>,
    ) -> Option<Self> {
        let children = children.into_iter().map(Self::into_raw);
        Self::from_raw(RawPropertyTypeName::with_children(name, children))
    }

    fn from_raw(raw: RawPropertyTypeName) -> Option<Self> {
        let RawPropertyTypeName { name, children } = raw;
        match (name.as_deref()?, children.len()) {
            (NAME_ARRAY_PROPERTY, 1) => {
                let [inner] = children.0.try_into().ok()?;
                let inner = Self::from_raw(inner)?;
                Some(Self::Array(Box::new(inner)))
            }
            (NAME_BOOL_PROPERTY, 0) => Some(Self::Bool),
            (NAME_BYTE_PROPERTY, 0) => Some(Self::Byte(None)),
            (NAME_BYTE_PROPERTY, 1) => {
                let [enum_node] = children.0.try_into().ok()?;
                enum_node
                    .children
                    .is_empty()
                    .then_some(Self::Byte(Some(enum_node.name)))
            }
            (NAME_DELEGATE_PROPERTY, 0) => Some(Self::Delegate),
            (NAME_DOUBLE_PROPERTY, 0) => Some(Self::Double),
            (NAME_ENUM_PROPERTY, 0..=2) => Self::from_raw_enum(children),
            (NAME_FLOAT_PROPERTY, 0) => Some(Self::Float),
            (NAME_INT_PROPERTY, 0) => Some(Self::Int),
            (NAME_INT8_PROPERTY, 0) => Some(Self::Int8),
            (NAME_INT16_PROPERTY, 0) => Some(Self::Int16),
            (NAME_INT64_PROPERTY, 0) => Some(Self::Int64),
            (NAME_MAP_PROPERTY, 2) => {
                let [key, value] = children.0.try_into().ok()?;
                let key = Box::new(Self::from_raw(key)?);
                let value = Box::new(Self::from_raw(value)?);
                Some(Self::Map { key, value })
            }
            (NAME_MULTICAST_INLINE_DELGATE_PROPERTY, 0) => Some(Self::MulticastInlineDelegate),
            (NAME_MULTICAST_SPARSE_DELGATE_PROPERTY, 0) => Some(Self::MulticastSparseDelegate),
            (NAME_NAME_PROPERTY, 0) => Some(Self::Name),
            (NAME_OBJECT_PROPERTY, 0) => Some(Self::Object),
            (NAME_SET_PROPERTY, 1) => {
                let [inner] = children.0.try_into().ok()?;
                let inner = Self::from_raw(inner)?;
                Some(Self::Set(Box::new(inner)))
            }
            (NAME_SOFT_OBJECT_PROPERTY, 0) => Some(Self::SoftObject),
            (NAME_STR_PROPERTY, 0) => Some(Self::Str),
            (NAME_STRUCT_PROPERTY, 0) => Some(Self::StructComplete {
                type_name: FString(None),
                class_name: FString(None),
                struct_guid: FGuid::default(),
            }),
            (NAME_STRUCT_PROPERTY, 1) => {
                let [type_node] = children.0.try_into().ok()?;
                Self::from_raw_struct(type_node, None)
            }
            (NAME_STRUCT_PROPERTY, 2) => {
                let [type_node, guid_node] = children.0.try_into().ok()?;
                Self::from_raw_struct(type_node, Some(guid_node))
            }
            (NAME_TEXT_PROPERTY, 0) => Some(Self::Text),
            (NAME_UINT16_PROPERTY, 0) => Some(Self::UInt16),
            (NAME_UINT32_PROPERTY, 0) => Some(Self::UInt32),
            (NAME_UINT64_PROPERTY, 0) => Some(Self::UInt64),
            _ => todo!("{name} {children:?}"), // None
        }
    }

    fn from_raw_enum(children: impl IntoIterator<Item = RawPropertyTypeName>) -> Option<Self> {
        let mut children = children.into_iter();
        let Some(enum_node) = children.next() else {
            return Some(Self::Enum {
                enum_class: FString(None),
                class_path: None,
                inner_type: None,
            });
        };
        let inner_type = children.next().and_then(Self::from_raw).map(Box::new);
        if children.next().is_some() {
            return None;
        }

        let mut enum_children = enum_node.children.into_iter();
        let class_path = match enum_children.next() {
            None => None,
            Some(class_path) if class_path.children.is_empty() => Some(class_path.name),
            Some(_) => return None,
        };

        if enum_children.next().is_some() {
            return None;
        }

        Some(Self::Enum {
            enum_class: enum_node.name,
            class_path,
            inner_type,
        })
    }

    fn from_raw_struct(
        type_node: RawPropertyTypeName,
        guid_node: Option<RawPropertyTypeName>,
    ) -> Option<Self> {
        /// Returns `Some(())` if `node.children.is_empty()` and `None` otherwise.
        fn no_children(node: &RawPropertyTypeName) -> Option<()> {
            node.children.is_empty().then_some(())
        }

        let [class_node] = type_node.children.0.try_into().ok()?;
        no_children(&class_node)?;

        let struct_guid = if let Some(guid_node) = guid_node {
            no_children(&guid_node)?;
            guid_node.name.as_deref()?.parse().ok()?
        } else {
            FGuid::default()
        };

        Some(Self::StructComplete {
            type_name: type_node.name,
            class_name: class_node.name,
            struct_guid,
        })
    }

    pub(crate) fn as_tag(&self, format: &SerializationFormat, size: u32) -> PropertyTag {
        if format.property_tag_complete_type_name() {
            let property_type = self.clone();
            PropertyTag::synthetic_complete(property_type, size)
        } else {
            let property_type = FString::from(self.name());
            PropertyTag::synthetic_incomplete(property_type, size)
        }
    }

    fn into_raw(self) -> RawPropertyTypeName {
        // zero children
        fn zero(name: impl Into<FString>) -> RawPropertyTypeName {
            RawPropertyTypeName::from_name(name)
        }
        // one child
        fn one(
            name: impl Into<FString>,
            child: impl Into<RawPropertyTypeName>,
        ) -> RawPropertyTypeName {
            RawPropertyTypeName::with_children(name, [child.into()])
        }
        // two children
        fn two(
            name: impl Into<FString>,
            one: impl Into<RawPropertyTypeName>,
            two: impl Into<RawPropertyTypeName>,
        ) -> RawPropertyTypeName {
            RawPropertyTypeName::with_children(name, [one.into(), two.into()])
        }

        match self {
            Self::Array(i) => one(NAME_ARRAY_PROPERTY, i.into_raw()),
            Self::Bool => zero(NAME_BOOL_PROPERTY),
            Self::Byte(None) => zero(NAME_BYTE_PROPERTY),
            Self::Byte(Some(i)) => one(NAME_BYTE_PROPERTY, zero(i)),
            Self::Delegate => zero(NAME_DELEGATE_PROPERTY),
            Self::Double => zero(NAME_DOUBLE_PROPERTY),
            Self::Enum {
                enum_class,
                class_path,
                inner_type,
            } => {
                let child1 = RawPropertyTypeName::with_children(enum_class, class_path.map(zero));
                let maybe_child2 = inner_type.map(|boxed| (*boxed).into_raw());
                let children = std::iter::once(child1).chain(maybe_child2);
                RawPropertyTypeName::with_children(NAME_ENUM_PROPERTY, children)
            }
            Self::Float => zero(NAME_FLOAT_PROPERTY),
            Self::Int => zero(NAME_INT_PROPERTY),
            Self::Int8 => zero(NAME_INT8_PROPERTY),
            Self::Int16 => zero(NAME_INT16_PROPERTY),
            Self::Int64 => zero(NAME_INT64_PROPERTY),
            Self::Map { key, value } => two(NAME_MAP_PROPERTY, key.into_raw(), value.into_raw()),
            Self::MulticastInlineDelegate => zero(NAME_MULTICAST_INLINE_DELGATE_PROPERTY),
            Self::MulticastSparseDelegate => zero(NAME_MULTICAST_SPARSE_DELGATE_PROPERTY),
            Self::Name => zero(NAME_NAME_PROPERTY),
            Self::Object => zero(NAME_OBJECT_PROPERTY),
            Self::Optional(i) => one(NAME_OPTIONAL_PROPERTY, i.into_raw()),
            Self::Set(i) => one(NAME_SET_PROPERTY, i.into_raw()),
            Self::SoftObject => zero(NAME_SOFT_OBJECT_PROPERTY),
            Self::Str => zero(NAME_STR_PROPERTY),
            Self::Struct => zero(NAME_STRUCT_PROPERTY),
            Self::StructComplete {
                type_name,
                class_name,
                struct_guid,
            } => {
                let type_node = RawPropertyTypeName {
                    name: type_name,
                    children: TArray(vec![RawPropertyTypeName {
                        name: class_name,
                        children: TArray::empty(),
                    }]),
                };

                if struct_guid.is_valid() {
                    two(
                        NAME_STRUCT_PROPERTY,
                        type_node,
                        zero(struct_guid.to_string()),
                    )
                } else {
                    one(NAME_STRUCT_PROPERTY, type_node)
                }
            }
            Self::Text => zero(NAME_TEXT_PROPERTY),
            Self::UInt16 => zero(NAME_UINT16_PROPERTY),
            Self::UInt32 => zero(NAME_UINT32_PROPERTY),
            Self::UInt64 => zero(NAME_UINT64_PROPERTY),
        }
    }

    pub fn enum_class_name(&self) -> &FString {
        match self {
            Self::Byte(Some(enum_class)) | Self::Enum { enum_class, .. } => enum_class,
            _ => todo!("FPropertyTypeName::enum_class_name({self:?})"),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Array(_) => NAME_ARRAY_PROPERTY,
            Self::Bool => NAME_BOOL_PROPERTY,
            Self::Byte(_) => NAME_BYTE_PROPERTY,
            Self::Delegate => NAME_DELEGATE_PROPERTY,
            Self::Double => NAME_DOUBLE_PROPERTY,
            Self::Enum { .. } => NAME_ENUM_PROPERTY,
            Self::Float => NAME_FLOAT_PROPERTY,
            Self::Int => NAME_INT_PROPERTY,
            Self::Int8 => NAME_INT8_PROPERTY,
            Self::Int16 => NAME_INT16_PROPERTY,
            Self::Int64 => NAME_INT64_PROPERTY,
            Self::Map { .. } => NAME_MAP_PROPERTY,
            Self::MulticastInlineDelegate => NAME_MULTICAST_INLINE_DELGATE_PROPERTY,
            Self::MulticastSparseDelegate => NAME_MULTICAST_SPARSE_DELGATE_PROPERTY,
            Self::Name => NAME_NAME_PROPERTY,
            Self::Object => NAME_OBJECT_PROPERTY,
            Self::Optional(_) => NAME_OPTIONAL_PROPERTY,
            Self::Set(_) => NAME_SET_PROPERTY,
            Self::SoftObject => NAME_SOFT_OBJECT_PROPERTY,
            Self::Str => NAME_STR_PROPERTY,
            Self::Struct => NAME_STRUCT_PROPERTY,
            Self::StructComplete { .. } => NAME_STRUCT_PROPERTY,
            Self::Text => NAME_TEXT_PROPERTY,
            Self::UInt16 => NAME_UINT16_PROPERTY,
            Self::UInt32 => NAME_UINT32_PROPERTY,
            Self::UInt64 => NAME_UINT64_PROPERTY,
        }
    }

    pub fn struct_guid(&self) -> Option<FGuid> {
        match self {
            Self::StructComplete { struct_guid, .. } => Some(*struct_guid),
            _ => None,
        }
    }

    pub fn enum_type(&self) -> Option<&Self> {
        match self {
            Self::Array(inner) | Self::Set(inner) => inner.enum_type(),
            Self::Byte(Some(_)) | Self::Enum { .. } => Some(self),
            _ => todo!("{self:?}"),
        }
    }
}

impl BinRead for FPropertyTypeName {
    type Args<'a> = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let pos = reader.stream_position()?;
        let raw = RawPropertyTypeName::read_options(reader, endian, args)?;
        Self::from_raw(raw).ok_or_else(|| binrw::Error::AssertFail {
            pos,
            message: "invalid FPropertyTypeName".to_string(),
        })
    }
}

impl BinWrite for FPropertyTypeName {
    type Args<'a> = ();

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        self.clone().into_raw().write_options(writer, endian, args)
    }
}
