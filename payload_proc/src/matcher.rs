use crate::root::Root;

pub(crate) enum MatcherType {
    Space,
    Alpha,
    Upper,
    Lower,
    Alphanumeric,
    Ident,
    Numeric,
    Range,
    Hex,
    Digit,
    Integer,
    Decimal,
    Literal,
    Symbol,
    Set,
    Flag,
    Enum,
    Sequence,
    One,
    Option,
    Enclose,
    Balance,
    Repeat,
    UserDefined
}
impl TryFrom<String> for MatcherType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        todo!()
    }
}