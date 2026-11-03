//! Shared, parser-independent type-reference syntax. No ownership or ABI inference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeReferenceV1 {
    Name(String),
    Apply { name: String, args: Vec<TypeReferenceV1> },
    Array(Box<TypeReferenceV1>),
    Nullable(Box<TypeReferenceV1>),
}

impl TypeReferenceV1 {
    pub fn parse(text: &str) -> Result<Self, String> {
        let mut parser = TypeRefParser::new(text);
        let reference = parser.parse_type_ref()?;
        parser.finish()?;
        Ok(reference)
    }

    pub fn name(&self) -> Option<&str> {
        match self { Self::Name(name) => Some(name), _ => None }
    }
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

    fn parse_type_ref(&mut self) -> Result<TypeReferenceV1, String> {
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
        let mut reference = if args.is_empty() { TypeReferenceV1::Name(name) }
            else { TypeReferenceV1::Apply { name, args } };
        loop {
            if self.consume_str("[]") { reference = TypeReferenceV1::Array(Box::new(reference)); }
            else if self.consume_byte(b'?') { reference = TypeReferenceV1::Nullable(Box::new(reference)); }
            else { break; }
            self.skip_ws();
        }
        Ok(reference)
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
        let mut chars = self.text[self.pos..].char_indices();
        let Some((_, first)) = chars.next() else {
            return Err(format!("[generic/type-ref] expected type name in `{}`", self.text));
        };
        if first != '_' && !first.is_alphabetic() {
            return Err(format!("[generic/type-ref] expected type name in `{}`", self.text));
        }
        self.pos += first.len_utf8();
        for (_, ch) in chars {
            if ch != '_' && !ch.is_alphanumeric() { break; }
            self.pos += ch.len_utf8();
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

