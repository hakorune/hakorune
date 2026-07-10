//! Migration-only recognition for legacy `from` syntax.
//!
//! This module produces evidence records, never semantic AST nodes. The main
//! parser uses the same token classifier to reject source `from` before AST
//! publication.

use hakorune_frontend_grammar::contract::{GrammarProfile, ParseWitness};

use crate::parser::{ParseError, ParserBuildConfig};
use crate::tokenizer::{NyashTokenizer, Token, TokenType};

pub const BOX_FROM_ROW_ID: &str = "box_from_inheritance";
pub const FROM_CALL_ROW_ID: &str = "from_super_call";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MigrationTransportKind {
    BoxFromInheritance,
    FromCall,
}

impl MigrationTransportKind {
    pub const fn row_id(self) -> &'static str {
        match self {
            Self::BoxFromInheritance => BOX_FROM_ROW_ID,
            Self::FromCall => FROM_CALL_ROW_ID,
        }
    }

    pub const fn spelling_id(self) -> &'static str {
        match self {
            Self::BoxFromInheritance => "box_from_inheritance",
            Self::FromCall => "from_super_call",
        }
    }

    pub const fn transport_id(self) -> &'static str {
        match self {
            Self::BoxFromInheritance => "migration-transport/box-from-inheritance/v0",
            Self::FromCall => "migration-transport/from-call/v0",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationTransport {
    pub transport_id: String,
    pub row_id: String,
    pub profile: GrammarProfile,
    pub transport_kind: MigrationTransportKind,
    pub spelling_id: String,
    pub semantic_entry_allowed: bool,
    pub ast_publication_allowed: bool,
    pub mir_lowering_allowed: bool,
    pub runtime_lowering_allowed: bool,
    pub backend_lowering_allowed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationTransportBundle {
    pub witness: ParseWitness,
    pub transport: MigrationTransport,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FromTransportClassification {
    Closed(MigrationTransportKind),
    NotClosed(MigrationTransportKind),
}

pub fn classify_from_transport_tokens(tokens: &[Token]) -> Option<FromTransportClassification> {
    let kinds = tokens
        .iter()
        .map(|token| &token.token_type)
        .collect::<Vec<_>>();
    match kinds.as_slice() {
        [TokenType::BOX, TokenType::IDENTIFIER(_), TokenType::FROM, TokenType::IDENTIFIER(_), TokenType::LBRACE, TokenType::RBRACE, ..] => {
            Some(FromTransportClassification::Closed(
                MigrationTransportKind::BoxFromInheritance,
            ))
        }
        [TokenType::BOX, TokenType::IDENTIFIER(_), TokenType::FROM, ..] => Some(
            FromTransportClassification::NotClosed(MigrationTransportKind::BoxFromInheritance),
        ),
        [TokenType::FROM, TokenType::IDENTIFIER(_), TokenType::DOT, TokenType::IDENTIFIER(_), TokenType::LPAREN, TokenType::RPAREN, ..] => {
            Some(FromTransportClassification::Closed(
                MigrationTransportKind::FromCall,
            ))
        }
        [TokenType::FROM, ..] => Some(FromTransportClassification::NotClosed(
            MigrationTransportKind::FromCall,
        )),
        _ => None,
    }
}

pub fn parse_migration_transport_with_config(
    source: impl Into<String>,
    config: ParserBuildConfig,
) -> Result<MigrationTransportBundle, ParseError> {
    if config.grammar_profile != GrammarProfile::Compat2025 {
        return Err(ParseError::MigrationTransport {
            stable_reject_tag: "parser/profile_required_for_compat",
            line: 1,
        });
    }
    let mut tokenizer = NyashTokenizer::with_grammar_profile(source, config.grammar_profile);
    let tokens = tokenizer.tokenize()?;
    let classification =
        classify_from_transport_tokens(&tokens).ok_or(ParseError::MigrationTransport {
            stable_reject_tag: "parser/from_transport_not_closed_form",
            line: 1,
        })?;
    let FromTransportClassification::Closed(kind) = classification else {
        return Err(ParseError::MigrationTransport {
            stable_reject_tag: "parser/from_transport_not_closed_form",
            line: 1,
        });
    };
    let transport = MigrationTransport {
        transport_id: kind.transport_id().to_string(),
        row_id: kind.row_id().to_string(),
        profile: GrammarProfile::Compat2025,
        transport_kind: kind,
        spelling_id: kind.spelling_id().to_string(),
        semantic_entry_allowed: false,
        ast_publication_allowed: false,
        mir_lowering_allowed: false,
        runtime_lowering_allowed: false,
        backend_lowering_allowed: false,
    };
    let witness = ParseWitness::accepted_transport(
        kind.row_id(),
        GrammarProfile::Compat2025,
        transport.transport_id.clone(),
    );
    Ok(MigrationTransportBundle { witness, transport })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compat_config() -> ParserBuildConfig {
        ParserBuildConfig {
            grammar_profile: GrammarProfile::Compat2025,
            ..ParserBuildConfig::default()
        }
    }

    #[test]
    fn adapter_emits_box_from_transport_without_semantic_permissions() {
        let bundle =
            parse_migration_transport_with_config("box Child from Parent {}", compat_config())
                .expect("closed box-from transport");
        assert_eq!(
            bundle.transport.transport_kind,
            MigrationTransportKind::BoxFromInheritance
        );
        assert_eq!(bundle.witness.normalized_kind, "CompatibilityTransport");
        assert_eq!(
            bundle.witness.migration_transport_ref.as_deref(),
            Some(bundle.transport.transport_id.as_str())
        );
        assert!(!bundle.transport.ast_publication_allowed);
        assert!(!bundle.transport.mir_lowering_allowed);
    }

    #[test]
    fn adapter_requires_explicit_compat2025() {
        let error = parse_migration_transport_with_config(
            "from Parent.method()",
            ParserBuildConfig::default(),
        )
        .expect_err("Canonical migration adapter entry must reject");
        assert!(format!("{error:?}").contains("parser/profile_required_for_compat"));
    }
}
