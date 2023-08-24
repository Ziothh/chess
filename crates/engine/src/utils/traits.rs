// /// Used to convert enums from and to indices
// ///
// /// # Examples
// ///
// /// Basic usage:
// ///
// /// ```rust
// /// enum Color {
// ///     Red,
// ///     Green,
// ///     Blue,
// /// }
// ///
// /// impl Indexer for Color {
// ///     fn to_index(&self) -> usize {
// ///         *self as usize
// ///     }
// ///
// ///     fn from_index(index: usize) -> Self {
// ///         match index {
// ///             0 => Some(Self::Red),
// ///             1 => Some(Self::Green),
// ///             2 => Some(Self::Blue),
// ///             _ => !unreachable!(),
// ///         }
// ///     }
// /// }
// /// ```
// pub trait Indexer
// where
//     Self: Sized,
// {
//     /// Converts `self` to an index
//     ///
//     /// This is mostly done via `*self as usize`
//     fn to_index(&self) -> usize;
//
//     /// Takes in a `usize` and converts it to `Self`.
//     /// Panics when the given `index` is not a valid represenation of `Self`.
//     fn from_index(index: usize) -> Self;
//     
//     // /// Takes in a `usize` and converts it to `Self`.
//     // ///
//     // /// Panics when the given `index` is not a valid represenation of `Self`.
//     // fn from_index_unchecked(index: usize) -> Self {
//     //     Self::from_index(index).expect("Index to be valid representation of Self")
//     // }
// }
//

// Old docs:
// enum Color {
//     Red,
//     Green,
//     Blue,
// }
//
// impl Indexer for Color {
//     fn to_index(&self) -> usize {
//         *self as usize
//     }
//
//     fn from_index(index: usize) -> Option<Self> {
//         match index {
//             0 => Some(Self::Red),
//             1 => Some(Self::Green),
//             2 => Some(Self::Blue),
//             _ => None,
//         }
//     }
//
//     // You can also optionaly implement the unchecked method
//     // fn from_index_unchecked(index: usize) -> Self {
//     //      // Default behaviour
//     //      Self::from_index(index).expect("Index to be valid representation of Self")
//     // }
// }
