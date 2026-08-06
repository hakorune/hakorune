use crate::ast::ASTNode;
use crate::mir::compiler::generic_g0_projection::{
    issue_generic_g0_structural_facts_v1, GenericG0ProjectionRejectV1,
};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::loop_structural_facts::generic_g0::{
    issue_generic_g0_structural_facts_v1 as issue_structural_facts_v1,
    GenericG0StructuralObservationV1, GenericG0StructuralRejectV1,
};
use crate::parser::NyashParser;

const CANONICAL: &str = r#"
function generic_g0(i, j) {
    loop(i < 3) {
        loop(j < 3) {
            j = j + 1
        }
        i = i + 1
    }
    return j
}
"#;

fn parse_function(source: &str) -> ASTNode {
    let program = NyashParser::parse_from_string(source).expect("fixture parses");
    let ASTNode::Program { statements, .. } = program else {
        panic!("fixture must produce a Program")
    };
    statements
        .into_iter()
        .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
        .expect("fixture function")
}

fn resolved_input(source: &str) -> (ASTNode, VerifiedResolvedSourceUnitV1) {
    let function = parse_function(source);
    let snapshot = function.clone();
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function).expect("resolve fixture");
    assert_eq!(unit.syntax_root(), &snapshot);
    (snapshot, unit)
}

fn facts(
    source: &str,
) -> Result<
    crate::mir::loop_structural_facts::generic_g0::VerifiedGenericStructuralFactsG0,
    GenericG0ProjectionRejectV1,
> {
    let (_, unit) = resolved_input(source);
    issue_generic_g0_structural_facts_v1(unit.root_function_input().expect("root input"))
}

#[test]
fn canonical_source_emits_move_only_structural_facts() {
    let facts = facts(CANONICAL).expect("canonical G0 structure");
    assert_eq!(facts.function_body().len(), 2);
    assert_eq!(facts.root_body().len(), 2);
    assert_eq!(facts.child_body().len(), 1);
    assert_ne!(
        facts.outer_condition().binding,
        facts.inner_condition().binding
    );
    assert_eq!(facts.inner_condition().binding, facts.tail().binding);
    assert_eq!(facts.forest().members().len(), 2);
    assert_eq!(facts.coverage().len(), 20);
}

#[test]
fn source_ast_is_not_rewritten_by_projection() {
    let (snapshot, unit) = resolved_input(CANONICAL);
    let _ = issue_generic_g0_structural_facts_v1(unit.root_function_input().expect("root input"))
        .expect("canonical G0 structure");
    assert_eq!(unit.syntax_root(), &snapshot);
}

#[test]
fn reordered_function_body_rejects_before_structural_issuer() {
    let source = CANONICAL.replace("    loop(i < 3) {", "    i = i + 1\n    loop(i < 3) {");
    assert_eq!(
        facts(&source),
        Err(GenericG0ProjectionRejectV1::FunctionBodySchedule)
    );
}

#[test]
fn extra_root_statement_rejects_exact_schedule() {
    let source = CANONICAL.replace("    return j", "    i = i + 1\n    return j");
    assert_eq!(
        facts(&source),
        Err(GenericG0ProjectionRejectV1::FunctionBodySchedule)
    );
}

#[test]
fn reordered_outer_body_rejects_loop_shape() {
    let source = CANONICAL.replace(
        "        loop(j < 3) {\n            j = j + 1\n        }\n        i = i + 1",
        "        i = i + 1\n        loop(j < 3) {\n            j = j + 1\n        }",
    );
    assert_eq!(facts(&source), Err(GenericG0ProjectionRejectV1::LoopShape));
}

#[test]
fn missing_inner_update_rejects_exact_schedule() {
    let source = CANONICAL.replace("            j = j + 1\n", "");
    assert_eq!(
        facts(&source),
        Err(GenericG0ProjectionRejectV1::ChildBodySchedule)
    );
}

#[test]
fn wrong_recurrence_binding_rejects_without_name_fallback() {
    let source = CANONICAL.replace("            j = j + 1", "            i = i + 1");
    assert_eq!(
        facts(&source),
        Err(GenericG0ProjectionRejectV1::Structural(
            crate::mir::loop_structural_facts::generic_g0::GenericG0StructuralRejectV1::BindingRelation,
        ))
    );
}

#[test]
fn foreign_root_frame_rejects_at_structural_issuer() {
    let (_, unit) = resolved_input(CANONICAL);
    let facts =
        issue_generic_g0_structural_facts_v1(unit.root_function_input().expect("root input"))
            .expect("canonical G0 structure");
    let (
        owner,
        origin,
        source_kind,
        forest,
        function_body,
        root_body,
        child_body,
        root_loop,
        child_loop,
        outer_condition,
        inner_condition,
        outer_update,
        inner_update,
        tail,
        coverage,
        _root_frame,
    ) = facts.into_parts();
    let foreign_frame = forest.members()[1].source().frame_key();
    let observation = GenericG0StructuralObservationV1 {
        owner,
        origin,
        source_kind,
        forest,
        expected_root_frame: foreign_frame,
        function_body,
        root_body,
        child_body,
        root_loop,
        child_loop,
        outer_condition,
        inner_condition,
        outer_update,
        inner_update,
        tail,
        coverage,
    };
    assert_eq!(
        issue_structural_facts_v1(observation),
        Err(GenericG0StructuralRejectV1::ForestIdentity)
    );
}
