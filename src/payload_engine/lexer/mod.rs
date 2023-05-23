pub mod token;
pub mod regex;
use super::util::multidef_enum::MultidefEnum;

pub mod lexer {
    use std::ops::Deref;
    use super::MultidefEnum;

    pub type TokenType = usize;

    pub struct Lexer<'a> {
        pub token_types : MultidefEnum<'a, TokenType>
    }

    impl<'a> Lexer<'a> {
        pub fn new() -> Lexer<'a> {
            return Lexer {
                token_types : MultidefEnum::new()
            }
        }

        fn lex() {

        }
    }

    impl<'a> Deref for Lexer<'a> {
        type Target = MultidefEnum<'a, TokenType>;

        fn deref(&self) -> &Self::Target {
            &self.token_types
        }
    }

    impl<'a> std::ops::DerefMut for Lexer<'a> {

        fn deref_mut(&mut self) -> &mut MultidefEnum<'a, usize> {
            &mut self.token_types
        }
    }
}