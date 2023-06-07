use crate::payload_engine::lexer::regex::ascii_converter::{ascii, ascii_l};
use crate::payload_engine::lexer::regex::regex_parser::Char::Sequence;
use crate::payload_engine::util::macros::{sample_macro, pair_macro, to_struct_body, to_struct, to_trait_impl};

enum Res {
    Ok,
    No
}

struct Range {
    from: u8,
    to: u8,
}

type Flag2 = u8;
trait Flag2Trait {
    fn a(self) -> bool;
    fn b(self) -> bool;
    fn a_or_b(self) -> bool;
    fn a_and_b(self) -> bool;
}
impl Flag2Trait for Flag2 {
    fn a(self) -> bool {
        return self & 0b01 != 0;
    }
    fn b(self) -> bool {
        return self & 0b10 != 0;
    }
    fn a_or_b(self) -> bool {
        return self > 0;
    }
    fn a_and_b(self) -> bool {
        return 0b11 == self;
    }
}

type CharFlags = u8;
trait CharFlagsTrait {
    const UPCASE : u8 =            0b1000_0000u8; // [A-Z]
    const DOWNCASE : u8 =          0b0100_0000u8; // [a-z]
    const DIGIT : u8 =             0b0010_0000u8; // \d or [0-9]
    const WHITESPACE : u8 =        0b0001_0000u8; // \s
    const UNCLASSIFIED : u8 =      0b0000_1000u8; // symbols or whatever else
    const LINE_END: u8 =           0b0000_0100u8; // user-defined line terminator (usually '\n' and/or ';')
    // these two are special and at the end since they don't match actual characters but rather spaces between them
    const WORD_BOUNDARY: u8 =      0b0000_0010u8; // \b
    const NON_WORD_BOUNDARY: u8 =  0b0000_0001u8; // \B

    const NON_WORD : u8  =         Self::UNCLASSIFIED | Self::WHITESPACE; // \W [^A-z0-9_]
    const NON_ALPHA : u8 =         Self::DIGIT | Self::UNCLASSIFIED | Self::WHITESPACE; // [^A-z]
    const ALPHA : u8 =             Self::UPCASE | Self::DOWNCASE; // [A-z]
    const NON_DIGIT : u8 =         Self::UPCASE | Self::DOWNCASE | Self::UNCLASSIFIED | Self::WHITESPACE; // \D or [^\d] or [^0-9]
    const WORD : u8  =             Self::UPCASE | Self::DOWNCASE | Self::DIGIT; // \w or [A-z0-9_]
    const NON_WHITESPACE : u8 =    Self::UPCASE | Self::DOWNCASE | Self::DIGIT | Self::UNCLASSIFIED; // \S
    const ANY : u8 =               Self::UPCASE | Self::DOWNCASE | Self::DIGIT | Self::UNCLASSIFIED | Self::WHITESPACE; // .
    const CLASSIFIED : u8 =        Self::UPCASE | Self::DOWNCASE | Self::DIGIT | Self::WHITESPACE;

