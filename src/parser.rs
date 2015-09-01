use std::result;
use ::token;
use ::token::keywords;
use ::lexer::{self, Lexer};
//use ::ast::{Chunk, Block};
use ::ast;
pub use self::Error::*;

#[derive(Debug)]
pub enum Error {
    Reject,
    RejectWith(String),
    ParserError(String),
    LexerError(lexer::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl From<lexer::Error> for Error {
    fn from(err: lexer::Error) -> Self { LexerError(err) }
}

pub trait Parse {
    fn read_chunk(&mut self) -> Result<ast::Chunk>;
}

pub struct Parser<L> {
    lexer: L,
    _next_token: Option<token::Token>,
}

// recursive parser
impl<L: Lexer> Parse for Parser<L> {
    fn read_chunk(&mut self) -> Result<ast::Chunk> {
        self.parse_chunk()
    }
}

macro_rules! accept {
    ($this:ident, $expr:expr) => {
        match $expr {
            Ok(val) => { $this.consume(); Some(val) },
            Err(Reject) => None,
            Err(err) => { return Err(From::from(err)) },
        }
    };
    ($this:ident { $($pat:pat => $expr:expr),* } else $none:expr) => {
        match $this.next_token() {
            $(Ok(&$pat) => { $this.consume(); $expr },)*
            Ok(_) | Err(Reject) => $none,
            Err(err) => { return Err(From::from(err)) }
        }
    }
}

macro_rules! expect {
    ($token:expr) => {
        match self.next_token() {
            Ok(&$token) => { self.consume() },
            Ok(_) => { return Err(Reject) },
            Err(err) => { return Err(From::from(err)) },
        }
    }
}

impl<L: Lexer> Parser<L> {
    pub fn new(lexer: L) -> Parser<L> {
        Parser { lexer: lexer, _next_token: None }
    }

    fn consume(&mut self) {
        self._next_token = None;
    }

    fn next_token(&mut self) -> Result<&token::Token> {
        if let Some(ref token) = self._next_token {
            Ok(token)
        } else {
            let token = try!(self.lexer.next_real_token());
            self._next_token = Some(token);
            Ok(self._next_token.as_ref().unwrap())
        }
    }

    pub fn parse_chunk(&mut self) -> Result<ast::Chunk> {
        if let Some(block) = accept!(self, self.parse_block()) {
            accept! { self {
                token::Eof => Ok(ast::Chunk { block: block })
            } else Err(RejectWith("expected <eof> instead".into())) }
        } else {
            Err(RejectWith("could not parse block".into()))
        }
    }

    pub fn parse_block(&mut self) -> Result<ast::Block> {
        let stmts = try!(self.parse_stat());
        let ret = accept!(self, self.parse_retstat());
        Ok(ast::Block { stmts: stmts, ret: ret })
    }

    pub fn parse_stat(&mut self) -> Result<ast::V<ast::Stmt>> {
        let mut stmts = vec![];
        loop {
            let stmt = accept! { self {
                token::Semi => continue,
                token::Keyword(token::Break) => ast::StmtBreak
            } else break };
            stmts.push(stmt);
        }
        // TODO
        Ok(stmts.into_boxed_slice())
    }

    pub fn parse_retstat(&mut self) -> Result<ast::V<ast::Expr>> {
        accept! { self {
            token::Keyword(token::Return) => ()
        } else { return Err(Reject) } }
        let exprs = vec![];
        Ok(exprs.into_boxed_slice())
    }

    /*
    pub fn parse_stat(&mut self) -> Result<ast::Stmt> {
        let stmts = vec![];
        loop {
            let stmt = match try!(self.next_token()) {
                token::Semi => (),
                token::Keyword(kw) => match {
                    token::Break => advance!(ast::StmtBreak),
                    token::Goto => advance!(ast::StmtGoto(accept!(self.parse_name()))),
                    token::Do => {
                        advance!();
                        let inner_stmt = accept!(self.parse_block());
                        expect_keyword!(token::End);
                        ast::StmtDo(inner_stmt)
                    },
                    token::While => {
                        advance!();
                        let cond_expr = accept!(self.parse_expr());
                        expect_keyword!(token::Do);
                        let inner_stmt = accept!(self.parse_block());
                        expect_keyword!(token::End);
                        ast::StmtWhile(cond_expr, inner_stmt)
                    },
                    token::Repeat => {
                        advance!();
                        let inner_stmt = accept!(self.parse_block());
                        expect_keyword!(token::End);
                        let cond_expr = accept!(self.parse_expr());
                        expect_keyword!(token::Do);
                        ast::StmtRepeat(inner_stmt, cond_expr)
                    },
                    token::If => {
                        advance!();
                        unimplemented!()
                    },
                    token::For => {
                        advance!();
                        unimplemented!()
                    },
                    token::Function => {
                        advance!();
                        let funcname = accept!(self.parse_funcname());
                        let funcbody = accept!(self.parse_funcbody());
                        ast::StmtFunction(funcname, funcbody)
                    },
                    token::Local => {
                        advance!();
                        match try!(self.next_token()) {
                            token::Keyword(token::Function) => {
                                let name = accept!(self.parse_name());
                                let funcbody = accept!(self.parse_funcbody());
                                ast::StmtLocalFunction(name, funcbody)
                            },
                            _ => {
                                let namelist = accept!(parse_namelist());
                                if self.next_token() == Ok(token::Eq) {
                                    advance!();
                                    let explist = accept!(parse_explist());
                                    ast::Local(namelist, Some(explist))
                                } else {
                                    ast::Local(namelist, None)
                                }
                            }
                        }
                    },
                },
            }
            stmts.push(stmt);
        }
        Ok(stmts.into_boxed_slice())
    }

    pub fn parse_name(&mut self) -> Result<ast::Name>
        */
}
