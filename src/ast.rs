//! Abstract Syntax Tree structures
//!
//! The following AST aims to abstract the Lua 5.2 grammar:
//!
//! ```EBNF
//! chunk = block
//!
//! block = {stat} [retstat]
//!
//! stat = ';'
//!      | varlist '=' explist
//!      | functioncall
//!      | label
//!      | 'break'
//!      | 'goto' Name
//!      | 'do' block 'end'
//!      | 'while' exp 'do' block 'end'
//!      | 'repeat' block 'until' exp
//!      | 'if' exp 'then' block {'elseif' exp 'then' block} ['else' block] 'end'
//!      | 'for' Name '=' exp ',' exp [',' exp] 'do' block 'end'
//!      | 'for' namelist 'in' explist 'do' block 'end'
//!      | 'function' funcname funcbody
//!      | 'local' 'function' Name funcbody
//!      | 'local' namelist ['=' explist]
//!
//! retstat = 'return' [explist] [';']
//!
//! label = '::' Name '::'
//!
//! funcname = Name {'.' Name} [':' Name]
//!
//! varlist = var {',' var}
//!
//! var = Name
//!     | prefixexp '[' exp ']'
//!     | prefixexp '.' Name
//!
//! namelist = Name {',' Name}
//!
//! explist = exp {',' exp}
//!
//! exp = 'nil'
//!     | 'false'
//!     | 'true'
//!     | Number
//!     | String
//!     | '...'
//!     | functiondef
//!     | prefixexp
//!     | tableconstructor
//!     | exp binop exp
//!     | unop exp
//!
//! prefixexp = var
//!           | functioncall
//!           | '(' exp ')'
//!
//! functioncall = prefixexp args
//!              | prefixexp ':' Name args
//!
//! args = '(' [explist] ')'
//!      | tableconstructor
//!      | String
//!
//! functiondef = 'function' funcbody
//!
//! funcbody = '(' [parlist] ')' block 'end'
//!
//! parlist = namelist [',' '...']
//!         | '...'
//!
//! tableconstructor = '{' [fieldlist] '}'
//!
//! fieldlist = field {fieldsep field} [fieldsep]
//!
//! field = '[' exp ']' '=' exp
//!       | Name '=' exp
//!       | exp
//!
//! fieldsep = ',' | ';'
//!
//! binop = '+' | '-' | '*' | '/' | '^' | '%' | '..'
//!       | '<' | '<=' | '>' | '>=' | '==' | '~='
//!       | 'and' | 'or'
//!
//! unop = '-' | 'not' | '#'
//! ```

// Abstract Syntax Tree:
//
// ```text
// chunk = block
//
// block = stat* exp+?
//
// stat = var+ exp+               (set)
//      | Name                    (label)
//      | Break
//      | Name                    (goto)
//      | block                   (do)
//      | exp block               (while)
//      | block exp               (repeat)
//      | [exp block]+ block?     (if)
//      | Name exp exp exp? block (for)
//      | Name+ exp+ block        (for in)
//      | Name+ Name? Name* Ellipsis? block (function)
//      | Name Name* Ellipsis? block (local function)
//      | Name+ exp+?             (local set)
//      | call                    (function call)
//
// exp = Nil
//     | False
//     | True
//     | Number
//     | String
//     | Ellipsis
//     | Name* Ellispsis? block (function)
//     | field+                 (table)
//     | exp Binop exp
//     | Unop exp
//     | call
//     | var
//
// call = exp args      (function)
//      | exp Name args (method)
//
// args = exp*    (paren call)
//      | field*  (table call)
//      | String  (string call)
//
// var = Name     (variable)
//     | exp exp  (property)
//     | exp Name (member)
//
// field = exp exp
//       | Name exp
//       | exp
// ```

use std::mem;
pub use self::Stmt::*;
pub use self::Expr::*;
pub use self::Call::*;
pub use self::Args::*;
pub use self::Var::*;
pub use self::Field::*;
pub use self::BinOp::*;
pub use self::UnOp::*;

pub type P<T> = Box<T>;
pub type V<T> = Box<[T]>;

