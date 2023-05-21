pub mod token;

pub mod lexer {
    use crate::payload_engine::lexer::token::TokenTypeData;

    pub struct Lexer<'a> {
        pub tt_data :TokenTypeData<'a>
    }

    impl<'a> Lexer<'a> {
        pub fn new() -> Lexer<'a> {

            return Lexer {
                tt_data : TokenTypeData::new()
            }
        }

        fn lex() {

        }
    }
}