use std::process::Output;
#[cfg(test)]

#[test]
fn test1() {
    let ptr: *const usize = generated_fn(1);

    let val = unsafe {
        ptr as *const UberPattern<&str, &str, &str>
    };

    unsafe {
        println!("{}", (*val).abc_var);
    }

    println!("{}", parse_regexp("[]"))
}

// an ordered list of patterns that must all complete to match


struct CharClass<const N: usize> {

}

trait Empty {
    type Output;
    fn empty() -> Output;
}

trait Pattern {

}

// match literal fixed-width tokens like keywords
struct LiteralPattern {

}

// match any of the contained patterns, whichever completes first
// struct OrPattern<const N : usize> {
//     patterns : []
// }

// does not need to be completed
struct OptionalPattern<T : Pattern> {
    pattern : T
}

struct RepeatingPattern {

}

struct DelimitedPattern {

}

impl Pattern for u32 {

}

impl Pattern for &str {

}

enum PatternVariant {
    Literal,
    Or,
    And,
    Repeating,
    Delimited,
    Optional,
}

macro_rules! generate_descending_struct {
    ($name:ident; $leading_id:ident; $leading_type:ty, $($ids:ident; $types:ty),*) => {
        paste::item! {
            pub struct [<$name _ $leading_type>] <$leading_type, $($types),*> {
                $leading_id: $leading_type,
                $(
                    $ids: $types,
                )*
            }
        }

        generate_descending_struct!{$name; $($ids; $types),*}
    };
    ($name:ident; $leading_id:ident; $leading_type:ty) => {
        paste::item! {
            pub struct [<$name _ $leading_type>]<$leading_type> {
                $leading_id: $leading_type,
            }
        }
    };
}

macro_rules! generate_descending_fn {
    // largest (denoted by # before fn)
    (#fn $name:ident<$leading_targ:ident : Pattern,$($t:ident : Pattern),*>($info_arg:ident: $info_type:ty, $leading_arg:ident: $leading_type:ty,$($arg:ident: $arg_type:ty),*)) => {
        paste::item! {
            fn [<$name _ $leading_targ>]<$leading_targ : Pattern, $($t : Pattern),*>($info_arg: $info_type, $leading_arg: $leading_type, $($arg: $arg_type),*) -> *const usize {
                println!("Generated!");
                println!("{}", std::any::type_name::<$leading_targ>());
                $(
                    println!("{}", std::any::type_name::<$t>());
                )*
                Box::into_raw(Box::new(UberPattern::<$leading_targ, $($t),*> {
                    $leading_arg,
                    $(
                        $arg,
                    )*
                })) as *const usize
            }

            generate_descending_fn!{fn $name [<$name _ $leading_targ>]<$($t : Pattern),*>($info_arg: $info_type, $($arg: $arg_type),*)}
        }

    };
    (fn $name:ident $super_name:ident<$leading_targ:ident : Pattern,$($t:ident : Pattern),*>($info_arg:ident: $info_type:ty, $leading_arg:ident: $leading_type:ty,$($arg:ident: $arg_type:ty),*)) => {
        paste::item! {
            fn [<$name _ $leading_targ>]<$leading_targ : Pattern, $($t : Pattern),*>($info_arg: $info_type, $leading_arg: $leading_type, $($arg: $arg_type),*) -> *const usize {
                if $info_arg == 0 {
                    println!("Was 0");
                    $super_name($info_arg, $leading_arg, $($arg),*, 0u32)
                } else {
                    println!("Was 1");
                    $super_name($info_arg, $leading_arg, $($arg),*, "wow")
                }
            }

            generate_descending_fn!{fn $name [<$name _ $leading_targ>]<$($t : Pattern),*>($info_arg: $info_type, $($arg: $arg_type),*)}
        }

    };
    // root layer
    (fn $name:ident $super_name:ident<$leading_targ:ident : Pattern>($info_arg:ident: $info_type:ty, $leading_arg:ident: $leading_type:ty)) => {
        paste::item! {
            fn [<$name _ $leading_targ>]<$leading_targ : Pattern>($info_arg: $info_type, $leading_arg: $leading_type) -> *const usize {
                if $info_arg == 0 {
                    println!("Was 0");
                    $super_name($info_arg, $leading_arg, 0u32)
                } else {
                    println!("Was 1");
                    $super_name($info_arg, $leading_arg, "wow")
                }
            }
        }

        fn $name($info_arg: $info_type) -> *const usize {
            paste::item! {
                if $info_arg == 0 {
                    println!("Was 0");
                    [<$name _ $leading_targ>]($info_arg, 0u32)
                } else {
                    println!("Was 1");
                    [<$name _ $leading_targ>]($info_arg, "wow")
                }
            }
        }
    }
}

