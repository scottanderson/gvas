#[cfg(feature = "hints")]
mod enabled {
    use std::{borrow::Cow, collections::HashMap};

    pub type HintMap = HashMap<String, String>;

    #[derive(Clone, Debug, Default)]
    pub struct Path(Vec<Cow<'static, str>>);

    impl Path {
        pub const fn root() -> Self {
            Self(vec![])
        }
        pub fn child(&self, name: impl Into<Cow<'static, str>>) -> Self {
            let mut p = self.clone();
            p.0.push(name.into());
            p
        }

        /// Returns a special case path for recursing that should never be present in the hashmap
        pub(crate) fn recursing() -> Self {
            Self(vec![Cow::Borrowed("#DO_NOT_ADD_THIS_KEY..")])
        }
    }

    impl std::fmt::Display for Path {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0.join("."))
        }
    }
}
#[cfg(feature = "hints")]
pub use enabled::*;

#[cfg(not(feature = "hints"))]
mod disabled {
    use std::borrow::Cow;

    #[derive(Default)]
    pub struct HintMap;

    impl HintMap {
        pub fn new() -> Self {
            HintMap
        }

        pub fn get(&self, _: &str) -> Option<&String> {
            None
        }
    }

    impl<K, V, const SIZE: usize> From<[(K, V); SIZE]> for HintMap {
        fn from(_: [(K, V); SIZE]) -> Self {
            HintMap
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct Path;

    impl Path {
        pub const fn root() -> Self {
            Self
        }

        pub fn child(&self, _: impl Into<Cow<'static, str>>) -> Self {
            Self
        }

        /// Returns a special case path for recursing that should never be present in the hashmap
        pub(crate) fn recursing() -> Self {
            Self
        }
    }

    impl std::fmt::Display for Path {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "disabled")
        }
    }
}
#[cfg(not(feature = "hints"))]
pub use disabled::*;
