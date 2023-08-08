use std::cell::RefCell;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::{mem, ptr};
use std::ops::{ControlFlow, Deref, FromResidual, Try};
use crate::expr::{Expr, ExprParser};
use crate::matcher::MatcherType;
use crate::modifier::ModifierType;
use crate::parse::ParseResult::*;
use crate::root::{BranchValue, Root, RootType};

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
enum StringAccumulatorPurpose {
    MatcherLiteral,
    WithinExpression(usize),
    Parameter,
}

#[derive(PartialEq, Clone)]
enum ParsingRootType {
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
enum ParseState {
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

pub(crate) struct ParserArena<'a> {
    accumulator: String,
    roots: Vec<Root>,
    current_root: &'a mut Root,
    parsers: Vec<RootParser>,
    parser_index: usize,
}
impl<'a> ParserArena<'a> {
    /// swaps the pointer of self.accumulator with a pointer to an equal-capacity empty string
    /// basically self.accumulator.clone() and self.accumulator.clear() in one
    fn use_accumulator(&mut self) -> String {
        let mut to_be_swapped = String::with_capacity(self.accumulator.capacity());
        mem::swap(&mut self.accumulator, &mut to_be_swapped);
        to_be_swapped
    }

    pub(crate) fn parse(&mut self, char: u8) -> ParseResult {
        match &mut self.parsers.get(self.parser_index.clone()) {
            Some(parser) => match parser.parse(char) {
                Parsed => match &parser.parent_parser_index {
                    Some(parent_index) => {
                        self.parser_index = parent_index.clone();
                        match &self.current_root.parent {
                            Some(i) => {
                                self.current_root = &mut self.roots[i.clone()];
                            }
                            None =>
                        }

                        Continue
                    }
                    None => {
                        Parsed
                    }
                }
                Continue => Continue,
                ParseError(err) => ParseError(err),
                ParsedExpr(_) => ParseError("Unexpected internal error [code 1]".to_string()),
                Defer => {
                    let new_root = Root {
                        branches: Vec::new(),
                        root_type: RootType::Root,
                        args: Vec::new(),
                        parent: Some(self.current_root.index.clone()),
                        index: self.current_root.branches.len(),
                    };
                    self.current_root.branches.push(BranchValue::Root(new_root));
                    self.current_root = match &mut self.current_root.branches.get(self.current_root.branches.len() - 1) {
                        Some(BranchValue::Root(mut root)) => &mut root,
                        _ => return ParseError("Unexpected internal error [code 2]".to_string()),
                    };
                    self.parsers.push(RootParser {
                        state: ParseState::Root,
                        root_type: ParsingRootType::Root,
                        parent_parser_index: Some(self.parser_index.clone()),
                        prefix_type: None,
                    });
                    self.parser_index = self.parsers.len() - 1;
                    Continue
                }
                Accumulate => {
                    self.accumulator.push(char::from(char.clone()));
                    Continue
                }
                ParseAccumulated(accumulator_repr) => {
                    match accumulator_repr {
                        AccumulatorRepr::Expression => match Expr::try_from(self.use_accumulator()) {
                            Ok(expr) => {
                                self.current_root.args.push(expr);
                                Continue
                            }
                            Err(msg) => ParseError(msg)
                        }
                        AccumulatorRepr::MatcherName => match MatcherType::try_from(self.use_accumulator()) {
                            Ok(matcher_type) => {
                                self.current_root.root_type = RootType::Matcher(matcher_type);
                                Continue
                            },
                            Err(msg) => ParseError(msg),
                        }
                        AccumulatorRepr::ModifierName => match ModifierType::try_from(self.use_accumulator()) {
                            Ok(modifier_type) => {
                                self.current_root.root_type = RootType::Modifier(modifier_type);
                                Continue
                            },
                            Err(msg) => ParseError(msg),
                        }
                        AccumulatorRepr::MatcherLiteral => {
                            self.current_root.root_type = RootType::Matcher(MatcherType::Literal);
                            self.current_root.args.push(Expr::String(self.use_accumulator()));
                            Continue
                        }
                        AccumulatorRepr::RustSrc => {
                            self.current_root.branches.push(BranchValue::Source(self.use_accumulator()));
                            Continue
                        }
                    }
                }
            }
            None => ParseError(format!("Unexpected internal error [code 3:{}:{}]", self.parsers.len(), self.parser_index))
        }
    }
}

pub(crate) trait Parser {
    fn parse(&mut self, char: u8) -> ParseResult;
}

pub(crate) struct RootParser {
    state: ParseState,
    root_type: ParsingRootType,
    parent_parser_index: Option<usize>,
    prefix_type: Option<ParsingRootType>,
}
impl RootParser {
    pub(crate) fn new() -> RootParser {
        RootParser {
            state: ParseState::Root,
            root_type: ParsingRootType::Root,
            prefix_type: None,
            parent_parser_index: None,
        }
    }

