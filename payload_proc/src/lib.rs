#![feature(portable_simd)]
#![feature(try_trait_v2)]

use proc_macro::TokenStream;
use crate::parse::payload_parser::PayloadParser;

pub(crate) mod matcher;
pub(crate) mod modifier;
pub(crate) mod product;
pub(crate) mod root;
pub(crate) mod parse;
mod variable;
mod describe;
mod accumulator;

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