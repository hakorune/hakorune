//! Pure raw-Loop child-entry quarantine.
//!
//! This module answers one question before the future invocation-aware raw
//! Loop boundary calls `cf_loop`: can this exact raw syntax enter a Box child
//! function? It owns no Builder, JoinIR route, module collector, header port,
//! source-site identity, or AST rewrite authority.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    BodyChildRoleV1, ExprChildRoleV1, SourceBodyKindV1, SourcePathSegmentV1,
};
use crate::mir::{MirBuilder, ValueId};

use super::normal_callable_loop_handoff::VerifiedCallableSemanticLoopBindingScheduleV1;
use super::raw_invocation_source_transport::RawInvocationSourceContextV1;

/// Exact child-entry result for one raw Loop syntax surface.
///
/// `NoChildFunctionEntry` is deliberately narrow: it says only that the
/// executable syntax has no reachable `BoxDeclaration`. It does not prove a
/// JoinIR route, recipe, CFG, type fact, or general Loop acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawLoopChildEntryDispositionV1 {
    NoChildFunctionEntry,
    ReachableBoxDeclaration,
}

/// One located invocation Loop admitted at the existing child-entry seam.
///
/// The exact parent, condition, and body-root receipts stay owned until the
/// single route terminal returns. This product deliberately does not claim
/// located JoinIR planning; it only prevents the invocation path from erasing
/// source location before the current route owner.
pub(in crate::mir::builder) struct PreparedLocatedRawLoopChildEntryV1<'source> {
    parent_source: &'source RawInvocationSourceContextV1,
    condition_source: RawInvocationSourceContextV1,
    body_source: RawInvocationSourceContextV1,
    condition: ASTNode,
    body: Vec<ASTNode>,
    disposition: RawLoopChildEntryDispositionV1,
    callable_handoff: Option<VerifiedCallableSemanticLoopBindingScheduleV1>,
}

impl<'source> PreparedLocatedRawLoopChildEntryV1<'source> {
    pub(in crate::mir::builder) fn prepare(
        parent_source: &'source RawInvocationSourceContextV1,
        loop_node: ASTNode,
        callable_handoff: Option<VerifiedCallableSemanticLoopBindingScheduleV1>,
    ) -> Result<Self, String> {
        if !matches!(&parent_source, RawInvocationSourceContextV1::Located { .. }) {
            return Err(
                "[freeze:contract][raw-loop-child-entry/requires-located-loop-source]".to_owned(),
            );
        }

        let condition_source =
            parent_source.child_expression(&loop_node, ExprChildRoleV1::LoopCondition)?;
        let body_source = parent_source.child_body(&loop_node, BodyChildRoleV1::LoopBody)?;
        verify_exact_loop_child_receipts(&condition_source, &body_source)?;

        let ASTNode::Loop {
            condition, body, ..
        } = loop_node
        else {
            return Err("[freeze:contract][raw-loop-child-entry/expected-loop]".to_owned());
        };
        let disposition = classify_raw_loop_child_entry_v1(&condition, &body);

        Ok(Self {
            parent_source,
            condition_source,
            body_source,
            condition: *condition,
            body,
            disposition,
            callable_handoff,
        })
    }

    pub(in crate::mir::builder) fn lower_with_existing_route_v1(
        self,
        builder: &mut MirBuilder,
    ) -> Result<ValueId, String> {
        let Self {
            parent_source,
            condition_source,
            body_source,
            condition,
            body,
            disposition,
            callable_handoff,
        } = self;
        let _pre_effect_receipt = callable_handoff
            .map(|handoff| {
                handoff.consume_pre_effect(
                    parent_source.site().ok_or_else(|| {
                        "[freeze:contract][raw-loop-child-entry/missing-parent-site]".to_owned()
                    })?,
                    condition_source.site().ok_or_else(|| {
                        "[freeze:contract][raw-loop-child-entry/missing-condition-site]".to_owned()
                    })?,
                    body_source.site().ok_or_else(|| {
                        "[freeze:contract][raw-loop-child-entry/missing-body-site]".to_owned()
                    })?,
                )
            })
            .transpose()?;

        match disposition {
            RawLoopChildEntryDispositionV1::NoChildFunctionEntry => {
                super::control_flow::joinir::routing::lower_loop_or_freeze_v1(
                    builder, condition, body,
                )
            }
            RawLoopChildEntryDispositionV1::ReachableBoxDeclaration => Err(
                super::control_flow::lower::Freeze::contract(
                    "raw_loop_child_entry: reachable BoxDeclaration requires a pure-plan/function-session bridge",
                )
                .to_string(),
            ),
        }
    }
}

