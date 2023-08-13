use std::{mem};

use std::ops::{ControlFlow, Deref, FromResidual, Try};
use crate::accumulator::Accumulator;

use crate::matcher::MatcherType;
use crate::modifier::ModifierType;
use crate::parse::{AccumulatorRepr, ParseResult, ParseState};
use crate::parse::expr::Expr;
use crate::parse::ParseResult::*;
use crate::parse::root_parser::RootParser;
use crate::root::{BranchVariant, ParentInfo, Root, RootCollection, RootType};

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
impl<T> FromResidual<Option<T>> for ParsedByteReturnIndication {
    fn from_residual(residual: Option<T>) -> Self {
        match residual {
            Some(_) => Self::Continue,
            None => Self::Error("Unknown error from missing optional value".to_string())
        }
    }
}

pub(crate) struct PayloadParser {
    accumulator: Accumulator,
    roots: Vec<Root>,
    current_root_index: usize,
    parsers: Vec<RootParser>,
    parser_index: usize,
}
impl PayloadParser {
    pub fn new() -> PayloadParser {
        let root = Root {
            branches: vec![],
            root_type: RootType::Root,
            args: vec![],
            parent_info: None,
        };
        let root_parser = RootParser {
            state: ParseState::Root,
            parent_parser_index: None,
        };
        PayloadParser {
            accumulator: Accumulator::new(),
            roots: vec![root],
            current_root_index: 0,
            parsers: vec![root_parser],
            parser_index: 0,
        }
    }

    fn handle_parsed(
        current_parser_index: &mut usize,
        current_root_index: &mut usize,
        parsers: &mut Vec<RootParser>,
        roots: &mut Vec<Root>,
    ) -> ParsedByteReturnIndication {

        match (&parsers[current_parser_index.clone()].parent_parser_index, &roots[current_root_index.clone()].parent_info) {
            (
                Some(parser_index),
                Some(ParentInfo {
                         index_of_parent,
                         index_within_parent: _
                     })
            ) => {
                *current_parser_index = parser_index.clone();
                *current_root_index = index_of_parent.clone();
                ParsedByteReturnIndication::Continue
            }
            _ => ParsedByteReturnIndication::Return
        }
    }

