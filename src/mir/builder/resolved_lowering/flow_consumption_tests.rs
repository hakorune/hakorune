use std::collections::BTreeMap;

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::resolved_region_flow::{
    analyze_resolved_function_flow_v1, VerifiedResolvedFunctionFlowV1,
};
use crate::mir::resolved_semantics::BindingRefV1;
use crate::mir::ValueId;

use super::branch_transaction::{
    AuthorizedBranchRebindV1, BranchValueStoreV1, ResolvedBranchTransactionV1,
    ResolvedEffectBindingClassV1,
};
use super::flow_consumption::ResolvedFlowConsumptionV1;

fn int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
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

fn assign(name: &str, value: i64) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(var(name)),
        value: Box::new(int(value)),
        span: Span::unknown(),
    }
}

fn if_stmt(then_body: Vec<ASTNode>, else_body: Option<Vec<ASTNode>>) -> ASTNode {
    ASTNode::If {
        condition: Box::new(int(1)),
        then_body,
        else_body,
        span: Span::unknown(),
    }
}

fn flow(body: Vec<ASTNode>) -> VerifiedResolvedFunctionFlowV1 {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(ASTNode::FunctionDeclaration {
        name: "flow_consumption".into(),
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
    })
    .unwrap();
    let input = unit.root_function_input().unwrap();
    let completion = verify_function_completion_v1(input).unwrap();
    analyze_resolved_function_flow_v1(input, &completion).unwrap()
}

fn nested_flow() -> VerifiedResolvedFunctionFlowV1 {
    flow(vec![
        local("x", 0),
        if_stmt(
            vec![
                assign("x", 1),
                if_stmt(vec![assign("x", 2)], Some(Vec::new())),
                assign("x", 3),
            ],
            None,
        ),
    ])
}

struct ValueStoreV1(BTreeMap<BindingRefV1, ValueId>);

impl BranchValueStoreV1 for ValueStoreV1 {
    fn branch_current_value(&self, binding: BindingRefV1) -> Result<ValueId, String> {
        self.0
            .get(&binding)
            .copied()
            .ok_or_else(|| "missing".into())
    }

    fn branch_rebind_authorized(
        &mut self,
        authorization: AuthorizedBranchRebindV1,
    ) -> Result<ValueId, String> {
        let value = self.0.get_mut(&authorization.binding()).ok_or("missing")?;
        let old = *value;
        *value = authorization.value();
        Ok(old)
    }
}

#[test]
fn source_preorder_and_nested_coverage_frames_are_exact() {
    let flow = nested_flow();
    let outer_site = flow.if_flows()[0].site().clone();
    let inner_site = flow.if_flows()[1].site().clone();
    let mut cursor = ResolvedFlowConsumptionV1::new(flow);
    assert_eq!(cursor.expected_if_control_regions(), 2);
    assert_eq!(cursor.expected_if_branch_pairs(), 3);

    let outer = cursor.claim_next_if(&outer_site).unwrap();
    cursor.begin_condition(&outer).unwrap();
    cursor.finish_condition(&outer_site).unwrap();
    cursor.begin_then(&outer).unwrap();
    let binding = outer.then_port().may_rebind_outer()[0];
    cursor
        .claim_assignment(
            &outer.coverage().then_direct()[0],
            binding,
            ResolvedEffectBindingClassV1::Visible,
        )
        .unwrap();

    let inner = cursor.claim_next_if(&inner_site).unwrap();
    cursor.begin_condition(&inner).unwrap();
    cursor.finish_condition(&inner_site).unwrap();
    cursor.begin_then(&inner).unwrap();
    cursor
        .claim_assignment(
            &inner.coverage().then_direct()[0],
            binding,
            ResolvedEffectBindingClassV1::Visible,
        )
        .unwrap();
    cursor.finish_then(&inner_site).unwrap();
    cursor.begin_else(&inner).unwrap();
    cursor.finish_else(&inner_site).unwrap();

    cursor
        .claim_assignment(
            &outer.coverage().then_direct()[1],
            binding,
            ResolvedEffectBindingClassV1::Visible,
        )
        .unwrap();
    cursor.finish_then(&outer_site).unwrap();
    assert_eq!(cursor.coverage_depth(), 1);
    cursor.finish().unwrap();
}

