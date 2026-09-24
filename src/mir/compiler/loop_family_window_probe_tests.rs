//! M10b-I0-P1 spine recon: empirical family<->route correspondence.
//!
//! Runs the five per-family source-attempt issuers against every
//! attested fixture shape and records which family claims Candidate.
//! The resulting table pins which `LoopRouteId` each family owns for
//! the 19-row coverage set. This is caller-zero reconnaissance: it
//! issues no Recipe, touches no Builder, and selects nothing.

use super::direct_accum_observation::issue_direct_accum_source_attempt_v1;
use super::function_input::ResolvedFunctionLoweringInputV1;
use super::generic_g0_observation::issue_generic_g0_source_attempt_v1;
use super::loop_cond_break_continue_observation::issue_loop_cond_source_attempt_v1;
use super::loop_true_break_continue_observation::issue_loop_true_source_attempt_v1;
use super::nested_predicate_observation::issue_nested_predicate_source_attempt_v1;
use super::VerifiedResolvedSourceUnitV1;
use crate::ast::ASTNode;
use crate::mir::loop_structural_facts::{
    DirectAccumObservationCoverageV1, DirectAccumObservationModeV1,
    DirectAccumSourceAttemptOutcomeV1, GenericG0ObservationCoverageV1,
    GenericG0ObservationModeV1, GenericG0SourceAttemptOutcomeV1,
    LoopCondObservationCoverageV1, LoopCondObservationModeV1,
    LoopCondSourceAttemptOutcomeV1, LoopTrueObservationCoverageV1,
    LoopTrueObservationModeV1, LoopTrueSourceAttemptOutcomeV1,
    NestedPredicateObservationCoverageV1, NestedPredicateObservationModeV1,
    NestedPredicateSourceAttemptOutcomeV1,
};
use crate::mir::numeric_substrate::NumericTarget;
use crate::parser::NyashParser;

fn parsed_method(source: &str, box_name: &str, method_name: &str) -> ASTNode {
    let program = NyashParser::parse_from_string(source).expect("probe fixture parses");
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
    let program = NyashParser::parse_from_string(source).expect("probe fixture parses");
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

struct ProbeInput<'a> {
    input: ResolvedFunctionLoweringInputV1<'a>,
    body: super::located::LocatedBodyV1<'a>,
    loop_index: usize,
}

fn probe_input<'a>(
    unit: &'a VerifiedResolvedSourceUnitV1,
    function: ASTNode,
) -> ProbeInput<'a> {
    let _ = function;
    let input = unit.root_function_input().expect("root function input");
    let body = input.source().root_body().expect("root body");
    let loop_index = body
        .statements()
        .iter()
        .position(|node| matches!(node, ASTNode::Loop { .. }))
        .expect("fixture holds one top-level loop");
    ProbeInput {
        input,
        body,
        loop_index,
    }
}

impl ProbeInput<'_> {
    fn loop_stmt(&self) -> super::located::LocatedStmtV1<'_> {
        self.input
            .source()
            .body_stmt(&self.body, self.loop_index)
            .expect("loop statement locates")
    }

    fn loop_source(&self) -> crate::mir::resolved_semantics::VerifiedResolvedLoopSourceV1 {
        self.input
            .function()
            .resolved_loop_source(self.loop_stmt().site())
            .expect("resolved loop source")
    }
}

#[derive(Debug)]
struct FamilyRow {
    direct_accum: &'static str,
    nested_predicate: &'static str,
    loop_true: &'static str,
    loop_cond: &'static str,
    generic_g0: &'static str,
}

fn direct_accum_label(outcome: &DirectAccumSourceAttemptOutcomeV1) -> &'static str {
    match outcome {
        DirectAccumSourceAttemptOutcomeV1::Candidate(_) => "Candidate",
        DirectAccumSourceAttemptOutcomeV1::Declined(_) => "Declined",
        DirectAccumSourceAttemptOutcomeV1::Unresolved(_) => "Unresolved",
        DirectAccumSourceAttemptOutcomeV1::Rejected(_) => "Rejected",
    }
}

fn nested_label(outcome: &NestedPredicateSourceAttemptOutcomeV1) -> &'static str {
    match outcome {
        NestedPredicateSourceAttemptOutcomeV1::Candidate(_) => "Candidate",
        NestedPredicateSourceAttemptOutcomeV1::Declined(_) => "Declined",
        NestedPredicateSourceAttemptOutcomeV1::Unresolved(_) => "Unresolved",
        NestedPredicateSourceAttemptOutcomeV1::Rejected(_) => "Rejected",
    }
}

