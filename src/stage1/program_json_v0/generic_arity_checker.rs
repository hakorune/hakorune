use crate::ast::{ASTNode, FieldDecl, ParamDecl};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
struct TypeRef {
    name: String,
    args: Vec<TypeRef>,
}

pub(super) fn check_generic_arities(ast: &ASTNode) -> Result<(), String> {
    let arities = collect_generic_arities(ast);
    check_node(ast, &arities)
}

fn collect_generic_arities(ast: &ASTNode) -> BTreeMap<String, usize> {
    let mut arities = builtin_generic_arities();
    let ASTNode::Program { statements, .. } = ast else {
        return arities;
    };

    for statement in statements {
        match statement {
            ASTNode::BoxDeclaration {
                name,
                type_parameters,
                ..
            } => {
                arities.insert(name.clone(), type_parameters.len());
            }
            ASTNode::EnumDeclaration {
                name,
                type_parameters,
                ..
            } => {
                arities.insert(name.clone(), type_parameters.len());
            }
            _ => {}
        }
    }
    arities
}

fn builtin_generic_arities() -> BTreeMap<String, usize> {
    [
        ("Array", 1),
        ("PackedArray", 1),
        ("Span", 1),
        ("Option", 1),
        ("Result", 2),
    ]
    .into_iter()
    .map(|(name, arity)| (name.to_string(), arity))
    .collect()
}

fn check_node(node: &ASTNode, arities: &BTreeMap<String, usize>) -> Result<(), String> {
    match node {
        ASTNode::Program { statements, .. } => check_statements(statements, arities),
        ASTNode::BoxDeclaration {
            field_decls,
            methods,
            constructors,
            static_init,
            ..
        } => {
            check_field_decls(field_decls, arities)?;
            for method in methods.values() {
                check_node(method, arities)?;
            }
            for constructor in constructors.values() {
                check_node(constructor, arities)?;
            }
            if let Some(static_init) = static_init {
                check_statements(static_init, arities)?;
            }
            Ok(())
        }
        ASTNode::EnumDeclaration { variants, .. } => {
            for variant in variants {
                if let Some(payload_type_name) = variant.payload_type_name.as_deref() {
                    check_type_text(payload_type_name, arities)?;
                }
                for payload_type_name in &variant.tuple_payload_type_names {
                    check_type_text(payload_type_name, arities)?;
                }
                check_field_decls(&variant.record_field_decls, arities)?;
            }
            Ok(())
        }
        ASTNode::BrandDeclaration {
            underlying_type_name,
            ..
        } => check_type_text(underlying_type_name, arities),
        ASTNode::TypeAliasDeclaration {
            target_type_name, ..
        } => check_type_text(target_type_name, arities),
        ASTNode::FunctionDeclaration {
            params,
            param_decls,
            return_type_name,
            body,
            ..
        } => {
            for decl in ParamDecl::with_name_fallback(param_decls, params).iter() {
                if let Some(type_name) = decl.declared_type_name.as_deref() {
                    check_type_text(type_name, arities)?;
                }
            }
            if let Some(return_type_name) = return_type_name.as_deref() {
                check_type_text(return_type_name, arities)?;
            }
            check_statements(body, arities)
        }
        _ => Ok(()),
    }
}

fn check_statements(
    statements: &[ASTNode],
    arities: &BTreeMap<String, usize>,
) -> Result<(), String> {
    for statement in statements {
        check_node(statement, arities)?;
    }
    Ok(())
}

fn check_field_decls(
    field_decls: &[FieldDecl],
    arities: &BTreeMap<String, usize>,
) -> Result<(), String> {
    for decl in field_decls {
        if let Some(type_name) = decl.declared_type_name.as_deref() {
            check_type_text(type_name, arities)?;
        }
    }
    Ok(())
}

fn check_type_text(type_text: &str, arities: &BTreeMap<String, usize>) -> Result<(), String> {
    let mut parser = TypeRefParser::new(type_text);
    let type_ref = parser.parse_type_ref()?;
    parser.finish()?;
    check_type_ref(&type_ref, arities)
}

fn check_type_ref(type_ref: &TypeRef, arities: &BTreeMap<String, usize>) -> Result<(), String> {
    if let Some(expected) = arities.get(&type_ref.name) {
        let actual = type_ref.args.len();
        if actual != *expected {
            return Err(format!(
                "[generic/arity] type={} expected={} actual={}",
                type_ref.name, expected, actual
            ));
        }
    }
    for arg in &type_ref.args {
        check_type_ref(arg, arities)?;
    }
    Ok(())
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
