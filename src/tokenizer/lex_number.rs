use super::{NyashTokenizer, TokenType, TokenizeError};

impl NyashTokenizer {
    /// 数値リテラル（整数または浮動小数点数）を読み取り
    pub(crate) fn read_numeric_literal(&mut self) -> Result<TokenType, TokenizeError> {
        let start_line = self.line;
        let mut number_str = String::new();
        let mut has_dot = false;

        // 整数部分を読み取り
        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                number_str.push(c);
                self.advance();
            } else if c == '.'
                && !has_dot
                && self.peek_char().map_or(false, |ch| ch.is_ascii_digit())
            {
                // 小数点の後に数字が続く場合のみ受け入れる
                has_dot = true;
                number_str.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if has_dot {
            // 浮動小数点数として解析
            number_str
                .parse::<f64>()
                .map(TokenType::FLOAT)
                .map_err(|_| TokenizeError::InvalidNumber { line: start_line })
        } else {
            let mut suffix = String::new();
            while let Some(c) = self.current_char() {
                if c.is_ascii_alphanumeric() || c == '_' {
                    suffix.push(c);
                    self.advance();
                } else {
                    break;
                }
            }

            // 整数として解析
            let value = number_str
                .parse::<i64>()
                .map_err(|_| TokenizeError::InvalidNumber { line: start_line })?;
            if suffix.is_empty() {
                Ok(TokenType::NUMBER(value))
            } else {
                Ok(TokenType::TypedNumber(value, suffix))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_integer_literal_suffix() {
        let mut tokenizer = NyashTokenizer::new("0usize");

        assert_eq!(
            tokenizer.read_numeric_literal().unwrap(),
            TokenType::TypedNumber(0, "usize".to_string())
        );
    }

    #[test]
    fn leaves_float_suffix_out_of_numeric_literal() {
        let mut tokenizer = NyashTokenizer::new("1.0usize");

        assert_eq!(
            tokenizer.read_numeric_literal().unwrap(),
            TokenType::FLOAT(1.0)
        );
    }
}
