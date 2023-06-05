use std::process::Output;
use crate::payload_engine::lexer::regex::ascii_converter::{ascii, ascii_l};

const ALPHA_NUMERIC : u8  = 0b1100_0000u8;
const ALPHA : u8 =          0b1000_0000u8;
const DIGIT : u8 =          0b0100_0000u8;
const NON_DIGIT : u8 =      0b1011_0000u8;
const WHITESPACE : u8 =     0b0010_0000u8;
const NON_WHITESPACE : u8 = 0b1101_0000u8;
const UNCLASSIFIED : u8 =   0b0001_0000u8;
const ANY : u8 =            0b1111_0000u8;


#[cfg(test)]


macro_rules! pre_format_regex {
    // prev_state is empty so we're done
    (() ($($processed:tt)*) () $($res:tt)*) => {
        parse_regex!($($res)*)
        //println!("{} {}", stringify!($($res)*), test!($($res)*));
    };
    // we've reached the end of a recursive call. Time to go back to its caller
    (() ($($processed:tt)*) (($($prev_todo:tt)*) ($($prev_processed:tt)*) ($($prev_prev_state:tt)*) $($prev_result:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($prev_todo)*) ($($prev_processed)* ($($processed)*)) ($($prev_prev_state)*) $($prev_result)* ($($res)*))
    };
    ((() $($t:tt)*) $processed:tt $prev_state:tt $($res:tt)*) => {
        custom_panic!(
            (($($t)*) $processed $prev_state),
            "Empty group not permitted",
            (())
        )
    };
    // match invalid char class
    (([($($stuff:tt)*)] $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        // panic!("{}", stringify!((($($t)*) ($($processed)*) ($($prev_state)*))))
        custom_panic!(
            (($($t)*) ($($processed)*) ($($prev_state)*)),
            "Char classes may not contain groups",
            ([($($stuff)*)])
        )
    };
    // match invalid char class
    (([{$($stuff:tt)*}] $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        custom_panic!(
            (($($t)*) ($($processed)*) ($($prev_state)*)),
            "Char classes may not contain ranges",
            ([{$($stuff)*}])
        )
    };
    // start a new recursion since we found a group. Pass the current state into the new prev_state
    ((($($stuff:tt)*) $($t:tt)*) ($($processed:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($stuff)*) () (($($t)*) ($($processed)*) $($res)*))
    };
    // match single-escape char class
    (([$literal:tt] $($t:tt)*) ($($processed:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* [$literal]) $($res)* $literal)
    };
    (($any:tt {0,0} $($t:tt)*) ($($processed:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any {0,0}) $($res)*)
    };
    (($any:tt {,0} $($t:tt)*) ($($processed:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any {,0}) $($res)*)
    };
    (($any:tt {0} $($t:tt)*) ($($processed:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any {0}) $($res)*)
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
    (({,} $($t:tt)*) ($($processed:tt)*) $prev_state:tt $($res:tt)*) => {
        custom_panic!(
            (($($t)*) ($($processed)*) $prev_state),
            "Range must have at least one bound",
            (,) 'use_brace
        )
    };
    (({} $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        custom_panic!(
            (($($todo)*) ($($processed)*) $prev_state),
            "Range must not be empty",
            ({})
        )
    };
    (({$($stuff:tt)*} $($t:tt)*) ($($processed:tt)*) $prev_state:tt $($res:tt)*) => {
        custom_panic!(
            (($($t)*) ($($processed)*) $prev_state),
            "Illegal range contents",
            ($($stuff)*) 'use_brace
        )
    };
    (([] $($t:tt)*) ($($processed:tt)*) $prev_state:tt $($res:tt)*) => {
        custom_panic!(
            (($($t)*) ($($processed)*) $prev_state),
            "Empty char class not permitted",
            ([])
        )
    };
    (([$($stuff:tt)*] $($t:tt)*) ($($processed:tt)*) $prev_state:tt $($res:tt)*) => {
        pre_check_class!( ($($stuff)*) () (($($t)*) ($($processed)*) $prev_state) );
    };
    (($any:tt $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        pre_format_regex!(($($t)*) ($($processed)* $any) ($($prev_state)*) $($res)* $any)
    };
}

macro_rules! pre_check_class {
    // illegal caret in class
    ( (^ $($todo:tt)*) () $prev_state:tt ) => {
        pre_check_class!(($($todo)*) (^) $prev_state )
    };
    // illegal caret in class
    ( (^ $($todo:tt)*) ($($processed:tt)*) $prev_state:tt ) => {
        custom_panic!(
            (($($todo)*) ($($processed)*) $prev_state),
            "Illegal caret in char class",
            (^) 'use_bracket
        )
    };
    ( ({$($stuff:tt)*} $($todo:tt)*) ($($processed:tt)*) $prev_state:tt ) => {
        custom_panic!(
            (($($todo)*) ($($processed)*) $prev_state),
            "Char classes may not contain ranges",
            ({$($stuff)*}) 'use_bracket
        )
    };
    ( (($($stuff:tt)*) $($todo:tt)*) ($($processed:tt)*) $prev_state:tt ) => {
        custom_panic!(
            (($($todo)*) ($($processed)*) $prev_state),
            "Char classes may not contain groups",
            (($($stuff)*)) 'use_bracket
        )
    };
    ( ($from:ident - $to:ident $($todo:tt)*) ($($processed:tt)*) $prev_state:tt ) => {
        pre_check_class!(($($todo)*) ($($processed)* $from - $to) $prev_state )
    };
    ( (- $to:ident $($todo:tt)*) ($($processed:tt)*) $prev_state:tt) => {
        custom_panic!(
            (($($todo)*) ($($processed)*) $prev_state),
            "Illegal leading '-'. Did you mean to put an character before the '-'?",
            (- $to) 'use_bracket
        )
    };
    ( ($from:ident - $($todo:tt)*) ($($processed:tt)*) $prev_state:tt ) => {
        custom_panic!(
            (($($todo)*) ($($processed)*) $prev_state),
            "Illegal trailing '-'. Did you mean to put an character after the '-'?",
            ($from -) 'use_bracket
        )
    };
    ( (-) () $prev_state:tt $($res:tt)*) => {
        custom_panic!(
            (($($todo)*) ($($processed)*) $prev_state),
            "'-' cannot be the only content of a class",
            (-) 'use_bracket
        )
    };
    ( ($char:ident $($todo:tt)*) ($($processed:tt)*) $prev_state:tt $($res:tt)* ) => {
        pre_check_class!(($($todo)*) ($($processed)* $char) $prev_state)
    };
    ( () ($($processed:tt)*) (($($prev_todo:tt)*) ($($prev_processed:tt)*) $prev_state:tt $($prev_res:tt)*) ) => {
        pre_format_regex!(($($prev_todo)*) ($($prev_processed)* [$($processed)*]) $prev_state $($prev_res)* [$($processed)*])
    };
}

macro_rules! regex {
    ($($t:tt)*) => {
        println!("Started regex");
        pre_format_regex!(($($t)*) () ())
    };
}

macro_rules! parse_regex {
    (# $($t:tt)*) => {
        println!("Start literal");
        parse_literal!(# $($t)*)
        //parse_literal!($esc $($t)*)
    };
    (# $esc:tt $($t:tt)*) => {
        println!("Start literal");
        parse_literal!($esc $($t)*)
    };
    ($esc:ident $($t:tt)*) => {
        println!("Start literal");
        parse_literal!($esc $($t)*)
    };
    (($($group:tt)*) * $($t:tt)*) => {
        println!("Repeat group *");
        println!("Start group");
        parse_regex!($($group)* 'e $($t)*)
    };
    (($($group:tt)*) + $($t:tt)*) => {
        println!("Repeat group +");
        println!("Start group");
        parse_regex!($($group)* 'e $($t)*)
    };
    (($($group:tt)*) ? $($t:tt)*) => {
        println!("Repeat group ?");
        println!("Start group");
        parse_regex!($($group)* 'e $($t)*)
    };
    (($($group:tt)*) {$repeats:literal} $($t:tt)*) => {
        println!("Repeat group {} times", $repeats);
        println!("Start group");
        parse_regex!($($group)* 'e $($t)*)
    };
    (($($group:tt)*) {$from:literal,$to:literal} $($t:tt)*) => {
        println!("Repeat group from {} to {} times", $from, $to);
        println!("Start group");
        parse_regex!($($group)* 'e $($t)*)
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
    () => {

    }
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
        println!("Range from {} to {} {} - {}", stringify!($from), stringify!($to), ascii!($from), ascii!($to));
        println!("Ended char class");
        parse_regex!($($t)*)
    };
    ($from:ident - $to:ident $($t:tt)+) => {
        println!("Range from {} to {} {} - {}", stringify!($from), stringify!($to), ascii!($from), ascii!($to));
        parse_class!($($t)+)
    };
    ($id:ident $id_2:ident $id_3:ident $($t:tt)*) => {
        println!("batched char {} {}", stringify!($id), ascii!($id));
        println!("batched char {} {}", stringify!($id_2), ascii!($id_2));
        parse_class!($id_3 $($t)*)
    };
    ($id:ident $($t:tt)*) => {
        println!("char {} {}", stringify!($id), ascii!($id));
        parse_class!($($t)*)
    };
    (#$id:tt #$id_2:tt #$id_3:tt $($t:tt)*) => {
        println!("batched escaped {} {}", stringify!($id), ascii_l!($id));
        println!("batched escaped {} {}", stringify!($id_2), ascii_l!($id_2));
        parse_class!($id_3 $($t)*)
    };
    (#$lit:tt $($t:tt)*) => {
        println!("escaped {} {}", stringify!($lit), ascii_l!($lit));
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
}

macro_rules! range_err {
    ({} ($($after:tt)*) ($($before:tt)*)) => {
        panic!("Parse error: {}: \n {}  HERE >> {{}} {}",
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
    ($($char:ident)+ # $($t:tt)*) => {
        $(
            println!("batched char {} {}", stringify!($char), ascii!($char));
        )+
        // We can't know if the literal has a following repetition operator
        parse_literal!(# $($t)*)
    };
    ($(#$lit:tt)+ $char:ident $($t:tt)*) => {
        $(
            println!("batched escaped {} {}", stringify!(#$lit), ascii_l!(#$lit));
        )+
        // We can't know if the char has a following repetition operator
        parse_literal!($char $($t)*)
    };
    ($char_1:ident $char_2:ident $char_3:ident $char_4:ident $($t:tt)*) => {
        println!("batched char {} {}", stringify!($char_1), ascii!($char_1));
        println!("batched char {} {}", stringify!($char_2), ascii!($char_2));
        println!("batched char {} {}", stringify!($char_3), ascii!($char_3));
        // We can't know if the 4th one has a following repetition operator
        parse_literal!($char_4 $($t)*)
    };
    ($char_1:ident $char_2:ident $char_3:ident $($t:tt)*) => {
        println!("batched char {} {}", stringify!($char_1), ascii!($char_1));
        println!("batched char {} {}", stringify!($char_2), ascii!($char_2));
        // We can't know if the 3rd one has a following repetition operator
        parse_literal!($char_3 $($t)*)
    };
    (#$char_1:tt #$char_2:tt #$char_3:tt #$char_4:tt $($t:tt)*) => {
        println!("batched escaped {} {}", stringify!($char_1), ascii_l!($char_1));
        println!("batched escaped {} {}", stringify!($char_2), ascii_l!($char_2));
        println!("batched escaped {} {}", stringify!($char_3), ascii_l!($char_3));
        // We can't know if the 4th one has a following repetition operator
        parse_literal!($char_4 $($t)*)
    };
    (#$char_1:tt #$char_2:tt #$char_3:tt $($t:tt)*) => {
        println!("batched escaped {} {}", stringify!($char_1), ascii_l!($char_1));
        println!("batched escaped {} {}", stringify!($char_2), ascii_l!($char_2));
        // We can't know if the 3rd one has a following repetition operator
        parse_literal!($char_3 $($t)*)
    };
    ($char:ident + $($t:tt)*) => {
        println!("repeat char + {} {}", stringify!($char), ascii!($char));
        parse_literal!($($t)*)
    };
    (#$esc:tt + $($t:tt)*) => {
        println!("repeat escaped + {} {}", stringify!($esc), ascii_l!($esc));
        parse_literal!($($t)*)
    };
    ($char:ident * $($t:tt)*) => {
        println!("repeat char * {} {}", stringify!($char), ascii!($char));
        parse_literal!($($t)*)
    };
    (#$esc:tt * $($t:tt)*) => {
        println!("repeat escaped * {} {}", stringify!($esc), ascii_l!($esc));
        parse_literal!($($t)*)
    };
    ($char:ident ? $($t:tt)*) => {
        println!("repeat char ? {} {}", stringify!($char), ascii!($char));
        parse_literal!($($t)*)
    };
    (#$esc:tt ? $($t:tt)*) => {
        println!("repeat escaped ? {} {}", stringify!($esc), ascii_l!($esc));
        parse_literal!($($t)*)
    };
    ($char:ident {$repeats:literal} $($t:tt)*) => {
        println!("Repeat char {} ({}) {} times", stringify!($char), ascii!($char), $repeats);
        parse_literal!($($t)*)
    };
    (#$esc:tt {$repeats:literal} $($t:tt)*) => {
        println!("Repeat escaped {} ({}) {} times", stringify!($esc), ascii_l!($esc), $repeats);
        parse_literal!($($t)*)
    };
    ($char:ident {$from:literal,$to:literal} $($t:tt)*) => {
        println!("Repeat char {} ({}) from {} to {} times", stringify!($char), ascii!($char), $from, $to);
        parse_literal!($($t)*)
    };
    (#$esc:tt {$from:literal,$to:literal} $($t:tt)*) => {
        println!("Repeat escaped {} ({}) from {} to {} times", stringify!($esc), ascii_l!($esc), $from, $to);
        parse_literal!($($t)*)
    };
    ($char:ident $($t:tt)*) => {
        println!("char {} {}", stringify!($char), ascii!($char));
        parse_literal!($($t)*)
    };
    (#$esc:tt $($t:tt)*) => {
        println!("escaped {} {}", stringify!($esc), ascii_l!($esc));
        parse_literal!($($t)*)
    };
    // groups
    (($($group:tt)*) * $($t:tt)*) => {
        println!("End literal");
        println!("Repeat group *");
        println!("Start group");
        parse_regex!($($group)* 'e $($t)*)
    };
    (($($group:tt)*) + $($t:tt)*) => {
        println!("End literal");
        println!("Repeat group +");
        println!("Start group");
        parse_regex!($($group)* 'e $($t)*)
    };
    (($($group:tt)*) ? $($t:tt)*) => {
        println!("End literal");
        println!("Repeat group ?");
        println!("Start group");
        parse_regex!($($group)* 'e $($t)*)
    };
    (($($group:tt)*) {$repeats:literal} $($t:tt)*) => {
        println!("End literal");
        println!("Repeat group {} times", $repeats);
        println!("Start group");
        parse_regex!($($group)* 'e $($t)*)
    };
    (($($group:tt)*) {$from:literal,$to:literal} $($t:tt)*) => {
        println!("End literal");
        println!("Repeat group from {} to {} times", $from, $to);
        println!("Start group");
        parse_regex!($($group)* 'e $($t)*)
    };
    (($($group:tt)*) $($t:tt)*) => {
        println!("End literal");
        println!("Start group");
        //panic!("{}", stringify!($($group)*));
        parse_regex!($($group)* 'e $($t)*)
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
        println!("Repeat class {} times", $repeats);
        parse_class!('b $($class)* 'e $($t)*)
    };
    ([$($class:tt)*] {$from:literal,$to:literal} $($t:tt)*) => {
        println!("End literal");
        println!("Repeat class from {} to {} times", $from, $to);
        parse_class!('b $($class)* 'e $($t)*)
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

macro_rules! custom_panic {
    // no prev state
    ((($($todo:tt)*) ($($processed:tt)*) () $($garbage:tt)*), $message:literal, ($($problem:tt)*) $($use_bracket:tt)? ) => {
        reverse_custom_panic!( (($($processed)*)) () (($($todo)*)) $message ($($use_bracket)? $($problem)*) ( $($problem)* ) )
    };
    // one prev state
    ( (($($todo:tt)*) ($($processed:tt)*) (($($prev_todo:tt)*) ($($prev_processed:tt)*) () $($garbage:tt)*)), $message:literal, ($($problem:tt)*) $($use_bracket:tt)? ) => {
        reverse_custom_panic!( (($($prev_processed)*)) ($($processed)*) (($($prev_todo)*)) $message ($($use_bracket)? $($problem)*) ($($processed)* $($problem)* $($todo)*) )
    };
    // more than one prev state
    ( (($($todo:tt)*) ($($processed:tt)*) (($($prev_todo:tt)*) ($($prev_processed:tt)*) $prev_state:tt $($garbage:tt)*) $($garbage_2:tt)*), $message:literal, ($($problem:tt)*) $($use_bracket:tt)? ) => {
        custom_panic!( $prev_state (($($prev_processed)*)) (($($prev_todo)*)) $message ($($use_bracket)? $($problem)*) ($($processed)* $($problem)* $($todo)*) () )
    };

    // prev state
    ( (($($todo:tt)*) ($($processed:tt)*) () $($garbage:tt)*) ($($before_layers:tt)*) ($($after_layers:tt)*) $message:literal $problem:tt $constructed:tt $before_concat:tt ) => {
        reverse_custom_panic!( (($($processed)*) $($before_layers)*) $before_concat (($($todo)*) $($after_layers)*) $message $problem $constructed )
    };
    // more than one prev state
    ( (($($todo:tt)*) ($($processed:tt)*) $prev_state:tt $($garbage:tt)*) ($($before_layers:tt)*) ($($after_layers:tt)*) $message:literal $problem:tt $constructed:tt $before_concat:tt ) => {
        custom_panic!( $prev_state (($($processed)*) $($before_layers)*) (($($todo)*) $($after_layers)*) $message $problem $constructed $before_concat )
    };

    // ($($stuff:tt)*) => {
    //     panic!("{}", stringify!($($stuff)*))
    // }

}

macro_rules! reverse_custom_panic {
    ( (($($before:tt)*) $($before_layers:tt)+) ($($before_concat:tt)*) (($($after:tt)*) $($after_layers:tt)+) $message:literal ($('use_bracket $problem:tt)*) ($($constructed:tt)*) $($depth_counter:literal)* ) => {
        reverse_custom_panic!(($($before_layers)+) ($($before)* $($before_concat)*) ($($after_layers)+) $message ($($problem)*) ($($before)* [$($constructed)*] $($after)*) $($depth_counter)* 1)
    };
    ( (($($before:tt)*) $($before_layers:tt)+) ($($before_concat:tt)*) (($($after:tt)*) $($after_layers:tt)+) $message:literal ($('use_brace $problem:tt)*) ($($constructed:tt)*) $($depth_counter:literal)* ) => {
        reverse_custom_panic!(($($before_layers)+) ($($before)* $($before_concat)*) ($($after_layers)+) $message ($($problem)*) ($($before)* {$($constructed)*} $($after)*) $($depth_counter)* 1)
    };
    ( (($($before:tt)*) $($before_layers:tt)+) ($($before_concat:tt)*) (($($after:tt)*) $($after_layers:tt)+) $message:literal $problem:tt ($($constructed:tt)*) $($depth_counter:literal)* ) => {
        reverse_custom_panic!(($($before_layers)+) ($($before)* $($before_concat)*) ($($after_layers)+) $message $problem ($($before)* ($($constructed)*) $($after)*) $($depth_counter)* 1)
    };
    ( (($($before:tt)*)) ($($before_concat:tt)*) (($($after:tt)*)) $message:literal ('use_brace $($problem:tt)*) ($($constructed:tt)*) $($depth_counter:literal)* ) => {
        panic!("{}:\n{}\n{}",
            $message,
            stringify!($($before)* {$($constructed)*} $($after)*),
            " ".repeat(stringify!($($before)* $($before_concat)*).len() + 3 $(+$depth_counter)*) + &"^".repeat(stringify!($($problem)*).len())
        )
    };
    ( (($($before:tt)*)) ($($before_concat:tt)*) (($($after:tt)*)) $message:literal ('use_bracket $($problem:tt)*) ($($constructed:tt)*) $($depth_counter:literal)* ) => {
        panic!("{}:\n{}\n{}",
            $message,
            stringify!($($before)* [$($constructed)*] $($after)*),
            " ".repeat(stringify!($($before)* $($before_concat)*).len() + 1 $(+$depth_counter)*) + &"^".repeat(stringify!($($problem)*).len())
        )
    };
    ( (($($before:tt)*)) ($($before_concat:tt)*) (($($after:tt)*)) $message:literal ($($problem:tt)*) ($($constructed:tt)*) $($depth_counter:literal)* ) => {
        panic!("{}:\n{}\n{}",
            $message,
            stringify!($($before)* ($($constructed)*) $($after)*),
            " ".repeat(stringify!($($before)* $($before_concat)*).len() + 1 $(+$depth_counter)*) + &"^".repeat(stringify!($($problem)*).len())
        )
    };
}

#[test]
fn wow() {
    regex!(a []);
}