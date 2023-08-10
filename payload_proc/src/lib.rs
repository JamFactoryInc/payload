#![feature(portable_simd)]
#![feature(try_trait_v2)]

use proc_macro::TokenStream;

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