use crate::payload_engine::lexer::token::Token;
use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::payload_engine::lexer::lexer::TokenType;

pub fn random_token_stream<'a>(len: usize) -> Vec<Token<'a>> {
    let mut start = 0;
    let mut len = 0;

    vec![rand_token(get_rand_token(start); len];

    fn get_rand_token(&mut start : usize) -> Token {
        start += 1;

        rand_token(len, start, token_type)
    }
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