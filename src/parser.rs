use std::collections::HashSet;

use crate::lexer::Lexer;
use crate::tokens::{Token, IDENT};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,

    symbols: HashSet<String>,
    labels_declared: HashSet<String>,
    labels_gotoed: HashSet<String>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let (cur_token, peek_token) = (lexer.get_next_token(), lexer.get_next_token());

        Parser {
            lexer,
            cur_token,
            peek_token,
            symbols: HashSet::new(),
            labels_declared: HashSet::new(),
            labels_gotoed: HashSet::new(),
        }
    }

    // TODO: this DX isn't great - having to pass the expected token type after doing the match externally is weird
    // it gives the error I want, which is good, but the whole thing isn't great otherwise
    // my guess is I need to do the whole thing with a macro?
    fn assert_and_advance_token(&mut self, valid: bool, expected: Token) {
        println!("    ASSERTING {:?} ({valid})", expected);
        if !valid {
            panic!(
                "ERR (parser): Expected {:?}, got {:?}",
                expected, self.cur_token
            )
        }
        self.advance_token();
    }

    fn advance_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.get_next_token();
    }

    // fn cur_token_is(&self, token: &Token) -> bool {
    //     matches!(&self.cur_token, token)
    // }
    // fn peek_token_is(&self, token: &Token) -> bool {
    //     matches!(&self.peek_token, token)
    // }

    pub fn program(&mut self) {
        println!("  PROGRAM");

        while matches!(self.cur_token, Token::Newline) {
            self.advance_token();
        }

        while !matches!(self.cur_token, Token::Eof) {
            self.statement();
        }

        for label in &self.labels_gotoed {
            if !self.labels_declared.contains(label) {
                panic!("Attempting to GOTO undeclared label: {label}");
            }
        }
    }

    fn statement(&mut self) {
        if matches!(self.cur_token, Token::Print) {
            println!("  STATEMENT-PRINT");
            self.advance_token();

            if matches!(self.cur_token, Token::String(_)) {
                self.advance_token();
            } else {
                self.expression();
            }
        } else if matches!(self.cur_token, Token::If) {
            println!("  STATEMENT-IF");

            self.advance_token();
            self.comparison();

            self.assert_and_advance_token(matches!(self.cur_token, Token::Then), Token::Then);
            self.nl();

            while !matches!(self.cur_token, Token::Endif) {
                self.statement();
            }

            self.assert_and_advance_token(matches!(self.cur_token, Token::Endif), Token::Endif);
        } else if matches!(self.cur_token, Token::While) {
            println!("  STATEMENT-WHILE");
            self.advance_token();
            self.comparison();

            self.assert_and_advance_token(matches!(self.cur_token, Token::Repeat), Token::Repeat);
            self.nl();

            while !matches!(self.cur_token, Token::Endwhile) {
                self.statement();
            }

            self.assert_and_advance_token(
                matches!(self.cur_token, Token::Endwhile),
                Token::Endwhile,
            );
        } else if matches!(self.cur_token, Token::Label) {
            println!("  STATEMENT-LABEL");
            self.advance_token();

            // should basically always match at this point
            let Token::Ident(label_text) = &self.cur_token else {
                panic!(
                    "unable to extract label text from LABEL: {:?}",
                    self.cur_token
                )
            };
            if self.labels_declared.contains(label_text) {
                panic!("Label already exists: {label_text}");
            } else {
                self.labels_declared.insert(label_text.to_string());
            }

            self.assert_and_advance_token(matches!(self.cur_token, Token::Ident(_)), IDENT);
        } else if matches!(self.cur_token, Token::Goto) {
            println!("  STATEMENT-GOTO");
            self.advance_token();

            // should basically always match at this point
            let Token::Ident(label_text) = &self.cur_token else {
                panic!(
                    "unable to extract label text from GOTO: {:?}",
                    self.cur_token
                )
            };

            self.labels_gotoed.insert(label_text.to_string());

            self.assert_and_advance_token(matches!(self.cur_token, Token::Ident(_)), IDENT);
        } else if matches!(self.cur_token, Token::Let) {
            println!("  STATEMENT-LET");
            self.advance_token();

            let Token::Ident(ident_name) = &self.cur_token else {
                panic!(
                    "unable to extract label text from IDENT: {:?}",
                    self.cur_token
                )
            };
            self.symbols.insert(ident_name.clone());

            self.assert_and_advance_token(matches!(self.cur_token, Token::Ident(_)), IDENT);
            self.assert_and_advance_token(matches!(self.cur_token, Token::Eq), Token::Eq);
            self.expression();
        } else if matches!(self.cur_token, Token::Input) {
            println!("  STATEMENT-INPUT");
            self.advance_token();

            let Token::Ident(ident_name) = &self.cur_token else {
                panic!(
                    "unable to extract label text from INPUT: {:?}",
                    self.cur_token
                )
            };
            self.symbols.insert(ident_name.clone());

            self.assert_and_advance_token(matches!(self.cur_token, Token::Ident(_)), IDENT);
        } else {
            panic!("Invalid statement at {:?}", self.cur_token);
        }

        self.nl();
    }

    fn comparison(&mut self) {
        println!("  COMPARISON");

        self.expression();
        // must be at least one comparison operator and another expression
        if self.is_comparison_operator() {
            self.advance_token();
            self.expression();
        } else {
            panic!("Expected comparison operator at {:?}", self.cur_token)
        }

        while self.is_comparison_operator() {
            self.advance_token();
            self.expression();
        }
    }

    fn is_comparison_operator(&self) -> bool {
        matches!(
            self.cur_token,
            Token::Gt | Token::Gteq | Token::Lt | Token::Lteq | Token::Eqeq | Token::Noteq
        )
    }

    fn expression(&mut self) {
        println!("  EXPRESSION");
        self.term();

        while matches!(self.cur_token, Token::Plus | Token::Minus) {
            self.advance_token();
            self.term();
        }
    }

    fn term(&mut self) {
        println!("  TERM");

        self.unary();

        while matches!(self.cur_token, Token::Asterisk | Token::Slash) {
            self.advance_token();
            self.unary();
        }
    }

    fn unary(&mut self) {
        println!("  UNARY");

        if matches!(self.cur_token, Token::Plus | Token::Minus) {
            self.advance_token();
        }
        self.primary();
    }

    fn primary(&mut self) {
        println!("  PRIMARY ({:?})", self.cur_token);

        match &self.cur_token {
            Token::Number(_) => {
                self.advance_token();
            }
            Token::Ident(ident) => {
                if !self.symbols.contains(ident) {
                    panic!("Referencing varible before assignment: {ident}");
                }
                self.advance_token();
            }
            _ => {
                panic!("Unexpectd token at {:?}", self.cur_token);
            }
        }
    }

    fn nl(&mut self) {
        println!("  NEWLINE");
        self.assert_and_advance_token(matches!(self.cur_token, Token::Newline), Token::Newline);

        while matches!(self.cur_token, Token::Newline) {
            self.advance_token();
        }
    }
}
