#![cfg(test)]

//! Caller-zero consumer for the nested resolver effect plan.
//!
//! This harness deliberately lives in tests. It exercises the existing
//! canonical identity/SSA owner in the sealed source order, but it is not a
//! production physicalizer or a route caller.

use std::collections::BTreeSet;

use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::resolved_lowering::canonical_ssa::ResolvedSsaIdentityStateV2;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::nested_predicate_effect_plan::{
    issue_nested_binding_execution_claims_v1, NestedBindingEffectEntryV1,
    NestedBindingEffectRoleV1, VerifiedNestedBindingEffectPlanV1,
    VerifiedNestedBindingExecutionClaimsV1,
};
use crate::mir::compiler::nested_predicate_producer::produce_nested_predicate_recipe_v1;
use crate::mir::compiler::nested_predicate_projection::issue_nested_predicate_source_projection_v1;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::BasicBlockId;
use crate::mir::ValueId;

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn assign(name: &str, value: i64) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(integer(value)),
        span: Span::unknown(),
    }
}

fn increment(name: &str) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(variable(name)),
            right: Box::new(integer(1)),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }
}

fn nested_function() -> ASTNode {
    let child = ASTNode::Loop {
        condition: Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(variable("j")),
            right: Box::new(integer(3)),
            span: Span::unknown(),
        }),
        body: vec![increment("sum"), increment("j")],
        span: Span::unknown(),
    };
    ASTNode::FunctionDeclaration {
        name: "nested_loop_minimal".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![
            ASTNode::Local {
                variables: vec!["i".into(), "sum".into()],
                initial_values: vec![Some(Box::new(integer(0))), Some(Box::new(integer(0)))],
                declared_type_names: vec![None, None],
                span: Span::unknown(),
            },
            ASTNode::Loop {
                condition: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Less,
                    left: Box::new(variable("i")),
                    right: Box::new(integer(3)),
                    span: Span::unknown(),
                }),
                body: vec![
                    ASTNode::Local {
                        variables: vec!["j".into()],
                        initial_values: vec![None],
                        declared_type_names: vec![None],
                        span: Span::unknown(),
                    },
                    assign("j", 0),
                    child,
                    increment("i"),
                ],
                span: Span::unknown(),
            },
        ],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

struct NestedEffectAdapter<'plan, 'source> {
    identity: &'plan mut ResolvedSsaIdentityStateV2<'source>,
    plan: &'plan VerifiedNestedBindingEffectPlanV1,
    claimed: BTreeSet<NestedBindingEffectRoleV1>,
    next_role: usize,
    child_activated: bool,
    block: BasicBlockId,
}

impl<'plan, 'source> NestedEffectAdapter<'plan, 'source> {
    fn new(
        identity: &'plan mut ResolvedSsaIdentityStateV2<'source>,
        plan: &'plan VerifiedNestedBindingEffectPlanV1,
    ) -> Self {
        Self {
            identity,
            plan,
            claimed: BTreeSet::new(),
            next_role: 0,
            child_activated: false,
            block: BasicBlockId::new(0),
        }
    }

    fn publish_initialized_prefix(
        &mut self,
        claims: &VerifiedNestedBindingExecutionClaimsV1,
    ) -> Result<(), String> {
        for (index, binding) in claims.prefix().initialized().iter().enumerate() {
            let value = ValueId::new(10 + index as u32);
            let published = self.identity.publish_declaration(
                binding.declaration_site(),
                binding.kind(),
                binding.name(),
                self.block,
                value,
            )?;
            if published != binding.binding() {
                return Err("[freeze:contract][nested_effect/prefix_binding_mismatch]".into());
            }
        }
        Ok(())
    }

    fn activate_child_declaration(
        &mut self,
        claims: &VerifiedNestedBindingExecutionClaimsV1,
    ) -> Result<(), String> {
        if self.child_activated {
            return Err("[freeze:contract][nested_effect/child_declaration_duplicate]".into());
        }
        if self.next_role != 1 {
            return Err("[freeze:contract][nested_effect/child_declaration_order]".into());
        }
        let binding = claims.prefix().uninitialized();
        let activated = self.identity.activate_declaration_without_value(
            binding.declaration_site(),
            binding.kind(),
            binding.name(),
        )?;
        if activated != binding.binding() {
            return Err("[freeze:contract][nested_effect/child_binding_mismatch]".into());
        }
        self.child_activated = true;
        Ok(())
    }

