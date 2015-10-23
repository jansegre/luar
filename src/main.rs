extern crate unicode_xid;

pub mod token;
pub mod lexer;
pub mod parser;
pub mod ast;
pub mod grammar;
mod charread;

use std::io;
//use std::io::{self, Write, BufReader, BufRead};
//use parser::Parse;

fn main() {
    //let stdin = BufReader::new(io::stdin());
    let stdin = io::stdin();
    //let mut stdout = io::stdout();
    let lexer = lexer::Reader::new(stdin);
    let mut parser = grammar::Parser::new();
    for token in lexer.filter(|t| match *t { Ok(ref k) => k.is_useful(), Err(..) => true }) {
        println!("{:?}", token);
        if let Ok(token) = token {
            parser.parse(token.into());
        }
    }
    println!("{:#?}", parser.into_chunk());
    //print!("> ");
    //stdout.flush().unwrap();
    //for line in stdin.lines() {
    //    if let Ok(ref line) = line {
    //        println!("{:#?}", grammar::parse_expr(line));
    //        println!("{:#?}", grammar::parse_stmt(line));
    //        //println!("{:#?}", grammar::parse_chunk(line));
    //        print!("> ");
    //        stdout.flush().unwrap();
    //    } else {
    //        println!("Error: {:?}", line);
    //    }
    //}
}
