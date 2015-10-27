extern crate unicode_xid;

pub mod token;
pub mod lexer;
pub mod parser;
pub mod ast;
mod grammar;
mod charread;

use std::io;
use std::process::exit;
use parser::Parser;

fn main() {
    let stdin = io::stdin();
    let lexer = lexer::Reader::new(stdin);
    let mut parser = Parser::new();
    for token in lexer.filter(|t| match *t { Ok(ref k) => k.is_useful(), Err(..) => true }) {
        //println!("{:?}", token);
        if let Ok(token) = token {
            match parser.push(token.into()) {
                Ok(..) => (),
                Err(e) => {
                    println!("{:?}", e);
                    if !e.recoverable {
                        break;
                    }
                }
            }
        } else {
            println!("{:?}", token);
        }
    }
    if parser.has_errors() {
        println!("has errors");
        exit(1);
    }
    //println!("{:#?}", parser.into_chunk());
}
