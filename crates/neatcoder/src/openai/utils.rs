use anyhow::{anyhow, Result};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::{marker::PhantomData, ops::Deref};

/// A struct representing a bounded float where the bounds are defined by the `T: MinMax` type parameter.
/// The actual float value is stored in the `inner` field, and the bounds are enforced by the associated
/// constants of the `T` type.
#[derive(Debug, Clone, Copy)]
pub struct BoundedFloat<T> {
    inner: f64,
    _marker: PhantomData<T>,
}

/// A trait defining constants for minimum and maximum values that serve as bounds for `BoundedFloat`.
pub trait MinMax {
    const MIN: f64;
    const MAX: f64;
}

/// A trait for creating a new bounded value, ensuring that the provided value is within the specified bounds.
pub trait Bounded {
    fn new(value: f64) -> Result<Self>
    where
        Self: Sized;
}

impl<T> Bounded for BoundedFloat<T>
where
    T: MinMax,
{
    /// Creates a new `BoundedFloat`, returning an error if the value is not within the bounds specified by `T`.
    fn new(value: f64) -> Result<Self> {
        if T::MIN <= value && value <= T::MAX {
            Ok(Self {
                inner: value,
                _marker: PhantomData,
            })
        } else {
            Err(anyhow!(
                "The value {} is not within the bounds of [{}; {}]",
                value,
                T::MIN,
                T::MAX
            ))
        }
    }
}

/// Implements deference coercion such that we can directly access the
/// inner f64 value
impl<T> Deref for BoundedFloat<T> {
    type Target = f64;

    fn deref(&self) -> &f64 {
        &self.inner
    }
}

impl<T> Serialize for BoundedFloat<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f64(self.inner)
    }
}

impl<'de, T> Deserialize<'de> for BoundedFloat<T>
where
    T: MinMax,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = f64::deserialize(deserializer)?;
        BoundedFloat::<T>::new(value).map_err(de::Error::custom)
    }
}

// Definitions for various range types with specific minimum and maximum values.
#[derive(Debug, Clone, Copy)]
pub struct Range22;
#[derive(Debug, Clone, Copy)]
pub struct Range01;
#[derive(Debug, Clone, Copy)]
pub struct Range100s;

impl MinMax for Range22 {
    const MIN: f64 = -2.0;
    const MAX: f64 = 2.0;
}
impl MinMax for Range01 {
    const MIN: f64 = 0.0;
    const MAX: f64 = 1.0;
}
impl MinMax for Range100s {
    const MIN: f64 = -100.0;
    const MAX: f64 = 100.0;
}

// Type aliases for BoundedFloat with specific range types.
pub type Scale22 = BoundedFloat<Range22>;
pub type Scale01 = BoundedFloat<Range01>;
pub type Scale100s = BoundedFloat<Range100s>;