fn loop_true_label(outcome: &LoopTrueSourceAttemptOutcomeV1) -> &'static str {
    match outcome {
        LoopTrueSourceAttemptOutcomeV1::Candidate(_) => "Candidate",
        LoopTrueSourceAttemptOutcomeV1::Declined(_) => "Declined",
        LoopTrueSourceAttemptOutcomeV1::Unresolved(_) => "Unresolved",
        LoopTrueSourceAttemptOutcomeV1::Rejected(_) => "Rejected",
    }
}

fn loop_cond_label(outcome: &LoopCondSourceAttemptOutcomeV1) -> &'static str {
    match outcome {
        LoopCondSourceAttemptOutcomeV1::Candidate(_) => "Candidate",
        LoopCondSourceAttemptOutcomeV1::Declined(_) => "Declined",
        LoopCondSourceAttemptOutcomeV1::Unresolved(_) => "Unresolved",
        LoopCondSourceAttemptOutcomeV1::Rejected(_) => "Rejected",
    }
}

fn generic_g0_label(outcome: &GenericG0SourceAttemptOutcomeV1) -> &'static str {
    match outcome {
        GenericG0SourceAttemptOutcomeV1::Candidate(_) => "Candidate",
        GenericG0SourceAttemptOutcomeV1::Declined(_) => "Declined",
        GenericG0SourceAttemptOutcomeV1::Unresolved(_) => "Unresolved",
        GenericG0SourceAttemptOutcomeV1::Rejected(_) => "Rejected",
    }
}

fn row(unit: &VerifiedResolvedSourceUnitV1, function: ASTNode) -> FamilyRow {
    let probe = probe_input(unit, function);
    let input = probe.input;
    let direct = issue_direct_accum_source_attempt_v1(
        input,
        probe.loop_stmt(),
        probe.loop_source(),
        Some(DirectAccumObservationModeV1::Release),
        DirectAccumObservationCoverageV1::Complete,
    );
    let nested = issue_nested_predicate_source_attempt_v1(
        input,
        probe.loop_stmt(),
        probe.loop_source(),
        Some(NestedPredicateObservationModeV1::Release),
        NestedPredicateObservationCoverageV1::Complete,
    );
    let loop_true = issue_loop_true_source_attempt_v1(
        input,
        probe.loop_stmt(),
        probe.loop_source(),
        Some(LoopTrueObservationModeV1::Release),
        LoopTrueObservationCoverageV1::Complete,
    );
    let loop_cond = issue_loop_cond_source_attempt_v1(
        input,
        probe.loop_stmt(),
        probe.loop_source(),
        Some(LoopCondObservationModeV1::Release),
        LoopCondObservationCoverageV1::Complete,
    );
    let generic = issue_generic_g0_source_attempt_v1(
        input,
        probe.loop_stmt(),
        probe.loop_source(),
        NumericTarget::host(),
        Some(GenericG0ObservationModeV1::Release),
        GenericG0ObservationCoverageV1::Complete,
    );
    let (direct_outcome, ..) = direct.into_parts();
    let (nested_outcome, ..) = nested.into_parts();
    let (true_outcome, ..) = loop_true.into_parts();
    let (cond_outcome, ..) = loop_cond.into_parts();
    let (generic_outcome, ..) = generic.into_parts();
    eprintln!(
        "   detail: accum={:?} nested={:?} true={:?} cond={:?} g0={:?}",
        direct_outcome, nested_outcome, true_outcome, cond_outcome, generic_outcome
    );
    FamilyRow {
        direct_accum: direct_accum_label(&direct_outcome),
        nested_predicate: nested_label(&nested_outcome),
        loop_true: loop_true_label(&true_outcome),
        loop_cond: loop_cond_label(&cond_outcome),
        generic_g0: generic_g0_label(&generic_outcome),
    }
}

fn resolve(source: &str, method: &str) -> (VerifiedResolvedSourceUnitV1, ASTNode) {
    let function = parsed_method(source, "Main", method);
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function.clone())
        .expect("fixture resolves");
    (unit, function)
}

