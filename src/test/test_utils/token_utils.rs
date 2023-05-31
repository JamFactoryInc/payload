use std::alloc::{alloc, Layout};
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
    // const size : ((usize, usize, usize, usize,),
    //               (usize, usize, usize, usize,),
    //               (usize, usize, usize, usize,)) = parse_regexp(r"[]");

    // let arr = [0u8; size.0.0];
    //
    // let val = CharClass::<{ size.0.0 }> {};
    // println!("{}", size.0.0)
}

// enum Pattern {
//     Literal(LiteralPattern)
// }

struct Regexp<const N : usize> {

    subgroups : [Group; N]
}

struct Group {

}

// keywords and hardcoded values
struct LiteralPattern<const N : usize> {
    index: usize,
    literal: [u8; N]
}

// struct Or2Pattern<A : Pattern, B : Pattern> {
//     a: A,
//     b: B,
// }
// struct Or3Pattern<A : Pattern, B : Pattern, C : Pattern> {
//     a: A,
//     b: B,
//     c: C,
// }
// struct Or4Pattern<A : Pattern, B : Pattern, C : Pattern, D : Pattern> {
//     a: A,
//     b: B,
//     c: C,
//     d: D,
// }

struct CharClass<const N: usize> {

}

macro_rules! arr {
    ([$lit:literal; $id:ident]) => {
        match $id {
            0 =>  [$lit; 0],
            1 =>  [$lit; 1],
            2 =>  [$lit; 2],
            3 =>  [$lit; 3],
            4 =>  [$lit; 4],
            5 =>  [$lit; 5],
            6 =>  [$lit; 6],
            7 =>  [$lit; 7],
            8 =>  [$lit; 8],
            9 =>  [$lit; 9],
            10 =>  [$lit; 10],
            _ => panic!("Number outside available range")
        }
    }
}

/// validates the input and returns a tuple of tuples:
/// The first array is 4 usizes indicating the lengths of up to 4 character classes
/// The first usize describes how many character classes there are
/// The second array is 4 usizes indicating the lengths of up to 4 groups
/// The second usize describes how many groups there are
const fn parse_regexp<const N: usize>(string : &str) -> ([usize; N], usize, [usize; N], usize) {
    let bytes = string.as_bytes();

    let c_class_sizes = [0usize; N];
    let group_sizes = [0usize; N];

    let mut paren_depth = 0;
    let mut bracket_depth = 0;
    let mut is_escaped = false;
    let mut in_char_class = false;
    let mut can_be_repeated = false;

    let mut c_class_size_index = 0;
    let mut group_size_index = 0;

    let mut i = 0;
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
                b')' => {
                    paren_depth -= 1;
                    can_be_repeated = true;
                },
                b'[' => {
                    in_char_class = true;
                    if bracket_depth == 1 {
                        panic!("Cannot have nested char classes");
                    }
                    bracket_depth += 1;
                    can_be_repeated = false;
                },
                b']' => {
                    in_char_class = false;
                    bracket_depth -= 1;
                    can_be_repeated = true;
                },
                b'*' | b'+' => {
                    if !can_be_repeated {
                        panic!("Token cannot be repeated ");
                    }
                },
                b'\\' => is_escaped = true,
                _ => ()
            }
        } else {

        }
        i += 1;
    }

    if paren_depth != 0 {
        panic!("Unbalanced parentheses in regular expression")
    }

    todo!()

}