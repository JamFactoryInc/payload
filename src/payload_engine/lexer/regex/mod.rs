mod ascii_converter;
mod regex_parser;

use std::ops::BitOr;

#[repr(u32)]
pub enum CharCategoryFlag {
    /// \[A-z_] aka \w
    Word =         0b0000_0000_0000_0000_0000_0000_0000_0001,
    /// \[0-9] aka \d
    Digit =        0b0000_0000_0000_0000_0000_0000_0000_0010,
    /// \[A-Z]
    UpCase =       0b0000_0000_0000_0000_0000_0000_0000_0100,
    /// \[a-z]
    DownCase =     0b0000_0000_0000_0000_0000_0000_0000_1000,
    /// \[[]]
    Bracket =      0b0000_0000_0000_0000_0000_0000_0001_0000,
    /// \[()]
    Paren =        0b0000_0000_0000_0000_0000_0000_0010_0000,
    /// \[{}]
    Brace =        0b0000_0000_0000_0000_0000_0000_0100_0000,
    /// \[<>]
    LtGt =         0b0000_0000_0000_0000_0000_0000_1000_0000,
    /// Dedicated flag for ?
    QMark =        0b0000_0000_0000_0000_0000_0001_0000_0000,
    /// Dedicated flag for *
    Star =         0b0000_0000_0000_0000_0000_0010_0000_0000,
    /// Dedicated flag for +
    Pos =          0b0000_0000_0000_0000_0000_0100_0000_0000,
    /// Dedicated flag for -
    Neg =          0b0000_0000_0000_0000_0000_1000_0000_0000,
    /// Dedicated flag for ^
    Caret =        0b0000_0000_0000_0000_0001_0000_0000_0000,
    /// Dedicated flag for !
    Bang =         0b0000_0000_0000_0000_0010_0000_0000_0000,
    /// Dedicated flag for \
    BackSlash =    0b0000_0000_0000_0000_0100_0000_0000_0000,
    /// Dedicated flag for .
    Dot =          0b0000_0000_0000_0000_1000_0000_0000_0000,
    /// Dedicated flag for :
    Colon =        0b0000_0000_0000_0001_0000_0000_0000_0000,
    /// Dedicated flag for $
    DSign =        0b0000_0000_0000_0010_0000_0000_0000_0000,
    /// Dedicated flag for ,
    Comma =        0b0000_0000_0000_0100_0000_0000_0000_0000,
    /// Dedicated flag for |
    Or =           0b0000_0000_0000_1000_0000_0000_0000_0000,
    /// Dedicated flag for &
    And =          0b0000_0000_0001_0000_0000_0000_0000_0000,
    /// Dedicated flag for '
    SingleQuote =  0b0000_0000_0010_0000_0000_0000_0000_0000,
    /// Dedicated flag for '
    DoubleQuote =  0b0000_0000_0100_0000_0000_0000_0000_0000,
    /// Dedicated flag for &
    Quote =        0b0000_0000_0110_0000_0000_0000_0000_0000,
    /// Dedicated flag for =
    Eq =           0b0000_0000_1000_0000_0000_0000_0000_0000,
    /// if the char is a valid wildcard
    NotWildcard =  0b0000_0001_0000_0000_0000_0000_0000_0000,
    /// \s
    WhiteSpace =   0b0000_0010_0000_0000_0000_0000_0000_0000,
    /// Requires a backslash escape if it's the last char in a class
    ClassEscEnd =  0b0000_0100_0000_0000_0000_0000_0000_0000,
    /// Requires a backslash escape if it's neither the first not last char in a class
    ClassEscMid =  0b0000_1000_0000_0000_0000_0000_0000_0000,
    /// Requires a backslash escape if it's the first char in a class
    ClassEscBeg =  0b0001_0000_0000_0000_0000_0000_0000_0000,
    /// Always requires a backlash escape in a char class
    ClassEscAny =  0b0001_1100_0000_0000_0000_0000_0000_0000,
    /// Reserved regex token.
    ///
    /// Indicates that it must be preceded by a backslash to match the literal value
    /// unless within a char class, then whether it must be escaped is denoted by the
    /// ClassEscXXX enums
    RegexReserved = 0b0100_0000_0000_0000_0000_0000_0000_0000,
    /// Indicates the corresponding bracket is the open variant
    Open =         0b1000_0000_0000_0000_0000_0000_0000_0000,
}

impl BitOr for CharCategoryFlag {
    type Output = u32;

    fn bitor(self, rhs: Self) -> Self::Output {
        return (self as u32) | (rhs as u32)
    }
}

impl BitOr<CharCategoryFlag> for u32 {
    type Output = u32;

    fn bitor(self, rhs: CharCategoryFlag) -> Self::Output {
        return self | rhs as u32
    }
}

trait Flag<T> {
    fn has_flag(self, flag : T) -> bool;
}

impl Flag<CharCategoryFlag> for u32 {
    fn has_flag(self, flag : CharCategoryFlag) -> bool {
        return (self & (flag as u32)) != 0;
    }
}