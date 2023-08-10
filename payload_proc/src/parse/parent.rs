use std::cell::RefCell;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::{mem, ptr};
use std::ffi::c_uint;
use std::io::Read;
use std::ops::{ControlFlow, Deref, FromResidual, Try};
use std::pin::Pin;
use crate::matcher::MatcherType;
use crate::modifier::ModifierType;
use crate::parse::{AccumulatorRepr, ParseResult, ParseState, ParsingRootType};
use crate::parse::expr::Expr;
use crate::parse::ParseResult::*;
use crate::parse::state_parsers::RootParser;
use crate::root::{BranchValue, Root, RootType};

enum ParsedByteReturnIndication {
    Continue,
    Return,
    Error(String)
}
impl From<ParseResult> for ParsedByteReturnIndication {
    fn from(value: ParseResult) -> Self {
        match value {
            ParseError(msg) => Self::Error(msg),
            _ => Self::Continue,
        }
    }
}

impl<A> FromResidual<Result<A, String>> for ParsedByteReturnIndication {
    fn from_residual(residual: Result<A, String>) -> Self {
        match residual {
            Ok(_) => Self::Continue,
            Err(msg) => Self::Error(msg)
        }
    }
}

struct CachedVec<T> {
    vec: Vec<T>,
    item: Box<Pin<T>>,
}

pub(crate) struct ParserArena {
    accumulator: String,
    roots: Vec<Root>,
    current_root_index: usize,
    parsers: Vec<RootParser>,
    parser_index: usize,
}
impl ParserArena {

    pub fn new() -> ParserArena {
        let root = Root {
            branches: vec![],
            root_type: RootType::Root,
            args: vec![],
            parent: None,
        };
        let root_parser = RootParser {
            state: ParseState::LineCommentStart,
            root_type: ParsingRootType::Block,
            parent_parser_index: None,
        };
        ParserArena {
            accumulator: String::new(),
            roots: vec![root],
            current_root_index: 0,
            parsers: vec![root_parser],
            parser_index: 0,
        }
    }

    /// swaps the pointer of self.accumulator with a pointer to an equal-capacity empty string
    /// basically self.accumulator.clone() and self.accumulator.clear() in one
    fn use_accumulator(accumulator: &mut String) -> String {
        let mut to_be_swapped = String::with_capacity(accumulator.capacity());
        mem::swap(accumulator, &mut to_be_swapped);
        to_be_swapped
    }

    fn handle_parsed(current_parser_index: &mut usize,
                     current_root_index: &mut usize,
                     parsers: &mut Vec<RootParser>,
                     roots: &mut Vec<Root>,
    ) -> ParsedByteReturnIndication {
        match (&parsers[current_parser_index.clone()].parent_parser_index, &roots[current_root_index.clone()].parent) {
            (
                Some(parser_index), Some(root_index)
            ) => {
                *current_parser_index = parser_index.clone();
                *current_root_index = root_index.clone();
                ParsedByteReturnIndication::Continue
            },
            _ => ParsedByteReturnIndication::Return
        }
    }

    fn handle_parse_accumulated(current_root_index: &mut usize,
                                roots: &mut Vec<Root>,
                                accumulator: &mut String,
                                accumulator_repr: AccumulatorRepr
    ) -> ParsedByteReturnIndication {
        let current_root = &mut roots[current_root_index.clone()];
        match accumulator_repr {
            AccumulatorRepr::Expression => current_root.args.push(
                Expr::try_from(Self::use_accumulator(accumulator))?
            ),
            AccumulatorRepr::MatcherName => current_root.root_type = RootType::Matcher(
                MatcherType::try_from(Self::use_accumulator(accumulator))?
            ),
            AccumulatorRepr::ModifierName => current_root.root_type = RootType::Modifier(
                ModifierType::try_from(Self::use_accumulator(accumulator))?
            ),
            AccumulatorRepr::MatcherLiteral => {
                current_root.root_type = RootType::Matcher(MatcherType::Literal);
                current_root.args.push(Expr::String(Self::use_accumulator(accumulator)));
            }
            AccumulatorRepr::RustSrc => {
                current_root.branches.push(BranchValue::Source(Self::use_accumulator(accumulator)));
            }
        };
        ParsedByteReturnIndication::Continue
    }

    fn handle_defer(
        current_parser_index: &mut usize,
        current_root_index: &mut usize,
        roots: &mut Vec<Root>,
        parsers: &mut Vec<RootParser>,
    ) -> ParsedByteReturnIndication {
        let len = roots.len();
        let current_root = &mut roots[current_root_index.clone()];
        let new_root = Root {
            branches: Vec::new(),
            root_type: RootType::Root,
            args: Vec::new(),
            parent: Some(current_root_index.clone()),
        };

        current_root.branches.push(BranchValue::Root(len.clone()));
        *current_root_index = roots.len();
        roots.push(new_root);


        parsers.push(RootParser {
            state: ParseState::Root,
            root_type: ParsingRootType::Root,
            parent_parser_index: Some(current_parser_index.clone()),
        });
        *current_parser_index = parsers.len() - 1;

        ParsedByteReturnIndication::Continue
    }

    fn parse_with(current_parser_index: &mut usize,
                  current_root_index: &mut usize,
                  parsers: &mut Vec<RootParser>,
                  roots: &mut Vec<Root>,
                  accumulator: &mut String,
                  byte: &u8,
    ) -> ParsedByteReturnIndication {

        match parsers[current_parser_index.clone()].parse(byte) {
            Defer => Self::handle_defer(current_parser_index, current_root_index, roots, parsers),
            Parsed => Self::handle_parsed(current_parser_index, current_root_index, parsers, roots),
            ParseAccumulated(accumulator_repr) =>
                Self::handle_parse_accumulated(current_root_index, roots, accumulator, accumulator_repr),
            Accumulate => {
                accumulator.push(char::from(byte.clone()));
                ParsedByteReturnIndication::Continue
            }
            result => ParsedByteReturnIndication::from(result),
        }
    }

    pub(crate) fn parse(&mut self, source: &[u8]) -> Result<&Vec<Root>, String> {
        let ParserArena {
                ref mut accumulator,
                ref mut roots,
                ref mut current_root_index,
                ref mut parsers,
                ref mut parser_index
            } = self;
        for byte in source {
            let result = Self::parse_with(
                parser_index,
                current_root_index,
                parsers,
                roots,
                accumulator,
                byte
            );
            match result {
                ParsedByteReturnIndication::Return => return Ok(&self.roots),
                ParsedByteReturnIndication::Error(msg) => return Err(msg),
                ParsedByteReturnIndication::Continue => ()
            }
        };


        todo!()
    }
}