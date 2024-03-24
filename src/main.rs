mod lexer;
mod parser;
mod tokens;

use std::{env::args, fs::read_to_string};

use lexer::Lexer;
use parser::Parser;

fn main() {
    println!("Teeny Tiny Compiler!\n");

    // if args().len() != 2 {

    // };
    let filename = args()
        .nth(1)
        .expect("ERR: Compiler needs single source file as argument");

    // let file = File::open(filename).expect("file {filename} not found");
    let file_contents = read_to_string(filename).expect("unable to read file!");

    // println!("{:?}", file_contents);

    // ---

    // let s = "LET foobar = 123";
    // let s = "+- */\n+";
    // let s = "+- */ >>= = !=";
    // let s = "+- # This is a comment!\n */";
    // let s = "+- \"This is a string\" # This is a comment!\n */";
    // let s = "+-123 9.8654*/";
    // let s = "IF+-123 foo*THEN/";
    // let s = "a = \"gates\"# neat";

    // assert_eq!(Token::Endif, Token::Endif);
    // assert_eq!(
    //     Token::Ident(String::from("asdf")),
    //     Token::Ident(String::from("asdf"))
    // );
    // assert!(matches!(
    //     Token::String(String::from("asdf")),
    //     Token::String(_),
    // ));
    // I want the lexer to take care of padding this, not do it out here
    // let padded_source = format!("{s}\n");
    let l = Lexer::new(&file_contents);
    let mut p = Parser::new(l);
    p.program();
    println!("\nParsing Completed")
    // loop {
    //     let t = l.get_next_token();
    //     if matches!(t, Token::Eof) {
    //         break;
    //     }
    //     println!("{:#?}", t);
    //     // l.advance_pointer();
    // }

    // println!("Hello, world!");
}
