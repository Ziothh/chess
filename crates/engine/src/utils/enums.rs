pub trait StrEnum: Sized {
    type Error;
    fn to_str(&self) -> &str;
    fn from_str(value: &str) -> Result<Self, Self::Error>;
}

/// Calculates the amount of variants in an enum
/// and provides an array to iterate over all of the variants
pub trait ArrayEnum<const SIZE: usize>: Sized
{
    /// The number of variants of the enum
    ///
    /// It is equal to `Self::ALL.len()`
    const SIZE: usize = SIZE;

    /// An array containing all the variants of the enum
    /// ```ignore
    /// // Generic enum example
    /// let variant = Self::SomeVariant;
    ///
    /// assert_eq!(variant, Self::ALL[variant.to_index()]);
    /// ```
    const ALL: [Self; SIZE];

    /// Gets the variant index in the enum (= nth variant)
    ///
    /// Range: `0..Self::SIZE`
    fn to_index(&self) -> usize;
}

// pub trait T: Sized
// {
//     const T: usize = std::mem::variant_count::<Self>();
//
//     fn test() {
//         ()
//     }
// }
//
// pub enum Cases {
//     One,
//     Two,
//     Three
// }
//
// impl T for Cases {
// }
//
//
// impl Cases {
//     /// ```rust
//     /// use engine::utils::enums::*;
//     ///
//     /// assert_eq!(Cases::T, 3);
//     /// ```
//     pub fn test_size() {
//         ()
//     }
//
// }
//
