use super::payload_engine::lexer::token::TokenType;

#[cfg(test)]
mod token {
    use crate::payload_engine::lexer::lexer::Lexer;
    use crate::payload_engine::lexer::token::TokenType;

    #[cfg(test)]
    pub fn test_token_add_index() {
        let lex : Lexer = Lexer::new();

        let t_type_1 = TokenType::new("Name1", lex.tt_data);
    }
}