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

macro_rules! some_try { ($expr:expr) => {
    match $expr {
        Ok(val) => val,
        Err(err) => { return Some(Err(From::from(err))) },
    }
}}

fn next_token<R: CharRead>(chars: &mut Peekable<Fuse<Chars<R>>>) -> Option<Result<token::Token>> {
    macro_rules! try_next { () => {
        match chars.next() {
            Some(Ok(val)) => val,
            Some(Err(err)) => { return Some(Err(From::from(err))) },
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
            Tmp::Nope => { return Some(Err(From::from(chars.next().unwrap().err().unwrap()))) }
        }
    }}}
    Some(Ok(match try_next!() {
        '+' => token::Plus,
        '-' => match try_peek!() {
            '-' => {
                try_next!();
                if try_peek!() == '[' {
                    // TODO: block comment
                    unimplemented!();
                } else {
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
            _ => { return Some(Err(TokenError("Expected '=' after '~'".to_string()))) }
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
            '[' | '=' => {
                // TODO: bracket string
                unimplemented!()
            },
            _ => token::OpenDelim(token::Bracket),
        },
        ']' => token::CloseDelim(token::Bracket),
        c if c.is_whitespace() => {
            while try_peek!().is_whitespace() {
                try_next!();
            }
            token::Whitespace
        },
        c @ '"' | c @ '\'' => {
            let mut s = "".to_string();
            while try_peek!() != c {
                match try_peek!() {
                    '\\' => s.push(try_next!()),
                    '\n' => {
                        return Some(Err(TokenError("Unterminated literal".to_string())))
                    },
                    _ => ()
                }
                s.push(try_next!());
            }
            // consume the closing quote
            try_next!();
            token::Str(unescape(s))
        },
        c if c.is_numeric() => {
            let mut s = c.to_string();
            while try_peek!().maybe_numeric() {
                s.push(try_next!());
            }
            token::Num(some_try!(s.parse()))
            //token::Int(some_try!(s.parse()))
        },
        c if UnicodeXID::is_xid_start(c) => {
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
        c => { return Some(Err(TokenError(format!("Invalid character: {:?}", c)))) }
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

fn unescape(s: String) -> String {
    // TODO
    s
}
