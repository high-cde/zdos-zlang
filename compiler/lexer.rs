#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Func,
    If,
    Else,
    For,
    In,
    While,
    Return,
    Import,
    Module,
    Throw,
    Try,
    Catch,
    Identifier(String),
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,
    EqEq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
    Not,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Colon,
    Dot,
    Arrow,
    Range,
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.pos >= self.input.len() {
            return Token::EOF;
        }

        let ch = self.input[self.pos];
        match ch {
            '+' => {
                self.pos += 1;
                Token::Plus
            }
            '-' => {
                self.pos += 1;
                if self.pos < self.input.len() && self.input[self.pos] == '>' {
                    self.pos += 1;
                    Token::Arrow
                } else {
                    Token::Minus
                }
            }
            '*' => {
                self.pos += 1;
                Token::Star
            }
            '/' => {
                self.pos += 1;
                Token::Slash
            }
            '=' => {
                self.pos += 1;
                if self.pos < self.input.len() && self.input[self.pos] == '=' {
                    self.pos += 1;
                    Token::EqEq
                } else {
                    Token::Eq
                }
            }
            '(' => {
                self.pos += 1;
                Token::LParen
            }
            ')' => {
                self.pos += 1;
                Token::RParen
            }
            '{' => {
                self.pos += 1;
                Token::LBrace
            }
            '}' => {
                self.pos += 1;
                Token::RBrace
            }
            '[' => {
                self.pos += 1;
                Token::LBracket
            }
            ']' => {
                self.pos += 1;
                Token::RBracket
            }
            ',' => {
                self.pos += 1;
                Token::Comma
            }
            ':' => {
                self.pos += 1;
                Token::Colon
            }
            '.' => {
                self.pos += 1;
                if self.pos < self.input.len() && self.input[self.pos] == '.' {
                    self.pos += 1;
                    Token::Range
                } else {
                    Token::Dot
                }
            }
            '"' => self.read_string(),
            _ => {
                if ch.is_alphabetic() {
                    self.read_identifier()
                } else if ch.is_numeric() {
                    self.read_number()
                } else {
                    panic!("Unexpected character: {}", ch);
                }
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len()
            && (self.input[self.pos].is_alphanumeric() || self.input[self.pos] == '_')
        {
            self.pos += 1;
        }
        let s: String = self.input[start..self.pos].iter().collect();
        match s.as_str() {
            "let" => Token::Let,
            "func" => Token::Func,
            "if" => Token::If,
            "else" => Token::Else,
            "for" => Token::For,
            "in" => Token::In,
            "while" => Token::While,
            "return" => Token::Return,
            "import" => Token::Import,
            "module" => Token::Module,
            "throw" => Token::Throw,
            "try" => Token::Try,
            "catch" => Token::Catch,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            _ => Token::Identifier(s),
        }
    }

    fn read_number(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos].is_numeric() {
            self.pos += 1;
        }
        if self.pos < self.input.len() && self.input[self.pos] == '.' {
            self.pos += 1;
            while self.pos < self.input.len() && self.input[self.pos].is_numeric() {
                self.pos += 1;
            }
            let s: String = self.input[start..self.pos].iter().collect();
            Token::Float(s.parse().unwrap())
        } else {
            let s: String = self.input[start..self.pos].iter().collect();
            Token::Int(s.parse().unwrap())
        }
    }

    fn read_string(&mut self) -> Token {
        self.pos += 1; // skip "
        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos] != '"' {
            self.pos += 1;
        }
        let s: String = self.input[start..self.pos].iter().collect();
        self.pos += 1; // skip "
        Token::Str(s)
    }
}
