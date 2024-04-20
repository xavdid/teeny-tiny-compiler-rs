mod emitter;
mod lexer;
mod parser;
mod tokens;

use std::{env::args, fs::read_to_string};

use lexer::Lexer;
use parser::Parser;

use crate::emitter::Emitter;

fn main() {
    println!("Teeny Tiny Compiler!\n");

    let filename = args()
        .nth(1)
        .expect("ERR: Compiler needs single source file as argument");

    let file_contents = read_to_string(filename).expect("unable to read file!");

    let lexer = Lexer::new(&file_contents);
    let mut emitter = Emitter::new(String::from("out.c"));
    let mut parser = Parser::new(lexer, &mut emitter);
    parser.program();
    emitter.write_file();
}
