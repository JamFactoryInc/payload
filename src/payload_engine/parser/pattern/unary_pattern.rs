use crate::payload_engine::lexer::lexer::TokenType;
use crate::payload_engine::lexer::token::Token;
use crate::payload_engine::parser::pattern::{MatchStatus, Pattern};

pub struct UnaryPattern {
    enabled : bool,
    token_type : usize,
}

impl UnaryPattern {
    pub fn new(token_type : TokenType) -> UnaryPattern {
        UnaryPattern {
            enabled : true,
            token_type
        }
    }
}

impl Pattern for UnaryPattern {

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn consume(&mut self, current: Token, _: Token) -> MatchStatus {
        if current.token_type == self.token_type {
            MatchStatus::Complete
        } else {
            MatchStatus::Nil
        }
    }

    fn reset(&mut self) {
        self.enabled = true;
    }
}