use crate::errors::ParsingError;
use crate::lexer::{Lexer, TokenType};
use crate::types::module::Module;

pub fn parse(lexer: &mut Lexer) -> Result<Module, ParsingError> {
    lexer.next();
    Module::parse(lexer)
}

pub fn parse_sequence<T>(
    lexer: &mut Lexer,
    parse_element: fn(&mut Lexer) -> Result<T, ParsingError>,
    separator: TokenType,
    terminator: TokenType,
) -> Result<Vec<T>, ParsingError> {
    let mut seq = Vec::new();
    while !try_consume_token(lexer, &terminator) {
        match parse_element(lexer) {
            Ok(element) => seq.push(element),
            Err(err) => return Err(err),
        };

        try_consume_token(lexer, &separator);
    }

    Ok(seq)
}

pub fn try_consume_token(lexer: &mut Lexer, expected: &TokenType) -> bool {
    let ok = match lexer.get_type() {
        Some(token_type) => token_type == expected,
        _ => false,
    };

    if ok {
        lexer.next();
    }

    ok
}

pub fn try_parse_prefixed<T>(
    lexer: &mut Lexer,
    parse_element: fn(&mut Lexer) -> Result<T, ParsingError>,
    prefix: TokenType,
) -> Option<T> {
    if try_consume_token(lexer, &prefix) {
        match parse_element(lexer) {
            Ok(element) => Some(element),
            Err(_) => None,
        }
    } else {
        None
    }
}

pub fn parse_expected(lexer: &mut Lexer, expected_type: TokenType) -> Result<(), ParsingError> {
    match lexer.get_type() {
        Some(token_type) if token_type == &expected_type => Ok({
            lexer.next();
        }),
        _ => Err(ParsingError::UnexpectedEndOfFileError),
    }
}
