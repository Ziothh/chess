/// Describe a file (column) on a chess board
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug, Hash)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

/// How many files are there?
pub const NUM_FILES: usize = 8;

/// Enumerate all files
pub const ALL_FILES: [File; NUM_FILES] = [
    File::A,
    File::B,
    File::C,
    File::D,
    File::E,
    File::F,
    File::G,
    File::H,
];

impl File {
    /// Convert a `usize` into a `File` (the inverse of to_index).  If i > 7, wrap around.
    #[inline]
    pub fn from_index(i: usize) -> File {
        // match is optimized to no-op with opt-level=1 with rustc 1.53.0
        match i & 7 {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => unreachable!(),
        }
    }

    /// Go one file to the left.  If impossible, wrap around.
    #[inline]
    pub fn left(&self) -> File {
        File::from_index(self.to_index().wrapping_sub(1))
    }

    /// Go one file to the right.  If impossible, wrap around.
    #[inline]
    pub fn right(&self) -> File {
        File::from_index(self.to_index() + 1)
    }

    /// Convert this `File` into a `usize` from 0 to 7 inclusive.
    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    pub fn to_char(&self) -> char {
      char::from_u32((self.to_index() + (b'a' as usize)) as u32).unwrap()
    }
}

impl TryFrom<char> for File {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let index = (value as usize) - ('a' as usize);

        // Prevent wrapping. E.g.: value > 'hfile'
        if index > NUM_FILES {
            return Err(format!("Invalid file \"{value}\""));
        };

        Ok(Self::from_index(index))

        // match value.unwrap() {
        //     'a' => Ok(File::A),
        //     'b' => Ok(File::B),
        //     'c' => Ok(File::C),
        //     'd' => Ok(File::D),
        //     'e' => Ok(File::E),
        //     'f' => Ok(File::F),
        //     'g' => Ok(File::G),
        //     'h' => Ok(File::H),
        //     _ => Err(format!("Invalid file \"{value}\"")),
        // }
    }
}

// impl FromStr for File {
//     type Err = Error;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         if s.len() < 1 {
//             return Err(Error::InvalidFile);
//         }
//         match s.chars().next().unwrap() {
//             'a' => Ok(File::A),
//             'b' => Ok(File::B),
//             'c' => Ok(File::C),
//             'd' => Ok(File::D),
//             'e' => Ok(File::E),
//             'f' => Ok(File::F),
//             'g' => Ok(File::G),
//             'h' => Ok(File::H),
//             _ => Err(Error::InvalidFile),
//         }
//     }
// }