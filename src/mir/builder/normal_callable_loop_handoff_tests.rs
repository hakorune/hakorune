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
        std::collections::BTreeMap::new(),
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
        CallableLoopReadyBindingClassV1::Carrier
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
        std::collections::BTreeMap::new(),
    )
    .is_err());
    assert!(VerifiedCallableSemanticLoopBindingScheduleV1::seal(
        owner_id,
        loop_site.clone(),
        vec![receipt(), receipt()],
        std::collections::BTreeMap::new(),
    )
    .is_err());
    assert!(VerifiedCallableSemanticLoopBindingScheduleV1::seal(
        owner_id,
        loop_site.clone(),
        Vec::new(),
        std::collections::BTreeMap::new(),
    )
    .is_err());
    assert!(VerifiedCallableSemanticLoopBindingScheduleV1::seal(
        owner_id,
        loop_site.clone(),
        vec![receipt()],
        std::collections::BTreeMap::new(),
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
        std::collections::BTreeMap::new(),
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
        BTreeMap::from([(
            iteration_local,
            SourceBindingSiteV1::Local {
                statement: SourcePathV1::from_node(&loop_site)
                    .child(SourcePathSegmentV1::LoopBody(0))
                    .stmt(),
                ordinal: 0,
            },
        )]),
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
        std::collections::BTreeMap::new(),
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
        BTreeMap::from([(
            binding(foreign, 9),
            SourceBindingSiteV1::Local {
                statement: SourcePathV1::from_node(&loop_site)
                    .child(SourcePathSegmentV1::LoopBody(0))
                    .stmt(),
                ordinal: 0
            }
        )]),
    )
    .is_err());
}

#[test]
fn body_only_rebind_moves_with_ready_remainder_and_source_evidence() {
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
    let CallableLoopBindingProjectionDispositionV1::ReadyWithBodyOnly(product) = disposition else {
        panic!("body-only rebind must move with the Ready remainder")
    };
    assert_eq!(product.loop_site(), &loop_site);
    assert_eq!(product.owner(), owner_id);
    assert_eq!(product.body_only_rows().len(), 1);
    let row = &product.body_only_rows()[0];
    assert_eq!(row.binding(), outside);
    assert_eq!(row.kind(), CallableLoopOutsideKindV1::BodyOnlyRebind);
    assert_eq!(row.receipts().len(), 2);
    assert_eq!(row.receipts()[0].binding(), outside);
    assert_eq!(row.receipts()[1].binding(), outside);
    assert!(row.receipts().iter().any(|receipt| {
        receipt.site() == &outside_read && receipt.role() == CallableLoopBindingRoleV1::BodyRead
    }));
    assert!(row.receipts().iter().any(|receipt| {
        receipt.site() == &outside_rebind && receipt.role() == CallableLoopBindingRoleV1::BodyRebind
    }));
    assert_eq!(row.kind(), CallableLoopOutsideKindV1::BodyOnlyRebind);
}

#[test]
fn production_skip_while_keeps_one_carrier_and_variable_operand_rows() {
    let function = parsed_skip_while();
    let syntax =
        CallableFunctionSyntaxViewV1::from_function_ast(&function).expect("callable syntax view");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let ResolveSelectedCallableForestsOutcomeV1::Complete(forests) = resolver
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
            .filter(|row| row.class() == CallableLoopReadyBindingClassV1::Carrier)
            .count(),
        1
    );
    assert!(
        schedule
            .rows()
            .iter()
            .filter(|row| row.class() == CallableLoopReadyBindingClassV1::ReadOnlyOperand)
            .count()
            >= 3
    );
    assert!(schedule
        .rows()
        .iter()
        .any(|row| row.class() == CallableLoopReadyBindingClassV1::IterationLocal));
    assert!(schedule.receipt_count() > 3);
}

#[test]
fn production_esc_json_keeps_body_only_rebinds_in_one_source_product() {
    let function = parsed_method(
        include_str!("../../../lang/src/compiler/parser/scan/parser_common_utils_box.hako"),
        "ParserCommonUtilsBox",
        "esc_json",
    );
    let syntax =
        CallableFunctionSyntaxViewV1::from_function_ast(&function).expect("callable syntax view");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let ResolveSelectedCallableForestsOutcomeV1::Complete(forests) = resolver
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
    let CallableLoopBindingProjectionDispositionV1::ReadyWithBodyOnly(product) = disposition else {
        panic!("esc_json body-only rebinds must move with Ready")
    };
    assert_eq!(product.body_only_rows().len(), 2);
    assert!(product
        .body_only_rows()
        .iter()
        .all(|row| { matches!(row.kind(), CallableLoopOutsideKindV1::BodyOnlyRebind) }));
    assert!(
        product
            .body_only_rows()
            .iter()
            .flat_map(|row| row.receipts())
            .count()
            >= 4
    );
    assert!(product.body_only_rows().iter().all(|row| {
        row.receipts()
            .iter()
            .all(|receipt| receipt.binding() == row.binding())
    }));
    assert!(product.body_only_rows().iter().all(|row| {
        row.receipts()
            .iter()
            .any(|receipt| receipt.role() == CallableLoopBindingRoleV1::BodyRead)
            && row
                .receipts()
                .iter()
                .any(|receipt| receipt.role() == CallableLoopBindingRoleV1::BodyRebind)
    }));
}

