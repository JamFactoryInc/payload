use std::collections::linked_list;
use std::fmt::{Display, Formatter};
use std::future::Future;
use std::mem;
use std::ops::Add;
use std::process::Output;
use crate::payload_engine::lexer::token::Token;
use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::payload_engine::lexer::lexer::TokenType;

#[cfg(test)]

pub fn random_token_stream<'a>(len: usize) -> Vec<Token<'a>> {
    let mut start = 0;
    let mut len = 0;

    //vec![rand_token(get_rand_token(start); len];

    // fn get_rand_token(&mut start : usize) -> Token {
    //     start += 1;
    //
    //     rand_token(len, start, token_type)
    // }
    //return vec![rand_token(5, 0, 0); 0];
    todo!()
}

// pub fn rand_token<'a>(len : usize, start: usize, token_type : TokenType) -> Token<'a> {
//
//     let text : Vec<u8> = rand::thread_rng()
//         .sample_iter(&Alphanumeric)
//         .take(len)
//         .collect();
//
//     Token {
//         text : &text,
//         token_type,
//         start,
//     }
// }

#[test]
fn test1() {
    println!("{}", parse_regexp("[[]"))
}

struct Group {

    //subgroups : Box<[RegexpGroup; D]>
}

struct CharClass<const N: usize> {

}

trait Empty {
    type Output;
    fn empty() -> Output;
}

macro_rules! gen_fixed_arrays {
    ($t:ty,$def:literal;$($num:literal)*) => {
        paste::item! {
                $(
                    const fn [<fixed_arr_ $num>]() -> [$t; $num] { [$def; $num] }
                )*
        }
    }
}

gen_fixed_arrays!(u8,0; 0 1 2 3 4 5 6 7 8 9 10);

const fn parse_regexp(string : &str) -> u8 {
    let mut ret = 0
    let bytes = string.as_bytes();
    let mut i = 0;

    let mut paren_depth = 0;
    let mut bracket_depth = 0;
    let mut is_escaped = false;
    let mut in_char_class = false;

    let dyn_arr = match paren_depth {
        0 => fixed_arr_0(),
        _ =>
    }

    while i != bytes.len() {
        let char = bytes[i];

        if !is_escaped {

            match char {
                b'(' => {
                    if paren_depth == 1 {
                        panic!("Cannot have nested groups");
                    }
                    paren_depth += 1;
                },
                b')' => paren_depth -= 1,
                b'[' => {
                    in_char_class = true;
                    if bracket_depth == 1 {
                        panic!("Cannot have nested char classes");
                    }
                    bracket_depth += 1;
                },
                b']' => {
                    in_char_class = false;
                    bracket_depth -= 1;
                },
                b'*' | b'+' => (),

                _ => ()
            }
        } else {
            match char {
                _ => {

                }
            }
        }
        i += 1;
    }

    if paren_depth != 0 {
        panic!("Unbalanced parentheses in regular expression")
    }

    return ret as u8;
}