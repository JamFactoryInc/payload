use crate::parse::expr::Expr;

pub mod parent;
mod state_parsers;
pub mod expr;

pub(crate) enum AccumulatorRepr {
    Expression,
    MatcherName,
    MatcherLiteral,
    ModifierName,
    RustSrc,
}

pub(crate) enum ParseResult {
    // the parser is not happy :(
    ParseError(String),
    Accumulate,
    ParseAccumulated(AccumulatorRepr),
    // the parser is happy :)
    Continue,
    // defer responsibility to a new child parser and set its root type
    Defer,
    // the parser's done and gives its value back to the caller
    Parsed,
    // an enum variant hijacked by ExprParser
    ParsedExpr(Expr)
}

#[derive(Clone)]
pub(crate) enum StringAccumulatorPurpose {
    MatcherLiteral,
    WithinExpression(usize),
    Parameter,
}

#[derive(PartialEq, Clone)]
pub(crate) enum ParsingRootType {
    // the parser began after its parent found {
    Block,
    // the parser began after its parent found #
    Matcher,
    // the parser began after its parent found @
    Modifier,
    // the parser began at the global root level
    Root,
}

// signifies what it just found
// e.g. after parsing '$', would be `PrefixVariable`
#[derive(Clone)]
pub(crate) enum ParseState {
    LineCommentStart,
    LineComment(Box<ParseState>),
    Prefix(ParsingRootType),
    Identifier,
    ArgsEnd,
    AwaitingArgOrArgsEnd,
    AwaitingDelimOrArgsEnd,
    // we're building an expression (usually a parameter)
    // accepts a depth for tracking parentheses
    AccumulatingExpr(usize),
    // we're building a string literal
    AccumulatingString(StringAccumulatorPurpose),
    // we're still building a string literal but the last char was an escape
    AccumulatingStringEscaped(StringAccumulatorPurpose),
    Root,
}
