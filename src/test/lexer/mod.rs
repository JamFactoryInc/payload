mod token;

#[cfg(test)]
mod lexer {
    use crate::payload_engine::lexer::lexer::Lexer;

    #[test]
    pub fn test_token_type_ctor() {
        let mut lex : Lexer = Lexer::new();

        let t_type_1 = lex.create_type("Name1");
        let t_type_2 = lex.create_type("Name2");

        assert_eq!(0, t_type_1);
        assert_eq!(1, t_type_2);

        assert_eq!("Name1", lex.nameof(t_type_1));
        assert_eq!("Name2", lex.nameof(t_type_2));
    }
}