use super::*;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveScriptOutcomeV1, ScriptRootResolvedDemandV1,
    ScriptRootRuntimeDispositionV1, ScriptRootSemanticDispositionV1, ScriptSyntaxViewV1,
    SourcePathV1, VerifiedResolvedScriptV1, VerifiedScriptRootDemandEntryV1,
    VerifiedScriptRootDemandWindowV1,
};

pub(super) fn source(text: &str) -> VerifiedResolvedScriptV1 {
    let ast = crate::parser::NyashParser::parse_from_string(text).unwrap();
    let window = VerifiedScriptRootDemandWindowV1::seal(
        vec![VerifiedScriptRootDemandEntryV1::new(
            SourcePathV1::program_body()
                .child(SourcePathSegmentV1::ProgramBody(0))
                .stmt(),
            ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::LexicalCore),
            ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
        )],
        1,
    )
    .unwrap();
    let ResolveScriptOutcomeV1::Complete(product) = FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve_script(ScriptSyntaxViewV1::from_program(&ast).unwrap(), &window)
        .unwrap()
    else {
        panic!("Complete")
    };
    product
}

#[test]
fn actual_array_traversal_seals_empty_populated_and_nested_cardinality() {
    for (text, counts, edges) in [
        ("local a = []", vec![0], 0),
        ("local a = [10, 20]", vec![2], 2),
        ("local a = [[10], []]", vec![2, 1, 0], 3),
    ] {
        let product = source(text);
        let shape = product.body_shape();
        assert_eq!(
            shape
                .expressions()
                .iter()
                .filter_map(|row| match row {
                    BodyExpressionShapeV1::ArrayLiteral { element_count, .. } =>
                        Some(*element_count),
                    _ => None,
                })
                .collect::<Vec<_>>(),
            counts
        );
        assert_eq!(
            shape
                .relations()
                .iter()
                .filter(|row| matches!(row.role(), SourcePathSegmentV1::Element(_)))
                .count(),
            edges
        );
        validate_array_relations(shape.expressions(), shape.relations()).unwrap();
    }
}

#[test]
fn array_seal_rejects_dropped_duplicate_extra_and_foreign_child_edges() {
    let product = source("local a = [10, 20]");
    let shape = product.body_shape();
    let original = shape.relations().to_vec();
    let index = original
        .iter()
        .position(|row| row.role() == &SourcePathSegmentV1::Element(0))
        .unwrap();
    let edge = original[index].clone();
    let without_all = original
        .iter()
        .filter(|row| !matches!(row.role(), SourcePathSegmentV1::Element(_)))
        .cloned()
        .collect::<Vec<_>>();
    assert!(validate_array_relations(shape.expressions(), &without_all).is_err());
    let mut missing = original.clone();
    missing.remove(index);
    assert!(validate_array_relations(shape.expressions(), &missing).is_err());
    let mut duplicate = original.clone();
    duplicate.push(edge.clone());
    assert!(validate_array_relations(shape.expressions(), &duplicate).is_err());
    let mut extra = original.clone();
    let mut extra_edge = edge.clone();
    extra_edge.role = SourcePathSegmentV1::Element(2);
    extra.push(extra_edge);
    assert!(validate_array_relations(shape.expressions(), &extra).is_err());
    let mut foreign = original.clone();
    foreign[index].child = SourcePathV1::program_body()
        .child(SourcePathSegmentV1::ProgramBody(99))
        .expr();
    assert!(validate_array_relations(shape.expressions(), &foreign).is_err());
    let mut wrong_ordinal = original.clone();
    wrong_ordinal[index].role = SourcePathSegmentV1::Element(1);
    assert!(validate_array_relations(shape.expressions(), &wrong_ordinal).is_err());
    let mut wrong_parent = original.clone();
    wrong_parent[index].parent = SourcePathV1::program_body().node().clone();
    assert!(validate_array_relations(shape.expressions(), &wrong_parent).is_err());
    let mut second_parent = original.clone();
    let mut other = edge;
    other.parent = SourcePathV1::program_body().node().clone();
    other.role = SourcePathSegmentV1::Argument(0);
    second_parent.push(other);
    assert!(validate_array_relations(shape.expressions(), &second_parent).is_err());
}