fn verify_exact_loop_child_receipts(
    condition: &RawInvocationSourceContextV1,
    body: &RawInvocationSourceContextV1,
) -> Result<(), String> {
    let condition_is_exact = condition
        .site()
        .is_some_and(|site| site.segments().last() == Some(&SourcePathSegmentV1::LoopCondition));
    let body_is_exact = matches!(
        body,
        RawInvocationSourceContextV1::Located {
            site,
            body_kind: Some(SourceBodyKindV1::Loop),
            ..
        } if site.segments().last() == Some(&SourcePathSegmentV1::LoopBodyRoot)
    );
    if condition_is_exact && body_is_exact {
        Ok(())
    } else {
        Err("[freeze:contract][raw-loop-child-entry/exact-child-receipts]".to_owned())
    }
}

/// Classify the original condition and body of one raw `ASTNode::Loop`.
///
/// The AST traversal API is the generic child-topology SSOT. Lambda and nested
/// function declaration bodies are deferred ownership surfaces: neither is
/// executed by the surrounding raw Loop lowering, so this classifier does not
/// descend into them. A `BoxDeclaration` itself is executable on the raw
/// dispatcher path and is therefore a direct child-entry boundary.
pub(in crate::mir::builder) fn classify_raw_loop_child_entry_v1(
    condition: &ASTNode,
    body: &[ASTNode],
) -> RawLoopChildEntryDispositionV1 {
    let has_child_entry = contains_reachable_box_declaration(condition)
        || body.iter().any(contains_reachable_box_declaration);

    if has_child_entry {
        RawLoopChildEntryDispositionV1::ReachableBoxDeclaration
    } else {
        RawLoopChildEntryDispositionV1::NoChildFunctionEntry
    }
}

