use crate::errors::ParsingError;
use crate::lexer::Lexer;
use crate::parser::parse_identifier;
use crate::types::Location;

#[derive(Debug)]
pub struct TypeParameter {
    location: Location,
    name: String,
}

impl TypeParameter {
    pub fn parse(lexer: &mut Lexer) -> Result<TypeParameter, ParsingError> {
        let name = parse_identifier(lexer)?;

        Ok(TypeParameter {
            name,
            location: Default::default(),
        })
    }
}
