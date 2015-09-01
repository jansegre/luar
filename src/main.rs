extern crate unicode_xid;

pub mod token;
pub mod lexer;
//pub mod parser;
//pub mod ast;
mod charread;

use std::io;
//use std::io::{self, Write};
//use parser::Parse;

fn main() {
    let stdin = io::stdin();
    //let mut stdout = io::stdout();
    let lexer = lexer::Reader::new(stdin);
    for token in lexer {
        println!("{:?}", token);
    }
    //let mut parser = parser::Parser::new(lexer);
    //print!("> ");
    //stdout.flush();
    //let chunk = parser.read_chunk();
    //println!("{:#?}", chunk);
}