fn contains_reachable_box_declaration(node: &ASTNode) -> bool {
    match node {
        ASTNode::BoxDeclaration { .. } => true,
        ASTNode::Lambda { .. } | ASTNode::FunctionDeclaration { .. } => false,
        _ => node.any_child(contains_reachable_box_declaration),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{
        classify_raw_loop_child_entry_v1, PreparedLocatedRawLoopChildEntryV1,
        RawLoopChildEntryDispositionV1,
    };
    use crate::ast::{ASTNode, DeclarationAttrs, Span};
    use crate::mir::builder::normal_callable_loop_handoff::{
        CallableLoopBindingReceiptV1, CallableLoopBindingRoleV1,
        VerifiedCallableSemanticLoopBindingScheduleV1,
    };
    use crate::mir::builder::raw_invocation_source_transport::{
        RawInvocationRootLineageV1, RawInvocationSourceContextV1, RawUnlocatedPortalV1,
    };
    use crate::mir::resolved_semantics::{
        FunctionOwnerIssuerV1, SourceBodyKindV1, SourcePathSegmentV1, SourcePathV1,
    };
    use hakorune_mir_core::BindingId;

    fn span() -> Span {
        Span::unknown()
    }

    fn literal_bool(value: bool) -> ASTNode {
        ASTNode::Literal {
            value: crate::ast::LiteralValue::Bool(value),
            span: span(),
        }
    }

    fn box_declaration() -> ASTNode {
        ASTNode::BoxDeclaration {
            name: "Nested".to_string(),
            fields: Vec::new(),
            field_decls: Vec::new(),
            public_fields: Vec::new(),
            private_fields: Vec::new(),
            methods: crate::ast::BoxMethodInventoryV1::empty(),
            constructors: HashMap::new(),
            init_fields: Vec::new(),
            weak_fields: Vec::new(),
            delegates: Vec::new(),
            invariants: Vec::new(),
            transitions: Vec::new(),
            is_interface: false,
            is_record: false,
            extends: Vec::new(),
            implements: Vec::new(),
            type_parameters: Vec::new(),
            is_sync: false,
            is_static: true,
            static_init: None,
            attrs: DeclarationAttrs::default(),
            span: span(),
        }
    }

    fn loop_node(body: Vec<ASTNode>) -> ASTNode {
        ASTNode::Loop {
            condition: Box::new(literal_bool(true)),
            body,
            span: span(),
        }
    }

    fn located_loop_source() -> RawInvocationSourceContextV1 {
        RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::ScriptRoot,
            site: SourcePathV1::root_body(3).node(),
            body_kind: None,
        }
    }

    fn callable_handoff(
        loop_site: &crate::mir::resolved_semantics::SourceNodeSiteV1,
    ) -> super::VerifiedCallableSemanticLoopBindingScheduleV1 {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let owner = issuer.issue().unwrap();
        let binding = crate::mir::resolved_semantics::BindingRefV1::new(owner, BindingId::new(0));
        let condition = SourcePathV1::from_node(loop_site)
            .child(SourcePathSegmentV1::LoopCondition)
            .child(SourcePathSegmentV1::Lhs)
            .node();
        let body_read = SourcePathV1::from_node(loop_site)
            .child(SourcePathSegmentV1::LoopBody(0))
            .child(SourcePathSegmentV1::Value)
            .child(SourcePathSegmentV1::Lhs)
            .node();
        let target = SourcePathV1::from_node(loop_site)
            .child(SourcePathSegmentV1::LoopBody(0))
            .child(SourcePathSegmentV1::Target)
            .node();
        VerifiedCallableSemanticLoopBindingScheduleV1::seal(
            owner,
            loop_site.clone(),
            vec![
                CallableLoopBindingReceiptV1::new(
                    condition,
                    binding,
                    CallableLoopBindingRoleV1::ConditionRead,
                ),
                CallableLoopBindingReceiptV1::new(
                    body_read,
                    binding,
                    CallableLoopBindingRoleV1::BodyRead,
                ),
                CallableLoopBindingReceiptV1::new(
                    target,
                    binding,
                    CallableLoopBindingRoleV1::BodyRebind,
                ),
            ],
        )
        .unwrap()
    }

    #[test]
    fn accepts_plain_loop_syntax_without_box_declaration() {
        let body = vec![ASTNode::If {
            condition: Box::new(literal_bool(true)),
            then_body: vec![ASTNode::Print {
                expression: Box::new(literal_bool(false)),
                span: span(),
            }],
            else_body: None,
            span: span(),
        }];

        assert_eq!(
            classify_raw_loop_child_entry_v1(&literal_bool(true), &body),
            RawLoopChildEntryDispositionV1::NoChildFunctionEntry,
        );
    }

    #[test]
    fn rejects_box_declaration_in_loop_body_or_expression() {
        let direct = vec![box_declaration()];
        let nested_expression = vec![ASTNode::FunctionCall {
            name: "consume".to_string(),
            arguments: vec![box_declaration()],
            span: span(),
        }];

        assert_eq!(
            classify_raw_loop_child_entry_v1(&literal_bool(true), &direct),
            RawLoopChildEntryDispositionV1::ReachableBoxDeclaration,
        );
        assert_eq!(
            classify_raw_loop_child_entry_v1(&literal_bool(true), &nested_expression),
            RawLoopChildEntryDispositionV1::ReachableBoxDeclaration,
        );
    }

    #[test]
    fn rejects_box_declaration_in_nested_executable_loop() {
        let body = vec![ASTNode::Loop {
            condition: Box::new(literal_bool(true)),
            body: vec![box_declaration()],
            span: span(),
        }];

        assert_eq!(
            classify_raw_loop_child_entry_v1(&literal_bool(true), &body),
            RawLoopChildEntryDispositionV1::ReachableBoxDeclaration,
        );
    }

    #[test]
    fn ignores_box_declaration_inside_deferred_lambda_body() {
        let body = vec![ASTNode::Lambda {
            params: Vec::new(),
            body: vec![box_declaration()],
            span: span(),
        }];

        assert_eq!(
            classify_raw_loop_child_entry_v1(&literal_bool(true), &body),
            RawLoopChildEntryDispositionV1::NoChildFunctionEntry,
        );
    }

    #[test]
    fn located_entry_co_seals_exact_condition_and_body_root_receipts() {
        let source = located_loop_source();
        let prepared = PreparedLocatedRawLoopChildEntryV1::prepare(
            &source,
            loop_node(vec![ASTNode::Break { span: span() }]),
            None,
        )
        .expect("located Loop entry");

        assert_eq!(
            prepared.condition_source.site().unwrap().segments(),
            &[
                SourcePathSegmentV1::Body(3),
                SourcePathSegmentV1::LoopCondition,
            ]
        );
        assert!(matches!(
            prepared.body_source,
            RawInvocationSourceContextV1::Located {
                body_kind: Some(SourceBodyKindV1::Loop),
                ref site,
                ..
            } if site.segments() == [
                SourcePathSegmentV1::Body(3),
                SourcePathSegmentV1::LoopBodyRoot,
            ]
        ));
        assert_eq!(
            prepared.disposition,
            RawLoopChildEntryDispositionV1::NoChildFunctionEntry
        );
    }

    #[test]
    fn located_entry_carries_callable_handoff_before_route_effects() {
        let source = located_loop_source();
        let loop_site = source.site().unwrap().clone();
        let prepared = PreparedLocatedRawLoopChildEntryV1::prepare(
            &source,
            loop_node(vec![ASTNode::Break { span: span() }]),
            Some(callable_handoff(&loop_site)),
        )
        .expect("located Loop entry with callable handoff");

        assert!(prepared.callable_handoff.is_some());
    }

    #[test]
    fn unlocated_entry_is_rejected_before_child_classification() {
        let error = PreparedLocatedRawLoopChildEntryV1::prepare(
            &RawInvocationSourceContextV1::UnlocatedCompatibility(RawUnlocatedPortalV1::CallObject),
            loop_node(vec![box_declaration()]),
            None,
        )
        .err()
        .expect("unlocated Loop must fail");

        assert!(error.contains("requires-located-loop-source"));
    }
}
