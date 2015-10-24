use std::io;
use std::io::{Read};
use std::result;
use std::iter::{Fuse, Peekable};
use std::num;
use unicode_xid::UnicodeXID;
use ::token;
use ::token::keywords;
use ::charread::{Chars, CharRead, CharReader};
pub use self::Error::*;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    TokenFloatError(num::ParseFloatError),
    TokenIntError(num::ParseIntError),
    TokenError(String),
}

pub type Result<T> = result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self { IoError(err) }
}

impl From<num::ParseFloatError> for Error {
    fn from(err: num::ParseFloatError) -> Self { TokenFloatError(err) }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Self { TokenIntError(err) }
}

pub trait Lexer {
    fn next_token(&mut self) -> Result<token::Token>;
    fn next_real_token(&mut self) -> Result<token::Token> {
        loop {
            match self.next_token() {
                Ok(token::Whitespace) | Ok(token::Comment) => continue,
                other => { return other },
            }
        }
    }
}

pub struct Reader<R: Read> {
    chars: Peekable<Fuse<Chars<CharReader<R>>>>,
    done: bool,
}

impl<R: Read> Reader<R> {
    pub fn new(inner: R) -> Reader<R> {
        Reader {
            chars: CharReader::new(inner).chars().fuse().peekable(),
            done: false,
        }
    }
}

impl<I> Lexer for I where I: Iterator<Item=Result<token::Token>> {
    fn next_token(&mut self) -> Result<token::Token> {
        match self.next() {
            Some(Ok(t)) => Ok(t),
            Some(Err(e)) => Err(e),
            None => Ok(token::Eof),
        }
    }
}

impl<R: Read> Iterator for Reader<R> {
    type Item = Result<token::Token>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done { return None; }
        let token = next_token(&mut self.chars);
        if let Some(Ok(token::Eof)) = token {
            self.done = true;
        }
        token
    }
}

macro_rules! throw { ($err:expr) => ({ return Some(Err(From::from($err))) }) }

macro_rules! some_try { ($expr:expr) => {
    match $expr {
        Ok(val) => val,
        Err(err) => throw!(err),
    }
}}

