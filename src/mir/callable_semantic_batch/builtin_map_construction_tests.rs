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
            if is_builtin_empty_map_new_v1(node, coverage)
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

#[test]
fn a_static_box_named_map_box_is_a_declaration_and_keeps_its_neutral_row() {
    let source = "static box MapBox { ping(a) { return a } }\n\
                  static box Main { main() { local m = new MapBox()\n return 1 } }";
    let (maps, others, _) = rows(source);
    assert!(maps.is_empty(), "a static MapBox is never sealed as the builtin Map");
    assert!(!others.is_empty());
    let batch = batch(source);
    let coverage = batch.ordinary_box_coverage();
    assert!(coverage.declares_non_ordinary_box("MapBox"));
    assert!(!coverage.contains_box("MapBox"));
    assert!(coverage.declares_box("MapBox"));
}

#[test]
fn a_static_box_without_methods_still_declares_its_name() {
    // No callable row mentions `MapBox`; the declaration alone shadows it.
    let batch = batch("static box MapBox {}\nstatic box Main { main() { return 1 } }");
    let coverage = batch.ordinary_box_coverage();
    assert!(coverage.declares_non_ordinary_box("MapBox"));
    assert!(!coverage.declares_box("ArrayBox"));
}

#[test]
fn a_static_declaration_does_not_hide_the_literal_or_a_user_box() {
    let (maps, _, _) = rows(
        "static box MapBox { ping(a) { return a } }\n\
         static box Main { main() { local m = %{\"k\" => 1}\n return 1 } }",
    );
    assert_eq!(maps.len(), 1, "only the `%{{}}` literal is a Map row");
}

// The limited re-issue: the sealer is the only entrance, and it changes a row
// only for the original source `new` of the same owner that the declaration
// environment resolves to the builtin Map.

use super::builtin_map_construction::seal_builtin_empty_map_constructions_v1;
use crate::mir::resolved_semantics::VerifiedResolvedBodyShapeInventoryV1;

const STATIC_DECL: &str = "static box MapBox { ping(a) { return a } }\n\
    static box Main { main() { local m = new MapBox()\n return 1 }\n helper(a) { return a } }";
const NO_DECL: &str = "static box Main { main() { local m = new MapBox()\n return 1 }\n helper(a) { return a } }";

/// Run `f` on a private copy of the issued shape and the source root of the
/// callable with `parameters` parameters.
fn with_copy<R>(
    source: &str,
    parameters: u32,
    f: impl FnOnce(VerifiedResolvedBodyShapeInventoryV1, &crate::ast::ASTNode) -> R,
) -> R {
    let batch = batch(source);
    let slot = batch
        .declarations()
        .find(|row| row.parameter_count() == parameters)
        .expect("callable")
        .batch_slot();
    batch
        .with_lowering_input(slot, |input| {
            f(input.body_shape().expect("shape").duplicate_for_test(), input.source().root())
        })
        .expect("lowering input")
}

fn map_rows(shape: &VerifiedResolvedBodyShapeInventoryV1) -> usize {
    shape
        .expressions()
        .iter()
        .filter(|row| matches!(row, BodyExpressionShapeV1::MapLiteral { .. }))
        .count()
}

fn coverage_of(source: &str) -> crate::parser::ParserOrdinaryBoxSourceCoverageV1 {
    batch(source).ordinary_box_coverage().clone()
}

#[test]
fn the_environment_without_a_declaration_resolves_a_static_site_to_the_builtin() {
    let free = coverage_of(NO_DECL);
    // The static-declared program's neutral row is rewritten only when the
    // environment says the name is undeclared (positive control).
    let rewritten = with_copy(STATIC_DECL, 0, |shape, root| {
        assert_eq!(map_rows(&shape), 0);
        map_rows(&seal_builtin_empty_map_constructions_v1(shape, root, &free).unwrap())
    });
    assert_eq!(rewritten, 1);
}

