use std::{marker::PhantomData, ops::Deref};

pub struct BoundedFloat<T> {
    inner: f64,
    _marker: PhantomData<T>,
}

pub trait MinMax {
    const MIN: f64;
    const MAX: f64;
}

impl<T> BoundedFloat<T>
where
    T: MinMax,
{
    pub fn new(value: f64) -> Option<Self> {
        if T::MIN <= value && value <= T::MAX {
            Some(Self {
                inner: value,
                _marker: PhantomData,
            })
        } else {
            None
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

pub struct Range22;
pub struct Range01;
pub struct Range100s;

impl MinMax for Range22 {
    const MIN: f64 = -2.0;
    const MAX: f64 = 2.0;
}
impl MinMax for Range01 {
    const MIN: f64 = 0.0;
    const MAX: f64 = 1.0;
}

pub type Scale22 = BoundedFloat<Range22>;
pub type Scale01 = BoundedFloat<Range01>;
pub type Scale100s = BoundedFloat<Range100s>;
