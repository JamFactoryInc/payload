use super::lexer::TokenType;


pub struct Token<'a> {
    text : &'a str,
    token_type : TokenType,
    start : usize
}

impl<'a> Token<'a> {
    pub fn len(self) -> usize {
        return self.text.len()
    }
}


