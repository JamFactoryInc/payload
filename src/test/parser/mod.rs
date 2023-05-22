#[cfg(test)]

use crate::payload_engine::parser::Parser;


#[test]
pub fn test_token_type_ctor() {
    let mut parser : Parser = Parser::new();

    let p_type_1 = parser.create_type("Name1");
    let p_type_2 = parser.create_type("Name2");

    assert_eq!(0, p_type_1);
    assert_eq!(1, p_type_2);

    assert_eq!("Name1", parser.nameof(p_type_1));
    assert_eq!("Name2", parser.nameof(p_type_2));
}