    fn is_any(self) -> bool;
    fn is_line_end(self) -> bool;
    fn is_alpha(self) -> bool;
    fn non_alpha(self) -> bool;
    fn is_word(self) -> bool;
    fn non_word(self) -> bool;
    fn is_digit(self) -> bool;
    fn non_digit(self) -> bool;
    fn is_whitespace(self) -> bool;
    fn non_whitespace(self) -> bool;
    fn is_classified(self) -> bool;
    fn non_classified(self) -> bool;
    fn is_word_boundary(self) -> bool;
    fn non_word_boundary(self) -> bool;
}
impl CharFlagsTrait for CharFlags {
    /// Any alphanumeric, whitespace, or otherwise-unclassified ascii character other than line-end
    ///
    /// `.`
    #[inline] fn is_any(self) -> bool { self >= Self::ANY }
    /// Any user-defined line end character
    ///
    /// usually `[\n;]`, `;`, or `\n`
    #[inline] fn is_line_end(self) -> bool { self & Self::LINE_END != 0 }
    /// Any alphabet letter, case insensitive
    ///
    /// `[A-z]`
    #[inline] fn is_alpha(self) -> bool { self >= Self::ALPHA }
    /// Any non-alphabet letter, case insensitive
    ///
    /// `[^A-z]`
    #[inline] fn non_alpha(self) -> bool { self >= Self::NON_ALPHA }
    /// Any word character
    ///
    /// `[A-z0-9_]`
    #[inline] fn is_word(self) -> bool { self >= Self::WORD }
    #[inline] fn non_word(self) -> bool { self >= Self::NON_WORD }
    #[inline] fn is_digit(self) -> bool { self & Self::DIGIT != 0 }
    #[inline] fn non_digit(self) -> bool { self & Self::NON_DIGIT != 0 }
    #[inline] fn is_whitespace(self) -> bool { self & Self::WHITESPACE != 0 }
    #[inline] fn non_whitespace(self) -> bool { self & Self::NON_WHITESPACE != 0 }
    #[inline] fn is_classified(self) -> bool { self & Self::CLASSIFIED != 0 }
    #[inline] fn non_classified(self) -> bool { self & Self::UNCLASSIFIED != 0 }
    #[inline] fn is_word_boundary(self) -> bool { self == Self::WORD_BOUNDARY }
    #[inline] fn non_word_boundary(self) -> bool { self == Self::NON_WORD_BOUNDARY }
}

type Ascii = u8;
trait AsciiTrait {
    const ASCII_DIGIT_BLOCK_END : usize = 63;
    const ASCII_UPPER_BLOCK_END : usize = 95;
    const ASCII_LOWER_BLOCK_END : usize = 127;
    fn justify_as_digit(self) -> usize;
    fn justify_as_upper(self) -> usize;
    fn justify_as_lower(self) -> usize;
    fn to_upper(self) -> Ascii;
    fn to_lower(self) -> Ascii;
    fn as_flag(self) -> CharFlags;
}
impl AsciiTrait for Ascii {
    #[inline] fn justify_as_digit(self) -> usize {
        1usize << (Self::ASCII_DIGIT_BLOCK_END - self as usize)
    }
    #[inline] fn justify_as_upper(self) -> usize {
        1usize << (Self::ASCII_UPPER_BLOCK_END - self as usize)
    }
    #[inline] fn justify_as_lower(self) -> usize {
        1usize << (Self::ASCII_LOWER_BLOCK_END - self as usize)
    }
    #[inline] fn to_upper(self) -> Ascii {
        return self - 32
    }
    #[inline] fn to_lower(self) -> Ascii {
        return self + 32
    }
    fn as_flag(self) -> CharFlags {
        match self {
            b'a'..=b'z' => CharFlags::DOWNCASE,
            b'_' => CharFlags::ALPHA,
            b'A'..=b'Z' => CharFlags::UPCASE,
            b'0'..=b'9' => CharFlags::DIGIT,
            b' ' | b'\t' => CharFlags::WHITESPACE,
            _ => CharFlags::UNCLASSIFIED,
        };
        todo!()
    }
}

trait AsciiMatcher {
    fn matches(self, char: Ascii) -> bool;
}

struct AsciiDigits {
    value: u32
}
struct AsciiLower {
    value: u32
}
struct AsciiUpper {
    value: u32
}

impl AsciiMatcher for AsciiLower {
    fn matches(self, char: Ascii) -> bool {
        char.justify_as_lower() & self.value as usize != 0
    }
}
impl AsciiMatcher for AsciiUpper {
    fn matches(self, char: Ascii) -> bool {
        char.justify_as_upper() & self.value as usize != 0
    }
}
impl AsciiMatcher for AsciiDigits {
    fn matches(self, char: Ascii) -> bool {
        char.justify_as_digit() & self.value as usize != 0
    }
}

