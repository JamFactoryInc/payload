use std::vec::Vec;

pub struct TokenTypeData<'a> {
    token_type_names : Vec<&'a str>,
    token_indx : u16
}

impl<'a> TokenTypeData<'a> {
    pub fn new<'b>() -> TokenTypeData<'b> {
        return TokenTypeData {
            token_type_names : Vec::new(),
            token_indx : 0
        }
    }
}

pub struct TokenType {
    index: u16
}

impl TokenType {
    pub fn new<'a>(name: &'a str, mut tt_data: TokenTypeData<'a>) -> TokenType {
        tt_data.token_type_names.push(name);

        let ret = TokenType {
            index : tt_data.token_indx
        };

        tt_data.token_indx += 1;

        return ret
    }

    pub fn name(self, tt_data: TokenTypeData) -> &str {
        return tt_data.token_type_names.get(self.index)
    }
}

pub struct Token {

}