    fn consume_role(
        &mut self,
        role: NestedBindingEffectRoleV1,
        builder: &mut MirBuilder,
        phis: &mut PhiTxn,
    ) -> Result<(), String> {
        let expected = NestedBindingEffectRoleV1::ALL
            .get(self.next_role)
            .copied()
            .ok_or_else(|| "[freeze:contract][nested_effect/plan_exhausted]".to_string())?;
        if role != expected {
            return Err(format!(
                "[freeze:contract][nested_effect/order_mismatch] expected={expected:?} actual={role:?}"
            ));
        }
        if role == NestedBindingEffectRoleV1::ChildInitializeWriteJ && !self.child_activated {
            return Err("[freeze:contract][nested_effect/child_not_activated]".into());
        }
        if !self.claimed.insert(role) {
            return Err(format!(
                "[freeze:contract][nested_effect/duplicate] role={role:?}"
            ));
        }
        let result: Result<(), String> = (|| match self.plan.entry(role) {
            NestedBindingEffectEntryV1::Read(claim) => {
                self.identity
                    .claim_variable_use_binding(claim.site(), claim.binding())?;
                self.identity
                    .read_entry(builder, phis, self.block, claim.binding())?;
                Ok(())
            }
            NestedBindingEffectEntryV1::FirstAssignment(claim) => {
                self.identity.define_assignment_exact(
                    claim.target_site(),
                    claim.binding(),
                    self.block,
                    ValueId::new(
                        u32::try_from(claim.value())
                            .map_err(|_| "[freeze:contract][nested_effect/value_out_of_range]")?,
                    ),
                )
            }
            NestedBindingEffectEntryV1::Assignment(claim) => self.identity.define_assignment_exact(
                claim.target_site(),
                claim.binding(),
                self.block,
                ValueId::new(
                    u32::try_from(claim.delta())
                        .map_err(|_| "[freeze:contract][nested_effect/delta_out_of_range]")?,
                ),
            ),
        })();
        result.map_err(|error| format!("[role={role:?}] {error}"))?;
        self.next_role += 1;
        Ok(())
    }

    fn finish(&self) -> Result<(), String> {
        if !self.child_activated || self.next_role != NestedBindingEffectRoleV1::ALL.len() {
            return Err("[freeze:contract][nested_effect/incomplete]".into());
        }
        Ok(())
    }
}

fn claims_for(unit: &VerifiedResolvedSourceUnitV1) -> VerifiedNestedBindingExecutionClaimsV1 {
    let input = unit.root_function_input().expect("root function input");
    let body = input.source().root_body().expect("root body");
    let root = input.source().body_stmt(&body, 1).expect("root loop");
    let projection = issue_nested_predicate_source_projection_v1(input, &root)
        .expect("nested source projection");
    let product = produce_nested_predicate_recipe_v1(projection).expect("nested recipe product");
    issue_nested_binding_execution_claims_v1(input.function(), product.source_handoff())
        .expect("nested execution claims")
}

#[test]
fn ordered_effect_adapter_consumes_all_claims_once() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(nested_function())
        .expect("nested function resolves");
    let claims = claims_for(&unit);
    let input = unit.root_function_input().expect("root function input");
    let mut identity = ResolvedSsaIdentityStateV2::new(input.function());
    let mut adapter = NestedEffectAdapter::new(&mut identity, claims.effect_plan());
    adapter
        .publish_initialized_prefix(&claims)
        .expect("initialized prefix");
    let mut builder = MirBuilder::new();
    let mut phis = PhiTxn::begin("nested-effect-test");
    adapter
        .consume_role(
            NestedBindingEffectRoleV1::RootPredicateReadI,
            &mut builder,
            &mut phis,
        )
        .expect("root predicate");
    adapter
        .activate_child_declaration(&claims)
        .expect("child declaration-only activation");
    for role in NestedBindingEffectRoleV1::ALL.iter().copied().skip(1) {
        adapter
            .consume_role(role, &mut builder, &mut phis)
            .expect("ordered effect claim");
    }
    adapter.finish().expect("all effects consumed");
}

#[test]
fn effect_adapter_rejects_out_of_order_and_duplicate_claims() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(nested_function())
        .expect("nested function resolves");
    let claims = claims_for(&unit);
    let input = unit.root_function_input().expect("root function input");
    let mut identity = ResolvedSsaIdentityStateV2::new(input.function());
    let mut adapter = NestedEffectAdapter::new(&mut identity, claims.effect_plan());
    adapter
        .publish_initialized_prefix(&claims)
        .expect("initialized prefix");
    let mut builder = MirBuilder::new();
    let mut phis = PhiTxn::begin("nested-effect-order-test");
    assert!(adapter
        .consume_role(
            NestedBindingEffectRoleV1::ChildPredicateReadJ,
            &mut builder,
            &mut phis
        )
        .unwrap_err()
        .contains("order_mismatch"));
    adapter
        .consume_role(
            NestedBindingEffectRoleV1::RootPredicateReadI,
            &mut builder,
            &mut phis,
        )
        .expect("root predicate");
    adapter
        .activate_child_declaration(&claims)
        .expect("child declaration-only activation");
    assert!(adapter
        .consume_role(
            NestedBindingEffectRoleV1::ChildPredicateReadJ,
            &mut builder,
            &mut phis
        )
        .unwrap_err()
        .contains("order_mismatch"));
    adapter
        .consume_role(
            NestedBindingEffectRoleV1::ChildInitializeWriteJ,
            &mut builder,
            &mut phis,
        )
        .expect("first child assignment");
    assert!(adapter
        .consume_role(
            NestedBindingEffectRoleV1::ChildInitializeWriteJ,
            &mut builder,
            &mut phis,
        )
        .unwrap_err()
        .contains("order_mismatch"));
}