#[allow(non_snake_case)]
pub fn P<T: 'static>(value: T) -> P<T> {
    P::new(value)
}

//#[allow(non_snake_case)]
//fn V<T>(values: T) -> Box<[T]> {
//    Box::new(values)
//}

#[derive(Debug)]
pub struct Chunk {
    pub block: Block,
}

#[derive(Debug)]
pub struct Block {
    pub stmts: V<Stmt>,
    // None means no return statement, while Some([]) means an empty return statement
    pub ret: Option<V<Expr>>,
}

#[derive(Debug)]
pub enum Stmt {
    /// LHS and RHS both have at least 1 element
    StmtSet(V<Var>, V<Expr>),
    StmtLabel(Name),
    StmtBreak,
    StmtGoto(Name),
    StmtDo(P<Block>),
    StmtWhile(P<Expr>, P<Block>),
    StmtRepeat(P<Block>, P<Expr>),
    StmtIf(P<Expr>, P<Block>, V<(Expr, Block)>, Option<P<Block>>),
    StmtForNum(Name, P<Expr>, P<Expr>, Option<P<Expr>>, P<Block>),
    StmtForIn(V<Name>, V<Expr>, P<Block>),
    StmtFunction(V<Name>, Option<Name>, V<Name>, bool, P<Block>),
    StmtLocalFunction(Name, V<Name>, bool, P<Block>),
    StmtLocal(V<Name>, Option<V<Expr>>),
    StmtCall(P<Call>),
    StmtInvalid, // TODO: attach an error
}

#[derive(Debug)]
pub enum Expr {
    ExprNil,
    ExprFalse,
    ExprTrue,
    ExprEllipsis,
    ExprNumber(f64),
    ExprString(String),
    ExprFunction(V<Name>, bool, P<Block>),
    ExprTable(V<Field>),
    ExprBinary(P<Expr>, BinOp, P<Expr>),
    ExprUnary(UnOp, P<Expr>),
    ExprCall(P<Call>),
    ExprVar(P<Var>),
    ExprInvalid, // TODO: attach an error
}

#[derive(Debug)]
pub enum Call {
    CallFunction(P<Expr>, P<Args>),
    CallMethod(P<Expr>, Name, P<Args>),
}

#[derive(Debug)]
pub enum Args {
    ArgsParen(V<Expr>),
    ArgsTable(V<Field>),
    ArgsString(String),
}

#[derive(Debug)]
pub enum Var {
    VarName(Name),
    VarProperty(P<Expr>, P<Expr>),
    VarMember(P<Expr>, Name),
}

#[derive(Debug)]
pub enum Field {
    ///     '[' expr ']' '=' expr
    FieldExpr(P<Expr>, P<Expr>),
    ///     Name '=' expr
    FieldNamed(Name, P<Expr>),
    ///     expr
    FieldAuto(P<Expr>),
}

#[derive(Debug)]
pub enum BinOp {
    /// Addition: `+`
    BinAdd,
    /// Subtraction: `-`
    BinSub,
    /// Multiplication: `*`
    BinMul,
    /// Division: `/`
    BinDiv,
    /// Power: `^`
    BinPow,
    /// Modulo: `%`
    BinMod,
    /// Concatenation: `..`
    BinCon,
    /// Less than: `<`
    BinLt,
    /// Less or equal: `<=`
    BinLe,
    /// Greater than: `>`
    BinGt,
    /// Greater or equal: `>=`
    BinGe,
    /// Equal: `==`
    BinEq,
    /// Not equal: `~=`
    BinNe,
    /// Logic and: `and`
    BinAnd,
    /// Logic or: `or`
    BinOr,
}

#[derive(Debug)]
pub enum UnOp {
    /// Negative: `-`
    UnNeg,
    /// Logic not: `not`
    UnNot,
    /// Length of: `#`
    UnLen,
}

#[derive(Debug)]
pub struct Name {
    pub name: P<str>,
}

impl Name {
    pub fn new(s: String) -> Name {
        // XXX: simplify when/if String::into_boxed_slice is stabilized
        let s = s.as_bytes().to_owned().into_boxed_slice();
        Name { name: unsafe { mem::transmute(s) } }
    }
}
