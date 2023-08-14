use std::ops::Deref;
use crate::parse::{AccumulatorRepr, ParseResult, ParseState, RootType, AccumulatorPurpose, RangeParseState, RangeParseItem};
use crate::parse::ParseResult::*;
use self::ParseState::*;
use crate::parse::RangeParseItem::*;
use crate::parse::RangeParseState::*;

pub(crate) struct RootParser {
    pub(crate) state: ParseState,
    pub(crate) parent_parser_index: Option<usize>,
}

impl RootParser {
    pub(crate) fn new() -> RootParser {
        RootParser {
            state: Root,
            parent_parser_index: None,
        }
    }

    fn set_state(&mut self, state: ParseState) -> &mut Self {
        self.state = state;
        self
    }

    fn and_defer(&mut self) -> ParseResult<AccumulatorRepr> {
        Defer
    }

    fn and_continue(&mut self) -> ParseResult<AccumulatorRepr> {
        Continue
    }

    fn and_accumulate(&mut self, char: &u8) -> ParseResult<AccumulatorRepr> {
        Accumulate(char.clone())
    }

    fn and_parse_accumulated(&mut self, representation: AccumulatorRepr) -> ParseResult<AccumulatorRepr> {
        ParseAccumulated(representation)
    }

    fn is_valid_escape_char(char: &u8) -> bool {
        matches!(char, b'r' | b'n' | b't' | b'"' | b'\\')
    }

    fn is_valid_range_escape_char(char: &u8) -> bool {
        matches!(char, b'r' | b'n' | b't' | b'\'' | b'\\')
    }

    fn and_accumulate_escaped(&mut self, char: &u8) -> ParseResult<AccumulatorRepr> {
        Accumulate(match char {
            b'r' => b'\r',
            b'n' => b'\n',
            b't' => b'\t',
            b'"' => b'"',
            b'\'' => b'\'',
            b'\\' => b'\\',
            _ => return ParseError(format!("Unknown escape character \\{:?}", char::from(char.clone())))
        })
    }

    pub(crate) fn parse(&mut self, char: &u8) -> ParseResult<AccumulatorRepr> {
        println!("parsing {:?} with state `{:?}`", char::from(char.clone()), self.state);
        match self.state {
            Prefix(_) => self.prefix(char),
            Identifier(_) => self.identifier(char),
            Root => self.handle_root(char),
            AwaitingArgOrArgsEnd => self.awaiting_arg_or_end(char),
            AwaitingDelimOrArgsEnd => self.awaiting_delim_or_end(char),
            LineCommentStart => self.line_comment_start(char),
            LineComment { prev_state: _ } => self.line_comment(char),
            AccumulatingExpr { depth: _ } => self.accumulating_expr(char),
            AccumulatingString(_) => self.accumulating_string(char),
            AccumulatingStringEscaped(_) => self.accumulating_string_escaped(char),
            AwaitingRootOrArgsBegin => self.awaiting_root_or_args_begin(char),
            Range(_) => self.handle_range(char),
        }
    }

    fn handle_range(&mut self, char: &u8) -> ParseResult<AccumulatorRepr> {
        match (char, self.state.clone()) {
            (b'\\', Range(AwaitingChar(side))) =>
                self.set_state(Range(AwaitingEscapedChar(side)))
                    .and_continue(),
            (b'\'', Range(AwaitingChar(_))) => ParseError("Character literal must not be empty. Did you mean to escape?".to_string()),
            (_, Range(AwaitingChar(side))) =>
                self.set_state(Range(AwaitingClosingQuote(side)))
                    .and_accumulate(char),
            (_, Range(AwaitingEscapedChar(side))) =>
                self.set_state(Range(AwaitingClosingQuote(side)))
                    .and_accumulate_escaped(char),
            (b'.', Range(AwaitingDot(Left))) =>
                self.set_state(Range(AwaitingDot(Right)))
                    .and_continue(),
            (b'.', Range(AwaitingDot(Right))) =>
                self.set_state(Range(AwaitingOpeningQuote))
                    .and_continue(),
            (b'\'', Range(AwaitingOpeningQuote)) =>
                self.set_state(Range(AwaitingChar(Right)))
                    .and_continue(),
            (b'\'', Range(AwaitingClosingQuote(Left))) =>
                self.set_state(Range(AwaitingDot(Left)))
                    .and_continue(),
            (b'\'', Range(AwaitingClosingQuote(Right))) =>
                self.set_state(AwaitingRootOrArgsBegin)
                    .and_parse_accumulated(AccumulatorRepr::Range),
            (_, Range(AwaitingDot(_))) => ParseError("Expected '.'".to_string()),
            (_, Range(AwaitingClosingQuote(_))) => ParseError(format!("Expected closing single-quote but got {:?}", char::from(char.clone()))),
            (_, Range(AwaitingOpeningQuote)) => ParseError(format!("Expected opening single-quote but got {:?}", char::from(char.clone()))),
            _ => Continue
        }
    }

