#![feature(portable_simd)]
#![feature(try_trait_v2)]

use proc_macro::TokenStream;
use crate::parse::parent::ParserArena;

pub(crate) mod matcher;
pub(crate) mod modifier;
pub(crate) mod product;
pub(crate) mod root;
pub(crate) mod parse;
mod variable;


#[proc_macro]
pub fn regex(input: TokenStream) -> TokenStream {


    input
}

#[test]
fn test() {
    let input = r#"
    @link
    "#;

    let mut parser = ParserArena::new();

    let parse_result = parser.parse(input.as_bytes());
    println!("{:?}", parse_result)
}