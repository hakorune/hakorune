//! Finite resolver-only Call+Return coverage witness.
//!
//! This module deliberately does not issue a general effect/control product.
//! It borrows one already co-sealed body-shape product and admits only the
//! exact root-direct `return me.invoke()` cohort.  Home flow, conformance,
//! targets, Recipe, and physical lowering remain outside this boundary.

use super::body_shape::{
    BodyEffectKindV1, BodyEffectShapeV1, BodyExpressionShapeV1, BodyShapeRelationV1,
    BodyStatementShapeV1, ResolvedFunctionBodyShapeProductV1,
};
use super::function_view::ReceiverPolicyV1;
use super::records::{
    BindingKindV1, RegionKindV1, ResolvedControlTransferV1, ResolvedExitOriginV1,
    ResolvedExitRecordV1,
};
use super::source_site::{
    ResolvedExitSiteV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BodyEffectControlCoverageNoSafeSliceV1 {
    UnsupportedExpressionShape,
    UnsupportedStatementShape,
    UnsupportedEffectKind,
    UnsupportedControlTransfer,
    NestedOwnerBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BodyEffectControlCoverageRejectV1 {
    OwnerMismatch,
    BodyRootMismatch,
    ReceiverBindingMismatch,
    EffectCardinalityMismatch,
    EffectSiteMismatch,
    RelationCoverageMismatch,
    ExitCardinalityMismatch,
    ExitSiteMismatch,
    ReturnTargetMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BodyEffectControlCoverageIssueV1 {
    NoSafeSlice(BodyEffectControlCoverageNoSafeSliceV1),
    Rejected(BodyEffectControlCoverageRejectV1),
}

/// Borrowed witness for one finite effect/control cohort.
///
/// The product reference is the pairing authority.  No shadow vector or
/// source row is copied into this receipt, and the receipt is intentionally
/// not `Clone`.
#[derive(Debug)]
pub(crate) struct BodyEffectControlCoverageReceiptV1<'product> {
    product: &'product ResolvedFunctionBodyShapeProductV1,
    call_effect: &'product BodyEffectShapeV1,
    return_site: &'product SourceStmtSiteV1,
    return_exit_site: &'product ResolvedExitSiteV1,
    return_exit: &'product ResolvedExitRecordV1,
}

impl<'product> BodyEffectControlCoverageReceiptV1<'product> {
    pub(crate) fn product(&self) -> &'product ResolvedFunctionBodyShapeProductV1 {
        self.product
    }

    pub(crate) fn call_effect(&self) -> &'product BodyEffectShapeV1 {
        self.call_effect
    }

    pub(crate) fn return_site(&self) -> &'product SourceStmtSiteV1 {
        self.return_site
    }

    pub(crate) fn return_exit(
        &self,
    ) -> (&'product ResolvedExitSiteV1, &'product ResolvedExitRecordV1) {
        (self.return_exit_site, self.return_exit)
    }
}

pub(crate) struct BodyEffectControlCoverageIssuerV1;