#[test]
fn wrong_row_order_assignment_order_and_incomplete_frames_fail_fast() {
    let flow = nested_flow();
    let outer_site = flow.if_flows()[0].site().clone();
    let inner_site = flow.if_flows()[1].site().clone();
    let mut cursor = ResolvedFlowConsumptionV1::new(flow);
    assert!(cursor
        .claim_next_if(&inner_site)
        .unwrap_err()
        .contains("source_preorder_mismatch"));

    let outer = cursor.claim_next_if(&outer_site).unwrap();
    cursor.begin_condition(&outer).unwrap();
    cursor.finish_condition(&outer_site).unwrap();
    cursor.begin_then(&outer).unwrap();
    let binding = outer.then_port().may_rebind_outer()[0];
    assert!(cursor
        .claim_assignment(
            &outer.coverage().then_direct()[1],
            binding,
            ResolvedEffectBindingClassV1::Visible,
        )
        .unwrap_err()
        .contains("assignment_order_mismatch"));
    assert!(cursor
        .finish_then(&outer_site)
        .unwrap_err()
        .contains("incomplete"));
    cursor.abort_then(&outer_site).unwrap();
    assert_eq!(cursor.coverage_depth(), 1);
    assert!(cursor.finish().is_err());
}

#[test]
fn visible_binding_must_belong_to_the_frame_effect_port() {
    let flow = nested_flow();
    let outer_site = flow.if_flows()[0].site().clone();
    let inner_binding = flow.if_flows()[1].then_port().may_rebind_outer()[0];
    let mut cursor = ResolvedFlowConsumptionV1::new(flow);
    let outer = cursor.claim_next_if(&outer_site).unwrap();
    cursor.begin_condition(&outer).unwrap();
    cursor.finish_condition(&outer_site).unwrap();
    cursor.begin_then(&outer).unwrap();

    // The exact site is right, but an unrelated sealed binding is not enough.
    let mut foreign =
        crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation().unwrap();
    let foreign = crate::mir::resolved_semantics::BindingRefV1::new(
        foreign.issue().unwrap(),
        hakorune_mir_core::BindingId::new(0),
    );
    assert_ne!(foreign, inner_binding);
    assert!(cursor
        .claim_assignment(
            &outer.coverage().then_direct()[0],
            foreign,
            ResolvedEffectBindingClassV1::Visible,
        )
        .unwrap_err()
        .contains("assignment_effect_mismatch"));
}

#[test]
fn join_rows_consume_the_sealed_per_binding_source_matrix() {
    let flow = flow(vec![local("x", 0), if_stmt(vec![assign("x", 1)], None)]);
    let row = &flow.if_flows()[0];
    let binding = row.join().rows()[0].binding();
    let entry = ValueId::new(70);
    let mut values = ValueStoreV1([(binding, entry)].into_iter().collect());
    let mut transaction = ResolvedBranchTransactionV1::snapshot(
        &values,
        &[binding],
        row.then_port().may_rebind_outer(),
    )
    .unwrap();
    transaction
        .rebind(&mut values, binding, ValueId::new(71))
        .unwrap();
    let then_values = transaction.capture_and_restore(&mut values).unwrap();
    let else_values = transaction.implicit_false_values();
    let rows = transaction
        .join_rows_for_contract(row.join(), &then_values, &else_values)
        .unwrap();
    assert_eq!(rows[0].entry(), entry);
    assert_eq!(rows[0].then_value(), ValueId::new(71));
    assert_eq!(rows[0].else_value(), entry);
}
