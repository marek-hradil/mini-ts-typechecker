use crate::errors::LexingError;
use phf::phf_map;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Function,
    Var,
    Type,
    Return,
    Equals,
    NumericLiteral,
    StringLiteral,
    Identifier,
    Newline,
    Semicolon,
    Comma,
    Colon,
    Arrow,
    Whitespace,
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    LessThan,
    GreaterThan,
    Unknown,
    BOF,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub text: String,
    pub start: usize,
    pub end: usize,
}

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "function" => TokenType::Function,
    "var" => TokenType::Var,
    "type" => TokenType::Type,
    "return" => TokenType::Return,
};

pub struct Lexer {
    input: &'static str,
    pos: usize,
    current: Option<Result<Token, LexingError>>,
}

impl Lexer {
    pub fn new(input: &'static str) -> Lexer {
        Lexer {
            input,
            pos: 0,
            current: None,
        }
    }

    pub fn scan(&mut self) -> Option<Result<Token, LexingError>> {
        self.skip_whitespace();

        self.current = match self.get_current_char() {
            Some(current) => Some(self.scan_token(current)),
            None => Some(Ok(Token {
                token_type: TokenType::EOF,
                text: "".to_string(),
                start: self.pos,
                end: self.pos,
            })),
        };

        self.current.clone()
    }

    pub fn get(&self) -> Option<Token> {
        match self.current {
            Some(Ok(ref token)) => Some(token.clone()),
            _ => None,
        }
    }

    pub fn get_type(&self) -> Option<&TokenType> {
        match self.current {
            Some(Ok(ref token)) => Some(&token.token_type),
            _ => None,
        }
    }

    fn scan_token(&mut self, current: char) -> Result<Token, LexingError> {
        if current == '"' {
            self.scan_string_literal()
        } else if current.is_digit(10) {
            self.scan_numeric_literal()
        } else if current.is_alphabetic() || current == '_' {
            self.scan_identifier()
        } else {
            self.scan_operator()
        }
    }

    fn scan_string_literal(&mut self) -> Result<Token, LexingError> {
        let start = self.pos;
        self.pos += 1;

        while let Some(current) = self.get_current_char() {
            self.pos += 1;
            if current == '"' {
                break;
            }
        }

        if let None = self.get_current_char() {
            Err(LexingError::UnterminatedStringLiteralError)
        } else {
            Ok(Token {
                token_type: TokenType::StringLiteral,
                text: self.input[start..self.pos].to_string(),
                start: start,
                end: self.pos,
            })
        }
    }

    fn scan_numeric_literal(&mut self) -> Result<Token, LexingError> {
        let start = self.pos;
        self.pos += 1;

        while let Some(current) = self.get_current_char() {
            if !current.is_digit(10) {
                break;
            }

            self.pos += 1;
        }

        Ok(Token {
            token_type: TokenType::NumericLiteral,
            text: self.input[start..self.pos].to_string(),
            start,
            end: self.pos,
        })
    }

    fn scan_identifier(&mut self) -> Result<Token, LexingError> {
        let start = self.pos;
        self.pos += 1;

        while let Some(current) = self.get_current_char() {
            if !current.is_alphanumeric() && current != '_' {
                break;
            }
            self.pos += 1;
        }

        let text = &self.input[start..self.pos];
        let token_type = KEYWORDS.get(text).cloned().unwrap_or(TokenType::Identifier);

        Ok(Token {
            token_type,
            text: text.to_string(),
            start,
            end: self.pos,
        })
    }

    fn scan_operator(&mut self) -> Result<Token, LexingError> {
        let start = self.pos;
        let token = match self.get_current_char() {
            Some('=') => match self.get_next_char() {
                Some('>') => {
                    self.pos += 1;
                    Some(TokenType::Arrow)
                }
                _ => Some(TokenType::Equals),
            },
            Some(',') => Some(TokenType::Comma),
            Some(';') => Some(TokenType::Semicolon),
            Some(':') => Some(TokenType::Colon),
            Some('{') => Some(TokenType::OpenBrace),
            Some('}') => Some(TokenType::CloseBrace),
            Some('(') => Some(TokenType::OpenParen),
            Some(')') => Some(TokenType::CloseParen),
            Some('<') => Some(TokenType::LessThan),
            Some('>') => Some(TokenType::GreaterThan),
            _ => None,
        };

        self.pos += 1;

        if let None = token {
            Err(LexingError::UnexpectedEndOfFileError)
        } else {
            Ok(Token {
                token_type: token.unwrap(),
                text: self.input[self.pos - 1..self.pos].to_string(),
                start,
                end: self.pos,
            })
        }
    }

    fn get_current_char(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    fn get_next_char(&self) -> Option<char> {
        self.input.chars().nth(self.pos + 1)
    }

    fn skip_whitespace(&mut self) {
        self.pos += self
            .input
            .chars()
            .skip(self.pos)
            .take_while(|&c| c.is_whitespace())
            .count();
    }
}

impl Iterator for Lexer {
    type Item = Result<Token, LexingError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.scan()
    }
}
