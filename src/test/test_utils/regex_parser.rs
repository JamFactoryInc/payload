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
use crate::payload_engine::lexer::regex;

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

macro_rules! pre_format_regex {
    (() ($($processed:tt)*) $($res:tt)*) => {
        parse_regex!($($res)*)
    };
    (([$literal:ident] $($t:tt)*) ($($processed:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* [$literal]) $($res)* $literal)
    };
    (($any:tt {0,0} $($t:tt)*) ($($processed:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any:tt {0,0}) $($res)*)
    };
    (($any:tt {,0} $($t:tt)*) ($($processed:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any:tt {,0}) $($res)*)
    };
    (($any:tt {0} $($t:tt)*) ($($processed:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any:tt {0}) $($res)*)
    };
    (($any:tt {0,1} $($t:tt)*) ($($processed:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any {0,1}) $($res)* $any ?)
    };
    (($any:tt {1,} $($t:tt)*) ($($processed:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any {1,}) $($res)* $any +)
    };
    (($any:tt {0,} $($t:tt)*) ($($processed:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any {0,}) $($res)* $any *)
    };
    (($any:tt {1} $($t:tt)*) ($($processed:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any {1}) $($res)* $any)
    };
    (($any:tt {$repeats:literal} $($t:tt)*) ($($processed:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any {$repeats}) $($res)* $any {$repeats})
    };
    (($any:tt {,$to:literal} $($t:tt)*) ($($processed:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any {,$to}) $($res)* $any {0, $to})
    };
    (($any:tt {$from:literal,} $($t:tt)*) ($($processed:tt)* ) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any {$from,}) $($res)* $any {$from, 9999})
    };
    (($any:tt {,} $($t:tt)*) ($($processed:tt)*) $($res:tt)*) => {
        range_err!({,} ($($t)*) ($($processed)* $any))
    };
    (($any:tt {} $($t:tt)*) ($($processed:tt)*) $($res:tt)*) => {
        range_err!({} ($($t)*) ($($processed)* $any))
    };
    (($any:tt {$($stuff:tt)*} $($t:tt)*) ($($processed:tt)*) $($res:tt)*) => {
        range_err!({$($stuff)*} ($($t)*) ($($processed)* $any))
    };
    (([] $($t:tt)*) ($($processed:tt)*) $($res:tt)*) => {
        panic!("empty char class not allowed")
    };
    (([^] $($t:tt)*) ($($processed:tt)*) $($res:tt)*) => {
        panic!("empty char class not allowed")
    };
    (([^$($stuff:tt)*] $($t:tt)*) ($($processed:tt)*) $($res:tt)*) => {
        pre_check_class!(($($stuff)*) ($($processed)* ^) ($($t)*))
        pre_format_regex!(($($t)*) ($($processed)* $any) $($res)* $any)
    };
    (([$($stuff:tt)*] $($t:tt)*) ($($processed:tt)*) $($res:tt)*) => {
        pre_check_class!(($($stuff)*) ($($processed)*) ($($t)*))
        pre_format_regex!(($($t)*) ($($processed)* $any) $($res)* $any)
    };
    (($any:tt $($t:tt)*) ($($processed:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any) $($res)* $any)
    };
}

