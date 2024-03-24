#[derive(Debug, Clone, PartialEq)]
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

/// singleton to prevent re-initializing strings for matching
pub const STR: Token = Token::String(String::new());
/// singleton to prevent re-initializing strings for matching
pub const IDENT: Token = Token::Ident(String::new());
/// singleton to prevent re-initializing strings for matching
pub const NUM: Token = Token::Number(String::new());

impl Token {
    pub fn is_keyword(k: &str) -> Option<Token> {
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
