use std::cell::RefCell;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::{mem, ptr};
use std::io::Read;
use std::ops::{ControlFlow, Deref, FromResidual, Try};
use crate::matcher::MatcherType;
use crate::modifier::ModifierType;
use crate::parse::ParseResult;
use crate::parse::ParseResult::*;
use crate::parse::state_parsers::RootParser;
use crate::root::{BranchValue, Root, RootType};

pub(crate) struct ParserArena {
    accumulator: String,
    roots: Vec<Root>,
    current_root_index: usize,
    parsers: Vec<RootParser>,
    parser_index: usize,
}
impl ParserArena {
    /// swaps the pointer of self.accumulator with a pointer to an equal-capacity empty string
    /// basically self.accumulator.clone() and self.accumulator.clear() in one
    fn use_accumulator(&mut self) -> String {
        let mut to_be_swapped = String::with_capacity(self.accumulator.capacity());
        mem::swap(&mut self.accumulator, &mut to_be_swapped);
        to_be_swapped
    }

    pub(crate) fn parse(&mut self, source: &[u8]) -> Result<&Vec<Root>, String> {
        let mut current_parser = self.parsers.first().unwrap();
        let mut current_root = &self.roots[self.current_root_index.clone()];
        for byte in source {
            match match current_parser.parse(byte) {
                ParseError(msg) => Some(Err(msg)),
                Accumulate => {
                    self.accumulator.push(char::from(byte.clone()));
                    None
                }
                ParseAccumulated(_) => {

                }
                Continue => None,
                Defer => {}
                Parsed => {}
                ParsedExpr(_) => {}
            } {
                Some(Ok(())) => return Ok(&self.roots),
                Some(Err(msg)) => return Err(msg),
                None => {}
            }
        }

        let parsed = || {
            match (&current_parser.parent_parser_index, &current_root.parent) {
                (
                    Some(parser_index), Some(root_index)
                ) => {
                    current_parser = &self.parsers[parser_index.clone()];
                    current_root = &self.roots[root_index.clone()];
                    None
                },
                _ => Some(Ok(()))
            }
        };

        let parse_accumulated = ||

        todo!()
    }



    pub(crate) fn parse_byte(&mut self, root_parser: &mut RootParser, current_root: & Root, char: &u8) -> ParseResult {
        match root_parser.parse(char) {
            None => ParseError(format!("Unexpected internal error [code 3:{}:{}]", self.parsers.len(), self.parser_index)),
            Some(parser) => match parser.parse(char) {
                Parsed => match &parser.parent_parser_index {
                    Some(parent_index) => {
                        self.parser_index = parent_index.clone();
                        match &current_root.parent {
                            Some(i) => {
                                self.current_root_index = i.clone();
                                Continue
                            }
                            None => ParseError("Unexpected internal error [code 4]".to_string()),
                        }
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

                    self.current_root.branches.push(BranchValue::Root(new_root.index.clone()));
                    self.roots.push(new_root);
                    self.current_root = &mut self.roots.last().unwrap();

                    self.parsers.push(RootParser {
                        state: ParseState::Root,
                        root_type: ParsingRootType::Root,
                        parent_parser_index: Some(self.parser_index.clone()),
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
        }
    }
}