macro_rules! pre_check_class {
    // end of char range
    (($char:ident $($todo:tt)*) ($($processed:tt)*) ($($before:tt)*) ($($after:tt)*) #br #bm) => {
        pre_check_class!(($($todo)*) ($($processed)* $char) ($($before)*) ($($after)*))
    };
    // beginning of sequence
    (($char:ident $($todo:tt)*) ($($processed:tt)*) ($($before:tt)*) ($($after:tt)*)) => {
        pre_check_class!(($($todo)*) ($($processed)* $char) ($($before)*) ($($after)*) #br)
    };
    // possible beginning of char range
    (($char:ident $($todo:tt)*) ($($processed:tt)*) ($($before:tt)*) ($($after:tt)*) #br) => {
        pre_check_class!(($($todo)*) ($($processed)* $char) ($($before)*) ($($after)*) #br)
    };
    // invalid double '-'
    ((-- $($todo:tt)*) ($($processed:tt)*) ($($before:tt)*) ($($after:tt)*) #br) => {
        class_err!((- $($todo)*) ($($processed)*) ($($before)*) ($($after)*) )
    };
    // invalid '-' at the end of the class
    ((-) ($($processed:tt)*) ($($before:tt)*) ($($after:tt)*) #br) => {
        pre_check_class!((-) ($($processed)* -) ($($before)*) ($($after)*))
    };
    // invalid '-' as no char range has begun
    ((- $($todo:tt)*) ($($processed:tt)*) ($($before:tt)*) ($($after:tt)*)) => {
        class_err!((- $($todo)*) ($($processed)*) ($($before)*) ($($after)*) )
    };
    // valid '-'
    ((- $($todo:tt)*) ($($processed:tt)*) ($($before:tt)*) ($($after:tt)*) #br) => {
        pre_check_class!((- $($todo)*) ($($processed)*) ($($before)*) ($($after)*))
    };
}

macro_rules! regex {
    ($($t:tt)*) => {
        println!("Started regex");
        pre_format_regex!(($($t)*) ())
    };
}

macro_rules! parse_regex {
    ($esc:literal $($t:tt)*) => {
        println!("Start literal");
        parse_literal!($esc $($t)*)
    };
    ($esc:ident $($t:tt)*) => {
        println!("Start literal");
        parse_literal!($esc $($t)*)
    };
    (($($group:tt)*) * $($t:tt)*) => {
        println!("Repeat group *");
        parse_group!(#b $($group)* #e $($t)*)
    };
    (($($group:tt)*) + $($t:tt)*) => {
        println!("Repeat group +");
        parse_group!(#b $($group)* #e $($t)*)
    };
    (($($group:tt)*) ? $($t:tt)*) => {
        println!("Repeat group ?");
        parse_group!(#b $($group)* #e $($t)*)
    };
    (($($group:tt)*) {$repeats:literal} $($t:tt)*) => {
        println!("Repeat group {} times", $repeats);
        parse_group!(#b $($group)* #e $($t)*)
    };
    (($($group:tt)*) {$from:literal,$to:literal} $($t:tt)*) => {
        println!("Repeat group from {} to {} times", $from, $to);
        parse_group!(#b $($group)* #e $($t)*)
    };
    (#e) => {
        println!("Ended group");
        println!("Ended regex");
    };
    (#e $($t:tt)*) => {
        println!("Ended group");
        parse_regex!($($t)*)
    };
    ([$($class:tt)*] $($t:tt)*) => {
        parse_class!(#b $($class)* #e $($t)*)
    };
    ([$($class:tt)*] * $($t:tt)*) => {
        println!("Repeat class *");
        parse_class!(#b $($class)* #e $($t)*)
    };
    ([$($class:tt)*] + $($t:tt)*) => {
        println!("Repeat class +");
        parse_class!(#b $($class)* #e $($t)*)
    };
    ([$($class:tt)*] ? $($t:tt)*) => {
        println!("Repeat class ?");
        parse_class!(#b $($class)* #e $($t)*)
    };
    ([$($class:tt)*] {$repeats:literal} $($t:tt)*) => {
        println!("Repeat class {} times", $repeats);
        parse_class!(#b $($class)* #e $($t)*)
    };
    ([$($class:tt)*] {$from:literal,$to:literal} $($t:tt)*) => {
        println!("Repeat class from {} to {} times", $from, $to);
        parse_class!(#b $($class)* #e $($t)*)
    };
    ([$($class:tt)*] {$($stuff:tt)*} $($t:tt)*) => {
        range_err!($($stuff)*)
    };
    () => {

    }
}

macro_rules! parse_group {
    (#b #e) => {
        panic!("Cannot have empty group")
    };
    (#b $($t:tt)+) => {
        println!("Start group");
        parse_regex!($($t)+)
    };
}

macro_rules! parse_class {
    (#b #e) => {
        panic!("Cannot have empty char class")
    };
    (#b $($t:tt)+) => {
        println!("Start char class");
        parse_class!($($t)+)
    };
    (#b ^ $($t:tt)+) => {
        println!("Start char class inverted");
        parse_class!($($t)+)
    };
    ($id:ident $($t:tt)+) => {
        print!("char ");
        println!(stringify!($id));
        parse_class!($($t)+)
    };
    ($esc:literal $($t:tt)+) => {
        print!("escaped ");
        println!($esc);
        parse_class!($($t)+)
    };
    (#e) => {
        println!("Ended char class");
        println!("Ended regex");
    };
    (#e $($t:tt)+) => {
        println!("Ended char class");
        parse_regex!($($t)+)
    };
    (#e $($t:tt)+) => {
        print!("escaped ");
        println!($esc);
        println!("Ended char class");
        parse_regex!($($t)+)
    };
}

macro_rules! range_err {
    ({} ($($after:tt)*) ($($before:tt)*)) => {
        panic!("Parse error: {}: \n {} {{ HERE >> }} {}",
            "Range may not be empty",
            stringify!($($before)*),
            stringify!($($after)*)
        );
    };
    ({,} ($($after:tt)*) ($($before:tt)*)) => {
        panic!("Parse error: {}: \n {} {{ HERE >> ,}} {}",
            "Range must include a number before and/or after the comma",
            stringify!($($before)*),
            stringify!($($after)*)
        );
    };
    ({$($stuff:tt)*} ($($after:tt)*) ($($before:tt)*)) => {
        panic!("Parse error: {}: \n {} {{ HERE >> {} }} {}",
            "Illegal contents of range",
            stringify!($($before)*),
            stringify!($($stuff)*),
            stringify!($($after)*)
        );
    };
}

macro_rules! class_err {
    (($($todo:tt)*) ($($processed:tt)*) ($($before:tt)*) ($($after:tt)*) #br #bm) => {
        panic!("Parse error: {}: \n {} [{} HERE >> {}]  {}",
            "Unterminated character range",
            stringify!($($before)*),
            stringify!($($processed)*),
            stringify!($($todo)*),
            stringify!($($after)*)
        );
    };
    ((-) ($($processed:tt)*) ($($before:tt)*) ($($after:tt)*) #br) => {
        panic!("Parse error: {}: \n {} [{} HERE >> - {}]  {}",
            "Unterminated character range",
            stringify!($($before)*),
            stringify!($($processed)*),
            stringify!($($todo)*),
            stringify!($($after)*)
        );
    };
    ((- $($todo:tt)*) ($($processed:tt)*) ($($before:tt)*) ($($after:tt)*)) => {
        panic!("Parse error: {}: \n {} [{} HERE >> - {}]  {}",
            "Unexpected '-'",
            stringify!($($before)*),
            stringify!($($processed)*),
            stringify!($($todo)*),
            stringify!($($after)*)
        );
    };
}

macro_rules! group_err {
    () => {}
}
macro_rules! literal_err {
    () => {}
}
macro_rules! parse_err {
    () => {}
}

macro_rules! parse_literal {
    ($char:ident + $($t:tt)*) => {
        print!("repeat char + ");
        println!(stringify!($char));
        parse_literal!($($t)*)
    };
    ($esc:literal + $($t:tt)*) => {
        print!("repeat escaped + ");
        println!(stringify!($esc));
        parse_literal!($($t)*)
    };
    ($char:ident * $($t:tt)*) => {
        print!("repeat char * ");
        println!(stringify!($char));
        parse_literal!($($t)*)
    };
    ($esc:literal * $($t:tt)*) => {
        print!("repeat escaped * ");
        println!(stringify!($esc));
        parse_literal!($($t)*)
    };
    ($char:ident ? $($t:tt)*) => {
        print!("repeat char ? ");
        println!(stringify!($char));
        parse_literal!($($t)*)
    };
    ($esc:literal ? $($t:tt)*) => {
        print!("repeat escaped ? ");
        println!(stringify!($esc));
        parse_literal!($($t)*)
    };
    ($char:ident {$repeats:literal} $($t:tt)*) => {
        print!("Repeat char {} {} times", stringify!($char), $repeats);
        parse_literal!($($t)*)
    };
    ($esc:literal {$repeats:literal} $($t:tt)*) => {
        print!("Repeat escaped {} {} times", stringify!($esc), $repeats);
        parse_literal!($($t)*)
    };
    ($char:ident {$from:literal,$to:literal} $($t:tt)*) => {
        print!("Repeat char {} from {} to {} times", stringify!($char), $from, $to);
        parse_literal!($($t)*)
    };
    ($esc:literal {$from:literal,$to:literal} $($t:tt)*) => {
        print!("Repeat escaped {} from {} to {} times", stringify!($esc), $from, $to);
        parse_literal!($($t)*)
    };
    ($char:ident $($t:tt)*) => {
        println!("char {}", stringify!($char));
        parse_literal!($($t)*)
    };
    ($esc:literal $($t:tt)*) => {
        println!("escaped {}", stringify!($esc));
        parse_literal!($($t)*)
    };
    // groups
    (($($group:tt)*) * $($t:tt)*) => {
        println!("End literal");
        println!("Repeat group *");
        parse_group!(#b $($group)* #e $($t)*)
    };
    (($($group:tt)*) + $($t:tt)*) => {
        println!("End literal");
        println!("Repeat group +");
        parse_group!(#b $($group)* #e $($t)*)
    };
    (($($group:tt)*) ? $($t:tt)*) => {
        println!("End literal");
        println!("Repeat group ?");
        parse_group!(#b $($group)* #e $($t)*)
    };
    (($($group:tt)*) {$repeats:literal} $($t:tt)*) => {
        println!("End literal");
        println!("Repeat group {} times", $repeats);
        parse_group!(#b $($group)* #e $($t)*)
    };
    (($($group:tt)*) {$from:literal,$to:literal} $($t:tt)*) => {
        println!("End literal");
        println!("Repeat group from {} to {} times", $from, $to);
        parse_group!(#b $($group)* #e $($t)*)
    };
    (($($group:tt)*) {$($stuff:tt)*} $($t:tt)*) => {
        range_err!($($stuff)*)
    };
    (($($group:tt)*) $($t:tt)*) => {
        println!("End literal");
        parse_group!(#b $($group)* #e $($t)*)
    };
    // classes
    ([$($class:tt)*] * $($t:tt)*) => {
        println!("End literal");
        println!("Repeat class *");
        parse_class!(#b $($class)* #e $($t)*)
    };
    ([$($class:tt)*] + $($t:tt)*) => {
        println!("End literal");
        println!("Repeat class +");
        parse_class!(#b $($class)* #e $($t)*)
    };
    ([$($class:tt)*] ? $($t:tt)*) => {
        println!("End literal");
        println!("Repeat class ?");
        parse_class!(#b $($class)* #e $($t)*)
    };
    ([$($class:tt)*] {$repeats:literal} $($t:tt)*) => {
        println!("End literal");
        print!("Repeat class {} times", $repeats);
        parse_class!(#b $($class)* #e $($t)*)
    };
    ([$($class:tt)*] {$from:literal,$to:literal} $($t:tt)*) => {
        println!("End literal");
        print!("Repeat class from {} to {} times", $from, $to);
        parse_class!(#b $($class)* #e $($t)*)
    };
    ([$($class:tt)*] {$($stuff:tt)*} $($t:tt)*) => {
        range_err!($($stuff)*)
    };
    ([$($class:tt)*] $($t:tt)*) => {
        println!("End literal");
        parse_class!(#b $($class)* #e $($t)*)
    };
    () => {
        println!("End literal");
        println!("End regex");
    };
    ($($t:tt)+) => {
        println!("End literal ");
        parse_regex!($($t)+)
    };
}


#[test]
fn wow() {
    regex!(a b '\\' d (a b c) [a b c]);
}


const fn parse_regexp(string : &str) -> u8 {
    let mut ret = 0;
    let bytes = string.as_bytes();
    let mut i = 0;

    let mut paren_depth = 0;
    let mut bracket_depth = 0;
    let mut is_escaped = false;
    let mut in_char_class = false;

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