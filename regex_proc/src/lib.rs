use proc_macro::TokenStream;
use std::str::Chars;
use crate::ClassEntry::Range;
use patterns::patterns::Matcher;
use crate::GroupEntry::SubGroup;


enum GroupEntry<'a> {
    Single(Single, Repeat),
    Literal(Vec<char>),
    Series(Vec<Single>),
    SubGroup(&'a EntrySet<'a, GroupEntry<'a>>, Repeat),
    Class(&'a EntrySet<'a, ClassEntry>, Repeat),
    InvertedClass(&'a EntrySet<'a, ClassEntry>, Repeat),
}

enum Special {
    Any,
    Word,
    NonWord,
    Digit,
    NonDigit,
    Space,
    NonSpace,
    WordBoundary,
    NonWordBoundary,
}

enum Single {
    Special,
    Char(char)
}

enum ClassEntry {
    Range(u8, u8),
    Single(u8),
    Special(Special)
}

struct EntrySet<'a, T> {
    entries: Vec<T>,
    parent: Option<&'a mut EntrySet<'a, T>>
}
trait EntrySetTrait<'a, T> {
    fn new() -> EntrySet<'a, T>;
    fn add(&mut self, entry: T);
    fn optimize(&mut self) { }
}

impl<'a> EntrySetTrait<'a, ClassEntry> for EntrySet<'a, ClassEntry> {
    fn new() -> EntrySet<'a, ClassEntry> {
        return EntrySet {
            entries: Vec::new(),
            parent: None
        }
    }

    fn add(&mut self, entry: ClassEntry) {
        self.entries.push(entry)
    }

    fn optimize(&mut self) {
        let mut ranges = Vec::<(u8, u8)>::new();

        fn instert_into_ranges(ranges: &mut Vec<(u8, u8)>) {

        }

        for entry in &self.entries {
            match entry {
                Range(from, to) => {

                },
                Single => {

                }
                Special=> {

                }
            }
        }
    }
}
impl<'a> EntrySetTrait<'a, GroupEntry<'a>> for EntrySet<'a, GroupEntry<'a>> {
    fn new() -> EntrySet<'a, GroupEntry<'a>> {
        return EntrySet {
            entries: Vec::new(),
            parent: None
        }
    }
    fn add(&mut self, entry: GroupEntry<'a>) {
        self.entries.push(entry)
    }
}

struct Repeat {
    from: usize,
    to: usize,
}
impl Repeat {
    fn one() -> Repeat { Repeat { from : 1, to: 1 } }
    fn option() -> Repeat { Repeat { from : 0, to: 1 } }
    fn from_one() -> Repeat { Repeat { from : 1, to: 255 } }
    fn from_zero() -> Repeat { Repeat { from : 0, to: 255 } }
    fn repeat(repeats: usize) -> Repeat { Repeat { from : repeats, to: repeats } }
    fn range(from: usize, to: usize) -> Repeat { Repeat { from, to } }
}

fn err_problem(message: &str, problem: &str) -> TokenStream {
    String::from_iter(["compile_error!(\"", message, "\n\n Here >> ", problem, "\");"]).parse().unwrap()
}
fn err(message: &str) -> TokenStream {
    String::from_iter(["compile_error!(\"", message, "\");"]).parse().unwrap()
}

#[proc_macro]
pub fn regex(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let chars = input_str.chars();

    let mut stream = TokenStream::new();

    for c in chars {

        stream.extend::<TokenStream>(format!("println!(\"{{}}\", '{}');", c).parse::<TokenStream>().unwrap());
        let mut regexp = EntrySet::<GroupEntry>::new();
        let mut current_group = &mut regexp;


        match c {
            'A'..='Z' => {

            }
            '(' => {
                // let mut new_group = EntrySet::<GroupEntry>::new();
                // current_group.add(  GroupEntry::SubGroup(&new_group, Repeat::one()));
                // current_group = match current_group.entries.last().unwrap() {
                //     SubGroup(val, ..) => val,
                //     _ => panic!()
                // };
            }
            ')' => {
                match &mut current_group.parent {
                    Some( group) => {
                        current_group = group
                    },
                    None => {
                        current_group = &mut regexp
                    }
                }

            }
            'a'..='z' => {

            }
            '\\' => {

            }
            _ => {

            }
        }

    }

    stream
}