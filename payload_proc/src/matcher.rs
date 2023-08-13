use std::fmt::{Debug, Display, Formatter};
use crate::describe::Describe;
use crate::matcher::MatcherType::*;

#[derive(Clone)]
pub(crate) enum MatcherType {
    Space,
    Alpha,
    Upper,
    Lower,
    Alphanumeric,
    Ident,
    Numeric,
    Range { from: u8, to: u8 },
    Hex,
    Digit,
    Integer,
    Decimal,
    Literal(String),
    Symbol,
    Set,
    Flag,
    Enum,
    One,
    Option,
    Enclose,
    Balance,
    Repeat,
    UserDefined(String),
    Nil,
}
impl MatcherType {
    fn as_string_repr(&self) -> String {
        match self {
            Space => "space",
            Alpha => "alpha",
            Upper => "upper",
            Lower => "lower",
            Alphanumeric => "alphanumeric",
            Ident => "ident",
            Numeric => "numeric",
            Range { .. } => "",
            Hex => "hex",
            Digit => "digit",
            Integer => "integer",
            Decimal => "decimal",
            Literal(str) => str,
            Symbol => "symbol",
            Set => "set",
            Flag => "flag",
            Enum => "enum",
            One => "one",
            Option => "option",
            Enclose => "enclose",
            Balance => "balance",
            Repeat => "repeat",
            UserDefined(_) => "",
            Nil => "<nil>",
        }.to_string()
    }
}
impl TryFrom<String> for MatcherType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(match value.as_str() {
            "space" => Space,
            "alpha" => Alpha,
            "upper" => Upper,
            "lower" => Lower,
            "alphanumeric" => Alphanumeric,
            "ident" => Ident,
            "numeric" => Numeric,
            "range" => Range { from: 0, to: 0 },
            "hex" => Hex,
            "digit" => Digit,
            "integer" => Integer,
            "decimal" => Decimal,
            "literal" => Literal("".to_string()),
            "symbol" => Symbol,
            "set" => Set,
            "flag" => Flag,
            "enum" => Enum,
            "one" => One,
            "option" => Option,
            "enclose" => Enclose,
            "balance" => Balance,
            "repeat" => Repeat,
            _ => UserDefined(value),
        })
    }
}
impl Display for MatcherType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Range { .. } => write!(f, "'a'..'z'"),
            Literal(_) => write!(f, "\"some literal\""),
            UserDefined(_) => write!(f, "#SomeStruct"),
            other => write!(f, "#{}", other.as_string_repr())
        }
    }
}
impl Debug for MatcherType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Range { from, to } => write!(f, "{:?}..{:?}", char::from(from.clone()), char::from(to.clone())),
            Literal(str) => write!(f, "\"{str}\""),
            UserDefined(str) => write!(f, "#{str}"),
            other => write!(f, "#{}", other.as_string_repr())
        }
    }
}
impl Into<Vec<Self>> for MatcherType {
    fn into(self) -> Vec<Self> {
        vec![
            Space,
            Alpha,
            Upper,
            Lower,
            Alphanumeric,
            Ident,
            Numeric,
            Range { from: 0, to: 0 },
            Hex,
            Digit,
            Integer,
            Decimal,
            Literal("".to_string()),
            Symbol,
            Set,
            Flag,
            Enum,
            One,
            Option,
            Enclose,
            Balance,
            Repeat,
            UserDefined("".to_string()),
            Nil,
        ]
    }
}
impl Describe for MatcherType {
    fn describe(&self) -> &'static str {
        match self {
            Space => "",
            Alpha => "",
            Upper => "",
            Lower => "",
            Alphanumeric => "",
            Ident => "",
            Numeric => "",
            Range { .. } => "",
            Hex => "",
            Digit => "",
            Integer => "",
            Decimal => "",
            Literal(_) => "",
            Symbol => "",
            Set => "",
            Flag => "",
            Enum => "",
            One => "",
            Option => "",
            Enclose => "",
            Balance => "",
            Repeat => "",
            UserDefined(_) => "",
            Nil => "",
        }
    }
}