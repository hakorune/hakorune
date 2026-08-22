use hakorune_mir_core::BindingId;
use std::collections::BTreeMap;

use super::*;
use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{
    CallableFunctionSyntaxViewV1, FunctionOwnerIssuerV1, FunctionSemanticResolverSessionV1,
    ResolveSelectedCallableForestsOutcomeV1, SourcePathV1,
};
use crate::parser::NyashParser;

fn owner() -> FunctionOwnerIdV1 {
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
    issuer.issue().unwrap()
}

fn binding(owner: FunctionOwnerIdV1, slot: u32) -> BindingRefV1 {
    BindingRefV1::new(owner, BindingId::new(slot))
}

fn parsed_skip_while() -> ASTNode {
    parsed_method(
        include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"),
        "ParserScanLoopBox",
        "skip_while",
    )
}

fn parsed_method(source: &str, box_name: &str, method_name: &str) -> ASTNode {
    let program =
        NyashParser::parse_from_string(source).expect("production parser scan loop source");
    let ASTNode::Program { statements, .. } = program else {
        panic!("parser must return Program")
    };
    statements
        .into_iter()
        .find_map(|statement| match statement {
            ASTNode::BoxDeclaration { name, methods, .. } if name == box_name => {
                methods.get_declaration(method_name).cloned()
            }
            _ => None,
        })
        .expect("production skip_while declaration")
}

#[test]
fn seals_exact_condition_body_and_assignment_roles() {
    let owner = owner();
    let loop_site = SourcePathV1::root_body(2).node();
    let condition = SourcePathV1::from_node(&loop_site)
        .child(SourcePathSegmentV1::LoopCondition)
        .child(SourcePathSegmentV1::Lhs)
        .node();
    let body_read = SourcePathV1::from_node(&loop_site)
        .child(SourcePathSegmentV1::LoopBody(0))
        .child(SourcePathSegmentV1::Value)
        .child(SourcePathSegmentV1::Lhs)
        .node();
    let target = SourcePathV1::from_node(&loop_site)
        .child(SourcePathSegmentV1::LoopBody(0))
        .child(SourcePathSegmentV1::Target)
        .node();
    let schedule = VerifiedCallableSemanticLoopBindingScheduleV1::seal(
        owner,
        loop_site.clone(),
        vec![
            CallableLoopBindingReceiptV1::new(
                condition.clone(),
                binding(owner, 0),
                CallableLoopBindingRoleV1::ConditionRead,
            ),
            CallableLoopBindingReceiptV1::new(
                body_read.clone(),
                binding(owner, 0),
                CallableLoopBindingRoleV1::BodyRead,
            ),
            CallableLoopBindingReceiptV1::new(
                target.clone(),
                binding(owner, 0),
                CallableLoopBindingRoleV1::BodyRebind,
            ),
        ],
        BTreeSet::new(),
    )
    .unwrap();
    let receipt = schedule
        .consume_pre_effect(
            &loop_site,
            &SourcePathV1::from_node(&loop_site)
                .child(SourcePathSegmentV1::LoopCondition)
                .node(),
            &SourcePathV1::from_node(&loop_site)
                .child(SourcePathSegmentV1::LoopBodyRoot)
                .node(),
        )
        .unwrap();
    assert_eq!(receipt.owner(), owner);
    assert_eq!(receipt.loop_site(), &loop_site);
    assert_eq!(receipt.rows().len(), 1);
    assert_eq!(
        receipt.rows()[0].class(),
        CallableLoopBindingClassV1::Carrier
    );
    assert_eq!(receipt.rows()[0].receipts().len(), 3);
}

#[test]
fn rejects_foreign_duplicate_and_nested_receipts() {
    let owner_id = owner();
    let foreign = owner();
    let loop_site = SourcePathV1::root_body(2).node();
    let condition = SourcePathV1::from_node(&loop_site)
        .child(SourcePathSegmentV1::LoopCondition)
        .child(SourcePathSegmentV1::Lhs)
        .node();
    let receipt = || {
        CallableLoopBindingReceiptV1::new(
            condition.clone(),
            binding(owner_id, 0),
            CallableLoopBindingRoleV1::ConditionRead,
        )
    };
    assert!(VerifiedCallableSemanticLoopBindingScheduleV1::seal(
        owner_id,
        loop_site.clone(),
        vec![CallableLoopBindingReceiptV1::new(
            condition.clone(),
            binding(foreign, 0),
            CallableLoopBindingRoleV1::ConditionRead,
        )],
        BTreeSet::new(),
    )
    .is_err());
    assert!(VerifiedCallableSemanticLoopBindingScheduleV1::seal(
        owner_id,
        loop_site.clone(),
        vec![receipt(), receipt()],
        BTreeSet::new(),
    )
    .is_err());
    assert!(VerifiedCallableSemanticLoopBindingScheduleV1::seal(
        owner_id,
        loop_site.clone(),
        Vec::new(),
        BTreeSet::new(),
    )
    .is_err());
    assert!(VerifiedCallableSemanticLoopBindingScheduleV1::seal(
        owner_id,
        loop_site.clone(),
        vec![receipt()],
        BTreeSet::new(),
    )
    .is_err());
    let nested = SourcePathV1::from_node(&loop_site)
        .child(SourcePathSegmentV1::LoopBody(0))
        .child(SourcePathSegmentV1::LoopCondition)
        .child(SourcePathSegmentV1::Lhs)
        .node();
    assert!(VerifiedCallableSemanticLoopBindingScheduleV1::seal(
        owner_id,
        loop_site,
        vec![CallableLoopBindingReceiptV1::new(
            nested,
            binding(owner_id, 0),
            CallableLoopBindingRoleV1::BodyRead,
        )],
        BTreeSet::new(),
    )
    .is_err());
}

