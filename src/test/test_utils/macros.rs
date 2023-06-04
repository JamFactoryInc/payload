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