    fn handle_parse_accumulated(current_parent_root_index: &mut usize,
                                roots: &mut Vec<Root>,
                                accumulator: &mut Accumulator,
                                accumulator_repr: AccumulatorRepr
    ) -> ParsedByteReturnIndication {
        match accumulator_repr {
            AccumulatorRepr::Expression => {
                let latest_root = roots.last_mut()?;
                latest_root.args.push(
                    Expr::try_from(accumulator.move_string())?
                )
            },
            AccumulatorRepr::MatcherName => {
                let new_root_index = roots.len();
                let current_parent = &mut roots[current_parent_root_index.clone()];
                let matcher_type = MatcherType::try_from(accumulator.move_string())?;
                let new_root_index_within_parent = current_parent.branches.len();
                current_parent.branches.push(BranchVariant::Root(new_root_index));
                roots.push(Root {
                    branches: vec![],
                    root_type: RootType::Matcher(matcher_type),
                    args: vec![],
                    parent_info: Some(ParentInfo {
                        index_of_parent: current_parent_root_index.clone(),
                        index_within_parent: new_root_index_within_parent.clone(),
                    }),
                })
            },
            AccumulatorRepr::ModifierName => {
                let new_root_index = roots.len();
                let current_parent = &mut roots[current_parent_root_index.clone()];
                let modifier_type = ModifierType::try_from(accumulator.move_string())?;
                let new_root_index_within_parent = current_parent.branches.len();
                current_parent.branches.push(BranchVariant::Root(new_root_index));
                roots.push(Root {
                    branches: vec![],
                    root_type: RootType::Modifier(modifier_type),
                    args: vec![],
                    parent_info: Some(ParentInfo {
                        index_of_parent: current_parent_root_index.clone(),
                        index_within_parent: new_root_index_within_parent.clone(),
                    }),
                })
            },
            AccumulatorRepr::MatcherLiteral => {
                let new_root_index = roots.len();
                let current_parent = &mut roots[current_parent_root_index.clone()];
                let new_root_index_within_parent = current_parent.branches.len();
                current_parent.branches.push(BranchVariant::Root(new_root_index));
                roots.push(Root {
                    branches: vec![],
                    root_type: RootType::Matcher(MatcherType::Literal(accumulator.move_string())),
                    args: vec![],
                    parent_info: Some(ParentInfo {
                        index_of_parent: current_parent_root_index.clone(),
                        index_within_parent: new_root_index_within_parent.clone(),
                    }),
                })
            }
            AccumulatorRepr::RustSrc => {
                let current_parent = &mut roots[current_parent_root_index.clone()];
                current_parent.branches.push(BranchVariant::Source(accumulator.move_string()));
            }
            AccumulatorRepr::Range => {
                let accumulated = accumulator.move_string();
                let bytes = accumulated.as_bytes();

                match (bytes.first(), bytes.get(1)) {
                    (Some(from), Some(to)) => {
                        let new_root_index = roots.len();
                        let current_parent = &mut roots[current_parent_root_index.clone()];
                        let new_root_index_within_parent = current_parent.branches.len();
                        current_parent.branches.push(BranchVariant::Root(new_root_index));
                        roots.push(Root {
                            branches: vec![],
                            root_type: RootType::Matcher(MatcherType::Range{
                                from: from.clone(),
                                to: to.clone()
                            }),
                            args: vec![],
                            parent_info: Some(ParentInfo {
                                index_of_parent: current_parent_root_index.clone(),
                                index_within_parent: new_root_index_within_parent.clone(),
                            }),
                        })
                    }
                    _ => return ParsedByteReturnIndication::Error(
                        format!("Wrong length for accumulated range. Expected string of length 2, but was {accumulated:?}")
                    )
                }
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
        let index_within_parent = current_root.branches.len();
        let new_root = Root {
            branches: Vec::new(),
            root_type: RootType::Root,
            args: Vec::new(),
            parent_info: Some(ParentInfo { index_of_parent: current_root_index.clone(), index_within_parent }),
        };

        current_root.branches.push(BranchVariant::Root(len.clone()));
        *current_root_index = roots.len();
        roots.push(new_root);


        parsers.push(RootParser {
            state: ParseState::Root,
            parent_parser_index: Some(current_parser_index.clone()),
        });
        *current_parser_index = parsers.len() - 1;

        ParsedByteReturnIndication::Continue
    }

    fn parse_with(current_parser_index: &mut usize,
                  current_root_index: &mut usize,
                  parsers: &mut Vec<RootParser>,
                  roots: &mut Vec<Root>,
                  accumulator: &mut Accumulator,
                  byte: &u8,
    ) -> ParsedByteReturnIndication {

        match parsers[current_parser_index.clone()].parse(byte) {
            Defer => Self::handle_defer(current_parser_index, current_root_index, roots, parsers),
            Parsed => Self::handle_parsed(current_parser_index, current_root_index, parsers, roots),
            ParseAccumulated(accumulator_repr) =>
                Self::handle_parse_accumulated(current_root_index, roots, accumulator, accumulator_repr),
            Accumulate(c) => {
                accumulator.push(char::from(c));
                println!("accumulated {:?} (now {:?})", char::from(byte.clone()), accumulator);
                ParsedByteReturnIndication::Continue
            }
            result => ParsedByteReturnIndication::from(result),
        }
    }

    pub(crate) fn parse(mut self, source: &[u8]) -> Result<RootCollection, String> {
        let PayloadParser {
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
                ParsedByteReturnIndication::Return => return Ok(RootCollection::from(self.roots)),
                ParsedByteReturnIndication::Error(msg) => return Err(msg),
                ParsedByteReturnIndication::Continue => ()
            }
        };
        Ok(RootCollection::from(self.roots))
    }
}