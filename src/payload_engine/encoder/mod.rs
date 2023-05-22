use std::fs::File;
use std::io::{BufReader, ErrorKind, Read};

pub struct UnicodeReader {
    buf: BufReader<File>
}

impl UnicodeReader {
    pub fn new(loc : &str) -> UnicodeReader {

        let entry = File::open(loc)
            .expect(format!("Entry point {location} could not be read").as_str());

        return UnicodeReader {
            buf : BufReader::new(entry)
        }
    }
}

impl Read for UnicodeReader {

    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mid_encode = false;
        loop {
            match buf.read(&mut bytes) {
                Ok(0) => return Ok(0),
                Ok(n) => {
                    for i in 0..n {
                        let byte = bytes[i];
                        match byte & 0b1111_0000 {
                            0b1111_0000 => todo!(),
                            0b1110_0000 => todo!(),
                            0b1110_0000 => todo!(),
                            0b1100_0000 => todo!(),
                            ''a' | 'z'' => todo!(),
                            'A'..='Z' => todo!(),
                            '0'..'9' => todo!(),
                            // handle following bytes or single-byte utf chars
                            _ => {

                            }
                        }
                        num_bytes += 1;
                        x += bytes[i];
                    }
                }
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => panic!("{:?}", e),
            };
        }
    }
}