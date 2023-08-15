#![feature(portable_simd)]
#![feature(try_trait_v2)]

use proc_macro::TokenStream;
use crate::parse::payload_parser::PayloadParser;

#[macro_use]
pub(crate) mod stateful_parser;
pub(crate) mod matcher;
pub(crate) mod modifier;
pub(crate) mod product;
pub(crate) mod root;
pub(crate) mod parse;
pub(crate) mod variable;
pub(crate) mod describe;
pub(crate) mod accumulator;

#[proc_macro]
pub fn regex(input: TokenStream) -> TokenStream {


    input
}

#[test]
fn test() {
    let input = r#"
    @link
    @visibility {
        "wow a literal"
    }
    #alpha
    'a'..'b'
    "#;

    let mut parser = PayloadParser::new();

    let parse_result = parser.parse(input.as_bytes());
    match parse_result {
        Ok(result) => println!("{:?}", result),
        Err(err) => println!("{}", err)
    }

}