    pub(crate) fn parse(&mut self, char: u8) -> ParseResult {
        match self.state {
            ParseState::Prefix(_) => self.prefix(char),
            ParseState::Identifier => self.identifier(char),
            ParseState::ArgsEnd => self.args_end(char),
            ParseState::Root => self.root(char),
            ParseState::AwaitingArgOrArgsEnd => self.awaiting_arg_or_end(char),
            ParseState::AwaitingDelimOrArgsEnd => self.awaiting_delim_or_end(char),
            ParseState::LineCommentStart => self.line_comment_start(char),
            ParseState::LineComment(_) => self.line_comment(char),
            ParseState::AccumulatingExpr(_) => self.accumulating_expr(char),
            ParseState::AccumulatingString(_) => self.accumulating_string(char),
            ParseState::AccumulatingStringEscaped(_) => self.accumulating_string_escaped(char),
        }
    }

    //          _
    // #matcher()  {
    //           ^^^
    //          _
    // #matcher()  #matcher2
    //           ^^^
    //          _
    // #matcher()  @modifier
    //           ^^^
    fn args_end(&mut self, char: u8) -> ParseResult {
        match char {
            b' ' | b'\n' | b'\r' | b'\t' => Continue,
            b'/' => {
                self.state = ParseState::LineCommentStart;
                Continue
            }
            b'#' => {
                self.state = ParseState::Prefix(ParsingRootType::Matcher);
                Continue
            }
            b'@' => {
                self.state = ParseState::Prefix(ParsingRootType::Modifier);
                Continue
            }
            b'"' => {
                self.state = ParseState::AccumulatingString(StringAccumulatorPurpose::MatcherLiteral);
                Continue
            }
            b'{' => {
                // the state we should be in once we have responsibility again
                self.state = ParseState::Root;
                Defer
            }
            _ => ParseError("Illegal character at start of expression. Expected @ or #".to_string())
        }
    }

    //
    // <payload src>
    // ^
    //          _
    // #matcher {
    //           ^
    fn root(&mut self, char: u8) -> ParseResult {
        match char {
            b' ' | b'\n' | b'\r' | b'\t' => Continue,
            b'/' => {
                self.state = ParseState::LineCommentStart;
                Continue
            }
            b'@' => {
                self.state = ParseState::Prefix(ParsingRootType::Modifier);
                Continue
            }
            b'#' => {
                self.state = ParseState::Prefix(ParsingRootType::Matcher);
                self.root_type = ParsingRootType::Modifier;
                Continue
            }
            b'"' => {
                self.state = ParseState::AccumulatingString(StringAccumulatorPurpose::MatcherLiteral);
                self.root_type = ParsingRootType::Modifier;
                Continue
            }
            b'{' => {
                // the state to be in after defer is done
                self.state = ParseState::Root;
                Defer
            }
            b'}' => {
                // we're done
                Parsed(())
            }
            _ => ParseError("Illegal character at start of expression. Expected `@` | `#` | `{` | `//` | `\"`".to_string())
        }
    }

    fn line_comment(&mut self, char: u8) -> ParseResult {
        match (char, &self.state) {
            (b'\n', ParseState::LineComment(prev_state)) => {
                self.state = prev_state.deref().clone();
                Continue
            }
            _ => Continue
        }
    }

    fn line_comment_start(&mut self, char: u8) -> ParseResult {
        match char {
            b'/' => {
                self.state = ParseState::LineComment(Box::new(self.state.clone()));
                Continue
            }
            _ => ParseError("Illegal floating `/` at root level. Did you mean to write a comment with `//`?".to_string())
        }
    }

