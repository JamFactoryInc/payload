use std::ops::Deref;
use crate::parse::{AccumulatorRepr, ParseResult, ParseState, ParsingRootType, StringAccumulatorPurpose};
use crate::parse::ParseResult::*;

pub(crate) struct RootParser {
    pub(crate) state: ParseState,
    pub(crate) root_type: ParsingRootType,
    pub(crate) parent_parser_index: Option<usize>,
}
impl RootParser {
    pub(crate) fn new() -> RootParser {
        RootParser {
            state: ParseState::Root,
            root_type: ParsingRootType::Root,
            parent_parser_index: None,
        }
    }

    pub(crate) fn parse(&mut self, char: &u8) -> ParseResult {
        println!("parsing {:?} with state `{:?}`", char::from(char.clone()), self.state);
        match self.state {
            ParseState::Prefix(_) => self.prefix(char),
            ParseState::Identifier(_) => self.identifier(char),
            ParseState::Root => self.root(char),
            ParseState::AwaitingArgOrArgsEnd => self.awaiting_arg_or_end(char),
            ParseState::AwaitingDelimOrArgsEnd => self.awaiting_delim_or_end(char),
            ParseState::LineCommentStart => self.line_comment_start(char),
            ParseState::LineComment(_) => self.line_comment(char),
            ParseState::AccumulatingExpr(_) => self.accumulating_expr(char),
            ParseState::AccumulatingString(_) => self.accumulating_string(char),
            ParseState::AccumulatingStringEscaped(_) => self.accumulating_string_escaped(char),
            ParseState::AwaitingRootOrArgsBegin => self.awaiting_root_or_args_begin(char)
        }
    }

    //
    // <payload src>
    // ^
    //          _
    // #matcher {
    //           ^
    fn root(&mut self, char: &u8) -> ParseResult {
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
            b'}' => Parsed,
            _ => ParseError("Illegal character at start of expression. Expected `@` | `#` | `{` | `//` | `\"`".to_string())
        }
    }

    fn line_comment(&mut self, char: &u8) -> ParseResult {
        match (char, &self.state) {
            (b'\n', ParseState::LineComment(prev_state)) => {
                self.state = prev_state.deref().clone();
                Continue
            }
            _ => Continue
        }
    }

    fn line_comment_start(&mut self, char: &u8) -> ParseResult {
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
    fn prefix(&mut self, char: &u8) -> ParseResult {
        match (char, &self.root_type) {
            (b'a'..=b'z' | b'A'..=b'Z' | b'_', root_type) => {
                self.state = ParseState::Identifier(root_type.clone());
                Accumulate
            }
            (_, ParsingRootType::Matcher) => ParseError("Illegal character at start of matcher name. Expected [A-z_\"]".to_string()),
            _ => ParseError("Illegal character at start of modifier name. Expected [A-z_]".to_string())
        }
    }

    //  ______
    // #matcher {
    //   ^^^^^^
    fn identifier(&mut self, char: &u8) -> ParseResult {
        match char {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' => Accumulate,
            _ => {
                self.state = ParseState::AwaitingRootOrArgsBegin;
                Continue
            }
        }
    }

    fn awaiting_root_or_args_begin(&mut self, char: &u8) -> ParseResult {
        match char {
            b'(' => {
                self.state = ParseState::AwaitingArgOrArgsEnd;
                Continue
            },
            handle_root => self.root(handle_root)
        }
    }

    //             __      _
    // #matcher(some , other) {
    //              ^^      ^
    fn awaiting_delim_or_end(&mut self, char: &u8) -> ParseResult {
        match char {
            b' ' | b'\n' | b'\r' | b'\t' => {
                Continue
            },
            b',' => {
                Continue
            },
            b')' => {
                self.state = ParseState::Root;
                Continue
            }
            _ => ParseError("Expected start of expression ',' or ')'".to_string())
        }
    }

    fn accumulating_string(&mut self, char: &u8) -> ParseResult {
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

    fn accumulating_string_escaped(&mut self, char: &u8) -> ParseResult {
        match (char, &self.state) {
            (b'r' | b'n' | b't' | b'"' | b'\\', ParseState::AccumulatingStringEscaped(purpose)) => {
                self.state = ParseState::AccumulatingString(purpose.clone());
                Accumulate
            }
            _ => ParseError(format!("Unknown escape char \\{char}"))
        }
    }

    fn accumulating_expr(&mut self, char: &u8) -> ParseResult {
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
    fn awaiting_arg_or_end(&mut self, char: &u8) -> ParseResult {
        match char {
            b' ' | b'\n' | b'\r' | b'\t' => Continue,
            b'"' => {
                self.state = ParseState::AccumulatingString(StringAccumulatorPurpose::Parameter);
                Continue
            }
            b'(' => {
                self.state = ParseState::AccumulatingExpr(1);
                Accumulate
            }
            _ => {
                self.state = ParseState::AccumulatingExpr(0);
                Accumulate
            }
        }
    }
}