use crate::expr::{Expr, ExprParser};
use crate::parse::ParseResult::{Consumed, ParseError};
use crate::parse::ParseState::{ArgEnd, ArgsEnd, Deferred};
use crate::root::{Root, RootType};

pub(crate) enum ParseResult {
    ParseError(String),
    Consumed,
    Parsed(Root),
    ParsedExpr(Expr),
}

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
    Done,
    Prefix,
    Identifier,
    ArgsEnd,
    ArgEnd,
    AwaitingArgOrArgsEnd,
    BodyBegin{ depth: usize },
    BodyEnd{ depth: usize },
    Deferred,
}

pub(crate) struct Parser {
    state: ParseState,
    root_type: ParsingRootType,
    accumulator: Vec<u8>,
    root: Option<Root>,
    expr_parser: ExprParser

}
impl Parser {
    pub(crate) fn new() -> Parser {
        Parser {
            state: ParseState::Default,
            root_type: ParsingRootType::Root,
            accumulator: Vec::new(),
            root: Some(Root {
                branches: Vec::new(),
                root_type: RootType::Root,
                args: Vec::new(),
                modifiers: Vec::new(),
            }),
            expr_parser: Default::default(),
        }
    }

    pub(crate) fn parse(&mut self, char: u8) -> Option<Root> {
        let res = match self.state {
            ParseState::Prefix => self.prefix(char),
            ParseState::Identifier => self.identifier(char),
            ParseState::ArgStart => {}
            ParseState::ArgsStart => {}
            ParseState::Arg => {}
            ParseState::ArgEnd => {}
            ParseState::ArgsEnd => {}
            ParseState::BodyBegin { .. } => {}
            ParseState::BodyEnd { .. } => {}
            ParseState::Default => {}
            ParseState::Done => return self.root.take()
        };

        todo!()
    }

    fn prefix(&mut self, char: u8) -> ParseResult {
        match char {
            x @ (b'a'..=b'z' | b'A'..=b'Z' | b'_') => {
                self.accumulator.push(x);
                ParseResult::Consumed
            }
            _ => ParseError("Illegal character at start of identifier".to_string())
        }
    }

    fn identifier(&mut self, char: u8) -> ParseResult {
        match char {
            x @ (b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_') => {
                self.accumulator.push(x);
                ParseResult::Consumed
            }
            _ => ParseError("Illegal character at start of identifier".to_string())
        }
    }

    fn deferred(&mut self, char: u8) -> ParseResult {
        match self.expr_parser.parse(char) {
            err @ ParseError(_) => err,
            ParseResult::ParsedExpr(arg) => {
                let mut root = self.root.take().unwrap();
                root.add_arg(arg);
                self.root = Some(root);

                self.state = ParseState::ArgEnd;

                Consumed
            },
            x => x
        }
    }

    fn arg_end(&mut self, char: u8) -> ParseResult {
        match char {
            b' ' => {
                self.AwaitingArgOrArgsEnd
                Consumed
            },
            b')' => {
                self.state = ArgsEnd;
                Consumed
            }
            _ => ParseError("Illegal character at start of identifier".to_string())
        }
    }
}



