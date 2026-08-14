//! AST-free expression and local-initializer source rows.
//!
//! The shadow traversal observes syntax once. Canonicalization brands local
//! bindings with the resolved owner and publishes passive source relations;
//! it does not assign value classes, effects, Recipe roles, or MIR identity.

use std::collections::BTreeMap;

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

use super::shadow::ShadowBindingOrdinalV0;
use super::{
    BindingRefV1, SourceBindingSiteV1, SourceExprSiteV1, SourcePathSegmentV1, SourcePathV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResolvedBinaryOperatorV1 {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ResolvedLiteralSourceV1 {
    Integer(i64),
    TypedInteger {
        value: i64,
        declared_type_name: Box<str>,
    },
    String,
    Float,
    Bool,
    Null,
    Void,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedBinaryExpressionSourceV1 {
    site: SourceExprSiteV1,
    operator: ResolvedBinaryOperatorV1,
    lhs: SourceExprSiteV1,
    rhs: SourceExprSiteV1,
}

impl ResolvedBinaryExpressionSourceV1 {
    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn operator(&self) -> ResolvedBinaryOperatorV1 {
        self.operator
    }

    pub(crate) const fn lhs(&self) -> &SourceExprSiteV1 {
        &self.lhs
    }

    pub(crate) const fn rhs(&self) -> &SourceExprSiteV1 {
        &self.rhs
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedInitializerRelationV1 {
    declaration_site: SourceBindingSiteV1,
    binding: BindingRefV1,
    declared_type_name: Option<Box<str>>,
    initializer_site: Option<SourceExprSiteV1>,
}

impl ResolvedInitializerRelationV1 {
    pub(crate) const fn declaration_site(&self) -> &SourceBindingSiteV1 {
        &self.declaration_site
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) fn declared_type_name(&self) -> Option<&str> {
        self.declared_type_name.as_deref()
    }

    pub(crate) const fn initializer_site(&self) -> Option<&SourceExprSiteV1> {
        self.initializer_site.as_ref()
    }
}

#[derive(Debug, Default)]
pub(crate) struct ResolvedExpressionSourceInventoryV1 {
    binaries: BTreeMap<SourceExprSiteV1, ResolvedBinaryExpressionSourceV1>,
    literals: BTreeMap<SourceExprSiteV1, ResolvedLiteralSourceV1>,
    initializers: BTreeMap<SourceBindingSiteV1, ResolvedInitializerRelationV1>,
}

impl ResolvedExpressionSourceInventoryV1 {
    pub(crate) fn binaries(&self) -> impl Iterator<Item = &ResolvedBinaryExpressionSourceV1> {
        self.binaries.values()
    }

    pub(crate) fn literal(&self, site: &SourceExprSiteV1) -> Option<&ResolvedLiteralSourceV1> {
        self.literals.get(site)
    }

    pub(crate) fn initializers(&self) -> impl Iterator<Item = &ResolvedInitializerRelationV1> {
        self.initializers.values()
    }
}

#[derive(Debug, Default, Clone)]
pub(in crate::mir::resolved_semantics) struct ShadowExpressionSourceDraftV1 {
    binaries: BTreeMap<SourceExprSiteV1, ResolvedBinaryExpressionSourceV1>,
    literals: BTreeMap<SourceExprSiteV1, ResolvedLiteralSourceV1>,
    initializers: Vec<ShadowInitializerRelationV1>,
}

#[derive(Debug, Clone)]
struct ShadowInitializerRelationV1 {
    declaration_site: SourceBindingSiteV1,
    binding: ShadowBindingOrdinalV0,
    declared_type_name: Option<Box<str>>,
    initializer_site: Option<SourceExprSiteV1>,
}

impl<'ast, 'schema> super::shadow::resolver::ShadowResolverV0<'ast, 'schema> {
    pub(super) fn record_expression_source(
        &mut self,
        expression: &ASTNode,
        site: SourceExprSiteV1,
    ) {
        match expression {
            ASTNode::BinaryOp { operator, .. } => {
                let path = SourcePathV1::from_node(site.node());
                self.expression_source.binaries.insert(
                    site.clone(),
                    ResolvedBinaryExpressionSourceV1 {
                        site,
                        operator: map_binary_operator(operator),
                        lhs: path.child(SourcePathSegmentV1::Lhs).expr(),
                        rhs: path.child(SourcePathSegmentV1::Rhs).expr(),
                    },
                );
            }
            ASTNode::Literal { value, .. } => {
                self.expression_source
                    .literals
                    .insert(site, map_literal(value));
            }
            _ => {}
        }
    }

    pub(super) fn record_local_initializer_source(
        &mut self,
        declaration_site: SourceBindingSiteV1,
        binding: ShadowBindingOrdinalV0,
        declared_type_name: Option<&str>,
        initializer_site: Option<SourceExprSiteV1>,
    ) {
        self.expression_source
            .initializers
            .push(ShadowInitializerRelationV1 {
                declaration_site,
                binding,
                declared_type_name: declared_type_name.map(Into::into),
                initializer_site,
            });
    }
}

pub(super) fn seal_shadow_expression_source_v1(
    draft: ShadowExpressionSourceDraftV1,
    binding_ref: impl Fn(ShadowBindingOrdinalV0) -> BindingRefV1,
) -> Result<ResolvedExpressionSourceInventoryV1, &'static str> {
    let mut initializers = BTreeMap::new();
    for row in draft.initializers {
        let declaration_site = row.declaration_site.clone();
        let canonical = ResolvedInitializerRelationV1 {
            declaration_site: row.declaration_site,
            binding: binding_ref(row.binding),
            declared_type_name: row.declared_type_name,
            initializer_site: row.initializer_site,
        };
        if initializers.insert(declaration_site, canonical).is_some() {
            return Err("duplicate local initializer source relation");
        }
    }
    Ok(ResolvedExpressionSourceInventoryV1 {
        binaries: draft.binaries,
        literals: draft.literals,
        initializers,
    })
}

fn map_binary_operator(operator: &BinaryOperator) -> ResolvedBinaryOperatorV1 {
    match operator {
        BinaryOperator::Add => ResolvedBinaryOperatorV1::Add,
        BinaryOperator::Subtract => ResolvedBinaryOperatorV1::Subtract,
        BinaryOperator::Multiply => ResolvedBinaryOperatorV1::Multiply,
        BinaryOperator::Divide => ResolvedBinaryOperatorV1::Divide,
        BinaryOperator::Modulo => ResolvedBinaryOperatorV1::Modulo,
        BinaryOperator::BitAnd => ResolvedBinaryOperatorV1::BitAnd,
        BinaryOperator::BitOr => ResolvedBinaryOperatorV1::BitOr,
        BinaryOperator::BitXor => ResolvedBinaryOperatorV1::BitXor,
        BinaryOperator::Shl => ResolvedBinaryOperatorV1::Shl,
        BinaryOperator::Shr => ResolvedBinaryOperatorV1::Shr,
        BinaryOperator::Equal => ResolvedBinaryOperatorV1::Equal,
        BinaryOperator::NotEqual => ResolvedBinaryOperatorV1::NotEqual,
        BinaryOperator::Less => ResolvedBinaryOperatorV1::Less,
        BinaryOperator::Greater => ResolvedBinaryOperatorV1::Greater,
        BinaryOperator::LessEqual => ResolvedBinaryOperatorV1::LessEqual,
        BinaryOperator::GreaterEqual => ResolvedBinaryOperatorV1::GreaterEqual,
        BinaryOperator::And => ResolvedBinaryOperatorV1::And,
        BinaryOperator::Or => ResolvedBinaryOperatorV1::Or,
    }
}

fn map_literal(value: &LiteralValue) -> ResolvedLiteralSourceV1 {
    match value {
        LiteralValue::Integer(value) => ResolvedLiteralSourceV1::Integer(*value),
        LiteralValue::TypedInteger {
            value,
            declared_type_name,
        } => ResolvedLiteralSourceV1::TypedInteger {
            value: *value,
            declared_type_name: declared_type_name.clone().into(),
        },
        LiteralValue::String(_) => ResolvedLiteralSourceV1::String,
        LiteralValue::Float(_) => ResolvedLiteralSourceV1::Float,
        LiteralValue::Bool(_) => ResolvedLiteralSourceV1::Bool,
        LiteralValue::Null => ResolvedLiteralSourceV1::Null,
        LiteralValue::Void => ResolvedLiteralSourceV1::Void,
    }
}
