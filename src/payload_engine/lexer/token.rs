use super::lexer::TokenType;


pub struct Token<'a> {
    pub text : &'a str,
    pub token_type : TokenType,
    pub start : usize
}

impl<'a> Token<'a> {
    pub fn len(self) -> usize {
        return self.text.len()
    }
}

impl<'a> Clone for Token<'a> {
    fn clone(&self) -> Self {
        return Token {
            text : self.text,
            token_type : self.token_type,
            start : self.start
        }
    }
}


