use crate::parser::ParseError;
use crate::tokenizer::TokenType;

pub(crate) const DIRECT_BIRTH_CALL_EXPECTED: &str =
    "direct source birth calls are forbidden; birth is a constructor hook";

pub(crate) fn direct_birth_call_error(found: TokenType, line: usize) -> ParseError {
    ParseError::UnexpectedToken {
        found,
        expected: DIRECT_BIRTH_CALL_EXPECTED.to_string(),
        line,
    }
}
