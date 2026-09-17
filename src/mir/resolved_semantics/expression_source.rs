//! AST-free expression and local-initializer source rows.
//!
//! The shadow traversal observes syntax once. Canonicalization brands local
//! bindings with the resolved owner and publishes passive source relations;
//! it does not assign value classes, effects, Recipe roles, or MIR identity.

use std::collections::BTreeMap;

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, UnaryOperator};

use super::shadow::ShadowBindingOrdinalV0;
use super::{
    BindingRefV1, SourceBindingSiteV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
    SourcePathV1,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResolvedUnaryOperatorV1 {
    Minus,
    Not,
    BitNot,
    Weak,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedUnaryExpressionSourceV1 {
    site: SourceExprSiteV1,
    operator: ResolvedUnaryOperatorV1,
    operand: SourceExprSiteV1,
}

impl ResolvedUnaryExpressionSourceV1 {
    #[cfg(test)]
    pub(crate) fn from_parts_for_test(
        site: SourceExprSiteV1,
        operator: ResolvedUnaryOperatorV1,
        operand: SourceExprSiteV1,
    ) -> Self {
        Self {
            site,
            operator,
            operand,
        }
    }

    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn operator(&self) -> ResolvedUnaryOperatorV1 {
        self.operator
    }

    pub(crate) const fn operand(&self) -> &SourceExprSiteV1 {
        &self.operand
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ResolvedLiteralSourceV1 {
    Integer(i64),
    TypedInteger {
        value: i64,
        declared_type_name: Box<str>,
    },
    /// A sealed string literal. The payload is sealed with the row — a
    /// consumer never re-reads the source site.
    String(Box<str>),
    Float,
    Bool(bool),
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedExpressionIfConsumerV1 {
    parent: SourceNodeSiteV1,
    role: SourcePathSegmentV1,
}

impl ResolvedExpressionIfConsumerV1 {
    pub(crate) const fn parent(&self) -> &SourceNodeSiteV1 {
        &self.parent
    }

    pub(crate) const fn role(&self) -> &SourcePathSegmentV1 {
        &self.role
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedConditionalExpressionSourceV1 {
    site: SourceExprSiteV1,
    condition: SourceExprSiteV1,
    then_block: SourceExprSiteV1,
    then_tail: SourceExprSiteV1,
    else_block: SourceExprSiteV1,
    else_tail: SourceExprSiteV1,
    consumer: ResolvedExpressionIfConsumerV1,
}

impl ResolvedConditionalExpressionSourceV1 {
    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn condition(&self) -> &SourceExprSiteV1 {
        &self.condition
    }

    pub(crate) const fn then_block(&self) -> &SourceExprSiteV1 {
        &self.then_block
    }

    pub(crate) const fn then_tail(&self) -> &SourceExprSiteV1 {
        &self.then_tail
    }

    pub(crate) const fn else_block(&self) -> &SourceExprSiteV1 {
        &self.else_block
    }

    pub(crate) const fn else_tail(&self) -> &SourceExprSiteV1 {
        &self.else_tail
    }

    pub(crate) const fn consumer(&self) -> &ResolvedExpressionIfConsumerV1 {
        &self.consumer
    }
}

impl ResolvedBinaryExpressionSourceV1 {
    #[cfg(test)]
    pub(crate) fn from_parts_for_test(
        site: SourceExprSiteV1,
        operator: ResolvedBinaryOperatorV1,
        lhs: SourceExprSiteV1,
        rhs: SourceExprSiteV1,
    ) -> Self {
        Self {
            site,
            operator,
            lhs,
            rhs,
        }
    }

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
    unaries: BTreeMap<SourceExprSiteV1, ResolvedUnaryExpressionSourceV1>,
    literals: BTreeMap<SourceExprSiteV1, ResolvedLiteralSourceV1>,
    conditionals: BTreeMap<SourceExprSiteV1, ResolvedConditionalExpressionSourceV1>,
    initializers: BTreeMap<SourceBindingSiteV1, ResolvedInitializerRelationV1>,
}

impl ResolvedExpressionSourceInventoryV1 {
    #[cfg(test)]
    pub(crate) fn from_parts_for_test(
        binaries: impl IntoIterator<Item = ResolvedBinaryExpressionSourceV1>,
        unaries: impl IntoIterator<Item = ResolvedUnaryExpressionSourceV1>,
        literals: impl IntoIterator<Item = (SourceExprSiteV1, ResolvedLiteralSourceV1)>,
    ) -> Self {
        Self {
            binaries: binaries
                .into_iter()
                .map(|row| (row.site.clone(), row))
                .collect(),
            unaries: unaries
                .into_iter()
                .map(|row| (row.site.clone(), row))
                .collect(),
            literals: literals.into_iter().collect(),
            conditionals: BTreeMap::new(),
            initializers: BTreeMap::new(),
        }
    }

    pub(crate) fn binaries(&self) -> impl Iterator<Item = &ResolvedBinaryExpressionSourceV1> {
        self.binaries.values()
    }

    pub(crate) fn binary(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<&ResolvedBinaryExpressionSourceV1> {
        self.binaries.get(site)
    }

    pub(crate) fn literal(&self, site: &SourceExprSiteV1) -> Option<&ResolvedLiteralSourceV1> {
        self.literals.get(site)
    }

    pub(crate) fn unary(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<&ResolvedUnaryExpressionSourceV1> {
        self.unaries.get(site)
    }

    pub(crate) fn conditionals(
        &self,
    ) -> impl Iterator<Item = &ResolvedConditionalExpressionSourceV1> {
        self.conditionals.values()
    }

    pub(crate) fn conditional(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<&ResolvedConditionalExpressionSourceV1> {
        self.conditionals.get(site)
    }

    pub(crate) fn initializer(
        &self,
        declaration: &SourceBindingSiteV1,
    ) -> Option<&ResolvedInitializerRelationV1> {
        self.initializers.get(declaration)
    }

    pub(crate) fn initializers(&self) -> impl Iterator<Item = &ResolvedInitializerRelationV1> {
        self.initializers.values()
    }
}

#[derive(Debug, Default, Clone)]
pub(in crate::mir::resolved_semantics) struct ShadowExpressionSourceDraftV1 {
    binaries: BTreeMap<SourceExprSiteV1, ResolvedBinaryExpressionSourceV1>,
    unaries: Vec<ResolvedUnaryExpressionSourceV1>,
    literals: BTreeMap<SourceExprSiteV1, ResolvedLiteralSourceV1>,
    conditionals: Vec<ResolvedConditionalExpressionSourceV1>,
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
            ASTNode::UnaryOp { operator, .. } => {
                let operand = SourcePathV1::from_node(site.node())
                    .child(SourcePathSegmentV1::Operand)
                    .expr();
                self.expression_source
                    .unaries
                    .push(ResolvedUnaryExpressionSourceV1 {
                        site,
                        operator: map_unary_operator(operator),
                        operand,
                    });
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

    pub(super) fn record_conditional_expression_source(
        &mut self,
        site: SourceExprSiteV1,
        condition: SourceExprSiteV1,
        then_block: SourceExprSiteV1,
        then_tail: SourceExprSiteV1,
        else_block: SourceExprSiteV1,
        else_tail: SourceExprSiteV1,
    ) -> Result<(), &'static str> {
        let consumer = conditional_consumer(&site).ok_or("unsupported expression-if consumer")?;
        self.expression_source
            .conditionals
            .push(ResolvedConditionalExpressionSourceV1 {
                site,
                condition,
                then_block,
                then_tail,
                else_block,
                else_tail,
                consumer,
            });
        Ok(())
    }
}

pub(super) fn seal_shadow_expression_source_v1(
    draft: ShadowExpressionSourceDraftV1,
    binding_ref: impl Fn(ShadowBindingOrdinalV0) -> BindingRefV1,
) -> Result<ResolvedExpressionSourceInventoryV1, &'static str> {
    let mut unaries = BTreeMap::new();
    for row in draft.unaries {
        let site = row.site.clone();
        if unaries.insert(site, row).is_some() {
            return Err("duplicate unary expression source relation");
        }
    }
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
    let mut conditionals = BTreeMap::new();
    for row in draft.conditionals {
        if !valid_conditional_row(&row) {
            return Err("conditional expression source path drift");
        }
        let site = row.site.clone();
        if conditionals.insert(site, row).is_some() {
            return Err("duplicate conditional expression source relation");
        }
    }
    Ok(ResolvedExpressionSourceInventoryV1 {
        binaries: draft.binaries,
        unaries,
        literals: draft.literals,
        conditionals,
        initializers,
    })
}

fn conditional_consumer(site: &SourceExprSiteV1) -> Option<ResolvedExpressionIfConsumerV1> {
    let (role, parent) = site.node().segments().split_last()?;
    if !matches!(
        role,
        SourcePathSegmentV1::Value | SourcePathSegmentV1::Rhs | SourcePathSegmentV1::Initializer(_)
    ) {
        return None;
    }
    Some(ResolvedExpressionIfConsumerV1 {
        parent: SourceNodeSiteV1::from_segments(parent.to_vec()),
        role: role.clone(),
    })
}

fn valid_conditional_row(row: &ResolvedConditionalExpressionSourceV1) -> bool {
    let source = SourcePathV1::from_node(row.site.node());
    row.condition == source.child(SourcePathSegmentV1::IfCondition).expr()
        && row.then_block
            == source
                .child(SourcePathSegmentV1::IfThenBody)
                .child(SourcePathSegmentV1::IfThen(0))
                .expr()
        && row.then_tail
            == source
                .child(SourcePathSegmentV1::IfThenBody)
                .child(SourcePathSegmentV1::IfThen(0))
                .child(SourcePathSegmentV1::BlockExprTail)
                .expr()
        && row.else_block
            == source
                .child(SourcePathSegmentV1::IfElseBody)
                .child(SourcePathSegmentV1::IfElse(0))
                .expr()
        && row.else_tail
            == source
                .child(SourcePathSegmentV1::IfElseBody)
                .child(SourcePathSegmentV1::IfElse(0))
                .child(SourcePathSegmentV1::BlockExprTail)
                .expr()
        && SourcePathV1::from_node(row.consumer.parent())
            .child(row.consumer.role().clone())
            .expr()
            == row.site
}

fn map_unary_operator(operator: &UnaryOperator) -> ResolvedUnaryOperatorV1 {
    match operator {
        UnaryOperator::Minus => ResolvedUnaryOperatorV1::Minus,
        UnaryOperator::Not => ResolvedUnaryOperatorV1::Not,
        UnaryOperator::BitNot => ResolvedUnaryOperatorV1::BitNot,
        UnaryOperator::Weak => ResolvedUnaryOperatorV1::Weak,
    }
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
        LiteralValue::String(text) => ResolvedLiteralSourceV1::String(text.clone().into()),
        LiteralValue::Float(_) => ResolvedLiteralSourceV1::Float,
        LiteralValue::Bool(value) => ResolvedLiteralSourceV1::Bool(*value),
        LiteralValue::Null => ResolvedLiteralSourceV1::Null,
        LiteralValue::Void => ResolvedLiteralSourceV1::Void,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::resolved_semantics::{SourceNodeSiteV1, SourcePathSegmentV1};

    #[test]
    fn duplicate_unary_site_rejects_at_seal() {
        let site = SourcePathV1::from_node(&SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(0),
        ]))
        .expr();
        let row = ResolvedUnaryExpressionSourceV1 {
            site: site.clone(),
            operator: ResolvedUnaryOperatorV1::Minus,
            operand: SourcePathV1::from_node(site.node())
                .child(SourcePathSegmentV1::Operand)
                .expr(),
        };
        let mut draft = ShadowExpressionSourceDraftV1::default();
        draft.unaries.extend([row.clone(), row]);

        assert_eq!(
            seal_shadow_expression_source_v1(draft, |_| unreachable!()).unwrap_err(),
            "duplicate unary expression source relation"
        );
    }
}
