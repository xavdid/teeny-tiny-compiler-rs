use std::collections::HashSet;

use crate::emitter::Emitter;
use crate::lexer::Lexer;
use crate::tokens::{Token, IDENT};

pub struct Parser<'a, 'b> {
    lexer: Lexer<'a>,
    emitter: &'b mut Emitter,
    cur_token: Token,
    peek_token: Token,

    symbols: HashSet<String>,
    labels_declared: HashSet<String>,
    labels_gotoed: HashSet<String>,
}

impl<'a, 'b> Parser<'a, 'b> {
    pub fn new(mut lexer: Lexer<'a>, emitter: &'b mut Emitter) -> Self {
        let (cur_token, peek_token) = (lexer.get_next_token(), lexer.get_next_token());

        Parser {
            lexer,
            emitter,
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

    pub fn program(&mut self) {
        self.emitter.header_line("#include <stdio.h>");
        self.emitter.header_line("int main(void){");

        while matches!(self.cur_token, Token::Newline) {
            self.advance_token();
        }

        while !matches!(self.cur_token, Token::Eof) {
            self.statement();
        }

        self.emitter.emit_line("return 0;");
        self.emitter.emit_line("}");

        for label in &self.labels_gotoed {
            if !self.labels_declared.contains(label) {
                panic!("Attempting to GOTO undeclared label: {label}");
            }
        }
    }

    fn statement(&mut self) {
        if matches!(self.cur_token, Token::Print) {
            self.advance_token();

            if let Token::String(text) = &self.cur_token {
                self.emitter.emit_line(&format!("printf(\"{text}\\n\");"));
                self.advance_token();
            } else {
                self.emitter.emit("printf(\"%.2f\\n\", (float)(");
                self.expression();
                self.emitter.emit_line("));")
            }
        } else if matches!(self.cur_token, Token::If) {
            self.advance_token();
            self.emitter.emit("if(");
            self.comparison();

            self.assert_and_advance_token(matches!(self.cur_token, Token::Then), Token::Then);
            self.nl();
            self.emitter.emit_line("){");

            while !matches!(self.cur_token, Token::Endif) {
                self.statement();
            }

            self.assert_and_advance_token(matches!(self.cur_token, Token::Endif), Token::Endif);
            self.emitter.emit_line("}");
        } else if matches!(self.cur_token, Token::While) {
            self.advance_token();
            self.emitter.emit("while(");
            self.comparison();

            self.assert_and_advance_token(matches!(self.cur_token, Token::Repeat), Token::Repeat);
            self.nl();
            self.emitter.emit_line("){");

            while !matches!(self.cur_token, Token::Endwhile) {
                self.statement();
            }

            self.assert_and_advance_token(
                matches!(self.cur_token, Token::Endwhile),
                Token::Endwhile,
            );
            self.emitter.emit_line("}");
        } else if matches!(self.cur_token, Token::Label) {
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
            }

            self.labels_declared.insert(label_text.to_string());
            self.emitter.emit(&format!("{label_text}:"));
            self.assert_and_advance_token(matches!(self.cur_token, Token::Ident(_)), IDENT);
        } else if matches!(self.cur_token, Token::Goto) {
            self.advance_token();

            // should basically always match at this point
            let Token::Ident(label_text) = &self.cur_token else {
                panic!(
                    "unable to extract label text from GOTO: {:?}",
                    self.cur_token
                )
            };

            self.labels_gotoed.insert(label_text.to_string());
            self.emitter.emit(&format!("goto {label_text};"));
            self.assert_and_advance_token(matches!(self.cur_token, Token::Ident(_)), IDENT);
        } else if matches!(self.cur_token, Token::Let) {
            self.advance_token();

            let Token::Ident(ident_name) = &self.cur_token else {
                panic!(
                    "unable to extract label text from IDENT: {:?}",
                    self.cur_token
                )
            };
            if self.symbols.insert(ident_name.clone()) {
                self.emitter.header_line(&format!("float {ident_name};"));
            }

            self.emitter.emit(&format!("{ident_name} = "));
            self.assert_and_advance_token(matches!(self.cur_token, Token::Ident(_)), IDENT);
            self.assert_and_advance_token(matches!(self.cur_token, Token::Eq), Token::Eq);
            self.expression();
            self.emitter.emit_line(";");
        } else if matches!(self.cur_token, Token::Input) {
            self.advance_token();

            let Token::Ident(ident_name) = &self.cur_token else {
                panic!(
                    "unable to extract label text from INPUT: {:?}",
                    self.cur_token
                )
            };

            if self.symbols.insert(ident_name.clone()) {
                self.emitter.header_line(&format!("float {ident_name};"))
            }

            self.emitter
                .emit_line(&format!("if(0 == scanf(\"%f\", &{ident_name})) {{"));
            self.emitter.emit_line(&format!("{ident_name} = 0;"));
            self.emitter.emit_line("scanf(\"%*s\");");
            self.emitter.emit_line("}");

            self.assert_and_advance_token(matches!(self.cur_token, Token::Ident(_)), IDENT);
        } else {
            panic!("Invalid statement at {:?}", self.cur_token);
        }

        self.nl();
    }

    fn comparison(&mut self) {
        self.expression();
        // must be at least one comparison operator and another expression
        if self.is_comparison_operator() {
            self.emitter.emit(self.cur_token.as_str());
            self.advance_token();
            self.expression();
        } else {
            panic!("Expected comparison operator at {:?}", self.cur_token)
        }

        while self.is_comparison_operator() {
            self.emitter.emit(self.cur_token.as_str());
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
        self.term();

        while matches!(self.cur_token, Token::Plus | Token::Minus) {
            self.emitter.emit(self.cur_token.as_str());
            self.advance_token();
            self.term();
        }
    }

    fn term(&mut self) {
        self.unary();

        while matches!(self.cur_token, Token::Asterisk | Token::Slash) {
            self.emitter.emit(self.cur_token.as_str());
            self.advance_token();
            self.unary();
        }
    }

    fn unary(&mut self) {
        if matches!(self.cur_token, Token::Plus | Token::Minus) {
            self.emitter.emit(self.cur_token.as_str());
            self.advance_token();
        }
        self.primary();
    }

    fn primary(&mut self) {
        match &self.cur_token {
            Token::Number(num) => {
                self.emitter.emit(num);
                self.advance_token();
            }
            Token::Ident(ident) => {
                if !self.symbols.contains(ident) {
                    panic!("Referencing varible before assignment: {ident}");
                }
                self.emitter.emit(ident);
                self.advance_token();
            }
            _ => {
                panic!("Unexpectd token at {:?}", self.cur_token);
            }
        }
    }

    fn nl(&mut self) {
        self.assert_and_advance_token(matches!(self.cur_token, Token::Newline), Token::Newline);

        while matches!(self.cur_token, Token::Newline) {
            self.advance_token();
        }
    }
}
