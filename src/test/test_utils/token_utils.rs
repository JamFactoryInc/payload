use crate::payload_engine::lexer::token::Token;
use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::payload_engine::lexer::lexer::TokenType;

pub fn random_token_stream<'a>(len: usize) -> Vec<Token<'a>>{
    vec![rand_token<'a>(); len]
}

pub fn rand_token<'a>(len : usize, start: usize, token_type : TokenType) -> Token {

    let text : String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();

    Token {
        text,
        token_type,
        start,
    }
}