fn report(name: &str, source: &str, method: &str) -> FamilyRow {
    let (unit, function) = resolve(source, method);
    let row = row(&unit, function);
    eprintln!(
        "[probe] {name}: accum={} nested={} true={} cond={} g0={}",
        row.direct_accum, row.nested_predicate, row.loop_true, row.loop_cond, row.generic_g0
    );
    row
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

const VARIABLE_ACCUM_BREAK: &str = r#"
static box Main {
    main(): i64 {
        local sum: i64 = 0
        local i: i64 = 0
        loop(i < 10) {
            if (i == 5) {
                sum = sum + 10
                break
            } else {
                continue
            }
            sum = sum + 1
            i = i + 1
        }
        return 0
    }
}
"#;

const NESTED_MINIMAL: &str = r#"
static box Main {
    nested_loop_minimal() {
        local i = 0
        local sum = 0
        loop(i < 3) {
            local j
            j = 0
            loop(j < 3) {
                sum = sum + 1
                j = j + 1
            }
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

const GENERIC_RESIDUAL: &str = r#"
static box Main {
    generic_residual_projection(i: i64, limit: i64): i64 {
        loop(i < limit) {
            local tmp = 0
            i = i + 1
        }
        return i
    }
}
"#;

const SCAN_WITH_INIT: &str = r#"
static box Main {
    find_ok(s: StringBox, ch: StringBox): i64 {
        local i: i64 = 0
        loop(i < s.length()) {
            if s.substring(i, i + 1) == ch {
                return i
            }
            i = i + 1
        }
        return -1
    }
}
"#;

#[test]
fn probe_accum_const_loop() {
    report("AccumConstLoop", ACCUM_CONST, "accum");
}

#[test]
fn probe_variable_accum_recurrence() {
    report("LoopSimpleWhile/VariableAccumRecurrence", VARIABLE_ACCUM_RECURRENCE, "main");
}

#[test]
fn probe_variable_accum_break() {
    report("LoopBreakRecipe/VariableAccumBreak", VARIABLE_ACCUM_BREAK, "main");
}

#[test]
fn probe_nested_loop_minimal() {
    report("NestedLoopMinimal", NESTED_MINIMAL, "nested_loop_minimal");
}

#[test]
fn probe_loop_true_break_continue() {
    report("LoopTrueBreakContinue", LOOP_TRUE, "loop_true_projection");
}

#[test]
fn probe_loop_cond_break_continue() {
    report("LoopCondBreakContinue", LOOP_COND, "loop_cond_projection");
}

#[test]
fn probe_generic_residual() {
    report("GenericLoopV1/GenericResidual", GENERIC_RESIDUAL, "generic_residual_projection");
}

#[test]
fn probe_scan_with_init() {
    report("ScanWithInit", SCAN_WITH_INIT, "find_ok");
}

#[test]
fn probe_nested_canonical_ast() {
    let function = super::nested_function_for_p3_test();
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function.clone())
        .expect("canonical nested fixture resolves");
    let row = row(&unit, function);
    eprintln!(
        "[probe] NestedLoopMinimal(canonical-ast): accum={} nested={} true={} cond={} g0={}",
        row.direct_accum, row.nested_predicate, row.loop_true, row.loop_cond, row.generic_g0
    );
}

#[test]
fn probe_loop_cond_canonical_ast() {
    let function = super::loop_cond_break_continue_projection_tests::positive_function();
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function.clone())
        .expect("canonical loop-cond fixture resolves");
    let row = row(&unit, function);
    eprintln!(
        "[probe] LoopCondBreakContinue(canonical-ast): accum={} nested={} true={} cond={} g0={}",
        row.direct_accum, row.nested_predicate, row.loop_true, row.loop_cond, row.generic_g0
    );
}

const G0_TYPED: &str = r#"
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

#[test]
fn probe_generic_g0_canonical() {
    let function = parsed_function(G0_TYPED, "generic_g0");
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function.clone())
        .expect("canonical g0 fixture resolves");
    let row = row(&unit, function);
    eprintln!(
        "[probe] GenericG0(canonical): accum={} nested={} true={} cond={} g0={}",
        row.direct_accum, row.nested_predicate, row.loop_true, row.loop_cond, row.generic_g0
    );
}

#[test]
fn probe_generic_residual_canonical_ast() {
    let function = super::generic_residual_projection_tests::positive_function();
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function.clone())
        .expect("canonical residual fixture resolves");
    let row = row(&unit, function);
    eprintln!(
        "[probe] GenericResidual(canonical-ast): accum={} nested={} true={} cond={} g0={}",
        row.direct_accum, row.nested_predicate, row.loop_true, row.loop_cond, row.generic_g0
    );
}