    //
    // <payload src>
    // ^
    //          _
    // #matcher {
    //           ^
    fn handle_root(&mut self, char: &u8) -> ParseResult<AccumulatorRepr> {
        match char {
            c if c.is_ascii_whitespace() => Continue,
            b'/' => self.set_state(LineCommentStart)
                        .and_continue(),
            b'@' => self.set_state(Prefix(RootType::modifier()))
                        .and_continue(),
            b'#' => {
                self.set_state(Prefix(RootType::matcher()))
                    .and_continue()
            }
            b'"' => {
                let accumulator_purpose = AccumulatorPurpose::MatcherLiteral;
                self.set_state(AccumulatingString(accumulator_purpose))
                    .and_continue()
            }
            b'\'' => {
                self.set_state(Range(AwaitingChar(Left)))
                    .and_continue()
            }
            b'{' => {
                // the state to be in after defer is done
                self.set_state(Root)
                    .and_defer()
            }
            b'}' => Parsed,
            _ => ParseError("Illegal character at start of expression. Expected `@` | `#` | `{` | `//` | `\"`".to_string())
        }
    }

    fn line_comment(&mut self, char: &u8) -> ParseResult<AccumulatorRepr> {
        if let (b'\n', LineComment { prev_state }) = (char, &self.state) {
            self.set_state(prev_state.deref().clone());
        }
        Continue
    }

    fn line_comment_start(&mut self, char: &u8) -> ParseResult<AccumulatorRepr> {
        match char {
            b'/' => self.set_state(LineComment { prev_state: Box::new(self.state.clone()) })
                        .and_continue(),
            _ => ParseError("Illegal floating `/` at root level. Did you mean to write a comment with `//`?".to_string())
        }
    }

    // _
    // #matcher
    //  ^
    // _
    // @modifier
    //  ^
    fn prefix(&mut self, char: &u8) -> ParseResult<AccumulatorRepr> {
        match (char, self.state.clone()) {
            (b'a'..=b'z' | b'A'..=b'Z' | b'_', Prefix(root_type)) =>
                self.set_state(Identifier(root_type))
                    .and_accumulate(char),
            (_, Prefix(RootType::Matcher(_))) =>
                ParseError("Illegal character at start of matcher name. Expected [A-z_]".to_string()),
            _ =>
                ParseError("Illegal character at start of modifier name. Expected [A-z_]".to_string())
        }
    }

