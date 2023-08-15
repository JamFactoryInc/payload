use crate::accumulator::Accumulator;
use crate::parse::{AccumulatorRepr, ParseResult};
use crate::stateful_parser;
use crate::variable::Variable;

#[derive(Debug)]
pub(crate) enum Expr {
    None,
    Binary(Box<Expr>, ExprOperator, Box<Expr>),
    Postfix(Box<Expr>, ExprOperator),
    Prefix(ExprOperator, Box<Expr>),
    String(String),
    Int(isize),
    Float(f32),
    Variable(Variable),
    Scope,
}
impl Default for Expr {
    fn default() -> Self {
        Expr::None
    }
}
impl TryFrom<String> for Expr {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parser = ExprParser::new();

        todo!()
    }
}

#[derive(Debug)]
pub(crate) enum ExprOperator {
    Assign,
}

enum ExprParseState {
    Default,
    Ident(Box<ExprParseState>),
    String,
    StringEscaped,
    Int,
    Float,
    LeadingDot,
    Scope
}

enum ExprParseResult {
    Continue,
    Parsed(Expr),
    Error(String),
}

pub(crate) struct ExprParser {
    state: ExprParseState,
    accumulator: Accumulator,
}
impl ExprParser {
    pub(crate) fn new() -> ExprParser {
        ExprParser {
            state: ExprParseState::Default,
            accumulator: Accumulator::<u8>::new(),
        }
    }

    pub(crate) fn parse(&mut self, expr: String) -> Result<Vec<Expr>, String> {
        todo!()
    }

    pub(crate) fn parse_byte(&mut self, char: u8) -> ParseResult<AccumulatorRepr> {
        match (&char, &self.state) {
            (b'$', ExprParseState::Default) => {
                self.state = ExprParseState::Ident(Box::new(ExprParseState::Default));
                ParseResult::Continue
            }
            (b'a'..=b'z', ExprParseState::Ident(_)) => ParseResult::Accumulate(char),
            (_, ExprParseState::Ident(_)) => panic!(),
                //ParseResult::ParsedExpr(Expr::Variable(Variable::try_from( Accumulator::vec_to_str(self.accumulator.move_vec()))?)),
            (b'-' | b'0'..=b'9', ExprParseState::Default) => {
                self.state = ExprParseState::Int;
                ParseResult::Accumulate(char)
            }
            (b'.', ExprParseState::Default) => {
                self.state = ExprParseState::LeadingDot;
                ParseResult::Accumulate(char)
            }
            (b'.', ExprParseState::Int) | (b'0'..=b'9', ExprParseState::LeadingDot) => {
                self.state = ExprParseState::Float;
                ParseResult::Accumulate(char)
            }
            (b'/', ExprParseState::LeadingDot) => {
                self.state = ExprParseState::Scope;
                ParseResult::Accumulate(char)
            }
        }
    }
}

impl Default for ExprParser {
    fn default() -> Self {
        ExprParser {
            state: ExprParseState::Default,
            accumulator: Accumulator::<u8>::new()
        }
    }
}