#[test]
fn the_static_declaration_in_the_environment_keeps_the_row_neutral() {
    let declared = coverage_of(STATIC_DECL);
    let rows = with_copy(STATIC_DECL, 0, |shape, root| {
        map_rows(&seal_builtin_empty_map_constructions_v1(shape, root, &declared).unwrap())
    });
    assert_eq!(rows, 0);
}

#[test]
fn a_site_is_not_rewritten_against_another_owners_source() {
    let free = coverage_of(NO_DECL);
    let helper_root = with_copy(STATIC_DECL, 1, |_, root| root.clone());
    let rows = with_copy(STATIC_DECL, 0, |shape, _| {
        map_rows(&seal_builtin_empty_map_constructions_v1(shape, &helper_root, &free).unwrap())
    });
    assert_eq!(rows, 0);
}

#[test]
fn a_new_with_an_argument_is_not_rewritten() {
    let free = coverage_of(NO_DECL);
    let rows = with_copy(
        "static box Main { main() { local m = new MapBox(1)\n return 1 } }",
        0,
        |shape, root| map_rows(&seal_builtin_empty_map_constructions_v1(shape, root, &free).unwrap()),
    );
    assert_eq!(rows, 0);
}

#[test]
fn a_rewritten_row_with_a_forged_source_child_is_refused() {
    let free = coverage_of(NO_DECL);
    let refused = with_copy(STATIC_DECL, 0, |shape, root| {
        let neutral = shape
            .expressions()
            .iter()
            .find_map(|row| match row {
                BodyExpressionShapeV1::Other { site, .. } => Some(site.clone()),
                _ => None,
            })
            .expect("the neutral `new` row");
        let child = shape
            .expressions()
            .iter()
            .map(|row| row.site().clone())
            .find(|site| site != &neutral)
            .expect("another row");
        seal_builtin_empty_map_constructions_v1(
            shape.with_forged_entry_for_test(&neutral, &child),
            root,
            &free,
        )
        .is_err()
    });
    assert!(refused);
}

// The builder's states: it reads the issued row and never decides again.

use super::builtin_map_construction::issued_builtin_empty_map_new_v1;

fn issued_state(source: &str, shape_present: bool, row_present: bool) -> Result<bool, &'static str> {
    with_copy(source, 0, |shape, _| {
        let site = shape
            .expressions()
            .iter()
            .map(|row| row.site().clone())
            .find(|site| {
                matches!(
                    shape.expression_shape(site),
                    Some(BodyExpressionShapeV1::MapLiteral { .. })
                ) || matches!(
                    shape.expression_shape(site),
                    Some(BodyExpressionShapeV1::Other { kind, .. }) if kind.as_ref() == "New"
                )
            })
            .expect("the `new` row");
        use crate::mir::resolved_semantics::{SourcePathSegmentV1, SourcePathV1};
        let missing = SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(99))
            .expr();
        issued_builtin_empty_map_new_v1(
            shape_present.then_some(&shape),
            if row_present { &site } else { &missing },
        )
    })
}

#[test]
fn the_builder_reads_a_builtin_empty_map_user_construction_or_missing_source() {
    let new_map = "static box Main { main() { local m = new MapBox()\n return 1 } }";
    let user = "box MapBox { x: i64 }\nstatic box Main { main() { local m = new MapBox()\n return 1 } }";
    let static_decl = "static box MapBox { ping(a) { return a } }\n\
                       static box Main { main() { local m = new MapBox()\n return 1 } }";
    assert_eq!(issued_state(new_map, true, true), Ok(true));
    assert_eq!(issued_state(user, true, true), Ok(false));
    assert_eq!(issued_state(static_decl, true, true), Ok(false));
    assert_eq!(issued_state(new_map, false, true), Err("map-shape-missing"));
    assert_eq!(issued_state(new_map, true, false), Err("map-shape-row-missing"));
}
