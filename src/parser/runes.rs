use crate::ast::{ASTNode, DeclarationAttrs, RuneAttr};
use crate::parser::common::ParserUtils;
use crate::parser::{NyashParser, ParseError};
use crate::tokenizer::TokenType;
use std::collections::BTreeSet;

enum RuneTarget {
    Box,
    Function,
}

impl NyashParser {
    pub(super) fn take_pending_runes_for_box(&mut self) -> Result<DeclarationAttrs, ParseError> {
        self.take_pending_runes_for_target(RuneTarget::Box)
    }

    pub(super) fn take_pending_runes_for_function(
        &mut self,
    ) -> Result<DeclarationAttrs, ParseError> {
        self.take_pending_runes_for_target(RuneTarget::Function)
    }

    pub(super) fn attach_pending_runes_to_declaration(
        &mut self,
        node: &mut ASTNode,
    ) -> Result<(), ParseError> {
        if self.pending_runes.is_empty() {
            return Ok(());
        }

        let line = self.current_token().line;
        let runes = std::mem::take(&mut self.pending_runes);

        match node {
            ASTNode::BoxDeclaration { attrs, .. } => {
                validate_runes_for_target(&runes, RuneTarget::Box, line)?;
                self.rune_metadata.extend(runes.iter().cloned());
                attrs.runes = runes;
                Ok(())
            }
            ASTNode::FunctionDeclaration { attrs, .. } => {
                validate_runes_for_target(&runes, RuneTarget::Function, line)?;
                self.rune_metadata.extend(runes.iter().cloned());
                attrs.runes = runes;
                Ok(())
            }
            _ => Err(self.rune_error(
                "[freeze:contract][parser/rune] declaration required after @rune",
                line,
            )),
        }
    }

    pub(super) fn ensure_no_pending_runes(&self, context: &str) -> Result<(), ParseError> {
        if self.pending_runes.is_empty() {
            return Ok(());
        }
        Err(self.rune_error(
            format!("[freeze:contract][parser/rune] invalid placement on {}", context),
            self.current_token().line,
        ))
    }

    pub(super) fn rune_error(&self, expected: impl Into<String>, line: usize) -> ParseError {
        ParseError::UnexpectedToken {
            found: self.current_token().token_type.clone(),
            expected: expected.into(),
            line,
        }
    }

    fn take_pending_runes_for_target(
        &mut self,
        target: RuneTarget,
    ) -> Result<DeclarationAttrs, ParseError> {
        if self.pending_runes.is_empty() {
            return Ok(DeclarationAttrs::default());
        }

        let line = self.current_token().line;
        let runes = std::mem::take(&mut self.pending_runes);
        validate_runes_for_target(&runes, target, line)?;
        self.rune_metadata.extend(runes.iter().cloned());
        Ok(DeclarationAttrs { runes })
    }
}

fn validate_runes_for_target(
    runes: &[RuneAttr],
    target: RuneTarget,
    line: usize,
) -> Result<(), ParseError> {
    let mut seen = BTreeSet::new();
    let mut visibility: Option<&str> = None;

    for rune in runes {
        if !seen.insert(rune.name.clone()) {
            return Err(ParseError::UnexpectedToken {
                found: TokenType::IDENTIFIER(rune.name.clone()),
                expected: format!(
                    "[freeze:contract][parser/rune] duplicate rune {}",
                    rune.name
                ),
                line,
            });
        }

        if matches!(rune.name.as_str(), "Public" | "Internal") {
            if let Some(prev) = visibility {
                if prev != rune.name {
                    return Err(ParseError::UnexpectedToken {
                        found: TokenType::IDENTIFIER(rune.name.clone()),
                        expected:
                            "[freeze:contract][parser/rune] conflicting visibility runes"
                                .to_string(),
                        line,
                    });
                }
            }
            visibility = Some(&rune.name);
        }

        if matches!(target, RuneTarget::Box)
            && !matches!(rune.name.as_str(), "Public" | "Internal")
        {
            return Err(ParseError::UnexpectedToken {
                found: TokenType::IDENTIFIER(rune.name.clone()),
                expected: "[freeze:contract][parser/rune] box target supports only Public|Internal"
                    .to_string(),
                line,
            });
        }
    }

    Ok(())
}
