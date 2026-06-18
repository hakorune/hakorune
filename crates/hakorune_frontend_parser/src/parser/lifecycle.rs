use crate::tokenizer::TokenType;

use super::ParseError;

pub const DIRECT_BIRTH_CALL_EXPECTED: &str =
    "direct receiver `birth(...)` calls are forbidden; `birth` is a constructor hook fired only by `new`; use `new Box(...)` for construction";

pub fn direct_birth_call_error(found: TokenType, line: usize) -> ParseError {
    ParseError::UnexpectedToken {
        found,
        expected: DIRECT_BIRTH_CALL_EXPECTED.to_string(),
        line,
    }
}
