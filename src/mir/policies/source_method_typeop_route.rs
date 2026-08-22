//! Source-level typeop route policy shared by Builder and source observers.
//!
//! This is deliberately independent of Builder state.  The selected MethodCall
//! route must be classified from method spelling and argument syntax exactly
//! once, before receiver or argument effects.  Source target inventories use
//! the same disposition as the physical Builder route.

use crate::ast::{ASTNode, LiteralValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceMethodTypeOpKindV1 {
    Is,
    As,
}

impl SourceMethodTypeOpKindV1 {
    pub(crate) const fn spelling(self) -> &'static str {
        match self {
            Self::Is => "is",
            Self::As => "as",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SourceMethodTypeOpDispositionV1 {
    Ordinary,
    TypeOp {
        kind: SourceMethodTypeOpKindV1,
        type_name: Box<str>,
    },
}

pub(crate) fn classify_source_method_typeop_route_v1(
    method: &str,
    arguments: &[ASTNode],
) -> SourceMethodTypeOpDispositionV1 {
    let kind = match method {
        "is" => SourceMethodTypeOpKindV1::Is,
        "as" => SourceMethodTypeOpKindV1::As,
        _ => return SourceMethodTypeOpDispositionV1::Ordinary,
    };
    if arguments.len() != 1 {
        return SourceMethodTypeOpDispositionV1::Ordinary;
    }
    let Some(type_name) = extract_source_string_literal_v1(&arguments[0]) else {
        return SourceMethodTypeOpDispositionV1::Ordinary;
    };
    SourceMethodTypeOpDispositionV1::TypeOp {
        kind,
        type_name: type_name.into_boxed_str(),
    }
}

pub(crate) fn extract_source_string_literal_v1(node: &ASTNode) -> Option<String> {
    let mut current = node;
    loop {
        match current {
            ASTNode::Literal {
                value: LiteralValue::String(value),
                ..
            } => return Some(value.clone()),
            ASTNode::New {
                class, arguments, ..
            } if class == "StringBox" && arguments.len() == 1 => {
                current = &arguments[0];
            }
            _ => return None,
        }
    }
}
