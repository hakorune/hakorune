#![cfg(test)]
//! Focused tests for the caller-zero loop-node winner+Recipe spine.
//!
//! Positive: each admitted family on its canonical fixture reaches
//! `Issued(<family product>)`. Terminal: family-less shapes resolve to
//! `Declined`, unresolvable evidence resolves to `Unresolved`, and the
//! typed reject surfaces stay distinct.

use super::loop_node_winner_spine::{
    issue_loop_node_winner_recipe_v1, LoopNodeWinnerRecipeV1,
    LoopNodeWinnerSpineOutcomeV1,
};
use super::VerifiedResolvedSourceUnitV1;
use crate::ast::ASTNode;
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

fn parsed_function(source: &str, function_name: &str) -> ASTNode {
    let program = NyashParser::parse_from_string(source).expect("fixture parses");
    let ASTNode::Program { statements, .. } = program else {
        panic!("parser must return Program")
    };
    statements
        .into_iter()
        .find(|statement| {
            matches!(
                statement,
                ASTNode::FunctionDeclaration { name, .. } if name == function_name
            )
        })
        .expect("exact function declaration")
}

fn run(function: ASTNode) -> LoopNodeWinnerSpineOutcomeV1 {
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
    issue_loop_node_winner_recipe_v1(input, loop_stmt, NumericTarget::host())
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

const VARIABLE_ACCUM_RECURRENCE: &str = r#"
static box Main {
    main(): i64 {
        local i: i64 = 0
        local acc: i64 = 0
        loop(i < 4) {
            acc = acc + i
            i = i + 1
        }
        print(acc)
        return 0
    }
}
"#;

const G0_TYPED_FUNCTION: &str = r#"
function generic_g0(i: i64, j: i64): i64 {
    loop(i < 3) {
        loop(j < 3) {
            j = j + 1
        }
        i = i + 1
    }
    return j
}
"#;

const G0_TYPED_METHOD: &str = r#"
static box Main {
    generic_g0(i: i64, j: i64): i64 {
        loop(i < 3) {
            loop(j < 3) {
                j = j + 1
            }
            i = i + 1
        }
        return j
    }
}
"#;

#[test]
fn direct_accum_fixture_issues_recipe() {
    let function = parsed_method(ACCUM_CONST, "Main", "accum");
    let outcome = run(function);
    assert!(
        matches!(&outcome, LoopNodeWinnerSpineOutcomeV1::Issued(issued)
            if matches!(issued.recipe(), LoopNodeWinnerRecipeV1::DirectAccum(_))),
        "accum fixture must issue a DirectAccum recipe, got: {outcome:?}"
    );
}

#[test]
fn nested_predicate_fixture_issues_recipe() {
    let function = super::nested_function_for_p3_test();
    let outcome = run(function);
    assert!(
        matches!(&outcome, LoopNodeWinnerSpineOutcomeV1::Issued(issued)
            if matches!(issued.recipe(), LoopNodeWinnerRecipeV1::NestedPredicate(_))),
        "nested fixture must issue a NestedPredicate recipe, got: {outcome:?}"
    );
}

#[test]
fn loop_true_fixture_issues_recipe() {
    let function = parsed_method(LOOP_TRUE, "Main", "loop_true_projection");
    let outcome = run(function);
    assert!(
        matches!(&outcome, LoopNodeWinnerSpineOutcomeV1::Issued(issued)
            if matches!(issued.recipe(), LoopNodeWinnerRecipeV1::LoopTrue(_))),
        "loop-true fixture must issue a LoopTrue recipe, got: {outcome:?}"
    );
}

#[test]
fn loop_cond_fixture_issues_recipe() {
    let function = parsed_method(LOOP_COND, "Main", "loop_cond_projection");
    let outcome = run(function);
    assert!(
        matches!(&outcome, LoopNodeWinnerSpineOutcomeV1::Issued(issued)
            if matches!(issued.recipe(), LoopNodeWinnerRecipeV1::LoopCond(_))),
        "loop-cond fixture must issue a LoopCond recipe, got: {outcome:?}"
    );
}

#[test]
fn generic_g0_box_method_fixture() {
    let function = parsed_method(G0_TYPED_METHOD, "Main", "generic_g0");
    let outcome = run(function);
    eprintln!("[spine] g0 box-method outcome: {outcome:?}");
}

#[test]
fn generic_g0_standalone_function_is_typed_terminal() {
    let function = parsed_function(G0_TYPED_FUNCTION, "generic_g0");
    let outcome = run(function);
    // The standalone-function shape leaves the DirectAccum arm
    // `Unresolved(SourceNavigation)`, so the window cannot assemble
    // `Ready`; the spine surfaces the typed `Unresolved` terminal.
    assert!(
        matches!(outcome, LoopNodeWinnerSpineOutcomeV1::Unresolved(_)),
        "standalone g0 fixture must terminate Unresolved, got: {outcome:?}"
    );
}

#[test]
fn family_less_variable_accum_declines() {
    let function = parsed_method(VARIABLE_ACCUM_RECURRENCE, "Main", "main");
    let outcome = run(function);
    // `acc += i` is outside the selected five-family boundary (D0): no
    // family admits it, the whole-unit set is entirely pre-effect
    // declined, and the spine returns the typed `Declined` terminal.
    assert!(
        matches!(outcome, LoopNodeWinnerSpineOutcomeV1::Declined(_)),
        "family-less fixture must terminate Declined, got: {outcome:?}"
    );
}