macro_rules! generate_descending_from_ids {
    ($($types:ident; $vars:ident),*) => {

        pub struct UberPattern <
        $(
            $types
        ),*> {
            $(
                $vars: $types,
            )*
        }

        generate_descending_fn! {
            #fn generated_fn<
            $(
                $types: Pattern
            ),*>(info: u32, $(
                $vars: $types
            ),*)
        }
    };
}

macro_rules! pair_args {
    ($($ids:ident),*) => {
        paste::item! {
            generate_descending_from_ids! {
                $(
                    $ids; [<$ids _var>]
                ),*
            }

            generate_descending_struct! {
                Group;
                $(
                    [<$ids _var>]; $ids
                ),*
            }
        }
    }
}

#[repr(u8)]
enum CharFlags {
    Alphanum =      0b1100_0000,
    Alpha =         0b1000_0000,
    Digit =         0b0100_0000,
    NonDigit =      0b1011_0000,
    Whitespace =    0b0010_0000,
    NonWhitespace = 0b1101_0000,
    Unclassified =  0b0001_0000,
    Any =           0b1111_0000,
}

macro_rules! ascii {
    (A) => { 65 };
    (B) => { 66 };
    (C) => { 67 };
    (D) => { 68 };
    (E) => { 69 };
    (F) => { 70 };
    (G) => { 71 };
    (H) => { 72 };
    (I) => { 73 };
    (J) => { 74 };
    (K) => { 75 };
    (L) => { 76 };
    (M) => { 77 };
    (N) => { 78 };
    (O) => { 79 };
    (P) => { 80 };
    (Q) => { 81 };
    (R) => { 82 };
    (S) => { 83 };
    (T) => { 84 };
    (U) => { 85 };
    (V) => { 86 };
    (W) => { 87 };
    (X) => { 88 };
    (Y) => { 89 };
    (Z) => { 90 };
    (a) => { 97 };
    (b) => { 98 };
    (c) => { 99 };
    (d) => { 100 };
    (e) => { 101 };
    (f) => { 102 };
    (g) => { 103 };
    (h) => { 104 };
    (i) => { 105 };
    (j) => { 106 };
    (k) => { 107 };
    (l) => { 108 };
    (m) => { 109 };
    (n) => { 110 };
    (o) => { 111 };
    (p) => { 112 };
    (q) => { 113 };
    (r) => { 114 };
    (s) => { 115 };
    (t) => { 116 };
    (u) => { 117 };
    (v) => { 118 };
    (w) => { 119 };
    (x) => { 120 };
    (y) => { 121 };
    (z) => { 122 };
    (0) => { 48 };
    (1) => { 48 };
    (2) => { 48 };
    (3) => { 48 };
    (4) => { 48 };
    (5) => { 48 };
    (6) => { 48 };
    (7) => { 48 };
    (8) => { 48 };
    (9) => { 48 };
    (_) => { 255 };
    ('d') => { 254 };
    ('w') => { 253 };
    ('t') => { 9 };
    ('n') => { 10 };
    ('[') => { 123 };
    (']') => { 125 };
    ('{') => { 91 };
    ('}') => { 93 };
    ('(') => { 40 };
    (')') => { 41 };
    (' ') => { 41 };
    ('s') => { 41 };
    ('S') => { 41 };
    ('_') => { 95 +  };
}

pair_args!{abc, def, fgh}
generate_descending_struct!{ Group_1; id; usize}

struct Abc<const N: usize> {
    a: [u8; N]
}

#[test]
fn wow()  {
    //println!(parse_regex!((a b c d)))
    Abc {
        a: [1u8]
    };
    for c in b'A' ..= b'Z' {
        println!("({}) => {{ {} }};", c as char, c)
    }

}

const fn parse_regexp(string : &str) -> u8 {
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

    return 0;
}