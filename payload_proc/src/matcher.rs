use crate::root::Root;

pub(crate) enum MatcherType {
    Space,
    Alpha,
    Upper,
    Lower,
    Alphanumeric,
    Ident,
    Numeric,
    Range{ from: u8, to: u8 },
    Hex,
    Digit,
    Integer{ size: u16, signed: bool, base: u16 },
    Decimal{ size: u16, signed: bool, base: u16 },
    Literal(String),
    Symbol(String),
    Set,
    Flag,
    Enum,
    Sequence,
    One,
    Option,
    Enclose,
    Balance,
    Repeat,
    UserDefined(String)
}