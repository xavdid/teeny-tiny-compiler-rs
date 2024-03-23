mod lexer;

use lexer::Lexer;

use crate::lexer::Token;

fn main() {
    // let s = "LET foobar = 123";
    // let s = "+- */\n+";
    // let s = "+- */ >>= = !=";
    // let s = "+- # This is a comment!\n */";
    // let s = "+- \"This is a string\" # This is a comment!\n */";
    // let s = "+-123 9.8654*/";
    let s = "IF+-123 foo*THEN/";
    let s = "a = \"gates\"# neat";

    // I want the lexer to take care of padding this, not do it out here
    let padded_source = format!("{s}\n");
    let mut l = Lexer::new(&padded_source);

    loop {
        let t = l.get_next_token();
        if matches!(t, Token::Eof) {
            break;
        }
        println!("{:#?}", t);
        // l.advance_pointer();
    }

    // println!("Hello, world!");
}
