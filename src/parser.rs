use ::grammar;
use ::grammar::Token::*;
use ::token::Token;
use ::token::Token::*;
use ::token::DelimToken::*;
use ::token::keywords::Keyword::*;
use ::ast;
pub use self::ErrorType::*;

#[derive(Debug)]
pub struct ParserError {
    pub error_type: ErrorType,
    pub recoverable: bool,
}

#[derive(Debug)]
pub enum ErrorType {
    UnexpectedToken(Token),
    PreviousError,
    NotReady,
    NotAccepted,
    UnexpectedError,
}


pub struct Parser {
    inner: grammar::Parser,
    error_count: u8,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            inner: grammar::Parser::new(),
            error_count: 0,
        }
    }

    pub fn push(&mut self, token: Token) -> Result<(), ParserError> {
        self.inner.parse(token.into());
        if !self.inner.is_recoverable() && self.inner.error_count() <= self.error_count {
            return Err(ParserError {
                error_type: PreviousError,
                recoverable: false,
            })
        }
        if let Some(t) = self.inner.last_token_error() {
            let count = self.inner.error_count();
            if count == self.error_count {
                // already reported
                Ok(())
            } else {
                self.error_count = count;
                Err(ParserError {
                    error_type: UnexpectedToken(t.into()),
                    recoverable: self.inner.is_recoverable(),
                })
            }
        } else {
            Ok(())
        }
    }

    pub fn into_chunk(self) -> Result<ast::Chunk, ParserError> {
        let recoverable = self.inner.is_recoverable();
        if !self.inner.is_accepted() {
            Err(ParserError {
                error_type: NotAccepted,
                recoverable: recoverable,
            })
        } else {
            match self.inner.into_chunk() {
                Some(chunk) => Ok(chunk),
                None => Err(ParserError {
                    error_type: UnexpectedError,
                    recoverable: recoverable,
                }),
            }
        }
    }

    pub fn has_errors(&self) -> bool {
        self.error_count > 0 || self.inner.error_count_soft() > 0
    }
}

impl From<Token> for grammar::Token {
    fn from(t: Token) -> Self {
        match t {
            Plus => PLUS,
            Minus => MINUS,
            Star => STAR,
            Slash => SLASH,
            Percent => PERCENT,
            Caret => CARET,
            Pound => POUND,
            EqEq => EQEQ,
            Ne => NE,
            Le => LE,
            Ge => GE,
            Lt => LT,
            Gt => GT,
            Eq => EQ,
            Semi => SEMI,
            Colon => COLON,
            ColonColon => COLONCOLON,
            Comma => COMMA,
            Dot => DOT,
            DotDot => DOTDOT,
            DotDotDot => DOTDOTDOT,
            OpenDelim(d) => match d {
                Paren => OPENPAREN,
                Brace => OPENBRACE,
                Bracket => OPENBRACKET,
            },
            CloseDelim(d) => match d {
                Paren => CLOSEPAREN,
                Brace => CLOSEBRACE,
                Bracket => CLOSEBRACKET,
            },
            Keyword(k) => match k {
                And => AND,
                Break => BREAK,
                Do => DO,
                Else => ELSE,
                Elseif => ELSEIF,
                End => END,
                False => FALSE,
                For => FOR,
                Function => FUNCTION,
                Goto => GOTO,
                If => IF,
                In => IN,
                Local => LOCAL,
                Nil => NIL,
                Not => NOT,
                Or => OR,
                Repeat => REPEAT,
                Return => RETURN,
                Then => THEN,
                True => TRUE,
                Until => UNTIL,
                While => WHILE,
            },
            Ident(s) => NAME(s),
            Int(n) => NUMBER(n as f64),
            Num(n) => NUMBER(n),
            Str(s) => STRING(s),
            _ => EOI,
        }
    }
}

impl From<grammar::Token> for Token {
    fn from(t: grammar::Token) -> Self {
        match t {
            PLUS => Plus,
            MINUS => Minus,
            STAR => Star,
            SLASH => Slash,
            PERCENT => Percent,
            CARET => Caret,
            POUND => Pound,
            EQEQ => EqEq,
            NE => Ne,
            LE => Le,
            GE => Ge,
            LT => Lt,
            GT => Gt,
            EQ => Eq,
            SEMI => Semi,
            COLON => Colon,
            COLONCOLON => ColonColon,
            COMMA => Comma,
            COMMA2 => Comma,
            DOT => Dot,
            DOTDOT => DotDot,
            DOTDOTDOT => DotDotDot,
            OPENPAREN => OpenDelim(Paren),
            OPENPAREN2 => OpenDelim(Paren),
            OPENBRACE => OpenDelim(Brace),
            OPENBRACKET => OpenDelim(Bracket),
            CLOSEPAREN => CloseDelim(Paren),
            CLOSEBRACE => CloseDelim(Brace),
            CLOSEBRACKET => CloseDelim(Bracket),
            AND => Keyword(And),
            BREAK => Keyword(Break),
            DO => Keyword(Do),
            ELSE => Keyword(Else),
            ELSEIF => Keyword(Elseif),
            END => Keyword(End),
            FALSE => Keyword(False),
            FOR => Keyword(For),
            FUNCTION => Keyword(Function),
            GOTO => Keyword(Goto),
            IF => Keyword(If),
            IN => Keyword(In),
            LOCAL => Keyword(Local),
            NIL => Keyword(Nil),
            NOT => Keyword(Not),
            OR => Keyword(Or),
            REPEAT => Keyword(Repeat),
            RETURN => Keyword(Return),
            THEN => Keyword(Then),
            TRUE => Keyword(True),
            UNTIL => Keyword(Until),
            WHILE => Keyword(While),
            NAME(s) => Ident(s),
            NUMBER(n) => Num(n),
            STRING(s) => Str(s),
            EOI => Eof,
        }
    }
}
