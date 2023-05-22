pub mod pattern;

use super::util::multidef_enum::MultidefEnum;
use std::ops::Deref;

pub type ParseType = usize;

pub struct Parser<'a> {
    parse_types : MultidefEnum<'a, ParseType>
}

impl<'a> Parser<'a> {
    pub fn new() -> Parser<'a> {
        Parser {
            parse_types : MultidefEnum::new()
        }
    }
}

impl<'a> Deref for Parser<'a> {
    type Target = MultidefEnum<'a, ParseType>;

    fn deref(&self) -> &Self::Target {
        &self.parse_types
    }
}

impl<'a> std::ops::DerefMut for Parser<'a> {

    fn deref_mut(&mut self) -> &mut MultidefEnum<'a, usize> {
        &mut self.parse_types
    }
}