    // _
    // #matcher
    //  ^
    // _
    // @modifier
    //  ^
    fn prefix(&mut self, char: u8) -> ParseResult {
        match (char, &self.root_type) {
            (b'a'..=b'z' | b'A'..=b'Z' | b'_', _) => {
                Accumulate
            }
            (_, ParsingRootType::Matcher) => ParseError("Illegal character at start of matcher name. Expected [A-z_\"]".to_string()),
            _ => ParseError("Illegal character at start of modifier name. Expected [A-z_]".to_string())
        }
    }

    //  ______
    // #matcher {
    //   ^^^^^^
    fn identifier(&mut self, char: u8) -> ParseResult {
        match char {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' => Accumulate,
            _ => ParseError("Illegal character within of identifier. Expected [A-z0-9_]".to_string())
        }
    }

    //             __      _
    // #matcher(some , other) {
    //              ^^      ^
    fn awaiting_delim_or_end(&mut self, char: u8) -> ParseResult {
        match char {
            b' ' | b'\n' | b'\r' | b'\t' => {
                Continue
            },
            b',' => {
                Continue
            },
            b')' => {
                self.state = ParseState::AwaitingArgOrArgsEnd;
                Continue
            }
            _ => ParseError("Expected start of expression ',' or ')'".to_string())
        }
    }

    fn accumulating_string(&mut self, char: u8) -> ParseResult {
        match (char, &self.state) {
            (b'\\', ParseState::AccumulatingString(purpose)) => {
                self.state = ParseState::AccumulatingStringEscaped(purpose.clone());
                Accumulate
            }
            (b'"', ParseState::AccumulatingString(purpose)) => {
                match purpose {
                    StringAccumulatorPurpose::MatcherLiteral => {
                        self.state = ParseState::Root;
                        ParseAccumulated(AccumulatorRepr::MatcherLiteral)
                    }
                    StringAccumulatorPurpose::WithinExpression(depth) => {
                        self.state = ParseState::AccumulatingExpr(depth.clone());
                        Accumulate
                    }
                    StringAccumulatorPurpose::Parameter => {
                        self.state = ParseState::AwaitingDelimOrArgsEnd;
                        Continue
                    }
                }
            }
            _ => Accumulate
        }
    }

    fn accumulating_string_escaped(&mut self, char: u8) -> ParseResult {
        match (char, &self.state) {
            (b'r' | b'n' | b't' | b'"' | b'\\', ParseState::AccumulatingStringEscaped(purpose)) => {
                self.state = ParseState::AccumulatingString(purpose.clone());
                Accumulate
            }
            _ => ParseError(format!("Unknown escape char \\{char}"))
        }
    }

    fn accumulating_expr(&mut self, char: u8) -> ParseResult {
        match (char, &self.state) {
            (b' ' | b'\n' | b'\r' | b'\t', _) => Continue,
            (b'"', ParseState::AccumulatingExpr(depth)) => {
                self.state = ParseState::AccumulatingString(StringAccumulatorPurpose::WithinExpression(depth.clone()));
                Continue
            }
            (b',', ParseState::AccumulatingExpr(0)) => {
                self.state = ParseState::AwaitingArgOrArgsEnd;
                ParseAccumulated(AccumulatorRepr::Expression)
            }
            (b')', ParseState::AccumulatingExpr(1)) => {
                ParseAccumulated(AccumulatorRepr::Expression)
            }
            (b')', ParseState::AccumulatingExpr(depth)) => {
                self.state = ParseState::AccumulatingExpr(depth - 1);
                Accumulate
            }
            (b'(', ParseState::AccumulatingExpr(depth)) => {
                self.state = ParseState::AccumulatingExpr(depth + 1);
                Accumulate
            }
            _ => Accumulate
        }
    }

    //               __     _
    // #matcher(some , other,) {
    //                ^^     ^
    fn awaiting_arg_or_end(&mut self, char: u8) -> ParseResult {
        match char {
            b' ' | b'\n' | b'\r' | b'\t' => Continue,
            b'(' => {
                self.state = ParseState::AccumulatingExpr(1);
                Accumulate
            }
            b'"' => {
                self.state = ParseState::AwaitingDelimOrArgsEnd;
                Defer(ParseState::AccumulatingString(StringAccumulatorPurpose::Parameter))
            }
            _ => {
                self.state = ParseState::AccumulatingExpr(0);
                Accumulate
            }
        }
    }
}



