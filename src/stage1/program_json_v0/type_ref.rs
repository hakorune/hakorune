#[derive(Debug, Clone)]
pub(super) struct TypeRef {
    pub name: String,
    pub args: Vec<TypeRef>,
}

pub(super) fn parse_type_ref_text(type_text: &str) -> Result<TypeRef, String> {
    let mut parser = TypeRefParser::new(type_text);
    let type_ref = parser.parse_type_ref()?;
    parser.finish()?;
    Ok(type_ref)
}

struct TypeRefParser<'a> {
    text: &'a str,
    pos: usize,
}

impl<'a> TypeRefParser<'a> {
    fn new(text: &'a str) -> Self {
        Self { text, pos: 0 }
    }

    fn finish(&mut self) -> Result<(), String> {
        self.skip_ws();
        if self.pos == self.text.len() {
            Ok(())
        } else {
            Err(format!(
                "[generic/type-ref] unexpected trailing text in `{}`",
                self.text
            ))
        }
    }

    fn parse_type_ref(&mut self) -> Result<TypeRef, String> {
        self.skip_ws();
        let name = self.parse_type_path()?;
        self.skip_ws();

        let args = if self.consume_byte(b'<') {
            let mut args = Vec::new();
            loop {
                args.push(self.parse_type_ref()?);
                self.skip_ws();
                if self.consume_byte(b',') {
                    continue;
                }
                if self.consume_byte(b'>') {
                    break;
                }
                return Err(format!(
                    "[generic/type-ref] expected `,` or `>` in `{}`",
                    self.text
                ));
            }
            args
        } else {
            Vec::new()
        };

        self.skip_ws();
        while self.consume_str("[]") {
            self.skip_ws();
        }

        Ok(TypeRef { name, args })
    }

    fn parse_type_path(&mut self) -> Result<String, String> {
        let mut name = self.parse_ident()?;
        loop {
            self.skip_ws();
            if !self.consume_byte(b'.') {
                break;
            }
            self.skip_ws();
            name.push('.');
            name.push_str(&self.parse_ident()?);
        }
        Ok(name)
    }

    fn parse_ident(&mut self) -> Result<String, String> {
        self.skip_ws();
        let start = self.pos;
        let Some(first) = self.peek_byte() else {
            return Err(format!(
                "[generic/type-ref] expected type name in `{}`",
                self.text
            ));
        };
        if !is_ident_start(first) {
            return Err(format!(
                "[generic/type-ref] expected type name in `{}`",
                self.text
            ));
        }
        self.pos += 1;
        while let Some(byte) = self.peek_byte() {
            if !is_ident_continue(byte) {
                break;
            }
            self.pos += 1;
        }
        Ok(self.text[start..self.pos].to_string())
    }

    fn skip_ws(&mut self) {
        while let Some(byte) = self.peek_byte() {
            if !byte.is_ascii_whitespace() {
                break;
            }
            self.pos += 1;
        }
    }

    fn consume_str(&mut self, value: &str) -> bool {
        if self.text[self.pos..].starts_with(value) {
            self.pos += value.len();
            true
        } else {
            false
        }
    }

    fn consume_byte(&mut self, value: u8) -> bool {
        if self.peek_byte() == Some(value) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn peek_byte(&self) -> Option<u8> {
        self.text.as_bytes().get(self.pos).copied()
    }
}

fn is_ident_start(byte: u8) -> bool {
    byte == b'_' || byte.is_ascii_alphabetic()
}

fn is_ident_continue(byte: u8) -> bool {
    byte == b'_' || byte.is_ascii_alphanumeric()
}