impl BodyEffectControlCoverageIssuerV1 {
    pub(crate) fn issue<'product>(
        product: &'product ResolvedFunctionBodyShapeProductV1,
    ) -> Result<BodyEffectControlCoverageReceiptV1<'product>, BodyEffectControlCoverageIssueV1>
    {
        let function = product.function();
        let shape = product.body_shape();
        if shape.owner() != function.owner() {
            return Err(rejected(BodyEffectControlCoverageRejectV1::OwnerMismatch));
        }
        if shape.body_root() != &function.root_profile().body_root() {
            return Err(rejected(
                BodyEffectControlCoverageRejectV1::BodyRootMismatch,
            ));
        }
        if function.source_kind() != super::SemanticOwnerSourceKindV1::DeclaredFunction
            || function.root_profile().receiver_policy() != ReceiverPolicyV1::DeclaredInstance
        {
            return Err(no_safe(
                BodyEffectControlCoverageNoSafeSliceV1::NestedOwnerBoundary,
            ));
        }

        let return_site = match shape.statements() {
            [BodyStatementShapeV1::Return {
                site,
                value: Some(_),
            }] => site,
            _ => {
                return Err(no_safe(
                    BodyEffectControlCoverageNoSafeSliceV1::UnsupportedStatementShape,
                ));
            }
        };
        let (call_site, receiver_site, receiver) = match shape.expressions() {
            [BodyExpressionShapeV1::MethodCall {
                site: call_site,
                object: receiver_site,
                arity: 0,
                ..
            }, BodyExpressionShapeV1::Me {
                site: me_site,
                receiver,
            }] if receiver_site == me_site => (call_site, me_site, receiver),
            _ => {
                return Err(no_safe(
                    BodyEffectControlCoverageNoSafeSliceV1::UnsupportedExpressionShape,
                ));
            }
        };
        let receiver_is_valid = function
            .binding(*receiver)
            .map(|binding| binding.kind() == BindingKindV1::Receiver)
            .unwrap_or(false);
        if receiver.owner() != function.owner() || !receiver_is_valid {
            return Err(rejected(
                BodyEffectControlCoverageRejectV1::ReceiverBindingMismatch,
            ));
        }

        if shape.effects().len() != 1 {
            return Err(rejected(
                BodyEffectControlCoverageRejectV1::EffectCardinalityMismatch,
            ));
        }
        let call_effect = &shape.effects()[0];
        if call_effect.kind != BodyEffectKindV1::Call {
            return Err(no_safe(
                BodyEffectControlCoverageNoSafeSliceV1::UnsupportedEffectKind,
            ));
        }
        if call_effect.site != *call_site {
            return Err(rejected(
                BodyEffectControlCoverageRejectV1::EffectSiteMismatch,
            ));
        }

        let value_site = match shape.statements() {
            [BodyStatementShapeV1::Return {
                value: Some(value), ..
            }] => value,
            _ => unreachable!("return shape was checked above"),
        };
        if shape.relations().len() != 2
            || relation_count(
                shape.relations(),
                return_site.node(),
                SourcePathSegmentV1::Value,
                value_site,
            ) != 1
            || relation_count(
                shape.relations(),
                call_site.node(),
                SourcePathSegmentV1::Receiver,
                receiver_site,
            ) != 1
        {
            return Err(rejected(
                BodyEffectControlCoverageRejectV1::RelationCoverageMismatch,
            ));
        }

        let mut exits = function.resolved_exits();
        let (return_exit_site, return_exit) = exits
            .next()
            .ok_or_else(|| rejected(BodyEffectControlCoverageRejectV1::ExitCardinalityMismatch))?;
        if exits.next().is_some() {
            return Err(rejected(
                BodyEffectControlCoverageRejectV1::ExitCardinalityMismatch,
            ));
        }
        if return_exit_site != &ResolvedExitSiteV1::Statement(return_site.clone()) {
            return Err(rejected(
                BodyEffectControlCoverageRejectV1::ExitSiteMismatch,
            ));
        }
        let source_region = return_exit.source_region();
        let source_region_is_valid = source_region.owner() == function.owner()
            && function
                .region(source_region)
                .map(|region| region.kind() == RegionKindV1::Sequence)
                .unwrap_or(false);
        if !source_region_is_valid
            || return_exit.origin() != ResolvedExitOriginV1::ExplicitReturn
            || return_exit.transfer()
                != (ResolvedControlTransferV1::Return {
                    target_function: function.function_region(),
                })
        {
            return Err(rejected(
                BodyEffectControlCoverageRejectV1::ReturnTargetMismatch,
            ));
        }

        Ok(BodyEffectControlCoverageReceiptV1 {
            product,
            call_effect,
            return_site,
            return_exit_site,
            return_exit,
        })
    }
}

fn relation_count(
    relations: &[BodyShapeRelationV1],
    parent: &SourceNodeSiteV1,
    role: SourcePathSegmentV1,
    child: &SourceExprSiteV1,
) -> usize {
    relations
        .iter()
        .filter(|relation| {
            relation.parent == *parent && relation.role == role && relation.child == *child
        })
        .count()
}

const fn no_safe(
    reason: BodyEffectControlCoverageNoSafeSliceV1,
) -> BodyEffectControlCoverageIssueV1 {
    BodyEffectControlCoverageIssueV1::NoSafeSlice(reason)
}

