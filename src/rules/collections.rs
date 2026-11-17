use super::codes;
use crate::error::Violation;
use crate::rule::Rule;

/// The collection has at least one element. Works with anything a shared reference can iterate:
/// slices, arrays, `Vec`, maps, and sets.
///
/// ```
/// use validations_rs::rules::NotEmpty;
/// use validations_rs::Rule;
///
/// assert!(NotEmpty.check(&vec![1]).is_ok());
/// assert!(NotEmpty.check(&Vec::<i32>::new()).is_err());
/// assert!(NotEmpty.check(&[1, 2][..]).is_ok());
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct NotEmpty;

impl<T: ?Sized> Rule<T> for NotEmpty
where
    for<'a> &'a T: IntoIterator,
{
    fn check(&self, value: &T) -> Result<(), Violation> {
        if value.into_iter().next().is_none() {
            Err(Violation::new(codes::COLLECTION_EMPTY, "The collection must not be empty."))
        } else {
            Ok(())
        }
    }
}

/// The collection has between `min` and `max` elements, inclusive.
///
/// ```
/// use validations_rs::rules::Count;
/// use validations_rs::Rule;
///
/// assert!(Count::between(1, 3).check(&vec!["a", "b"]).is_ok());
/// assert_eq!(Count::at_most(1).check(&vec!["a", "b"]).unwrap_err().detail("size"), Some("2"));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Count {
    min: usize,
    max: usize,
}

impl Count {
    /// Between `min` and `max` elements, inclusive.
    #[must_use]
    pub const fn between(min: usize, max: usize) -> Self {
        Self { min, max }
    }

    /// At least `min` elements.
    #[must_use]
    pub const fn at_least(min: usize) -> Self {
        Self { min, max: usize::MAX }
    }

    /// At most `max` elements.
    #[must_use]
    pub const fn at_most(max: usize) -> Self {
        Self { min: 0, max }
    }
}

impl<T: ?Sized> Rule<T> for Count
where
    for<'a> &'a T: IntoIterator,
{
    fn check(&self, value: &T) -> Result<(), Violation> {
        let size = value.into_iter().count();
        if size < self.min || size > self.max {
            Err(Violation::new(codes::SIZE_OUT_OF_RANGE, "The number of elements must be within the allowed range.")
                .with_detail("size", size)
                .with_detail("min", self.min)
                .with_detail("max", self.max))
        } else {
            Ok(())
        }
    }
}
