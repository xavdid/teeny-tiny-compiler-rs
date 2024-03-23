use std::{iter::Peekable, str::Chars};

const EOF: char = '\0';

pub struct Lexer<'a> {
    // source: String,
    cur_char: char,
    // cursor_position: i32,
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        // pretty insistently can't add a newline here
        // let source_with_newline: &'a str = format!("{}\n", source).as_str();

        let mut l = Lexer {
            cur_char: ' ',
            chars: source.chars().peekable(),
        };
        l.advance_pointer();
        l
    }

    pub fn advance_pointer(&mut self) {
        // let old = self.cur_char;
        self.cur_char = match self.chars.next() {
            Some(c) => c,
            None => EOF,
        };
        // dbg!("advanced from `{old}` to `{}`", self.cur_char);
    }

    fn skip_whitespace(&mut self) {
        let c = self.cur_char;
        // there's skip_whitespace, but i'm not sure it'll play well with bounds
        // also, it moves self.chars, which isn't allowed
        if c == ' ' || c == '\t' || c == '\r' {
            self.advance_pointer()
        }
    }

    fn skip_comment(&mut self) {
        if self.cur_char == '#' {
            // TODO: if the source doesn't end with a newline, then this loops indefinitely
            // - could also check: `&& self.cur_char != EOF` to fix
            while self.cur_char != '\n' {
                self.advance_pointer();
            }
        }
    }

    pub fn get_next_token(&mut self) -> Token {
        self.skip_whitespace();
        self.skip_comment();
        let result: Token = match self.cur_char {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '=' => match self.chars.peek() {
                Some('=') => {
                    // consume the second eq
                    self.advance_pointer();
                    Token::Eqeq
                }
                Some(_) => Token::Eq,
                None => Token::Eof,
            },
            '>' => match self.chars.peek() {
                Some('=') => {
                    // consume the second eq
                    self.advance_pointer();
                    Token::Gteq
                }
                Some(_) => Token::Gt,
                None => Token::Eof,
            },
            '<' => match self.chars.peek() {
                Some('=') => {
                    // consume the second eq
                    self.advance_pointer();
                    Token::Lteq
                }
                Some(_) => Token::Lt,
                None => Token::Eof,
            },
            '!' => match self.chars.peek() {
                Some('=') => {
                    // consume the second eq
                    self.advance_pointer();
                    Token::Noteq
                }
                _ => panic!("expected !=, got !"),
            },
            '"' => {
                self.advance_pointer();
                let mut s = String::new();

                while self.cur_char != '"' {
                    if self.cur_char == '\r'
                        || self.cur_char == '\n'
                        || self.cur_char == '\t'
                        || self.cur_char == '\\'
                        || self.cur_char == '%'
                    {
                        panic!("illegal character in string: `{}`", self.cur_char)
                    }

                    s.push(self.cur_char);
                    self.advance_pointer();
                }
                Token::String(s)
            }
            c if c.is_ascii_digit() => {
                // leading character is a digit, so this must be a number
                // get all consecutive digits
                let mut s = String::from(c);
                while self.chars.peek().unwrap_or(&EOF).is_ascii_digit() {
                    self.advance_pointer();
                    s.push(self.cur_char);
                }

                if self.chars.peek().unwrap_or(&EOF) == &'.' {
                    self.advance_pointer();
                    s.push(self.cur_char); // a decimal

                    // 1+ numbers must follow
                    if !self.chars.peek().unwrap_or(&EOF).is_ascii_digit() {
                        panic!(
                            "expected number after a decimal, got {}",
                            self.chars.peek().unwrap_or(&EOF)
                        );
                    };

                    // have at least 1 number, read them all
                    while self.chars.peek().unwrap_or(&EOF).is_ascii_digit() {
                        self.advance_pointer();
                        s.push(self.cur_char);
                    }
                }

                Token::Number(s)
            }
            c if c.is_alphabetic() => {
                // identifier or keyword
                let mut s = String::from(c);

                while self.chars.peek().unwrap_or(&EOF).is_alphanumeric() {
                    self.advance_pointer();
                    s.push(self.cur_char);
                }

                match Token::is_keyword(&s) {
                    Some(t) => t,
                    None => Token::Ident(s),
                }
            }
            '\n' => Token::Newline,
            EOF => Token::Eof,
            _ => panic!("lexing error! uknown token: `{}`", self.cur_char),
        };

        self.advance_pointer();

        result
    }
}

#[derive(Debug)]
pub enum Token {
    Eof,
    Newline,
    Number(String),
    Ident(String),
    String(String),
    Label,
    Goto,
    Print,
    Input,
    Let,
    If,
    Then,
    Endif,
    While,
    Repeat,
    Endwhile,
    Eq,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Eqeq,
    Noteq,
    Lt,
    Lteq,
    Gt,
    Gteq,
}

impl Token {
    fn is_keyword(k: &str) -> Option<Token> {
        match k {
            "LABEL" => Some(Token::Label),
            "GOTO" => Some(Token::Goto),
            "PRINT" => Some(Token::Print),
            "INPUT" => Some(Token::Input),
            "LET" => Some(Token::Let),
            "IF" => Some(Token::If),
            "THEN" => Some(Token::Then),
            "ENDIF" => Some(Token::Endif),
            "WHILE" => Some(Token::While),
            "REPEAT" => Some(Token::Repeat),
            "ENDWHILE" => Some(Token::Endwhile),
            _ => None,
        }
    }
    fn value(&self) -> i32 {
        match self {
            Self::Eof => -1,
            Self::Newline => 0,
            Self::Number(_) => 1,
            Self::Ident(_) => 2,
            Self::String(_) => 3,
            //  Keywords
            Self::Label => 101,
            Self::Goto => 102,
            Self::Print => 103,
            Self::Input => 104,
            Self::Let => 105,
            Self::If => 106,
            Self::Then => 107,
            Self::Endif => 108,
            Self::While => 109,
            Self::Repeat => 110,
            Self::Endwhile => 111,
            // Operators
            Self::Eq => 201,
            Self::Plus => 202,
            Self::Minus => 203,
            Self::Asterisk => 204,
            Self::Slash => 205,
            Self::Eqeq => 206,
            Self::Noteq => 207,
            Self::Lt => 208,
            Self::Lteq => 209,
            Self::Gt => 210,
            Self::Gteq => 211,
        }
    }
}
