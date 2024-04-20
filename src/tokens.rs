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
// pub const STR: Token = Token::String(String::new());
/// singleton to prevent re-initializing strings for matching
pub const IDENT: Token = Token::Ident(String::new());
/// singleton to prevent re-initializing strings for matching
// pub const NUM: Token = Token::Number(String::new());

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
    pub fn as_str(&self) -> &str {
        match self {
            Self::Gt => ">",
            Self::Lt => "<",
            Self::Eq => "=",
            Self::Eqeq => "==",
            Self::Gteq => ">=",
            Self::Lteq => "<=",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Asterisk => "*",
            Self::Slash => "/",
            _ => panic!("NO STRING REPRESENTATION FOR {:?}", self),
        }
    }
}