fn next_token<R: CharRead>(chars: &mut Peekable<Fuse<Chars<R>>>) -> Option<Result<token::Token>> {
    macro_rules! try_next { () => {
        match chars.next() {
            Some(Ok(val)) => val,
            Some(Err(err)) => throw!(err),
            None => { return Some(Ok(token::Eof)) },
        }
    }}
    macro_rules! try_peek { () => {{
        enum Tmp<T> { Aight(T), Nope }
        let tmp = match chars.peek() {
            Some(r) => match r {
                &Ok(ref val) => { Tmp::Aight(*val) },
                &Err(ref _e) => { Tmp::Nope }
            },
            None => { Tmp::Aight('\x7f') }
        };
        match tmp {
            Tmp::Aight(t) => t,
            // this unwrapping is safe because there is a next (guaranteed by peekable) and there
            // is an error (guaruanteed by Tmp::Nope above).
            Tmp::Nope => throw!(chars.next().unwrap().err().unwrap()),
        }
    }}}
    macro_rules! read_long_brackets { ($c:ident) => {{
        let mut s = "".to_string();
        let mut k = 0;

        // count the long bracket level
        if $c == '=' {
            k = 1;
            try_next!();
            while try_peek!() == '=' {
                k = k + 1;
                try_next!();
            }
            if try_peek!() != '[' {
                throw!(TokenError(format!("Invalid long string delimiter: '{}'", $c)))
            }
        }

        // consume the second opening bracket
        try_next!();

        // search for a closing bracket until a long bracket of the same level is found
        // XXX: no escaping should done on this kind of literal
        loop {
            match try_peek!() {
                ']' => {
                    let mut z = "]".to_string();
                    let mut n = k;
                    try_next!();
                    while try_peek!() == '=' && n > 0 {
                        z.push('=');
                        n = n - 1;
                        try_next!();
                    }
                    if try_peek!() == ']' && n == 0 {
                        try_next!();
                        break;
                    }
                    z.push(try_next!());
                    s.push_str(&z);
                },
                _ => s.push(try_next!()),
            }
        }

        s
    }}}
    Some(Ok(match try_next!() {
        '+' => token::Plus,
        '-' => match try_peek!() {
            '-' => {
                try_next!();
                if match try_next!() {
                    '[' => match try_peek!() {
                        c @ '[' | c @ '=' => {
                            token::Str(read_long_brackets!(c));
                            false
                        }, _ => true
                    }, _ => true
                } {
                    while try_peek!() != '\n' { try_next!(); }
                    // consume the new line
                    try_next!();
                }
                token::Comment
            },
            _ => token::Minus,
        },
        '*' => token::Star,
        '/' => token::Slash,
        '%' => token::Percent,
        '^' => token::Caret,
        '#' => token::Pound,
        '=' => match try_peek!() {
            '=' => { try_next!(); token::EqEq },
            _ => token::Eq,
        },
        '~' => match try_peek!() {
            '=' => { try_next!(); token::Ne },
            _ => throw!(TokenError("Expected '=' after '~'".to_string())),
        },
        '<' => match try_peek!() {
            '=' => { try_next!(); token::Le },
            _ => token::Lt,
        },
        '>' => match try_peek!() {
            '=' => { try_next!(); token::Ge },
            _ => token::Gt,
        },
        ';' => token::Semi,
        ':' => match try_peek!() {
            ':' => { try_next!(); token::ColonColon },
            _ => token::Colon,
        },
        ',' => token::Comma,
        '.' => match try_peek!() {
            '.' => { try_next!(); match try_peek!() {
                '.' => { try_next!(); token::DotDotDot },
                _ => token::DotDot,
            }},
            c if c.is_numeric() => {
                let mut s = '.'.to_string();
                while try_peek!().maybe_numeric() {
                    s.push(try_next!());
                }
                //token::Float(some_try!(s.parse()))
                token::Int(some_try!(s.parse()))
            },
            _ => token::Dot,
        },
        '(' => token::OpenDelim(token::Paren),
        ')' => token::CloseDelim(token::Paren),
        '{' => token::OpenDelim(token::Brace),
        '}' => token::CloseDelim(token::Brace),
        '[' => match try_peek!() {
            c @ '[' | c @ '=' => token::Str(read_long_brackets!(c)),
            _ => token::OpenDelim(token::Bracket),
        },
        ']' => token::CloseDelim(token::Bracket),
        c if c.is_whitespace() => {
            while try_peek!().is_whitespace() { try_next!(); }
            token::Whitespace
        },
        c @ '"' | c @ '\'' => {
            let mut s = "".to_string();
            loop {
                match try_next!() {
                    // escape sequences: http://www.lua.org/manual/5.2/manual.html#3.1
                    '\\' => match try_next!() {
                        'a' => s.push('\x07'), // audible bell
                        'b' => s.push('\x08'), // backspace
                        'f' => s.push('\x0c'), // form feed
                        'n' | '\n' => s.push('\n'),
                        'r' => s.push('\r'),
                        't' => s.push('\t'),
                        'v' => s.push('\x0b'), // vertical tab
                        c @ '\\' | c @ '\'' | c @ '"' => s.push(c),
                        'z' => { while try_peek!().is_whitespace() { try_next!(); } },
                        'x' => {
                            //let mut z = "0x".to_string();
                            let mut z: Vec<char> = vec![];
                            z.push(try_next!());
                            z.push(try_next!());
                            //let n: u8 = some_try!(z.parse());
                            let n = some_try!(hex_to_int(&z)) as u8;
                            s.push(n as char);
                        },
                        n if n.is_digit(10) => {
                            let mut z = vec![n];
                            if try_peek!().is_digit(10) {
                                z.push(try_next!());
                                if try_peek!().is_digit(10) {
                                    z.push(try_next!());
                                }
                            }
                            let n = some_try!(dec_to_int(&z)) as u8;
                            s.push(n as char);
                        },
                        c => throw!(TokenError(format!("Invalid escape sequence: \\{}", c))),
                    },
                    '\n' => throw!(TokenError("Unfinished string".to_string())),
                    x if x == c => break,
                    x => s.push(x)
                }
            }
            token::Str(s)
        },
        c if c.is_numeric() => {
            let mut s = c.to_string();
            while try_peek!().maybe_numeric() {
                s.push(try_next!());
            }
            token::Num(some_try!(s.parse()))
            //token::Int(some_try!(s.parse()))
        },
        c if UnicodeXID::is_xid_start(c) || c == '_' => {
            let mut s = c.to_string();
            while UnicodeXID::is_xid_continue(try_peek!()) {
                s.push(try_next!());
            }
            if let Some(kw) = keywords::Keyword::from_str(&s) {
                token::Keyword(kw)
            } else {
                token::Ident(s)
            }
        },
        c => throw!(TokenError(format!("Invalid character: {:?}", c))),
    }))
}

trait MaybeNumeric {
    fn maybe_numeric(self) -> bool;
}

impl MaybeNumeric for char {
    fn maybe_numeric(self) -> bool {
        match self {
            c if c.is_numeric() => true,
            c if UnicodeXID::is_xid_continue(c) => true,
            '.' | '+' | '-' => true,
            _ => false,
        }
    }
}

fn hex_to_int(from: &[char]) -> Result<u32> {
    let mut out = 0;
    for &c in from {
        match c {
            '0'...'9' => { out = out * 16 + (c as u32) - ('0' as u32) },
            'A'...'F' => { out = out * 16 + 10 + (c as u32) - ('A' as u32) },
            'a'...'f' => { out = out * 16 + 10 + (c as u32) - ('a' as u32) },
            _ => return Err(TokenError(format!("Invalid hexadecimal character: {:?}", c)))
        }
    }
    Ok(out)
}

fn dec_to_int(from: &[char]) -> Result<u32> {
    let mut out = 0;
    for &c in from {
        match c {
            '0'...'9' => { out = out * 10 + (c as u32) - ('0' as u32) },
            _ => return Err(TokenError(format!("Invalid decimal character: {:?}", c)))
        }
    }
    Ok(out)
}