fn source_pre_effect(
    source: &str,
    loop_index: usize,
) -> CallableSemanticLoopHandoffPreEffectReceiptV1 {
    let function = parsed_method(source, "Probe", "run");
    let syntax = CallableFunctionSyntaxViewV1::from_function_ast(&function).unwrap();
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let ResolveSelectedCallableForestsOutcomeV1::Complete(forests) = resolver
        .resolve_selected_callable_forests(&[syntax.function()])
        .unwrap()
    else {
        panic!("declaration fixture must resolve")
    };
    let forest = forests.into_vec().pop().unwrap();
    let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
        &function,
        &forest,
        syntax.function().root_profile(),
    )
    .unwrap();
    let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
        &function,
        &forest,
        &projection,
    )
    .unwrap();
    let state = super::super::normal_callable_semantic_lowering_state::CallableSemanticLoweringState::from_exact_source(input).unwrap();
    let path = SourcePathV1::root_body(loop_index);
    let product = match state
        .loop_binding_source_projection()
        .project_disposition(path.node())
        .unwrap()
    {
        CallableLoopBindingProjectionDispositionV1::Ready(ready) => {
            CallableLoopReadyBodyOnlyProductV1::without_body_only(ready)
        }
        CallableLoopBindingProjectionDispositionV1::ReadyWithBodyOnly(product) => product,
        other => panic!("unexpected source disposition: {other:?}"),
    };
    product
        .consume_pre_effect(
            &path.node(),
            &path.child(SourcePathSegmentV1::LoopCondition).node(),
            &path.child(SourcePathSegmentV1::LoopBodyRoot).node(),
        )
        .unwrap()
}

#[test]
fn source_body_only_partition_retains_branch_and_unread_local_declarations() {
    let receipt = source_pre_effect(
        r#"
static box Probe {
    run(n) {
        local i = 0
        local sum = 0
        loop(i < n) {
            local inner = 1
            inner = inner + 1
            sum = sum + inner
            if i < 1 {
                local temp = 2
                temp = temp + 1
                sum = sum + temp
            }
            local unused = 42
            i = i + 1
        }
        return sum
    }
}
"#,
        2,
    );
    let path = SourcePathV1::root_body(2);
    let expected = BTreeSet::from([
        SourceBindingSiteV1::Local {
            statement: path.child(SourcePathSegmentV1::LoopBody(0)).stmt(),
            ordinal: 0,
        },
        SourceBindingSiteV1::Local {
            statement: path
                .child(SourcePathSegmentV1::LoopBody(3))
                .child(SourcePathSegmentV1::IfThen(0))
                .stmt(),
            ordinal: 0,
        },
        SourceBindingSiteV1::Local {
            statement: path.child(SourcePathSegmentV1::LoopBody(4)).stmt(),
            ordinal: 0,
        },
    ]);
    assert_eq!(
        receipt
            .local_declarations()
            .values()
            .cloned()
            .collect::<BTreeSet<_>>(),
        expected
    );
    assert_eq!(receipt.body_only_rows().len(), 3);
    // Two rebound locals keep their exact declarations; the pre-loop sum does
    // not become a local, and unread `unused` does not become a carrier row.
    assert_eq!(
        receipt
            .body_only_rows()
            .iter()
            .filter(|row| receipt.local_declarations().contains_key(&row.binding()))
            .count(),
        2
    );
    assert_eq!(
        receipt
            .local_declarations()
            .keys()
            .filter(
                |binding| !receipt.rows().iter().any(|row| row.binding() == **binding)
                    && !receipt
                        .body_only_rows()
                        .iter()
                        .any(|row| row.binding() == **binding)
            )
            .count(),
        1
    );
}

#[test]
fn source_ready_path_retains_local_declaration_without_body_only_rows() {
    let receipt = source_pre_effect(
        r#"
static box Probe {
    run(n) {
        local i = 0
        loop(i < n) {
            local step = 1
            i = i + step
        }
        return i
    }
}
"#,
        1,
    );
    assert!(receipt.body_only_rows().is_empty());
    let local = receipt
        .rows()
        .iter()
        .find(|row| row.class() == CallableLoopReadyBindingClassV1::IterationLocal)
        .unwrap();
    assert_eq!(
        receipt.local_declarations().get(&local.binding()),
        Some(&SourceBindingSiteV1::Local {
            statement: SourcePathV1::root_body(1)
                .child(SourcePathSegmentV1::LoopBody(0))
                .stmt(),
            ordinal: 0,
        })
    );
}

#[test]
fn local_declaration_projection_rejects_foreign_and_duplicate_bindings() {
    let owner = owner();
    let loop_path = SourcePathV1::root_body(0);
    let site = loop_path.child(SourcePathSegmentV1::LoopBody(0)).node();
    let other_site = loop_path.child(SourcePathSegmentV1::LoopBody(1)).node();
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
    let foreign = issuer.issue().unwrap();
    let variables = BTreeMap::new();
    let assignments = BTreeMap::new();
    let foreign_locals =
        BTreeMap::from([(site.clone(), vec![binding(foreign, 0)].into_boxed_slice())]);
    assert!(
        CallableLoopSourceProjectionV1::new(owner, &foreign_locals, &variables, &assignments)
            .local_declarations(&loop_path.node())
            .unwrap_err()
            .contains("foreign-local-declaration")
    );
    let duplicate = BTreeMap::from([
        (site, vec![binding(owner, 0)].into_boxed_slice()),
        (other_site, vec![binding(owner, 0)].into_boxed_slice()),
    ]);
    assert!(
        CallableLoopSourceProjectionV1::new(owner, &duplicate, &variables, &assignments)
            .local_declarations(&loop_path.node())
            .unwrap_err()
            .contains("duplicate-local-declaration-binding")
    );
}
