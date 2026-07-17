use crate::mir::resolved_semantics::{
    BodyChildRoleV1, ExprChildRoleV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
};

use super::super::{
    CallableResultLegacyLocationErrorV1, LegacyBodyInputV1, LegacyExprInputV1,
    VerifiedCallableResultActivationPlanV1, VerifiedCallableResultActivationRowsV1,
    VerifiedCallableResultLegacySourceViewV1,
};
use super::support::{
    declarations, instance_key, qualified_targets, seal_with_targets, CallSiteSpecV1,
};

const SOURCE: &str = r#"
    box ParserBox {
        parse(text, pos) {
            local next = Helpers.step(text, pos)
            if next {
                return next
            } else {
                return 0
            }
            return next
        }
    }
    static box Helpers {
        step(text, pos) { return pos }
    }
"#;

fn selected_site() -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
    ]))
}

fn seal_plan(source: &str) -> VerifiedCallableResultActivationPlanV1 {
    let declarations = Box::new(declarations(source));
    let targets = qualified_targets(
        declarations.as_ref(),
        &[],
        &[CallSiteSpecV1 {
            caller_owner: "ParserBox",
            caller_name: "parse",
            caller_arity: 2,
            site: selected_site(),
        }],
    );
    let results = seal_with_targets(declarations.as_ref(), &targets);
    let rows =
        VerifiedCallableResultActivationRowsV1::verify(declarations.as_ref(), &targets, &results)
            .expect("located legacy activation rows");
    drop(results);
    drop(targets);
    VerifiedCallableResultActivationPlanV1::seal(declarations, rows)
        .expect("located legacy activation plan")
}

fn caller(
    plan: &VerifiedCallableResultActivationPlanV1,
) -> crate::mir::builder::CanonicalSameModuleCallableKeyV1 {
    instance_key(plan.declaration_catalog(), "ParserBox", "parse", 2)
}

#[test]
fn root_body_local_initializer_and_nested_arguments_keep_exact_sites() {
    let plan = seal_plan(SOURCE);
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let body = view.root_body();
    let local = view.body_stmt(&body, 0).unwrap();
    let call = view
        .child_expr_from_stmt(&local, ExprChildRoleV1::LocalInitializer(0))
        .unwrap();
    assert_eq!(call.activation_site().unwrap(), (&caller, &selected_site()));

    let receiver = view.child_expr(&call, ExprChildRoleV1::Receiver).unwrap();
    assert_eq!(
        receiver.activation_site().unwrap().1.node().segments(),
        &[
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Initializer(0),
            SourcePathSegmentV1::Receiver,
        ]
    );
    let argument = view
        .child_expr(&call, ExprChildRoleV1::CallArgument(1))
        .unwrap();
    assert_eq!(
        argument.activation_site().unwrap().1.node().segments(),
        &[
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Initializer(0),
            SourcePathSegmentV1::Argument(1),
        ]
    );
}

#[test]
fn if_body_uses_role_owned_body_and_item_segments() {
    let plan = seal_plan(SOURCE);
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let root = view.root_body();
    let if_stmt = view.body_stmt(&root, 1).unwrap();
    let condition = view
        .child_expr_from_stmt(&if_stmt, ExprChildRoleV1::IfCondition)
        .unwrap();
    assert_eq!(
        condition.activation_site().unwrap().1.node().segments(),
        &[
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::IfCondition
        ]
    );
    let then_body = view
        .child_body_from_stmt(&if_stmt, BodyChildRoleV1::IfThen)
        .unwrap();
    let return_stmt = view.body_stmt(&then_body, 0).unwrap();
    let value = view
        .child_expr_from_stmt(&return_stmt, ExprChildRoleV1::ReturnValue)
        .unwrap();
    assert_eq!(
        value.activation_site().unwrap().1.node().segments(),
        &[
            SourcePathSegmentV1::Body(1),
            SourcePathSegmentV1::IfThenBody,
            SourcePathSegmentV1::IfThen(0),
            SourcePathSegmentV1::Value,
        ]
    );
}

#[test]
fn unlocated_syntax_and_descendants_cannot_claim_activation() {
    let plan = seal_plan(SOURCE);
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let root = view.root_body();
    let local = view.body_stmt(&root, 0).unwrap();
    let call = view
        .child_expr_from_stmt(&local, ExprChildRoleV1::LocalInitializer(0))
        .unwrap();
    let synthetic = view.unlocated_expr(call.node());
    let child = view
        .child_expr(&synthetic, ExprChildRoleV1::CallArgument(0))
        .unwrap();
    assert!(matches!(child, LegacyExprInputV1::Unlocated(_)));
    assert_eq!(
        child.activation_site(),
        Err(CallableResultLegacyLocationErrorV1::UnlocatedCannotClaimActivation)
    );
}

#[test]
fn equal_foreign_plan_carriers_are_rejected() {
    let first = seal_plan(SOURCE);
    let second = seal_plan(SOURCE);
    let first_caller = caller(&first);
    let second_caller = caller(&second);
    let first_view =
        VerifiedCallableResultLegacySourceViewV1::verify(&first, &first_caller).unwrap();
    let second_view =
        VerifiedCallableResultLegacySourceViewV1::verify(&second, &second_caller).unwrap();
    let foreign_body = first_view.root_body();
    assert!(matches!(
        second_view.body_stmt(&foreign_body, 0),
        Err(CallableResultLegacyLocationErrorV1::ForeignCarrier { .. })
    ));
}

#[test]
fn wrong_roles_and_body_bounds_fail_without_path_search() {
    let plan = seal_plan(SOURCE);
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let body = view.root_body();
    let local = view.body_stmt(&body, 0).unwrap();
    assert!(matches!(
        view.child_expr_from_stmt(&local, ExprChildRoleV1::IfCondition),
        Err(CallableResultLegacyLocationErrorV1::ExpressionRoleParentMismatch(_))
    ));
    assert!(matches!(
        view.body_stmt(&body, 99),
        Err(CallableResultLegacyLocationErrorV1::BodyIndexOutOfBounds { .. })
    ));
}

#[test]
fn declaration_reorder_preserves_normalized_site() {
    let reordered = SOURCE
        .replace(
            "    box ParserBox {",
            "    static box Helpers { step(text, pos) { return pos } }\n    box ParserBox {",
        )
        .replace(
            "    static box Helpers {\n        step(text, pos) { return pos }\n    }",
            "",
        );
    let first = seal_plan(SOURCE);
    let second = seal_plan(&reordered);
    for plan in [&first, &second] {
        let caller = caller(plan);
        let view = VerifiedCallableResultLegacySourceViewV1::verify(plan, &caller).unwrap();
        let body = view.root_body();
        let local = view.body_stmt(&body, 0).unwrap();
        let call = view
            .child_expr_from_stmt(&local, ExprChildRoleV1::LocalInitializer(0))
            .unwrap();
        assert_eq!(call.activation_site().unwrap().1, &selected_site());
    }
}

#[test]
fn root_body_is_located_and_unknown_caller_is_rejected() {
    let plan = seal_plan(SOURCE);
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    assert!(matches!(view.root_body(), LegacyBodyInputV1::Located(_)));
    let foreign = declarations("box Missing { parse(text, pos) { return pos } }");
    let unknown = instance_key(&foreign, "Missing", "parse", 2);
    assert_eq!(
        VerifiedCallableResultLegacySourceViewV1::verify(&plan, &unknown).unwrap_err(),
        CallableResultLegacyLocationErrorV1::UnknownCaller(unknown)
    );
}
