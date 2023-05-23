use std::fs::File;
use std::future::Future;
use std::io::{BufReader, ErrorKind, Read, Result};
use std::pin::Pin;
use std::task::{Context, Poll};
use crate::payload_engine::lexer::regex::CharCategoryFlag::*;
use crate::payload_engine::lexer::token::Token;

pub struct Lexer<'a> {
    preloaded_buf:  &'a mut [u8; <Lexer as Sized>::SIZE],
    current_buf:  &'a mut [u8; <Lexer as Sized>::SIZE],
    reader: BufReader<File>,
    preloaded : Option<&'a dyn Future<Output=usize>>,
    preload_result : usize,
    init_result : Result<usize>,
}

struct  PreloadFuture {
    preloaded : bool
}

impl<'a> Future for PreloadFuture {
    type Output = bool;

    fn poll(self: Pin<&mut PreloadFuture>, _: &mut Context<'_>) -> Poll<bool> {
        if self.preloaded {
            Poll::Ready(true)
        } else {
            Poll::Pending
        }
    }
}

trait Sized {
    const SIZE : usize;
}

impl<'a> Sized for Lexer<'a> {
    const SIZE : usize = 512;
}

impl<'a> Lexer<'a> {
    pub fn new(loc : &str) -> Lexer<'a> {

        let entry = File::open(loc)
            .expect(format!("Entry point {loc} could not be read").as_str());

        let mut reader = BufReader::new(entry);

        let current_buf = &mut [0u8; <Lexer as Sized>::SIZE];

        Lexer {
            preloaded_buf : &mut [0u8; <Lexer as Sized>::SIZE],
            init_result : reader.read(current_buf),
            preload_result : 0,
            current_buf,
            reader,
            preloaded : None,
        }
    }

    fn preload(&mut self) -> usize {
        match self.reader.read(self.preloaded_buf) {
            Ok(n) => n,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => 0,
            Err(e) => panic!("{:?}", e),
        }
    }

    fn handle_reader(&mut self) {
        match &self.preloaded {
            Some(x) => {
                self.preload_result = x.;
                self.preloaded
            },
            None => self.preloaded = Some(&self.preload()),
        }
    }

    fn swap_buf(&mut self) {
        std::mem::swap(self.preloaded_buf, self.current_buf);
    }

}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {

        let mut raw_utf : Vec<u8> = vec![];

        // start preload if we've exhausted our current buf
        self.handle_reader();

        // iterate over bytes
        for mut buf_index in 0 .. <Lexer as Sized>::SIZE {

            let byte = self.current_buf[buf_index];

            match byte & 0b1111_1000 {
                0b1111_0000 => {
                    let bytes_left = <Lexer as Sized>::SIZE - buf_index;
                    if bytes_left < 3 {
                        for i in buf_index .. <Lexer as Sized>::SIZE {
                            raw_utf.push(self.current_buf[i])
                        }
                        self.swap_buf();
                        self.preload();
                        buf_index = 0;
                        continue;
                    }
                },
                0b1110_0000 => {
                    let bytes_left = <Lexer as Sized>::SIZE - buf_index;
                    if bytes_left < 2 {
                        for i in buf_index .. <Lexer as Sized>::SIZE {
                            raw_utf.push(self.current_buf[i])
                        }
                        self.swap_buf();
                        self.preload();
                        buf_index = 0;
                    }
                },
                0b1100_0000 => {
                    let bytes_left = <Lexer as Sized>::SIZE - buf_index;
                    if bytes_left < 1 {
                        for i in buf_index .. <Lexer as Sized>::SIZE {
                            raw_utf.push(self.current_buf[i])
                        }
                        self.swap_buf();
                        self.preload();
                        buf_index = 0;
                    }
                },
                _ => {
                    if <Lexer as Sized>::SIZE == buf_index {
                        self.swap_buf();
                        self.preload();
                        buf_index = 0;
                    }
                },
            };
        }

        // if we are currently encoding a unicode char
        let mid_encode = false;

        let mut char_cat_flags : u32 = 0;

            match self.reader.read(bytes) {
                Ok(0) => return Ok(0),
                Ok(n) => {
                    for i in 0..n {
                        let byte = bytes[i];
                        match byte & 0b1111_1000 {
                            // expect 3 more bytes
                            0b1111_0000 => todo!(),
                            // expect 2 more bytes
                            0b1110_0000 => todo!(),
                            // expect 1 more byte
                            0b1100_0000 => todo!(),
                            b'a'..=b'z' => {
                                char_cat_flags = Word | DownCase
                            }
                            b'A'..=b'Z' => {
                                char_cat_flags = Word | UpCase
                            }
                            b'0'..=b'9' => {
                                char_cat_flags = Word | Digit
                            }
                            b'[' => {
                                char_cat_flags = RegexReserved | ClassEscBeg
                                    | Bracket | Open
                            }
                            b']' => {
                                char_cat_flags = RegexReserved
                                    | ClassEscMid | Bracket
                            }
                            b'{' => {
                                char_cat_flags = RegexReserved | Brace | Open
                            }
                            b'}' => {
                                char_cat_flags = RegexReserved | Brace
                            }
                            b'(' => {
                                char_cat_flags = RegexReserved | Paren | Open
                            }
                            b')' => {
                                char_cat_flags = RegexReserved | Paren
                            }
                            b'<' => {
                                char_cat_flags = LtGt | Open
                            }
                            b'>' => {
                                char_cat_flags = LtGt as u32
                            }
                            b'\\' => {
                                char_cat_flags = RegexReserved | ClassEscAny
                                    | BackSlash
                            }
                            b' ' | b'\t' => {
                                char_cat_flags = WhiteSpace as u32
                            }
                            b'\n' | b'\r' => {
                                // we don't need a unique newline identifier since this is the only
                                // non-wildcard
                                // We'll discard multiple sequential newlines so it's fine to
                                // treat /n and /r the same
                                char_cat_flags = WhiteSpace | NotWildcard
                            }
                            b'!' => {
                                char_cat_flags = Bang as u32
                            }
                            b'-' => {
                                // this one's weird in char classes because it should match
                                // the literal unless it has another literal on both sides
                                // We can check this with ClassEscMid
                                char_cat_flags = ClassEscMid | Neg
                            }
                            b'+' => {
                                char_cat_flags = RegexReserved | Pos
                            }
                            b'^' => {
                                char_cat_flags = RegexReserved | ClassEscBeg | Caret
                            }
                            b'*' => {
                                char_cat_flags = RegexReserved | Star
                            }
                            b'.' => {
                                char_cat_flags = RegexReserved | Dot
                            }
                            b'?' => {
                                char_cat_flags = RegexReserved | QMark
                            }
                            b':' => {
                                char_cat_flags = RegexReserved | Colon
                            }
                            b'$' => {
                                char_cat_flags = RegexReserved | DSign
                            }
                            b',' => {
                                char_cat_flags = Comma as u32
                            }
                            b'|' => {
                                char_cat_flags = Or as u32
                            }
                            b'&' => {
                                char_cat_flags = And as u32
                            }
                            b'"' => {
                                char_cat_flags = DoubleQuote as u32
                            }
                            b'\'' => {
                                char_cat_flags = SingleQuote as u32
                            }
                            // handle following bytes or uncategorized single-byte ascii chars
                            _ => {

                            }
                    }
                }
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => panic!("{:?}", e),
            };
        }
    }
}