#[test]
fn variable_reads_are_rows_not_fixed_counts_or_cross_binding_repair() {
    let owner_id = owner();
    let foreign = owner();
    let loop_site = SourcePathV1::root_body(2).node();
    let condition = |side| {
        SourcePathV1::from_node(&loop_site)
            .child(SourcePathSegmentV1::LoopCondition)
            .child(side)
            .node()
    };
    let body = |index, tail| {
        SourcePathV1::from_node(&loop_site)
            .child(SourcePathSegmentV1::LoopBody(index))
            .child(tail)
            .node()
    };
    let carrier = binding(owner_id, 0);
    let read_only = binding(owner_id, 1);
    let iteration_local = binding(owner_id, 2);
    let receipts = vec![
        CallableLoopBindingReceiptV1::new(
            condition(SourcePathSegmentV1::Lhs),
            carrier,
            CallableLoopBindingRoleV1::ConditionRead,
        ),
        CallableLoopBindingReceiptV1::new(
            condition(SourcePathSegmentV1::Rhs),
            read_only,
            CallableLoopBindingRoleV1::ConditionRead,
        ),
        CallableLoopBindingReceiptV1::new(
            body(0, SourcePathSegmentV1::Value),
            iteration_local,
            CallableLoopBindingRoleV1::BodyRead,
        ),
        CallableLoopBindingReceiptV1::new(
            body(1, SourcePathSegmentV1::Lhs),
            carrier,
            CallableLoopBindingRoleV1::BodyRead,
        ),
        CallableLoopBindingReceiptV1::new(
            body(1, SourcePathSegmentV1::Target),
            carrier,
            CallableLoopBindingRoleV1::BodyRebind,
        ),
    ];
    let schedule = VerifiedCallableSemanticLoopBindingScheduleV1::seal(
        owner_id,
        loop_site.clone(),
        receipts,
        BTreeSet::from([iteration_local]),
    )
    .expect("variable exact reads are admitted by relation");
    assert_eq!(schedule.receipt_count(), 5);
    assert_eq!(schedule.rows().len(), 3);

    let cross_binding = vec![
        CallableLoopBindingReceiptV1::new(
            condition(SourcePathSegmentV1::Lhs),
            carrier,
            CallableLoopBindingRoleV1::ConditionRead,
        ),
        CallableLoopBindingReceiptV1::new(
            body(0, SourcePathSegmentV1::Lhs),
            read_only,
            CallableLoopBindingRoleV1::BodyRead,
        ),
        CallableLoopBindingReceiptV1::new(
            body(0, SourcePathSegmentV1::Target),
            carrier,
            CallableLoopBindingRoleV1::BodyRebind,
        ),
    ];
    assert!(VerifiedCallableSemanticLoopBindingScheduleV1::seal(
        owner_id,
        loop_site.clone(),
        cross_binding,
        BTreeSet::new(),
    )
    .is_err());
    assert!(VerifiedCallableSemanticLoopBindingScheduleV1::seal(
        owner_id,
        loop_site.clone(),
        vec![
            CallableLoopBindingReceiptV1::new(
                condition(SourcePathSegmentV1::Lhs),
                carrier,
                CallableLoopBindingRoleV1::ConditionRead,
            ),
            CallableLoopBindingReceiptV1::new(
                body(0, SourcePathSegmentV1::Lhs),
                carrier,
                CallableLoopBindingRoleV1::BodyRead,
            ),
            CallableLoopBindingReceiptV1::new(
                body(0, SourcePathSegmentV1::Target),
                carrier,
                CallableLoopBindingRoleV1::BodyRebind,
            ),
        ],
        BTreeSet::from([binding(foreign, 9)]),
    )
    .is_err());
}

