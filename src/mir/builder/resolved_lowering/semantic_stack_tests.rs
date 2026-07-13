use std::collections::BTreeMap;

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::source_view::ExprChildRoleV1;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_semantics::{BindingRefV1, RegionKindV1, ScopeKindV1};
use crate::mir::ValueId;

use super::branch_transaction::{
    AuthorizedBranchRebindV1, BranchValueStoreV1, ResolvedActiveEffectStackV1,
    ResolvedBranchTransactionV1, ResolvedEffectBindingClassV1,
};
use super::identity::ResolvedIdentityStateV1;
use super::semantic_stack::{ResolvedSemanticExpectedCountsV1, ResolvedSemanticStackV1};

fn int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn block(tail: ASTNode) -> ASTNode {
    ASTNode::BlockExpr {
        prelude_stmts: Vec::new(),
        tail_expr: Box::new(tail),
        span: Span::unknown(),
    }
}

fn returning(name: &str, expression: ASTNode) -> ASTNode {
    function(
        name,
        vec![ASTNode::Return {
            value: Some(Box::new(expression)),
            span: Span::unknown(),
        }],
    )
}

fn function(name: &str, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn local(name: &str, value: i64) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.into()],
        initial_values: vec![Some(Box::new(int(value)))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

#[derive(Default)]
struct EffectStoreV1(BTreeMap<BindingRefV1, ValueId>);

impl BranchValueStoreV1 for EffectStoreV1 {
    fn branch_current_value(&self, binding: BindingRefV1) -> Result<ValueId, String> {
        self.0
            .get(&binding)
            .copied()
            .ok_or_else(|| "[test/effect_value_missing]".to_string())
    }

    fn branch_rebind_authorized(
        &mut self,
        authorization: AuthorizedBranchRebindV1,
    ) -> Result<ValueId, String> {
        let slot = self
            .0
            .get_mut(&authorization.binding())
            .ok_or_else(|| "[test/effect_rebind_missing]".to_string())?;
        let old = *slot;
        *slot = authorization.value();
        Ok(old)
    }
}

fn nested_block_fixture() -> VerifiedResolvedSourceUnitV1 {
    VerifiedResolvedSourceUnitV1::resolve_function(returning(
        "semantic_stack_nested",
        block(block(int(1))),
    ))
    .unwrap()
}

fn nested_pairs(
    unit: &VerifiedResolvedSourceUnitV1,
) -> (
    crate::mir::resolved_semantics::ResolvedScopeRegionPairV1,
    crate::mir::resolved_semantics::ResolvedScopeRegionPairV1,
) {
    let input = unit.root_function_input().unwrap();
    let body = input.source().root_body().unwrap();
    let statement = input.source().body_stmt(&body, 0).unwrap();
    let outer = input
        .source()
        .child_expr_from_stmt(&statement, ExprChildRoleV1::ReturnValue)
        .unwrap();
    let inner = input
        .source()
        .child_expr_from_expr(&outer, ExprChildRoleV1::BlockExprTail)
        .unwrap();
    (
        input
            .function()
            .block_expr_scope_region_pair(outer.owner(), outer.site())
            .unwrap(),
        input
            .function()
            .block_expr_scope_region_pair(inner.owner(), inner.site())
            .unwrap(),
    )
}

#[test]
fn roots_seed_both_stacks_and_finish_requires_verified_count() {
    let unit = nested_block_fixture();
    let input = unit.root_function_input().unwrap();
    let semantics =
        ResolvedSemanticStackV1::new(input.function(), input.function().lowering_roots(), 2)
            .unwrap();
    assert_eq!(semantics.depths(), (2, 2));
    assert!(semantics.finish().is_err());

    let empty = VerifiedResolvedSourceUnitV1::resolve_function(function(
        "semantic_stack_empty",
        Vec::new(),
    ))
    .unwrap();
    let input = empty.root_function_input().unwrap();
    let semantics =
        ResolvedSemanticStackV1::new(input.function(), input.function().lowering_roots(), 0)
            .unwrap();
    assert_eq!(semantics.depths(), (2, 2));
    semantics.finish().unwrap();
}

#[test]
fn outermost_and_nested_block_expr_pairs_balance_exactly() {
    let unit = nested_block_fixture();
    let input = unit.root_function_input().unwrap();
    let (outer, inner) = nested_pairs(&unit);
    let mut semantics =
        ResolvedSemanticStackV1::new(input.function(), input.function().lowering_roots(), 2)
            .unwrap();
    let mut identity = ResolvedIdentityStateV1::new(input.function());

    let outer_session = semantics.enter_block_expr(input.function(), outer).unwrap();
    let inner_session = semantics.enter_block_expr(input.function(), inner).unwrap();
    assert_eq!(semantics.depths(), (4, 4));
    semantics
        .close_scope_region_error(inner_session, &mut identity)
        .unwrap();
    semantics
        .close_scope_region_error(outer_session, &mut identity)
        .unwrap();
    assert_eq!(semantics.depths(), (2, 2));
    semantics.finish().unwrap();
}

#[test]
fn wrong_parent_lifo_and_reconsumption_are_rejected() {
    let unit = nested_block_fixture();
    let input = unit.root_function_input().unwrap();
    let (outer, inner) = nested_pairs(&unit);

    let mut wrong_parent =
        ResolvedSemanticStackV1::new(input.function(), input.function().lowering_roots(), 2)
            .unwrap();
    assert!(wrong_parent
        .enter_block_expr(input.function(), inner)
        .is_err());

    let mut lifo =
        ResolvedSemanticStackV1::new(input.function(), input.function().lowering_roots(), 2)
            .unwrap();
    let mut identity = ResolvedIdentityStateV1::new(input.function());
    let outer_session = lifo.enter_block_expr(input.function(), outer).unwrap();
    let _inner_session = lifo.enter_block_expr(input.function(), inner).unwrap();
    assert!(lifo
        .close_scope_region_error(outer_session, &mut identity)
        .is_err());

    let mut reconsume =
        ResolvedSemanticStackV1::new(input.function(), input.function().lowering_roots(), 2)
            .unwrap();
    let mut identity = ResolvedIdentityStateV1::new(input.function());
    let outer_session = reconsume.enter_block_expr(input.function(), outer).unwrap();
    reconsume
        .close_scope_region_error(outer_session, &mut identity)
        .unwrap();
    assert!(reconsume.enter_block_expr(input.function(), outer).is_err());
}

#[test]
fn if_control_is_region_only_and_branch_pair_uses_distinct_parents() {
    let root = function(
        "semantic_stack_if",
        vec![ASTNode::If {
            condition: Box::new(int(1)),
            then_body: Vec::new(),
            else_body: None,
            span: Span::unknown(),
        }],
    );
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(root).unwrap();
    let input = unit.root_function_input().unwrap();
    let body = input.source().root_body().unwrap();
    let statement = input.source().body_stmt(&body, 0).unwrap();
    let bundle = *input.function().if_region_bundle(statement.site()).unwrap();
    let mut semantics = ResolvedSemanticStackV1::new_with_expectations(
        input.function(),
        input.function().lowering_roots(),
        ResolvedSemanticExpectedCountsV1::new(0, 1, 1),
    )
    .unwrap();
    let mut identity = ResolvedIdentityStateV1::new(input.function());

    assert_eq!(semantics.depths(), (2, 2));
    let control = semantics
        .enter_region(input.function(), bundle.control(), RegionKindV1::If)
        .unwrap();
    assert_eq!(semantics.depths(), (3, 2));
    let branch = semantics
        .enter_scope_region(
            input.function(),
            bundle.then_pair(),
            ScopeKindV1::IfThen,
            RegionKindV1::IfThen,
        )
        .unwrap();
    assert_eq!(semantics.depths(), (4, 3));
    semantics
        .close_scope_region_error(branch, &mut identity)
        .unwrap();
    semantics.close_region(control).unwrap();
    assert_eq!(semantics.depths(), (2, 2));
    semantics.finish().unwrap();
}

#[test]
fn finish_requires_exact_if_control_and_branch_counts() {
    let root = function(
        "semantic_stack_if_counts",
        vec![ASTNode::If {
            condition: Box::new(int(1)),
            then_body: Vec::new(),
            else_body: Some(Vec::new()),
            span: Span::unknown(),
        }],
    );
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(root).unwrap();
    let input = unit.root_function_input().unwrap();
    let body = input.source().root_body().unwrap();
    let statement = input.source().body_stmt(&body, 0).unwrap();
    let bundle = *input.function().if_region_bundle(statement.site()).unwrap();
    let mut identity = ResolvedIdentityStateV1::new(input.function());
    let mut semantics = ResolvedSemanticStackV1::new_with_expectations(
        input.function(),
        input.function().lowering_roots(),
        ResolvedSemanticExpectedCountsV1::new(0, 1, 2),
    )
    .unwrap();
    let control = semantics
        .enter_region(input.function(), bundle.control(), RegionKindV1::If)
        .unwrap();
    let then_session = semantics
        .enter_scope_region(
            input.function(),
            bundle.then_pair(),
            ScopeKindV1::IfThen,
            RegionKindV1::IfThen,
        )
        .unwrap();
    semantics
        .close_scope_region_error(then_session, &mut identity)
        .unwrap();
    semantics.close_region(control).unwrap();
    assert!(semantics.finish().unwrap_err().contains("if_branches=1/2"));
}

#[test]
fn active_effect_stack_distinguishes_condition_and_branch_locality() {
    let condition = ASTNode::BlockExpr {
        prelude_stmts: vec![local("c", 2)],
        tail_expr: Box::new(int(1)),
        span: Span::unknown(),
    };
    let root = function(
        "semantic_effect_stack",
        vec![
            local("x", 0),
            ASTNode::If {
                condition: Box::new(condition),
                then_body: vec![local("y", 3)],
                else_body: Some(vec![local("z", 4)]),
                span: Span::unknown(),
            },
        ],
    );
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(root).unwrap();
    let input = unit.root_function_input().unwrap();
    let product = input.function();
    let bindings = product
        .bindings()
        .map(|(binding, record)| (record.diagnostic_name().to_string(), binding))
        .collect::<BTreeMap<_, _>>();
    let (x, c, y, z) = (bindings["x"], bindings["c"], bindings["y"], bindings["z"]);
    let body_scope = product.lowering_roots().body_pair().scope();
    let body = input.source().root_body().unwrap();
    let if_stmt = input.source().body_stmt(&body, 1).unwrap();
    let bundle = *product.if_region_bundle(if_stmt.site()).unwrap();
    let then_scope = bundle.then_pair().scope();

    let mut values = EffectStoreV1(
        [
            (x, ValueId::new(1)),
            (c, ValueId::new(2)),
            (y, ValueId::new(3)),
        ]
        .into_iter()
        .collect(),
    );
    let mut effects = ResolvedActiveEffectStackV1::new();
    effects.push_condition(product, body_scope, &[x]).unwrap();
    assert_eq!(
        effects.authorize_current(product, x).unwrap(),
        ResolvedEffectBindingClassV1::Visible
    );
    assert_eq!(
        effects.authorize_current(product, c).unwrap(),
        ResolvedEffectBindingClassV1::Local
    );
    assert!(effects.authorize_current(product, y).is_err());
    effects
        .rebind_current(&mut values, product, x, ValueId::new(10))
        .unwrap();
    effects.finish_condition(body_scope).unwrap();
    assert_eq!(values.0[&x], ValueId::new(10));

    let transaction = ResolvedBranchTransactionV1::snapshot(&values, &[x], &[x]).unwrap();
    effects
        .push_branch(product, then_scope, transaction)
        .unwrap();
    assert_eq!(
        effects.authorize_current(product, y).unwrap(),
        ResolvedEffectBindingClassV1::Local
    );
    assert!(effects.authorize_current(product, z).is_err());
    effects.prime_current_effects(product, &[x, y]).unwrap();
    effects
        .rebind_current(&mut values, product, x, ValueId::new(11))
        .unwrap();
    effects
        .rebind_current(&mut values, product, y, ValueId::new(12))
        .unwrap();
    let (_transaction, exit) = effects.capture_branch(&mut values, then_scope).unwrap();
    assert_eq!(values.0[&x], ValueId::new(10));
    assert_eq!(values.0[&y], ValueId::new(12));
    assert!(effects.is_empty());
    let _ = exit;
}
