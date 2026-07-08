use binrw::{BinRead, BinWrite, binrw};

use crate::types::{
    FGuid, FString, NAME_ARRAY_PROPERTY, NAME_BOOL_PROPERTY, NAME_BYTE_PROPERTY,
    NAME_DOUBLE_PROPERTY, NAME_ENUM_PROPERTY, NAME_FLOAT_PROPERTY, NAME_INT_PROPERTY,
    NAME_MAP_PROPERTY, NAME_NAME_PROPERTY, NAME_OBJECT_PROPERTY, NAME_SET_PROPERTY,
    NAME_SOFT_OBJECT_PROPERTY, NAME_STR_PROPERTY, NAME_STRUCT_PROPERTY, NAME_TEXT_PROPERTY, TArray,
};

#[binrw]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawPropertyTypeName {
    name: FString,
    children: TArray<Self>,
}

impl RawPropertyTypeName {
    #[inline]
    fn from_name(name: impl Into<FString>) -> Self {
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
pub enum FPropertyTypeName {
    Array(Box<Self>),
    Bool,
    Byte(Option<FString>),
    Double,
    Enum {
        enum_class: FString,
        class_path: Option<FString>,
        inner_type: Option<Box<Self>>,
    },
    Float,
    Int,
    Map {
        key: Box<Self>,
        value: Box<Self>,
    },
    Name,
    Object,
    Set(Box<Self>),
    SoftObject,
    Str,
    Struct {
        type_name: FString,
        class_name: FString,
        struct_guid: FGuid,
    },
    Text,
    Unknown(RawPropertyTypeName),
}

impl FPropertyTypeName {
    pub fn from_name(name: impl Into<FString>) -> Option<Self> {
        Self::from_raw(RawPropertyTypeName {
            name: name.into(),
            children: TArray::empty(),
        })
    }

    pub fn with_children(
        name: impl Into<FString>,
        children: impl IntoIterator<Item = Self>,
    ) -> Option<Self> {
        Self::from_raw(RawPropertyTypeName {
            name: name.into(),
            children: TArray(children.into_iter().map(Self::into_raw).collect()),
        })
    }

    fn from_raw(raw: RawPropertyTypeName) -> Option<Self> {
        let RawPropertyTypeName { name, children } = raw;
        match (name.as_deref()?, children.len()) {
            (NAME_ARRAY_PROPERTY, 1) => {
                let [inner] = children.0.try_into().ok()?;
                let inner = Self::from_raw(inner.clone())?;
                Some(Self::Array(Box::new(inner)))
            }
            (NAME_BOOL_PROPERTY, 0) => Some(Self::Bool),
            (NAME_BYTE_PROPERTY, 0) => Some(Self::Byte(None)),
            (NAME_DOUBLE_PROPERTY, 0) => Some(Self::Double),
            (NAME_ENUM_PROPERTY, 1 | 2) => Self::from_raw_enum(children),
            (NAME_FLOAT_PROPERTY, 0) => Some(Self::Float),
            (NAME_INT_PROPERTY, 0) => Some(Self::Int),
            (NAME_MAP_PROPERTY, 2) => {
                let [key, value] = children.0.try_into().ok()?;
                let key = Box::new(Self::from_raw(key.clone())?);
                let value = Box::new(Self::from_raw(value.clone())?);
                Some(Self::Map { key, value })
            }
            (NAME_NAME_PROPERTY, 0) => Some(Self::Name),
            (NAME_OBJECT_PROPERTY, 0) => Some(Self::Object),
            (NAME_SET_PROPERTY, 1) => {
                let [inner] = children.0.try_into().ok()?;
                let inner = Self::from_raw(inner.clone())?;
                Some(Self::Set(Box::new(inner)))
            }
            (NAME_SOFT_OBJECT_PROPERTY, 0) => Some(Self::SoftObject),
            (NAME_STR_PROPERTY, 0) => Some(Self::Str),
            (NAME_STRUCT_PROPERTY, 1) => {
                let [type_node] = children.0.try_into().ok()?;
                Self::from_raw_struct(type_node, None)
            }
            (NAME_STRUCT_PROPERTY, 2) => {
                let [type_node, guid_node] = children.0.try_into().ok()?;
                Self::from_raw_struct(type_node, Some(guid_node))
            }
            (NAME_TEXT_PROPERTY, 0) => Some(Self::Text),
            _ => unimplemented!("{name}[{}]: {children:#?}", children.len()),
        }
    }

    fn from_raw_enum(children: impl IntoIterator<Item = RawPropertyTypeName>) -> Option<Self> {
        let mut children = children.into_iter();
        let enum_node = children.next()?;
        let inner_type = children.next().and_then(Self::from_raw).map(Box::new);
        if children.next().is_some() {
            return None;
        }

        let mut enum_children = enum_node.children.into_iter();
        let class_path = enum_children
            .next()
            .and_then(|n| n.children.is_empty().then_some(n.name));
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

        let guid = if let Some(guid) = guid_node {
            no_children(&guid)?;
            guid.name.as_deref()?.parse().ok()?
        } else {
            FGuid::default()
        };

        Some(Self::Struct {
            type_name: type_node.name,
            class_name: class_node.name,
            struct_guid: guid,
        })
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
            Self::Map { key, value } => two(NAME_MAP_PROPERTY, key.into_raw(), value.into_raw()),
            Self::Name => zero(NAME_NAME_PROPERTY),
            Self::Object => zero(NAME_OBJECT_PROPERTY),
            Self::Set(i) => one(NAME_SET_PROPERTY, i.into_raw()),
            Self::SoftObject => zero(NAME_SOFT_OBJECT_PROPERTY),
            Self::Str => zero(NAME_STR_PROPERTY),
            Self::Struct {
                type_name,
                class_name,
                struct_guid: guid,
            } => {
                let child1 = RawPropertyTypeName {
                    name: type_name,
                    children: TArray(vec![RawPropertyTypeName {
                        name: class_name,
                        children: TArray::empty(),
                    }]),
                };
                if guid.is_valid() {
                    two(NAME_STRUCT_PROPERTY, child1, zero(guid.to_string()))
                } else {
                    one(NAME_STRUCT_PROPERTY, child1)
                }
            }
            Self::Text => zero(NAME_TEXT_PROPERTY),
            Self::Unknown(raw) => raw,
        }
    }

    pub fn enum_class_name(&self) -> &FString {
        match self {
            Self::Enum { enum_class, .. } => enum_class,
            _ => todo!("{self:?}"),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Array(_) => NAME_ARRAY_PROPERTY,
            Self::Bool => NAME_BOOL_PROPERTY,
            Self::Byte(_) => NAME_BYTE_PROPERTY,
            Self::Double => NAME_DOUBLE_PROPERTY,
            Self::Enum { .. } => NAME_ENUM_PROPERTY,
            Self::Float => NAME_FLOAT_PROPERTY,
            Self::Int => NAME_INT_PROPERTY,
            Self::Map { .. } => NAME_MAP_PROPERTY,
            Self::Name => NAME_NAME_PROPERTY,
            Self::Object => NAME_OBJECT_PROPERTY,
            Self::Set(_) => NAME_SET_PROPERTY,
            Self::SoftObject => NAME_SOFT_OBJECT_PROPERTY,
            Self::Str => NAME_STR_PROPERTY,
            Self::Struct { .. } => NAME_STRUCT_PROPERTY,
            Self::Text => NAME_TEXT_PROPERTY,
            Self::Unknown(RawPropertyTypeName { name, .. }) if let Some(n) = name.as_deref() => n,
            Self::Unknown(RawPropertyTypeName { .. }) => todo!("{self:?}"),
        }
    }

    pub fn struct_guid(&self) -> Option<FGuid> {
        match self {
            Self::Struct {
                struct_guid: guid, ..
            } => Some(*guid),
            _ => None,
        }
    }

    pub fn enum_type(&self) -> Option<&Self> {
        match self {
            Self::Array(inner) => inner.enum_type(),
            Self::Enum { .. } => Some(self),
            Self::Set(inner) => inner.enum_type(),
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
            message: String::new(),
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
