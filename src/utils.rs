use anyhow::{anyhow, Result};
use serde::{Serialize, Serializer};
use std::{marker::PhantomData, ops::Deref};

#[derive(Clone, Copy)]
pub struct BoundedFloat<T> {
    inner: f64,
    _marker: PhantomData<T>,
}

pub trait MinMax {
    const MIN: f64;
    const MAX: f64;
}

pub trait Bounded {
    fn new(value: f64) -> Result<Self>
    where
        Self: Sized;
}

impl<T> Bounded for BoundedFloat<T>
where
    T: MinMax,
{
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

#[derive(Clone, Copy)]
pub struct Range22;
#[derive(Clone, Copy)]
pub struct Range01;
#[derive(Clone, Copy)]
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

pub type Scale22 = BoundedFloat<Range22>;
pub type Scale01 = BoundedFloat<Range01>;
pub type Scale100s = BoundedFloat<Range100s>;
