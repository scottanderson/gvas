pub(crate) mod custom_version_map;
pub(crate) mod tagged_properties_map;

#[inline]
pub(crate) fn is_default<T>(value: &T) -> bool
where
    T: Default + PartialEq,
{
    *value == T::default()
}

#[inline]
pub(crate) fn is_false(value: &bool) -> bool {
    !value
}
