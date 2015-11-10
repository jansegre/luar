pub use self::Token::*;
pub use self::DelimToken::*;
pub use self::keywords::Keyword::*;

pub mod keywords {
    pub use self::Keyword::*;

    macro_rules! define_keywords {
        ($($name:expr => $variant:ident),*) => {
            #[derive(Debug, PartialEq, Eq, Clone, Copy)]
            pub enum Keyword {
                $($variant),*
            }

            impl Keyword {
                pub fn from_str(s: &str) -> Option<Keyword> {
                    match s {
                        $($name => Some($variant),)*
                        _ => None,
                    }
                }
            }
        }
    }

    define_keywords! {
        "and"      =>  And,
        "break"    =>  Break,
        "do"       =>  Do,
        "else"     =>  Else,
        "elseif"   =>  Elseif,
        "end"      =>  End,
        "false"    =>  False,
        "for"      =>  For,
        "function" =>  Function,
        "goto"     =>  Goto,
        "if"       =>  If,
        "in"       =>  In,
        "local"    =>  Local,
        "nil"      =>  Nil,
        "not"      =>  Not,
        "or"       =>  Or,
        "repeat"   =>  Repeat,
        "return"   =>  Return,
        "then"     =>  Then,
        "true"     =>  True,
        "until"    =>  Until,
        "while"    =>  While
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DelimToken {
    Paren, // ( or )
    Brace, // { or }
    Bracket, // [ or ]
}

// #[derive(Debug, PartialEq, Eq, Clone)]
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // expression operators and symbols
    Plus, // +
    Minus, // -
    Star, // *
    Slash, // /
    Percent, // %
    Caret, // ^
    Pound, // #
    EqEq, // ==
    Ne, // ~=
    Le, // <=
    Ge, // >=
    Lt, // <
    Gt, // >
    Eq, // =

    // structural symbols
    Semi, // ;
    Colon, // :
    ColonColon, // ::
    Comma, // ,
    Dot, // .
    DotDot, // ..
    DotDotDot, // ...
    OpenDelim(DelimToken), // ( { [
    CloseDelim(DelimToken), // ) } ]

    // terminals
    // TODO: hold the data
    Ident(String),
    Num(f64),
    Int(u64),
    Str(String),
    Keyword(keywords::Keyword),

    // junk
    Whitespace,
    /// Comments begin with --
    Comment,
    // Shebang,
    Invalid,

    Eof,
}

impl Token {
    pub fn is_useful(&self) -> bool {
        match *self {
            Whitespace | Comment | Invalid => false,
            _ => true,
        }
    }

    pub fn as_keyword(self) -> Option<keywords::Keyword> {
        if let Keyword(kw) = self {
            Some(kw)
        } else {
            None
        }
    }
}
