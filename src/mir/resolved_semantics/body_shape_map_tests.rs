use super::array_tests::source;
use super::*;
use crate::mir::resolved_semantics::SourcePathV1;

#[test]
fn map_source_preserves_empty_duplicate_and_nested_keys() {
    for (text, keys, count) in [
        ("local m = %{}", vec![vec![]], 0),
        (
            "local m = %{\"dup\" => 1, \"dup\" => 2}",
            vec![vec!["dup", "dup"]],
            2,
        ),
        (
            "local m = %{\"nested\" => %{\"b\" => true}, \"array\" => [1]}",
            vec![vec!["nested", "array"], vec!["b"]],
            3,
        ),
    ] {
        let product = source(text);
        let shape = product.body_shape();
        let actual = shape
            .expressions()
            .iter()
            .filter_map(|row| match row {
                BodyExpressionShapeV1::MapLiteral { keys, .. } => {
                    Some(keys.iter().map(|key| key.as_ref()).collect::<Vec<_>>())
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(actual, keys);
        assert_eq!(
            shape
                .relations()
                .iter()
                .filter(|row| matches!(row.role(), SourcePathSegmentV1::EntryValue(_)))
                .count(),
            count
        );
        validate_map_relations(shape.expressions(), shape.relations()).unwrap();
    }
}

#[test]
fn map_seal_rejects_lost_duplicate_foreign_and_misindexed_children() {
    let product = source("local m = %{\"a\" => 1, \"a\" => 2}");
    let shape = product.body_shape();
    let original = shape.relations().to_vec();
    let index = original
        .iter()
        .position(|r| r.role() == &SourcePathSegmentV1::EntryValue(0))
        .unwrap();
    let edge = original[index].clone();
    let mut missing = original.clone();
    missing.remove(index);
    let mut duplicate = original.clone();
    duplicate.push(edge.clone());
    let mut wrong_ordinal = original.clone();
    wrong_ordinal[index].role = SourcePathSegmentV1::EntryValue(1);
    let mut wrong_parent = original.clone();
    wrong_parent[index].parent = SourcePathV1::program_body().node().clone();
    let mut foreign = original.clone();
    foreign[index].child = SourcePathV1::program_body()
        .child(SourcePathSegmentV1::ProgramBody(99))
        .expr();
    let mut extra_parent = original.clone();
    let mut other = edge;
    other.parent = SourcePathV1::program_body().node().clone();
    other.role = SourcePathSegmentV1::Argument(0);
    extra_parent.push(other);
    for rows in [
        missing,
        duplicate,
        wrong_ordinal,
        wrong_parent,
        foreign,
        extra_parent,
    ] {
        assert!(validate_map_relations(shape.expressions(), &rows).is_err());
    }
}
