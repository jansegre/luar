extern crate unicode_xid;

pub mod token;
pub mod lexer;
pub mod parser;
pub mod ast;
pub mod codegen;
mod grammar;
mod charread;

use std::io;
use std::process::exit;
use parser::Parser;

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
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
        println!("Exiting due to previous errors");
        exit(1);
    }
    if let Ok(chunk) = parser.into_chunk() {
        let mut cg = codegen::Codegen::new(&mut stdout);
        // FIXME: proper message for unwrap
        if let Err(err) = cg.gen_from(chunk) {
            println!("An error occurred: {:#?}", err);
            exit(2);
        }
    }
}
