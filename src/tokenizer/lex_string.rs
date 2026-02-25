use super::{NyashTokenizer, TokenizeError};

impl NyashTokenizer {
    /// 文字列リテラルを読み取り（区切り文字 quote を指定可: '"' or '\''）
    fn read_string_with_quote(&mut self, quote: char) -> Result<String, TokenizeError> {
        let start_line = self.line;
        // 開始の quote をスキップ
        self.advance();

        let mut string_value = String::new();

        while let Some(c) = self.current_char() {
            if c == quote {
                self.advance(); // 終了の quote をスキップ
                return Ok(string_value);
            }

            // エスケープ文字の処理
            if c == '\\' {
                self.advance();
                match self.current_char() {
                    Some('n') => string_value.push('\n'),
                    Some('t') => string_value.push('\t'),
                    Some('r') => string_value.push('\r'),
                    Some('b') => string_value.push('\u{0008}'), // backspace
                    Some('f') => string_value.push('\u{000C}'), // form feed
                    Some('\\') => string_value.push('\\'),
                    Some('"') => string_value.push('"'),
                    Some('\'') => string_value.push('\''), // 1-quote: エスケープされたシングルクォート
                    Some('/') => string_value.push('/'),   // \/ を許容
                    Some('u') => {
                        // Unicode decode (optional; default OFF)
                        if crate::config::env::parser_decode_unicode() {
                            let base = self.position; // index of 'u'
                                                      // read 4 hex digits without consuming; then advance position in bulk
                            let read_hex4 = |input: &Vec<char>, start: usize| -> Option<u32> {
                                if start + 4 > input.len() {
                                    return None;
                                }
                                let d0 = input.get(start)?.to_digit(16)?;
                                let d1 = input.get(start + 1)?.to_digit(16)?;
                                let d2 = input.get(start + 2)?.to_digit(16)?;
                                let d3 = input.get(start + 3)?.to_digit(16)?;
                                Some((d0 << 12) | (d1 << 8) | (d2 << 4) | d3)
                            };
                            let first_start = base + 1; // after 'u'
                            if let Some(u1) = read_hex4(&self.input, first_start) {
                                // consume 'u' + 4 hex
                                self.position = base + 5;
                                let mut out_char: Option<char> = None;
                                // surrogate pair
                                if (0xD800..=0xDBFF).contains(&u1) {
                                    if self.position + 6 <= self.input.len()
                                        && self.input.get(self.position) == Some(&'\\')
                                        && self.input.get(self.position + 1) == Some(&'u')
                                    {
                                        if let Some(u2) = read_hex4(&self.input, self.position + 2)
                                        {
                                            if (0xDC00..=0xDFFF).contains(&u2) {
                                                let high_ten = (u1 - 0xD800) as u32;
                                                let low_ten = (u2 - 0xDC00) as u32;
                                                let scalar = 0x10000 + ((high_ten << 10) | low_ten);
                                                out_char = std::char::from_u32(scalar);
                                                // consume '\\u' + 4 hex of low surrogate
                                                self.position += 6;
                                            }
                                        }
                                    }
                                }
                                if out_char.is_none() {
                                    out_char = std::char::from_u32(u1 as u32);
                                }
                                if let Some(ch) = out_char {
                                    string_value.push(ch);
                                    // Skip the generic advance at loop end to avoid double step
                                    continue;
                                } else {
                                    // Fallback to literal when invalid
                                    string_value.push('\\');
                                    string_value.push('u');
                                    continue;
                                }
                            } else {
                                // Not enough hex digits; keep literal
                                string_value.push('\\');
                                string_value.push('u');
                            }
                        } else {
                            // Decoding disabled → keep literal
                            string_value.push('\\');
                            string_value.push('u');
                        }
                    }
                    // TODO: 将来 `\uXXXX` デコード（既定OFF）
                    Some(c2) => {
                        // 未知のエスケープはそのまま残す（互換性維持）
                        string_value.push('\\');
                        string_value.push(c2);
                    }
                    None => break,
                }
            } else {
                string_value.push(c);
            }

            self.advance();
        }

        Err(TokenizeError::UnterminatedString { line: start_line })
    }

    /// 既存互換: ダブルクォート専用のリーダ（内部で read_string_with_quote を呼ぶ）
    pub(crate) fn read_string(&mut self) -> Result<String, TokenizeError> {
        self.read_string_with_quote('"')
    }

    /// シングルクォート文字列の読み取り（Stage‑3 の文法拡張）
    pub(crate) fn read_single_quoted_string(&mut self) -> Result<String, TokenizeError> {
        self.read_string_with_quote('\'')
    }
}
