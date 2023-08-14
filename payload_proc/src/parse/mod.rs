use std::ops::{FromResidual, Try};
use crate::parse::expr::Expr;
use crate::root::RootType;

pub mod payload_parser;
mod root_parser;
pub mod expr;
mod scope;

pub(crate) enum AccumulatorRepr {
    Expression,
    MatcherName,
    MatcherLiteral,
    ModifierName,
    RustSrc,
    Range
}

pub(crate) enum ParseResult<T> {
    // the parser is not happy :(
    ParseError(String),
    Accumulate(u8),
    ParseAccumulated(T),
    // the parser is happy :)
    Continue,
    // defer responsibility to a new child parser and set its root type
    Defer,
    // the parser's done and gives its value back to the caller
    Parsed,
}
impl<T> TryInto<ParseResult<T>> for String {
    type Error = ();

    fn try_into(self) -> Result<ParseResult<T>, Self::Error> {
        todo!()
    }
}
impl<A> FromResidual<Result<A, String>> for ParseResult<AccumulatorRepr> {
    fn from_residual(residual: Result<A, String>) -> Self {
        match residual {
            Ok(_) => Self::Continue,
            Err(msg) => Self::ParseError(msg)
        }
    }
}
impl<T> FromResidual<Option<T>> for ParseResult<AccumulatorRepr> {
    fn from_residual(residual: Option<T>) -> Self {
        match residual {
            Some(_) => Self::Continue,
            None => Self::ParseError("Unknown error from missing optional value".to_string())
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum AccumulatorPurpose {
    MatcherLiteral,
    WithinExpression(usize),
    Parameter,
}

// signifies what it just found
// e.g. after parsing '$', would be `PrefixVariable`
#[derive(Clone, Debug)]
pub(crate) enum ParseState {
    LineCommentStart,
    LineComment { prev_state: Box<ParseState> },
    Prefix(RootType),
    Identifier(RootType),
    AwaitingArgOrArgsEnd,
    AwaitingDelimOrArgsEnd,
    AwaitingRootOrArgsBegin,
    // we're building an expression (usually a parameter)
    // accepts a depth for tracking parentheses
    AccumulatingExpr { depth: usize },
    // we're building a string literal
    AccumulatingString(AccumulatorPurpose),
    // we're still building a string literal but the last char was an escape
    AccumulatingStringEscaped(AccumulatorPurpose),
    Root,
    Range(RangeParseState),
}

#[derive(Clone, Debug)]
pub(crate) enum RangeParseItem {
    Left,
    Right
}
#[derive(Clone, Debug)]
pub(crate) enum RangeParseState {
    AwaitingChar(RangeParseItem),
    AwaitingEscapedChar(RangeParseItem),
    AwaitingOpeningQuote,
    AwaitingClosingQuote(RangeParseItem),
    AwaitingDot(RangeParseItem)
}