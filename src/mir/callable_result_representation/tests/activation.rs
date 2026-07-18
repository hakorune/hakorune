use crate::mir::resolved_semantics::SourcePathSegmentV1;

use super::super::{
    CallableResultActivationDispositionV1, VerifiedCallableResultActivationPlanV1,
    VerifiedCallableResultActivationRowsV1,
};
use super::actual_parser_add_fixture;
use super::support::{
    declarations, instance_key, key, qualified_targets, seal_with_targets, site, CallSiteSpecV1,
};

const SOURCE: &str = r#"
    box ParserBox {
        static_const_parse_add(text, pos) {
            local next = ParserStringUtilsBox.skip_ws(text, pos)
            local width = text.length()
            return next
        }
    }
    static box ParserStringUtilsBox {
        skip_ws(text, pos) { return pos }
    }
"#;

fn selected_site() -> crate::mir::resolved_semantics::SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
    ])
}

fn unselected_site() -> crate::mir::resolved_semantics::SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(1),
        SourcePathSegmentV1::Initializer(0),
    ])
}

fn seal_plan(source: &str) -> VerifiedCallableResultActivationPlanV1 {
    let declarations = Box::new(declarations(source));
    let targets = qualified_targets(
        declarations.as_ref(),
        &[],
        &[CallSiteSpecV1 {
            caller_owner: "ParserBox",
            caller_name: "static_const_parse_add",
            caller_arity: 2,
            site: selected_site(),
        }],
    );
    let results = seal_with_targets(declarations.as_ref(), &targets);
    let rows =
        VerifiedCallableResultActivationRowsV1::verify(declarations.as_ref(), &targets, &results)
            .expect("owned activation rows");
    drop(results);
    drop(targets);
    VerifiedCallableResultActivationPlanV1::seal(declarations, rows).expect("owned activation plan")
}

#[test]
fn instance_caller_owns_selected_static_target_and_explicit_unselected_site() {
    let plan = seal_plan(SOURCE);
    let caller = instance_key(
        plan.declaration_catalog(),
        "ParserBox",
        "static_const_parse_add",
        2,
    );
    let rows = plan.rows_for(&caller).expect("instance caller rows");
    let target = key(
        plan.declaration_catalog(),
        "ParserStringUtilsBox",
        "skip_ws",
        2,
    );
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].site(), &selected_site());
    assert_eq!(
        rows[0].disposition(),
        &CallableResultActivationDispositionV1::SelectedExactI64 {
            target,
            required_i64_arguments: Box::new([1]),
        }
    );
    assert_eq!(rows[1].site(), &unselected_site());
    assert_eq!(
        rows[1].disposition(),
        &CallableResultActivationDispositionV1::Unselected
    );
}

#[test]
fn declaration_reorder_preserves_owned_activation_rows() {
    let forward = seal_plan(SOURCE);
    let reversed = seal_plan(
        r#"
        static box ParserStringUtilsBox {
            skip_ws(text, pos) { return pos }
        }
        box ParserBox {
            static_const_parse_add(text, pos) {
                local next = ParserStringUtilsBox.skip_ws(text, pos)
                local width = text.length()
                return next
            }
        }
        "#,
    );
    let forward_caller = instance_key(
        forward.declaration_catalog(),
        "ParserBox",
        "static_const_parse_add",
        2,
    );
    let reversed_caller = instance_key(
        reversed.declaration_catalog(),
        "ParserBox",
        "static_const_parse_add",
        2,
    );
    assert_eq!(
        forward.rows_for(&forward_caller),
        reversed.rows_for(&reversed_caller)
    );
}

#[test]
fn activation_rows_cannot_pair_with_an_equal_foreign_catalog() {
    let primary = Box::new(declarations(SOURCE));
    let targets = qualified_targets(
        primary.as_ref(),
        &[],
        &[CallSiteSpecV1 {
            caller_owner: "ParserBox",
            caller_name: "static_const_parse_add",
            caller_arity: 2,
            site: selected_site(),
        }],
    );
    let results = seal_with_targets(primary.as_ref(), &targets);
    let rows = VerifiedCallableResultActivationRowsV1::verify(primary.as_ref(), &targets, &results)
        .expect("primary rows");
    drop(results);
    drop(targets);

    let foreign = Box::new(declarations(SOURCE));
    assert!(VerifiedCallableResultActivationPlanV1::seal(foreign, rows).is_err());
}

#[test]
fn plan_is_owned_and_single_use() {
    let plan = seal_plan(SOURCE);
    let (catalog, rows) = plan.into_parts();
    assert_eq!(catalog.len(), 2);
    assert_eq!(rows.len(), 2);
}

#[test]
fn actual_parser_add_inventory_selects_only_two_static_skip_ws_sites() {
    let plan = actual_parser_add_fixture::plan();
    let caller = actual_parser_add_fixture::caller(&plan);
    let rows = plan.rows_for(&caller).expect("actual ParserBox rows");
    assert_eq!(rows.len(), 15);
    assert_eq!(
        rows.iter()
            .filter(|row| matches!(
                row.disposition(),
                CallableResultActivationDispositionV1::SelectedExactI64 { .. }
            ))
            .count(),
        2
    );
    assert_eq!(
        rows.iter()
            .filter(|row| row.disposition() == &CallableResultActivationDispositionV1::Unselected)
            .count(),
        13
    );
    for row in rows.iter().filter(|row| {
        matches!(
            row.disposition(),
            CallableResultActivationDispositionV1::SelectedExactI64 { .. }
        )
    }) {
        let CallableResultActivationDispositionV1::SelectedExactI64 {
            target,
            required_i64_arguments,
        } = row.disposition()
        else {
            unreachable!()
        };
        assert_eq!(target.owner(), "ParserStringUtilsBox");
        assert_eq!(target.name(), "skip_ws");
        assert_eq!(required_i64_arguments.as_ref(), &[1]);
    }
}
