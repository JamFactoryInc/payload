use std::fs::File;
use std::future::Future;
use std::io::{BufReader, Read, Result};
use std::pin::Pin;
use futures::executor::block_on;
use crate::payload_engine::lexer::regex::CharCategoryFlag::*;
use crate::payload_engine::lexer::token::Token;

type PreloadInfo<'a> = (
    Result<usize>,
    &'a mut BufReader<File>,
    [u8; <Lexer as Sized>::SIZE]
);

type PreloadFuture<'a> = Pin<Box<dyn Future<Output = PreloadInfo<'a>> + 'a>>;

pub struct Lexer<'a> {
    current_buf: [u8; <Lexer as Sized>::SIZE],
    preload_future : PreloadFuture<'a>,
    current_buf_bytes : usize,
    consumed_bytes : usize,
    char_index : usize,
}

trait Sized {
    const SIZE : usize;
}

impl<'a> Sized for Lexer<'a> {
    const SIZE : usize = 512;
}

impl<'a> Lexer<'a> {
    pub fn new(reader: &'a mut BufReader<File>,
               mut buf_a : [u8; <Lexer as Sized>::SIZE]) -> Lexer<'a> {

        Lexer {
            current_buf_bytes : reader.read(&mut buf_a).ok().expect("Failed"),
            current_buf : buf_a,
            preload_future : Box::pin(Self::preload_async(reader)),
            consumed_bytes : 0,
            char_index : 0,
        }
    }

    async fn preload_async(
        reader : &'a mut BufReader<File>
    ) -> PreloadInfo<'a> {
        let mut buf = [0; <Lexer as Sized>::SIZE];
        (reader.read(&mut buf), reader, buf)
    }

    // takes in a future, waits for it, swaps the buffers,
    // and returns a new future of the next preload
    pub fn wait_for_buf_and_swap(
        future_res : PreloadInfo<'a>,
    ) -> (usize, [u8; <Lexer as Sized>::SIZE], PreloadFuture<'a>)  {
        (
            match future_res.0 {
                Ok(0) => 0,
                Ok(n) => n,
                _ => panic!()
            },
            future_res.2,
            Box::pin(Self::preload_async(future_res.1))
        )
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&'_ mut self) -> Option<Self::Item> {

        let mut raw_utf : Vec<u8> = vec![];

        // iterate over bytes starting at last-consumed byte
        let mut buf_index = self.consumed_bytes;
        let mut bytes_left = self.current_buf_bytes - buf_index;

        'byte_loop: loop {
            let byte = self.current_buf[buf_index];

            'complete_utf : {
                let needed_bytes = match byte & 0b1111_1000 {
                    // handle utf
                    0b1111_0000 => 3,
                    0b1110_0000 => 2,
                    0b1100_0000 => 1,
                    // non-utf
                    _ => {
                        raw_utf.push(byte);
                        break 'complete_utf;
                    }
                };

                if bytes_left >= needed_bytes {
                    // finish utf
                    for buf_index in buf_index .. buf_index + needed_bytes {
                        raw_utf.push(self.current_buf[buf_index])
                    }

                    break 'complete_utf
                }

                // finish current buf
                for i in buf_index .. self.current_buf_bytes {
                    raw_utf.push(self.current_buf[i])
                }
                // swap and iterate over next buf until utf char is exhausted
                let (new_buf_bytes, new_buf, self_future) = Lexer::wait_for_buf_and_swap(
                    block_on(self.preload_future.as_mut())
                );
                self.current_buf_bytes = new_buf_bytes;
                self.current_buf = new_buf;
                self.preload_future = self_future;

                for i in 0 .. bytes_left {
                    raw_utf.push(self.current_buf[i])
                }
                buf_index = bytes_left;
            }

            let char_cat_flags;
            match byte {
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

            // decrement this first since the below check resets both counters
            bytes_left -= 1;

            if bytes_left == 0 {
                let (new_buf_bytes, new_buf, self_future) = Lexer::wait_for_buf_and_swap(
                    block_on(self.preload_future.as_mut())
                );
                self.current_buf_bytes = new_buf_bytes;
                self.current_buf = new_buf;
                self.preload_future = self_future;
                // bytes_left = Lexer::swap_buf(&mut self.preloaded,
                //                                          self.current_buf,
                //                                          self.preloaded_buf
                // );
                // self.preload();
                buf_index = 0;
                self.current_buf_bytes = bytes_left;
                self.char_index += 1;
                continue 'byte_loop
            }

            buf_index += 1;
            break;
        }

        // Token {
        //     text: raw_utf,
        //     token_type: 0,
        //     start: self.char_index,
        // }

        todo!()
    }
}