    //  ______
    // #matcher {
    //   ^^^^^^
    fn identifier(&mut self, char: &u8) -> ParseResult<AccumulatorRepr> {
        match (char, self.state.clone()) {
            (b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_', _) => Accumulate(char.clone()),
            (b'(', Identifier(ident_type)) =>
                self.set_state(AwaitingArgOrArgsEnd)
                    .and_parse_accumulated(
                        match ident_type {
                            RootType::Matcher(_) => AccumulatorRepr::MatcherName,
                            RootType::Modifier(_) => AccumulatorRepr::ModifierName,
                            _ => return ParseError(format!("Illegal RootType {:?} while parsing an identifier", ident_type)),
                    }),
            (_, Identifier(ident_type)) => {
                self.set_state(AwaitingRootOrArgsBegin);
                match ident_type {
                    RootType::Matcher(_) => ParseAccumulated(AccumulatorRepr::MatcherName),
                    RootType::Modifier(_) => ParseAccumulated(AccumulatorRepr::ModifierName),
                    _ => ParseError(format!("Illegal RootType {:?} while parsing an identifier", ident_type)),
                }
            }
            (_, state) => ParseError(format!("ParseState {:?} illegally evaluating as identifier", state))
        }
    }

    fn awaiting_root_or_args_begin(&mut self, char: &u8) -> ParseResult<AccumulatorRepr> {
        match char {
            b'(' => self.set_state(AwaitingArgOrArgsEnd)
                        .and_continue(),
            handle_root => self.handle_root(handle_root)
        }
    }

    //             __      _
    // #matcher(some , other) {
    //              ^^      ^
    fn awaiting_delim_or_end(&mut self, char: &u8) -> ParseResult<AccumulatorRepr> {
        match char {
            c if c.is_ascii_whitespace() => Continue,
            b',' => self.set_state(AwaitingArgOrArgsEnd)
                        .and_continue(),
            b')' => self.set_state(Root)
                        .and_continue(),
            _ => ParseError("Expected start of expression ',' or ')'".to_string())
        }
    }

    fn accumulating_string(&mut self, char: &u8) -> ParseResult<AccumulatorRepr> {
        match (char, &self.state) {
            (b'\\', AccumulatingString(purpose)) => {
                self.set_state(AccumulatingStringEscaped(purpose.clone()))
                    .and_accumulate(char)
            }
            (b'"', AccumulatingString(purpose)) => {
                match purpose {
                    AccumulatorPurpose::MatcherLiteral =>
                        self.set_state(Root)
                            .and_parse_accumulated(AccumulatorRepr::MatcherLiteral),
                    AccumulatorPurpose::WithinExpression(depth) =>
                        self.set_state(AccumulatingExpr { depth: depth.clone() })
                            .and_accumulate(char),
                    AccumulatorPurpose::Parameter =>
                        self.set_state(AwaitingDelimOrArgsEnd)
                            .and_continue(),
                }
            }
            _ => Accumulate(char.clone())
        }
    }

    fn accumulating_string_escaped(&mut self, char: &u8) -> ParseResult<AccumulatorRepr> {
        if let AccumulatingStringEscaped(purpose) = &self.state {
            self.set_state(AccumulatingString(purpose.clone()))
                .and_accumulate_escaped(char)
        } else {
            ParseError("Illegal state".to_string())
        }
    }

    fn accumulating_expr(&mut self, char: &u8) -> ParseResult<AccumulatorRepr> {
        match (char, &self.state) {
            (c, _) if c.is_ascii_whitespace() =>
                Continue,
            (b'"', AccumulatingExpr { depth }) => {
                let accumulator_purpose = AccumulatorPurpose::WithinExpression(depth.clone());
                self.set_state(AccumulatingString(accumulator_purpose))
                    .and_continue()
            },
            (b',', AccumulatingExpr { depth: 0 }) =>
                self.set_state(AwaitingArgOrArgsEnd)
                    .and_parse_accumulated(AccumulatorRepr::Expression),
            (b')', AccumulatingExpr { depth: 0 }) =>
                self.set_state(Root)
                    .and_parse_accumulated(AccumulatorRepr::Expression),
            (b')', AccumulatingExpr { depth: 1 }) =>
                ParseAccumulated(AccumulatorRepr::Expression),
            (b')', AccumulatingExpr { depth }) =>
                self.set_state(AccumulatingExpr { depth: depth - 1 })
                    .and_accumulate(char),
            (b'(', AccumulatingExpr { depth }) =>
                self.set_state(AccumulatingExpr { depth: depth + 1 })
                    .and_accumulate(char),
            _ => Accumulate(char.clone())
        }
    }

    //               __     _
    // #matcher(some , other,) {
    //                ^^     ^
    fn awaiting_arg_or_end(&mut self, char: &u8) -> ParseResult<AccumulatorRepr> {
        match char {
            c if c.is_ascii_whitespace() => Continue,
            b'"' => self.set_state(AccumulatingString(AccumulatorPurpose::Parameter))
                        .and_continue(),
            b'(' => self.set_state(AccumulatingExpr { depth: 1 })
                .and_accumulate(char),
            _ => self.set_state(AccumulatingExpr { depth: 0 })
                .and_accumulate(char)
        }
    }
}