enum Char {
    // a single char
    // 'a' == Char{ 97 }
    Char(Ascii),
    // a single char repeating as defined by its range, excluding Range{1,1}
    // 'a*' == RangedChar{ 97, Range { 0, 255 } }
    RepeatedChar(Ascii, Range),
    // 3 tuples of a character & a number of times it repeats and the number of tuples
    // advantageous as it can compress up to 765 chars in a single word
    // but in reality there are few times characters are repeated more than twice, so it's not much
    // more effective than a Sequence, as something like FF00FF can be encoded as either while
    // sequences are likely faster:
    // FF00FF == Sequence{ [70, 70, 48, 48, 70, 70], 6 } == CompressedSequence { [(70, 2), (48, 2), (70, 2)], 3 }
    // one possible case is hardcoded binary literals if you needed to match that for some reason:
    // 0b0000111100001111 == CompressedSequence{ (48, 1), (98, 1), (48, 4), 3 }, CompressedSequence{ (49, 4), (48, 4), (49, 4), 3 }
    // 'aaaaabbbbccccc' == CompressedSequence{ (97, 5), (98, 4), (99, 5), 3 }
    CompressedSequence([(Ascii, u8); 3], u8),
    // a sequence of up to 6 back-to-back chars and a u8 indicating the length
    // this exists because it's free memory and we might as well encode multiple characters
    // in a single OS word
    // FF00FF == Sequence{ [70, 70, 48, 48, 70, 70], 6 }
    Sequence([Ascii; 6], u8),

    // matches any single character
    // a == Any or Z == Any
    Any,
    // matches any x characters where x is defined by the range
    // aaaa matches Any{ Range{ 1, 4 } } and ZZZZZ matches Any Range{ 5, 6 }
    RepeatedAny(Range),

    // a class made up entirely of flags like \d, \w, \s, etc.
    // and a bool indicating whether it's inverted
    FlagClass(CharFlags, bool,),
    // a class made up entirely of flags like \d, \w, \s, etc.
    // and a bool indicating whether it's inverted
    // repeating as defined by its range
    RepeatedFlagClass(CharFlags, bool, Range),
    // a class made up entirely of flags like \d, \w, \s, etc.,
    // a bool indicating whether further classes are necessary,
    // and a bool indicating whether it's inverted
    PartialFlagClass(CharFlags, Flag2),
    // a set of up to 5 possible chars,
    // a u8 indicating the length,
    // and a bool indicating whether it's inverted
    ShortClass([Ascii; 5], u8, bool),
    // a set of up to 5 possible chars,
    // a u8 indicating the length,
    // a bool indicating whether further classes are necessary,
    // and a bool indicating whether it's inverted (will be the same of all joined partial classes)
    PartialShortClass([Ascii; 5], u8, Flag2),

    // for all of the following:
    // 8 type flags,
    // 32 ascii bits,
    // a bool indicating further classes are necessary,
    // and a bool indicating whether it's inverted (will be the same of all joined partial classes)
    // these exist to match the 3 core blocks of ascii (32 - 127)
    PartialSynNumClass(CharFlags, AsciiDigits, Flag2,),
    PartialUpperClass(CharFlags,  AsciiUpper, Flag2,),
    PartialLowerClass(CharFlags,  AsciiLower, Flag2,),

    // used to append a range onto classes that can't fit an additional 2-byte range
    PartialRepeatedClass(Range),
}
impl Char {
    fn from<const size: usize>(chars: [Ascii; size]) -> ([Ascii; size], usize, Char) {
        match size {
            1 ..= 6 => {
                Sequence([chars[0], chars[1], chars[2], chars[3], chars[4], chars[5]], size)
            }
            _ => {

            }
        }
    }
}

enum GroupType {
    Char(Char),
    Seq([u8; 8]),
    Literal(String),
}

struct Group<const size: usize> {
    sequence: [GroupType; size]
}