const fn rejected(reason: BodyEffectControlCoverageRejectV1) -> BodyEffectControlCoverageIssueV1 {
    BodyEffectControlCoverageIssueV1::Rejected(reason)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
    use crate::mir::resolved_semantics::{FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1};

    fn function(body: Vec<ASTNode>) -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: "coverage_fixture".into(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body,
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: false,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }
    }

    fn me() -> ASTNode {
        ASTNode::Me {
            span: Span::unknown(),
        }
    }

    fn method_call(object: ASTNode) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(object),
            method: "invoke".into(),
            arguments: Vec::new(),
            span: Span::unknown(),
        }
    }

    fn return_value(value: ASTNode) -> ASTNode {
        ASTNode::Return {
            value: Some(Box::new(value)),
            span: Span::unknown(),
        }
    }

    fn await_expression(value: ASTNode) -> ASTNode {
        ASTNode::AwaitExpression {
            expression: Box::new(value),
            span: Span::unknown(),
        }
    }

    fn field_access(object: ASTNode) -> ASTNode {
        ASTNode::FieldAccess {
            object: Box::new(object),
            field: "value".into(),
            span: Span::unknown(),
        }
    }

    fn allocation() -> ASTNode {
        ASTNode::New {
            class: "Box".into(),
            arguments: Vec::new(),
            field_initializers: Vec::new(),
            type_arguments: Vec::new(),
            span: Span::unknown(),
        }
    }

    fn resolve(body: Vec<ASTNode>) -> ResolvedFunctionBodyShapeProductV1 {
        FunctionSemanticResolverSessionV1::new(0)
            .unwrap()
            .resolve_with_body_shape(FunctionSyntaxViewV1::from_ast(&function(body)).unwrap())
            .unwrap()
    }

    #[test]
    fn issues_exact_call_and_return_witness() {
        let product = resolve(vec![return_value(method_call(me()))]);
        let receipt = BodyEffectControlCoverageIssuerV1::issue(&product).unwrap();
        assert_eq!(
            receipt.product().function().owner(),
            product.function().owner()
        );
        assert_eq!(receipt.call_effect().kind, BodyEffectKindV1::Call);
        assert_eq!(
            receipt.return_site().node().segments(),
            &[SourcePathSegmentV1::Body(0)]
        );
        assert_eq!(
            receipt.return_exit().1.origin(),
            ResolvedExitOriginV1::ExplicitReturn
        );
    }

    #[test]
    fn keeps_literal_return_outside_the_finite_cohort() {
        let product = resolve(vec![return_value(ASTNode::Literal {
            value: LiteralValue::Integer(0),
            span: Span::unknown(),
        })]);
        assert!(matches!(
            BodyEffectControlCoverageIssuerV1::issue(&product),
            Err(BodyEffectControlCoverageIssueV1::NoSafeSlice(
                BodyEffectControlCoverageNoSafeSliceV1::UnsupportedExpressionShape,
            ))
        ));
    }

    #[test]
    fn keeps_print_outside_the_finite_cohort() {
        let product = resolve(vec![ASTNode::Print {
            expression: Box::new(me()),
            span: Span::unknown(),
        }]);
        assert!(matches!(
            BodyEffectControlCoverageIssuerV1::issue(&product),
            Err(BodyEffectControlCoverageIssueV1::NoSafeSlice(
                BodyEffectControlCoverageNoSafeSliceV1::UnsupportedStatementShape,
            ))
        ));
    }

    #[test]
    fn keeps_empty_await_and_field_shapes_outside_the_finite_cohort() {
        for body in [
            Vec::new(),
            vec![return_value(await_expression(me()))],
            vec![return_value(field_access(me()))],
            vec![return_value(allocation())],
        ] {
            assert!(matches!(
                BodyEffectControlCoverageIssuerV1::issue(&resolve(body)),
                Err(BodyEffectControlCoverageIssueV1::NoSafeSlice(_))
            ));
        }
    }

    #[test]
    fn rejects_a_forged_cross_owner_product() {
        let first = resolve(vec![return_value(method_call(me()))]);
        let second = resolve(vec![return_value(method_call(me()))]);
        let (function, _) = first.into_parts();
        let (_, shape) = second.into_parts();
        let forged = ResolvedFunctionBodyShapeProductV1::from_parts(function, shape);
        assert!(matches!(
            BodyEffectControlCoverageIssuerV1::issue(&forged),
            Err(BodyEffectControlCoverageIssueV1::Rejected(
                BodyEffectControlCoverageRejectV1::OwnerMismatch,
            ))
        ));
    }
}
