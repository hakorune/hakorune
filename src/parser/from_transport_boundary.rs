//! Semantic-entry boundary for legacy `from` transport syntax.
//!
//! Migration tooling may preserve the two closed legacy forms as evidence.
//! The semantic parser must reject them before publishing an AST node.

use hakorune_frontend_parser::migration_transport::{
    classify_from_transport_tokens, FromTransportClassification, MigrationTransportKind,
};

use crate::parser::common::ParserUtils;
use crate::parser::{GrammarProfile, NyashParser, ParseError};

pub(crate) fn reject_legacy_from_before_ast(parser: &NyashParser) -> Result<(), ParseError> {
    let tokens = &parser.tokens[parser.current..];
    let Some(classification) = classify_from_transport_tokens(tokens) else {
        return Ok(());
    };
    let line = parser.current_token().line;
    match classification {
        FromTransportClassification::Closed(kind) => {
            reject_closed(kind, parser.build_config.grammar_profile, line)
        }
        FromTransportClassification::NotClosed(_) => Err(ParseError::MigrationTransport {
            stable_reject_tag: "parser/from_transport_not_closed_form",
            line,
        }),
    }
}

fn reject_closed(
    kind: MigrationTransportKind,
    profile: GrammarProfile,
    line: usize,
) -> Result<(), ParseError> {
    match profile {
        GrammarProfile::Canonical => Err(ParseError::MigrationTransport {
            stable_reject_tag: canonical_reject_tag(kind),
            line,
        }),
        GrammarProfile::Compat2025 => Err(ParseError::TransportOnly {
            row_id: kind.row_id(),
            profile,
            transport_kind: kind,
            stable_reject_tag: "parser/from_compat_transport_only",
            line,
        }),
    }
}

const fn canonical_reject_tag(kind: MigrationTransportKind) -> &'static str {
    match kind {
        MigrationTransportKind::BoxFromInheritance => "parser/from_inheritance_legacy",
        MigrationTransportKind::FromCall => "parser/from_call_legacy",
    }
}