trait Pattern {
    fn consume(self, some: u8) -> Res;
}

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
        compile_error!("Empty group not permitted")
    };
    // match invalid char class
    (([($($stuff:tt)*)] $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        // panic!("{}", stringify!((($($t)*) ($($processed)*) ($($prev_state)*))))
        compile_error!("Char classes may not contain groups")
    };
    // match invalid char class
    (([{$($stuff:tt)*}] $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        compile_error!("Char classes may not contain ranges")
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
        pre_format_regex!(($($t)*) ($($processed)* $any {$from,}) $($res)* $any {$from, 255})
    };
    (({,} $($t:tt)*) ($($processed:tt)*) $prev_state:tt $($res:tt)*) => {
        compile_error!("Range must have at least one bound")
    };
    (({} $($t:tt)*) ($($processed:tt)*) ($($prev_state:tt)*) $($res:tt)*) => {
        compile_error!("Range must not be empty")
    };
    (({$($stuff:tt)*} $($t:tt)*) ($($processed:tt)*) $prev_state:tt $($res:tt)*) => {
        compile_error!("Illegal range contents")
    };
    (([] $($t:tt)*) ($($processed:tt)*) $prev_state:tt $($res:tt)*) => {
        compile_error!("Empty char class not permitted")
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
        compile_error!("Illegal caret in char class")
    };
    ( ({$($stuff:tt)*} $($todo:tt)*) ($($processed:tt)*) $prev_state:tt ) => {
        compile_error!("Char classes may not contain ranges")
    };
    ( (($($stuff:tt)*) $($todo:tt)*) ($($processed:tt)*) $prev_state:tt ) => {
        compile_error!("Char classes may not contain groups")
    };
    ( ($from:ident - $to:ident $($todo:tt)*) ($($processed:tt)*) $prev_state:tt ) => {
        pre_check_class!(($($todo)*) ($($processed)* $from - $to) $prev_state )
    };
    ( (- $to:ident $($todo:tt)*) ($($processed:tt)*) $prev_state:tt) => {
        compile_error!("Illegal leading '-'. Did you mean to put an character before the '-'?")
    };
    ( ($from:ident - $($todo:tt)*) ($($processed:tt)*) $prev_state:tt ) => {
        compile_error!("Illegal trailing '-'. Did you mean to put an character after the '-'?")
    };
    ( (-) () $prev_state:tt $($res:tt)*) => {
        compile_error!("'-' cannot be the only content of a class")
    };
    ( ($char:ident $($todo:tt)*) ($($processed:tt)*) $prev_state:tt $($res:tt)* ) => {
        pre_check_class!(($($todo)*) ($($processed)* $char) $prev_state)
    };
    ( () ($($processed:tt)*) (($($prev_todo:tt)*) ($($prev_processed:tt)*) $prev_state:tt $($prev_res:tt)*) ) => {
        pre_format_regex!(($($prev_todo)*) ($($prev_processed)* [$($processed)*]) $prev_state $($prev_res)* [$($processed)*])
    };
}

macro_rules! body_generator {
    ( $($types:tt)+ ) => {
        fn consume(self, some: u8) -> Res {
            // todo: make this actual logic
            $(
                self. $types .consume(some)
            );+
        }
    };
}

macro_rules! regex {
    ($($regexp:tt)+) => {
        println!("Started regex");

        sample_macro!( (to_struct) (UberPattern (Pattern)) ($($regexp)+) ( (A) (B) (C) ));
        sample_macro!( (to_trait_impl) (Pattern UberPattern (Pattern) (body_generator)) ($($regexp)+) ( (A) (B) (C) ));

        pair_macro!( (to_struct_body) (UberPattern (pre_format_regex)) ($($regexp)+) ( (A) (B) (C) ));

        UberPattern {
             $( A: pre_format_regex!($regexp () ()) ),+
        }


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
        {println!("Start literal");
        parse_literal!($esc $($t)*)}
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
        {println!("Ended group");
        println!("Ended regex");}
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
        {println!("char {} {}", stringify!($char), ascii!($char));
        parse_literal!($($t)*)}
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
        {println!("End literal");
        println!("Start group");
        //panic!("{}", stringify!($($group)*));
        parse_regex!($($group)* 'e $($t)*)}
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
        {println!("End literal ");
        parse_regex!($($t)+)}
    };
}



#[test]
fn wow() {
     //println!("{}", size_of::<usize>());

    // regex!(
    //     ( [] )
    // );

}