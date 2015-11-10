// use std::str::{Chars};
use std::io::{BufRead, BufReader, Read, Result};

pub struct Chars<R> {
    inner: R,
}

pub trait CharRead : Sized {
    fn read_char(&mut self, chr: &mut char) -> Result<usize>;
    fn chars(self) -> Chars<Self>;
}

pub struct CharReader<R> {
    stream: BufReader<R>,
    line: String,
}

impl<R: Read> CharReader<R> {
    pub fn new(inner: R) -> CharReader<R> {
        CharReader {
            stream: BufReader::new(inner),
            line: "".to_string(),
        }
    }

// /// This methods checks if it starts with the given string slice
// /// it will pull the next line if the current is too short
// pub fn starts_with(&mut self, start: &str) -> Result<bool> {
//    if start.len() > self.line.len() {
//        let mut new_line = "".to_string();
//        if try!(self.stream.read_line(&mut new_line)) == 0 {
//            return Ok(false);
//        }
//        self.line.push_str(&new_line);
//    }
//    Ok(self.line.starts_with(start))
// }
}

impl<R: Read> CharRead for CharReader<R> {
    fn read_char(&mut self, chr: &mut char) -> Result<usize> {
        if self.line.is_empty() {
            if try!(self.stream.read_line(&mut self.line)) == 0 {
                return Ok(0);
            }
        }
        *chr = self.line.remove(0);
        Ok(1)
    }

    fn chars(self) -> Chars<Self> {
        Chars { inner: self }
    }
}

impl<R: CharRead> Iterator for Chars<R> {
    type Item = Result<char>;
    fn next(&mut self) -> Option<Result<char>> {
        let mut c = '\x00';
        match self.inner.read_char(&mut c) {
            Ok(0) => None,
            Ok(_) => Some(Ok(c)),
            Err(e) => Some(Err(e)),
        }
    }
}
