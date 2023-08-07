use std::sync::Arc;
use crate::expr::{Expr, ExprParser};
use crate::parse::ParseResult::{Consumed, Parsed, ParseError};
use crate::root::{Root, RootType};

pub(crate) enum ParseResult {
    ParseError(String),
    Consumed,
    Parsed(Arc<Root>),
    ParsedExpr(Expr),
}

#[derive(PartialEq)]
enum ParsingRootType {
    Matcher,
    Modifier,
    Variable,
    Root,
}

// signifies what it just found
// e.g. after parsing '$', would be `PrfixVariable`
enum ParseState {
    Default,
    LineCommentStart,
    LineComment,
    LiteralStart,
    Literal,
    LiteralEscaped,
    Done,
    Prefix,
    Identifier,
    ArgsEnd,
    AwaitingArgOrArgsEnd,
    AwaitingDelimOrArgsEnd,
    BodyBegin,
    BodyEnd,
    DeferredExpr,
    DeferredChild,
    AwaitingMatcherOrModifier,
}

pub(crate) struct Parser {
    state: ParseState,
    root_type: ParsingRootType,
    accumulator: Vec<u8>,
    root: Arc<Root>,
    expr_parser: ExprParser,
    parent_parser: Option<Box<Parser>>,
    child_parser: Option<Box<Parser>>,

}
impl Parser {
    pub(crate) fn new() -> Parser {
        Parser {
            state: ParseState::Default,
            root_type: ParsingRootType::Root,
            accumulator: Vec::new(),
            root: Arc::new(Root {
                branches: Vec::new(),
                root_type: RootType::Root,
                args: Vec::new(),
                modifiers: Vec::new(),
            }),
            expr_parser: Default::default(),
            parent_parser: None,
            child_parser: None,
        }
    }

    pub(crate) fn parse(&mut self, char: u8) -> ParseResult {
        let res = match self.state {
            ParseState::Prefix => self.prefix(char),
            ParseState::Identifier => self.identifier(char),
            ParseState::ArgsEnd => self.args_end(char),
            ParseState::BodyBegin { .. } => {}
            ParseState::BodyEnd { .. } => {}
            ParseState::Default => self.default(char),
            ParseState::Done => return Parsed(self.root.clone()),
            ParseState::AwaitingArgOrArgsEnd => self.awaiting_arg_or_end(char),
            ParseState::AwaitingDelimOrArgsEnd => self.awaiting_delim_or_end(char),
            ParseState::LineCommentStart => self.awaiting_delim_or_end(char),
            ParseState::DeferredExpr => self.deferred(char),
            ParseState::LineComment => {}
            ParseState::LiteralStart => {}
            ParseState::Literal => {}
            ParseState::LiteralEscaped => {}
            ParseState::DeferredChild => {}
        };

        todo!()
    }

    fn deferred_child(&mut self, char: u8) -> ParseResult {
        match self.child_parser.unwrap().parse(char) {
            Parsed(root) => {
                self.state = ParseState::AwaitingMatcherOrModifier;
                self.root.extend(root.to_owned().branches);
                Consumed
            }
            x => x,
        }
    }

    fn args_end(&mut self, char: u8) -> ParseResult {
        match char {
            b' ' | b'\n' | b'\t' => Consumed,
            b'#' => {
                self.state = ParseState::BodyBegin;
                self.child_parser = Some(Box::new(Parser::new()));
                Consumed
            }
            b'{' => {
                self.state = ParseState::BodyBegin;
                self.child_parser = Some(Box::new(Parser::new()));
                Consumed
            }
            _ => ParseError("Illegal character at start of expression. Expected @ or #".to_string())
        }
    }

    fn default(&mut self, char: u8) -> ParseResult {
        match char {
            b' ' | b'\n' | b'\t' => Consumed,
            b'/' => {
                self.state = ParseState::LineCommentStart;
                Consumed
            }
            b'@' => {
                self.state = ParseState::Prefix;
                self.root_type = ParsingRootType::Modifier;
                Consumed
            }
            b'#' => {
                self.state = ParseState::Prefix;
                self.root_type = ParsingRootType::Modifier;
                Consumed
            }
            _ => ParseError("Illegal character at start of expression. Expected @ or #".to_string())
        }
    }

    fn prefix(&mut self, char: u8) -> ParseResult {
        match (char, &self.root_type) {
            (x @ (b'a'..=b'z' | b'A'..=b'Z' | b'_'), _) => {
                self.accumulator.push(x);
                Consumed
            }
            (b'"', ParsingRootType::Matcher) => {
                self.state = ParseState::LiteralStart;
                Consumed
            }
            (_, ParsingRootType::Matcher) => ParseError("Illegal character at start of matcher name. Expected [A-z_\"]".to_string()),
            _ => ParseError("Illegal character at start of modifier name. Expected [A-z_]".to_string())
        }
    }

    fn identifier(&mut self, char: u8) -> ParseResult {
        match char {
            x @ (b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_') => {
                self.accumulator.push(x);
                Consumed
            }
            _ => ParseError("Illegal character within of identifier. Expected [A-z0-9_]".to_string())
        }
    }

    fn deferred(&mut self, char: u8) -> ParseResult {
        match self.expr_parser.parse(char) {
            err @ ParseError(_) => err,
            ParseResult::ParsedExpr(arg) => {
                self.root.add_arg(arg);
                self.state = ParseState::AwaitingArgOrArgsEnd;
                Consumed
            },
            x => x
        }
    }

    fn awaiting_delim_or_end(&mut self, char: u8) -> ParseResult {
        match char {
            b' ' | b'\n' | b'\t' => {
                Consumed
            },
            b',' => {
                Consumed
            },
            b')' => {
                self.state = ParseState::AwaitingArgOrArgsEnd;
                Consumed
            }
            _ => ParseError("Expected start of expression ',' or ')'".to_string())
        }
    }

    fn awaiting_arg_or_end(&mut self, char: u8) -> ParseResult {
        match char {
            b' ' | b'\n' | b'\t' => Consumed,
            b')' => {
                self.state = ParseState::ArgsEnd;
                Consumed
            }
            x => {
                match self.expr_parser.parse(x) {
                    Consumed => {
                        self.state = ParseState::DeferredExpr;
                        Consumed
                    },
                    ParseResult::ParsedExpr(e) => {
                        self.state = ParseState::AwaitingDelimOrArgsEnd;
                        self.root.add_arg(e);
                        Consumed
                    }
                    _ => ParseError("Expected start of expression or ')'".to_string())
                }
            }
        }
    }
}



