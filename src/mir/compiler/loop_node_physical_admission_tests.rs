#![cfg(test)]
//! Focused tests for the caller-zero loop-node physical admission issuer.
//!
//! Positive: each node-level family reaches a co-sealed
//! `VerifiedLoopNodePhysicalAdmissionV1` — prepared layout + entry input
//! set + demand-carried internal declaration rows. Terminal: the
//! `GenericG0` arm stays function-level (`FunctionLevelFamily`), never a
//! fallback.

use super::loop_node_physical_admission::{
    issue_loop_node_physical_admission_v1, LoopNodePhysicalAdmissionRejectV1,
};
use super::loop_node_winner_spine::{
    issue_loop_node_winner_recipe_v1, IssuedLoopNodeWinnerV1, LoopNodeWinnerRecipeV1,
    LoopNodeWinnerSpineOutcomeV1,
};
use super::VerifiedResolvedSourceUnitV1;
use crate::ast::ASTNode;
use crate::mir::loop_recipe_contract::{
    issue_generic_g0_recipe_demand_v1, produce_generic_g0_recipe_v1,
    LoopInternalDeclarationModeV1,
};
use crate::mir::loop_route_policy::generic_source_unit_and_selection_for_test;
use crate::mir::numeric_substrate::NumericTarget;
use crate::parser::NyashParser;

fn parsed_method(source: &str, box_name: &str, method_name: &str) -> ASTNode {
    let program = NyashParser::parse_from_string(source).expect("fixture parses");
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
        .expect("exact method declaration")
}

/// Run the caller-zero spine on the fixture's sole top-level loop, then the
/// admission issuer on the issued winner. Returns the admission.
fn admission_for(function: ASTNode) -> super::loop_node_physical_admission::VerifiedLoopNodePhysicalAdmissionV1 {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function).expect("fixture resolves");
    let input = unit.root_function_input().expect("root input");
    let body = input.source().root_body().expect("root body");
    let loop_index = body
        .statements()
        .iter()
        .position(|node| matches!(node, ASTNode::Loop { .. }))
        .expect("fixture holds one top-level loop");
    let loop_stmt = input
        .source()
        .body_stmt(&body, loop_index)
        .expect("loop statement locates");
    let outcome = issue_loop_node_winner_recipe_v1(input, loop_stmt, NumericTarget::host());
    let LoopNodeWinnerSpineOutcomeV1::Issued(issued) = outcome else {
        panic!("fixture family must issue a winner")
    };
    issue_loop_node_physical_admission_v1(input, issued).expect("admission issues")
}

const ACCUM_CONST: &str = r#"
static box Main {
    accum() {
        local i = 0
        local sum = 0
        loop(i < 3) {
            sum = sum + 1
            i = i + 1
        }
    }
}
"#;

const LOOP_TRUE: &str = r#"
static box Main {
    loop_true_projection() {
        local flag = 1
        loop(true) {
            if (flag == 1) {
                break
            } else {
                continue
            }
        }
    }
}
"#;

const LOOP_COND: &str = r#"
static box Main {
    loop_cond_projection() {
        local flag = 1
        loop(flag < 2) {
            if (flag == 1) {
                break
            } else {
                continue
            }
        }
    }
}
"#;



#[test]
fn direct_accum_admission_seals_layout_and_inputs() {
    let admission = admission_for(parsed_method(ACCUM_CONST, "Main", "accum"));
    assert!(
        admission.layout().segments().len() >= 2,
        "accum layout must carry body + after segments"
    );
    assert!(
        admission.layout().coverage().operation_count() > 0,
        "prepared program covers every recipe operation"
    );
    assert!(
        !admission.inputs().rows().is_empty(),
        "loop entry must publish at least one resolver input"
    );
    assert!(
        admission
            .layout()
            .program()
            .demand()
            .declarations()
            .rows()
            .is_empty(),
        "accum declares no loop-internal bindings"
    );
}

#[test]
fn nested_admission_carries_internal_declaration_rows() {
    let admission = admission_for(super::nested_function_for_p3_test());
    let rows = admission.layout().program().demand().declarations().rows();
    assert_eq!(rows.len(), 1, "nested `j` is the sole loop-internal declaration");
    assert!(matches!(
        rows[0].mode(),
        LoopInternalDeclarationModeV1::PublishWithEntry { .. }
    ));
}

#[test]
fn loop_true_always_layout_admits() {
    let admission = admission_for(parsed_method(LOOP_TRUE, "Main", "loop_true_projection"));
    assert!(
        !admission.layout().segments().is_empty(),
        "Always-loop layout must cover the Break-arm transfer"
    );
}

#[test]
fn loop_cond_layout_admits() {
    let admission = admission_for(parsed_method(LOOP_COND, "Main", "loop_cond_projection"));
    assert!(
        !admission.layout().segments().is_empty(),
        "cond-loop layout must cover Break/Continue arms"
    );
}

#[test]
fn generic_g0_arm_is_function_level_terminal() {
    let (unit, selection) = generic_source_unit_and_selection_for_test();
    let input = unit.root_function_input().expect("root input");
    let body = input.source().root_body().expect("root body");
    let loop_index = body
        .statements()
        .iter()
        .position(|node| matches!(node, ASTNode::Loop { .. }))
        .expect("fixture holds one top-level loop");
    let site = input
        .source()
        .body_stmt(&body, loop_index)
        .expect("loop statement locates")
        .site()
        .clone();
    let frame = input
        .function()
        .issue_loop_family_window_lease_v1(&site)
        .expect("lease issues")
        .frame();
    let demand = issue_generic_g0_recipe_demand_v1(selection).expect("g0 demand issues");
    let product = produce_generic_g0_recipe_v1(demand).expect("g0 recipe produces");
    let issued = IssuedLoopNodeWinnerV1::for_test(
        site,
        frame,
        LoopNodeWinnerRecipeV1::GenericG0(product),
    );
    let reject = issue_loop_node_physical_admission_v1(input, issued)
        .expect_err("G0 must not be admitted at the node edge");
    assert!(matches!(
        reject,
        LoopNodePhysicalAdmissionRejectV1::FunctionLevelFamily
    ));
}
