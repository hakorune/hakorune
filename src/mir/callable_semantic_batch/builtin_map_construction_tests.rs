//! The empty `new MapBox()` is the builtin Map only where no user box of the
//! same name is declared, and the decision is made against the source node at
//! the site, not from the shape row's name.

use super::builtin_map_construction::is_builtin_empty_map_new_v1;
use crate::mir::resolved_semantics::{project_source_node_v1, ProjectedSourceNodeV1};
use super::tests::batch;
use crate::mir::resolved_semantics::{BodyExpressionShapeV1, SourceExprSiteV1};

/// (sites sealed as `MapLiteral`, sites left neutral as `Other`) of `main`.
/// The source node at `site` is the builtin empty Map construction.
fn is_map_construction(
    owner_root: &crate::ast::ASTNode,
    site: &SourceExprSiteV1,
    coverage: &crate::parser::ParserOrdinaryBoxSourceCoverageV1,
) -> bool {
    matches!(
        project_source_node_v1(owner_root, site.node()),
        Some(ProjectedSourceNodeV1::Node(node))
            if is_builtin_empty_map_new_v1(node, |name| coverage.contains_box(name))
    )
}

fn rows(source: &str) -> (Vec<SourceExprSiteV1>, Vec<SourceExprSiteV1>, bool) {
    let batch = batch(source);
    let slot = batch
        .declarations()
        .find(|row| row.parameter_count() == 0)
        .expect("main")
        .batch_slot();
    batch
        .with_lowering_input(slot, |input| {
            let shape = input.body_shape().expect("shape");
            let mut maps = Vec::new();
            let mut others = Vec::new();
            for row in shape.expressions() {
                match row {
                    BodyExpressionShapeV1::MapLiteral { site, .. } => maps.push(site.clone()),
                    BodyExpressionShapeV1::Other { site, .. } => others.push(site.clone()),
                    _ => {}
                }
            }
            let owner_root = input.source().root();
            let coverage = batch.ordinary_box_coverage();
            let correspondence = others
                .iter()
                .chain(maps.iter())
                .filter(|site| is_map_construction(owner_root, site, coverage))
                .count()
                == maps.len();
            (maps, others, correspondence)
        })
        .expect("lowering input")
}

#[test]
fn a_new_map_box_without_a_user_declaration_is_the_empty_map_row() {
    let (maps, _, correspondence) =
        rows("static box Main { main() { local m = new MapBox()\n return 1 } }");
    assert_eq!(maps.len(), 1);
    assert!(correspondence);
}

#[test]
fn a_user_box_named_map_box_keeps_its_neutral_row() {
    let (maps, others, _) = rows(
        "box MapBox { x: i64 }\n\
         static box Main { main() { local m = new MapBox()\n return 1 } }",
    );
    assert!(maps.is_empty(), "a user MapBox is never sealed as the builtin Map");
    assert!(!others.is_empty());
}

#[test]
fn the_user_declaration_decides_wherever_it_is_placed() {
    let (before, _, _) = rows(
        "box MapBox { x: i64 }\n\
         static box Main { main() { local m = new MapBox()\n return 1 } }",
    );
    let (after, _, _) = rows(
        "static box Main { main() { local m = new MapBox()\n return 1 } }\n\
         box MapBox { x: i64 }",
    );
    assert!(before.is_empty() && after.is_empty());
}

#[test]
fn a_map_literal_and_a_user_map_box_do_not_confuse_each_other() {
    let (maps, _, _) = rows(
        "box MapBox { x: i64 }\n\
         static box Main { main() { local u = new MapBox()\n local m = %{\"k\" => 1}\n return 1 } }",
    );
    assert_eq!(maps.len(), 1, "only the `%{{}}` literal is a Map row");
}

#[test]
fn evidence_moved_to_another_new_site_is_refused_by_the_source_check() {
    let (_, others, _) = rows(
        "box Foo { x: i64 }\n\
         static box Main { main() { local a = new Foo()\n local b = new MapBox(1)\n return 1 } }",
    );
    // Neither a user `new` nor an argument-taking `new MapBox` is the builtin
    // empty Map, whichever row the evidence is attached to.
    let batch = batch(
        "box Foo { x: i64 }\n\
         static box Main { main() { local a = new Foo()\n local b = new MapBox(1)\n return 1 } }",
    );
    let slot = batch.declarations().find(|row| row.parameter_count() == 0).unwrap().batch_slot();
    let accepted = batch
        .with_lowering_input(slot, |input| {
            others
                .iter()
                .filter(|site| {
                    is_map_construction(
                        input.source().root(),
                        site,
                        batch.ordinary_box_coverage(),
                    )
                })
                .count()
        })
        .unwrap();
    assert_eq!(accepted, 0);
}