#[test]
fn body_only_rebind_is_explicit_outside_with_source_evidence() {
    let owner_id = owner();
    let loop_site = SourcePathV1::root_body(2).node();
    let carrier = binding(owner_id, 0);
    let outside = binding(owner_id, 1);
    let condition = SourcePathV1::from_node(&loop_site)
        .child(SourcePathSegmentV1::LoopCondition)
        .child(SourcePathSegmentV1::Lhs)
        .node();
    let carrier_read = SourcePathV1::from_node(&loop_site)
        .child(SourcePathSegmentV1::LoopBody(0))
        .child(SourcePathSegmentV1::Value)
        .child(SourcePathSegmentV1::Lhs)
        .node();
    let carrier_rebind = SourcePathV1::from_node(&loop_site)
        .child(SourcePathSegmentV1::LoopBody(0))
        .child(SourcePathSegmentV1::Target)
        .node();
    let outside_read = SourcePathV1::from_node(&loop_site)
        .child(SourcePathSegmentV1::LoopBody(1))
        .child(SourcePathSegmentV1::Value)
        .child(SourcePathSegmentV1::Lhs)
        .node();
    let outside_rebind = SourcePathV1::from_node(&loop_site)
        .child(SourcePathSegmentV1::LoopBody(1))
        .child(SourcePathSegmentV1::Target)
        .node();

    let mut variables = BTreeMap::new();
    variables.insert(condition.clone(), carrier);
    variables.insert(carrier_read, carrier);
    variables.insert(outside_read.clone(), outside);
    let mut assignments = BTreeMap::new();
    assignments.insert(carrier_rebind, carrier);
    assignments.insert(outside_rebind.clone(), outside);
    let locals = BTreeMap::new();
    let projection =
        CallableLoopSourceProjectionV1::new(owner_id, &locals, &variables, &assignments);

    let disposition = projection
        .project_disposition(loop_site.clone())
        .expect("complete body-only row is an explicit outside disposition");
    let CallableLoopBindingProjectionDispositionV1::Outside(reason) = disposition else {
        panic!("body-only rebind must not become Ready")
    };
    assert_eq!(reason.loop_site(), &loop_site);
    assert_eq!(reason.bindings(), &[outside]);
    assert_eq!(reason.sites(), &[outside_rebind, outside_read]);
    let terminal = reason.into_terminal_error();
    assert!(terminal.contains("callable-loop-handoff/outside-first-cohort"));
    assert!(terminal.contains("bindings=1"));
    assert!(terminal.contains("sites=2"));
}

#[test]
fn production_skip_while_keeps_one_carrier_and_variable_operand_rows() {
    let function = parsed_skip_while();
    let syntax =
        CallableFunctionSyntaxViewV1::from_function_ast(&function).expect("callable syntax view");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let ResolveSelectedCallableForestsOutcomeV1::Complete(mut forests) = resolver
        .resolve_selected_callable_forests(&[syntax.function()])
        .expect("resolved production function")
    else {
        panic!("production skip_while unexpectedly deferred")
    };
    let forest = forests
        .into_vec()
        .pop()
        .expect("one production function forest");
    let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
        &function,
        &forest,
        syntax.function().root_profile(),
    )
    .expect("source projection");
    let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
        &function,
        &forest,
        &projection,
    )
    .expect("resolved lowering input");
    let state = super::super::normal_callable_semantic_lowering_state::CallableSemanticLoweringState::from_exact_source(input)
            .expect("callable semantic lowering state");
    let schedule = state
        .loop_binding_source_projection()
        .project(SourcePathV1::root_body(1).node())
        .expect("production Loop source coverage");

    assert_eq!(
        schedule
            .rows()
            .iter()
            .filter(|row| row.class() == CallableLoopBindingClassV1::Carrier)
            .count(),
        1
    );
    assert!(
        schedule
            .rows()
            .iter()
            .filter(|row| row.class() == CallableLoopBindingClassV1::ReadOnlyOperand)
            .count()
            >= 3
    );
    assert!(schedule
        .rows()
        .iter()
        .any(|row| row.class() == CallableLoopBindingClassV1::IterationLocal));
    assert!(schedule.receipt_count() > 3);
}

#[test]
fn production_esc_json_uses_explicit_outside_for_body_only_rebinds() {
    let function = parsed_method(
        include_str!("../../../lang/src/compiler/parser/scan/parser_common_utils_box.hako"),
        "ParserCommonUtilsBox",
        "esc_json",
    );
    let syntax =
        CallableFunctionSyntaxViewV1::from_function_ast(&function).expect("callable syntax view");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let ResolveSelectedCallableForestsOutcomeV1::Complete(mut forests) = resolver
        .resolve_selected_callable_forests(&[syntax.function()])
        .expect("resolved production function")
    else {
        panic!("production esc_json unexpectedly deferred")
    };
    let forest = forests
        .into_vec()
        .pop()
        .expect("one production function forest");
    let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
        &function,
        &forest,
        syntax.function().root_profile(),
    )
    .expect("source projection");
    let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
        &function,
        &forest,
        &projection,
    )
    .expect("resolved lowering input");
    let state = super::super::normal_callable_semantic_lowering_state::CallableSemanticLoweringState::from_exact_source(input)
        .expect("callable semantic lowering state");
    let disposition = state
        .loop_binding_source_projection()
        .project_disposition(SourcePathV1::root_body(3).node())
        .expect("esc_json loop source projection");
    let CallableLoopBindingProjectionDispositionV1::Outside(reason) = disposition else {
        panic!("esc_json body-only rebinds must be Outside")
    };
    assert_eq!(reason.bindings().len(), 2);
    assert!(reason.sites().len() >= 4);
}
