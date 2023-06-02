use std::process::Output;


#[cfg(test)]

macro_rules! pre_format_regex {
    // prev_state is empty so we're done
    (() ($($processed:tt)*) () $($res:tt)*) => {
        parse_regex!($($res)*)
        //println!("{}", stringify!($($res)*));
    };
    // we've reached the end of a recursive call. Time to go back to its caller
    (() ($($processed:tt)*) (($($prev_todo:tt)*) ($($prev_processed:tt)*) ($($prev_prev_state:tt)*) $($prev_result:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($prev_todo)*) ($($prev_processed)* ($($processed)*)) ($($prev_prev_state)*) $($prev_result)* ($($res)*))
    };
    // start a new recursion since we found a group. Pass the current state into the new prev_state
    ((($($stuff:tt)*) $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($stuff)*) () (($($t)*) ($($processed)*) ($($prev_state)*) $($res)*))
    };
    // match single-letter char class
    (([$literal:ident] $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* [$literal]) ($($prev_state)*) $($res)* $literal)
    };
    // match single-escape char class
    (([$literal:literal] $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* [$literal]) ($($prev_state)*) $($res)* $literal)
    };
    (($any:tt {0,0} $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any:tt {0,0}) ($($prev_state)*) $($res)*)
    };
    (($any:tt {,0} $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any:tt {,0}) ($($prev_state)*) $($res)*)
    };
    (($any:tt {0} $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any:tt {0}) ($($prev_state)*) $($res)*)
    };
    (($any:tt {0,1} $($t:tt)*) ($($processed:tt)* ($($prev_state:tt)*)) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any {0,1}) ($($prev_state)*) $($res)* $any ?)
    };
    (($any:tt {1,} $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any {1,}) ($($prev_state)*) $($res)* $any +)
    };
    (($any:tt {0,} $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any {0,}) ($($prev_state)*) $($res)* $any *)
    };
    (($any:tt {1} $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any {1}) ($($prev_state)*) $($res)* $any)
    };
    (($any:tt {$repeats:literal} $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any {$repeats}) ($($prev_state)*) $($res)* $any {$repeats})
    };
    (($any:tt {,$to:literal} $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any {,$to}) ($($prev_state)*) $($res)* $any {0, $to})
    };
    (($any:tt {$from:literal,} $($t:tt)*) ($($processed:tt)* ) ($($prev_state:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any {$from,}) ($($prev_state)*) $($res)* $any {$from, 9999})
    };
    (($any:tt {,} $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        range_err!({,} ($($t)*) ($($processed)* $any))
    };
    (($any:tt {} $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        range_err!({} ($($t)*) ($($processed)* $any))
    };
    (($any:tt {$($stuff:tt)*} $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        range_err!({$($stuff)*} ($($t)*) ($($processed)* $any))
    };
    (([] $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        panic!("{}: \n {} HERE >> [] {}",
            "Empty char class not permitted",
            stringify!($($processed)*),
            stringify!($($t)*)
        );
    };
    (([^] $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        panic!("{}: \n {} HERE >> [^] {}",
            "Empty char class not permitted",
            stringify!($($processed)*),
            stringify!($($t)*)
        );
    };
    (([^$($stuff:tt)*] $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        pre_check_class!(($($stuff)*) () ($($processed)* ^) ($($t)*));
        pre_format_regex!(($($t)*) ($($processed)* [^$($stuff)*]) ($($prev_state)*) $($res)* [^$($stuff)*])
    };
    (([$($stuff:tt)*] $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        pre_check_class!(($($stuff)*) () ($($processed)*) ($($t)*));
        pre_format_regex!(($($t)*) ($($processed)* [$($stuff)*]) ($($prev_state)*) $($res)* [$($stuff)*])
    };
    (($any:tt $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any) ($($prev_state)*) $($res)* $any)
    };
}

macro_rules! pre_check_class {
    (($from:ident - $to:ident $($todo:tt)*) ($($processed:tt)*) ($($before:tt)*) ($($after:tt)*)) => {
        pre_check_class!(($($todo)*) ($($processed)* $from - $to) ($($before)*) ($($after)*))
    };
    ((- $to:ident $($todo:tt)*) ($($processed:tt)*) ($($before:tt)*) ($($after:tt)*)) => {
        panic!("{}: \n {} [{} HERE >> - {}] {}",
            "Illegal leading '-'. Did you mean to put an character before the '-'?",
            stringify!($($before)*),
            stringify!($($processed)*),
            stringify!($to $($todo)*),
            stringify!($($after)*)
        );
    };
    (($from:ident - $($todo:tt)*) ($($processed:tt)*) ($($before:tt)*) ($($after:tt)*)) => {
        panic!("{}: \n {} [{} {} - HERE >>   ] {}",
            "Illegal trailing '-'. Did you mean to put an character after the '-'?",
            stringify!($($before)*),
            stringify!($($processed)*),
            stringify!($from),
            stringify!($($after)*)
        );
    };
    ((-) () ($($before:tt)*) ($($after:tt)*)) => {
        panic!("{}: \n {} [ HERE >> - ] {}",
            "'-' cannot be the only content of a class",
            stringify!($($before)*),
            stringify!($($after)*)
        );
    };
    (($char:ident $($todo:tt)*) ($($processed:tt)*) ($($before:tt)*) ($($after:tt)*)) => {
        pre_check_class!(($($todo)*) ($($processed)* $char) ($($before)*) ($($after)*))
    };
    (() $($stuff:tt)*) => {

    };
}

macro_rules! regex {
    ($($t:tt)*) => {
        println!("Started regex");
        pre_format_regex!(($($t)*) () ())
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
        parse_group!('b $($group)* 'e $($t)*)
    };
    (($($group:tt)*) + $($t:tt)*) => {
        println!("Repeat group +");
        parse_group!('b $($group)* 'e $($t)*)
    };
    (($($group:tt)*) ? $($t:tt)*) => {
        println!("Repeat group ?");
        parse_group!('b $($group)* 'e $($t)*)
    };
    (($($group:tt)*) {$repeats:literal} $($t:tt)*) => {
        println!("Repeat group {} times", $repeats);
        parse_group!('b $($group)* 'e $($t)*)
    };
    (($($group:tt)*) {$from:literal,$to:literal} $($t:tt)*) => {
        println!("Repeat group from {} to {} times", $from, $to);
        parse_group!('b $($group)* 'e $($t)*)
    };
    ('e) => {
        println!("Ended group");
        println!("Ended regex");
    };
    ('e $($t:tt)*) => {
        println!("Ended group");
        parse_regex!($($t)*)
    };
    ([$($class:tt)*] $($t:tt)*) => {
        parse_class!('b $($class)* 'e $($t)*)
    };
    ([$($class:tt)*] * $($t:tt)*) => {
        println!("Repeat class *");
        parse_class!('b $($class)* 'e $($t)*)
    };
    ([$($class:tt)*] + $($t:tt)*) => {
        println!("Repeat class +");
        parse_class!('b $($class)* 'e $($t)*)
    };
    ([$($class:tt)*] ? $($t:tt)*) => {
        println!("Repeat class ?");
        parse_class!('b $($class)* 'e $($t)*)
    };
    ([$($class:tt)*] {$repeats:literal} $($t:tt)*) => {
        println!("Repeat class {} times", $repeats);
        parse_class!('b $($class)* 'e $($t)*)
    };
    ([$($class:tt)*] {$from:literal,$to:literal} $($t:tt)*) => {
        println!("Repeat class from {} to {} times", $from, $to);
        parse_class!('b $($class)* 'e $($t)*)
    };
    ([$($class:tt)*] {$($stuff:tt)*} $($t:tt)*) => {
        range_err!($($stuff)*)
    };
    () => {

    }
}

macro_rules! parse_group {
    ('b 'e) => {
        panic!("Cannot have empty group")
    };
    ('b $($t:tt)+) => {
        println!("Start group");
        parse_regex!($($t)+)
    };
}

macro_rules! parse_class {
    ('b 'e) => {
        panic!("Cannot have empty char class")
    };
    ('b $($t:tt)+) => {
        println!("Start char class");
        parse_class!($($t)+)
    };
    ('b ^ $($t:tt)+) => {
        println!("Start char class inverted");
        parse_class!($($t)+)
    };
    ($from:ident - $to:ident 'e $($t:tt)*) => {
        println!("Range from {} to {}", stringify!($from), stringify!($to));
        println!("Ended char class");
        parse_regex!($($t)*)
    };
    ($from:ident - $to:ident $($t:tt)+) => {
        println!("Range from {} to {}", stringify!($from), stringify!($to));
        parse_class!($($t)+)
    };
    ($id:ident $($t:tt)*) => {
        print!("batched char ");
        println!(stringify!($id));
        parse_class!($($t)*)
    };
    ($lit:literal $($t:tt)*) => {
        print!("batched char ");
        println!(stringify!($lit));
        parse_class!($($t)*)
    };
    ('e) => {
        println!("Ended char class");
        println!("Ended regex");
    };
    ('e $($t:tt)*) => {
        println!("Ended char class");
        parse_regex!($($t)*)
    };
    ('e $($t:tt)+) => {
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
    ($($char:ident)+ $lit:literal $($t:tt)*) => {
        $(
            print!("batched char ");
            println!(stringify!($char));
        )+
        // We can't know if the literal has a following repetition operator
        parse_literal!($lit $($t)*)
    };
    ($($lit:literal)+ $char:ident $($t:tt)*) => {
        $(
            print!("batched escaped ");
            println!(stringify!($lit));
        )+
        // We can't know if the char has a following repetition operator
        parse_literal!($char $($t)*)
    };
    ($char_1:ident $char_2:ident $char_3:ident $char_4:ident $($t:tt)*) => {
        print!("batched char ");
        println!(stringify!($char_1));
        print!("batched char ");
        println!(stringify!($char_2));
        print!("batched char ");
        println!(stringify!($char_3));
        // We can't know if the 4th one has a following repetition operator
        parse_literal!($char_4 $($t)*)
    };
    ($char_1:ident $char_2:ident $char_3:ident $($t:tt)*) => {
        print!("batched char ");
        println!(stringify!($char_1));
        print!("batched char ");
        println!(stringify!($char_2));
        // We can't know if the 3rd one has a following repetition operator
        parse_literal!($char_3 $($t)*)
    };
    ($char_1:literal $char_2:literal $char_3:literal $char_4:literal $($t:tt)*) => {
        print!("batched escaped ");
        println!(stringify!($char_1));
        print!("batched escaped ");
        println!(stringify!($char_2));
        print!("batched escaped ");
        println!(stringify!($char_3));
        // We can't know if the 4th one has a following repetition operator
        parse_literal!($char_4 $($t)*)
    };
    ($char_1:literal $char_2:literal $char_3:literal $($t:tt)*) => {
        print!("batched escaped ");
        println!(stringify!($char_1));
        print!("batched escaped ");
        println!(stringify!($char_2));
        // We can't know if the 3rd one has a following repetition operator
        parse_literal!($char_3 $($t)*)
    };
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
        parse_group!('b $($group)* 'e $($t)*)
    };
    (($($group:tt)*) + $($t:tt)*) => {
        println!("End literal");
        println!("Repeat group +");
        parse_group!('b $($group)* 'e $($t)*)
    };
    (($($group:tt)*) ? $($t:tt)*) => {
        println!("End literal");
        println!("Repeat group ?");
        parse_group!('b $($group)* 'e $($t)*)
    };
    (($($group:tt)*) {$repeats:literal} $($t:tt)*) => {
        println!("End literal");
        println!("Repeat group {} times", $repeats);
        parse_group!('b $($group)* 'e $($t)*)
    };
    (($($group:tt)*) {$from:literal,$to:literal} $($t:tt)*) => {
        println!("End literal");
        println!("Repeat group from {} to {} times", $from, $to);
        parse_group!('b $($group)* 'e $($t)*)
    };
    (($($group:tt)*) {$($stuff:tt)*} $($t:tt)*) => {
        range_err!($($stuff)*)
    };
    (($($group:tt)*) $($t:tt)*) => {
        println!("End literal");
        parse_group!('b $($group)* 'e $($t)*)
    };
    // classes
    ([$($class:tt)*] * $($t:tt)*) => {
        println!("End literal");
        println!("Repeat class *");
        parse_class!('b $($class)* 'e $($t)*)
    };
    ([$($class:tt)*] + $($t:tt)*) => {
        println!("End literal");
        println!("Repeat class +");
        parse_class!('b $($class)* 'e $($t)*)
    };
    ([$($class:tt)*] ? $($t:tt)*) => {
        println!("End literal");
        println!("Repeat class ?");
        parse_class!('b $($class)* 'e $($t)*)
    };
    ([$($class:tt)*] {$repeats:literal} $($t:tt)*) => {
        println!("End literal");
        print!("Repeat class {} times", $repeats);
        parse_class!('b $($class)* 'e $($t)*)
    };
    ([$($class:tt)*] {$from:literal,$to:literal} $($t:tt)*) => {
        println!("End literal");
        print!("Repeat class from {} to {} times", $from, $to);
        parse_class!('b $($class)* 'e $($t)*)
    };
    ([$($class:tt)*] {$($stuff:tt)*} $($t:tt)*) => {
        range_err!($($stuff)*)
    };
    ([$($class:tt)*] $($t:tt)*) => {
        println!("End literal");
        parse_class!('b $($class)* 'e $($t)*)
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
    regex!(a b '\\' d (a b c [a b]?) a b c ([b-c a] a 'a') );
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