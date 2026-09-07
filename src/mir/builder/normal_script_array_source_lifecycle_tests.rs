use super::*;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveScriptOutcomeV1, ScriptRootBindingRebindAdmissionV1,
    ScriptRootResolvedDemandV1, ScriptRootRuntimeDispositionV1, ScriptSyntaxViewV1, SourcePathV1,
    VerifiedScriptRootDemandEntryV1,
};

pub(super) fn source(
    text: &str,
    unit: u32,
) -> (VerifiedResolvedScriptV1, VerifiedScriptRootDemandWindowV1) {
    let ast = crate::parser::NyashParser::parse_from_string(text).unwrap();
    let crate::ast::ASTNode::Program { statements, .. } = &ast else {
        panic!("Program")
    };
    let entries = statements
        .iter()
        .enumerate()
        .map(|(index, statement)| {
            let demand = if matches!(statement, crate::ast::ASTNode::Return { .. }) {
                ScriptRootResolvedDemandV1::ReturnExit(
                    crate::mir::resolved_semantics::ScriptRootReturnExitAdmissionV1::new(),
                )
            } else if matches!(
                statement,
                crate::ast::ASTNode::Assignment { .. }
                    | crate::ast::ASTNode::CompoundAssignment { .. }
            ) {
                ScriptRootResolvedDemandV1::BindingRebind(ScriptRootBindingRebindAdmissionV1::new())
            } else {
                ScriptRootResolvedDemandV1::LexicalCore
            };
            VerifiedScriptRootDemandEntryV1::new(
                SourcePathV1::program_body()
                    .child(SourcePathSegmentV1::ProgramBody(index as u32))
                    .stmt(),
                ScriptRootSemanticDispositionV1::Resolved(demand),
                ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
            )
        })
        .collect();
    let window = VerifiedScriptRootDemandWindowV1::seal(entries, statements.len()).unwrap();
    let mut resolver = FunctionSemanticResolverSessionV1::new(unit).unwrap();
    let ResolveScriptOutcomeV1::Complete(product) = resolver
        .resolve_script(ScriptSyntaxViewV1::from_program(&ast).unwrap(), &window)
        .unwrap()
    else {
        panic!("Complete Script required: {text}")
    };
    (product, window)
}

#[test]
fn all_numeric_specs_preserve_cutpoints_and_consume_exactly_once() {
    for spec in ["i8", "i16", "i32", "i64", "u8", "u16", "u32"] {
        let (product, window) = source(&format!("local a: Array<{spec}> = [10, 20]"), 0);
        let mut rows = ArraySourceLifecycleRows::issue(&product, &window).unwrap();
        let relation = product.expression_source().initializers().next().unwrap();
        let ArraySourceCoverage::Available(row) = rows.rows.values().next().unwrap() else {
            panic!("available: {:?}", rows)
        };
        assert_eq!(row.spec.element.source_name(), spec);
        assert!(row.caller_homes.is_empty());
        assert_eq!(row.cutpoints.len(), 12);
        assert!(matches!(row.cutpoints[0], Cutpoint::BeforeAllocation));
        assert!(matches!(row.cutpoints[1], Cutpoint::AcquiredBeforeClaim));
        assert!(matches!(row.cutpoints[2], Cutpoint::Claimed));
        for (index, group) in row.cutpoints[3..9].chunks_exact(3).enumerate() {
            let Cutpoint::ChildEvaluating(site) = &group[0] else {
                panic!("child")
            };
            assert_eq!(group[1], Cutpoint::ChildReady(site.clone()));
            assert_eq!(group[2], Cutpoint::Written(site.clone()));
            assert_eq!(
                site.node(),
                SourcePathV1::from_node(relation.initializer_site().unwrap().node())
                    .child(SourcePathSegmentV1::Element(index as u32))
                    .expr()
                    .node()
            );
        }
        assert_eq!(
            &row.cutpoints[9..],
            &[
                Cutpoint::FaultUnwind,
                Cutpoint::LocalCommit,
                Cutpoint::CallerExit
            ]
        );
        assert!(rows.finish().unwrap_err().contains("unconsumed-row"));
        rows.consume(relation).unwrap();
        rows.complete(relation).unwrap();
        rows.finish().unwrap();
        assert!(rows
            .consume(relation)
            .unwrap_err()
            .contains("duplicate-consume"));
    }
}

#[test]
fn committed_homes_unwind_in_reverse_and_aliases_do_not_duplicate_them() {
    let (product, window) = source(
        "local n = 30\nlocal a: Array<i64> = []\nlocal alias = a\nlocal b: Array<u8> = [10]\nlocal alias2 = alias\nlocal c: Array<i64> = [20]", 0);
    let mut rows = ArraySourceLifecycleRows::issue(&product, &window).unwrap();
    let relations: Vec<_> = product.expression_source().initializers().collect();
    let SourceBindingSiteV1::Local { statement, .. } = relations[5].declaration_site() else {
        panic!("Local")
    };
    let ArraySourceCoverage::Available(row) = &rows.rows[statement.node()] else {
        panic!("available")
    };
    assert_eq!(
        &*row.caller_homes,
        &[relations[3].binding(), relations[1].binding()]
    );
    for relation in relations {
        rows.consume(relation).unwrap();
        rows.complete(relation).unwrap();
    }
    rows.finish().unwrap();
}

#[test]
fn unknown_or_invalidated_source_never_manufactures_empty_obligations() {
    for text in [
        "local a: Array<i64> = [[10]]",
        "local a: Array<i64> = [\"x\"]",
        "local a: Array<i64> = [null]",
        "local s = \"x\"\nlocal a: Array<i64> = []",
        "local x = []\nlocal a: Array<i64> = []",
        "local a: Array<i64> = []\na = []\nlocal b: Array<i64> = []",
        "local n = 1\nn += 1\nlocal b: Array<i64> = []",
    ] {
        let (product, window) = source(text, 0);
        let mut rows = ArraySourceLifecycleRows::issue(&product, &window).unwrap();
        let relation = product.expression_source().initializers().last().unwrap();
        assert!(
            rows.consume(relation)
                .unwrap_err()
                .contains("source-lifecycle-unavailable"),
            "{text}"
        );
        assert!(rows.finish().is_err());
    }
}

#[test]
fn primitive_child_ownership_does_not_claim_numeric_write_success() {
    for text in [
        "local a: Array<u8> = [true]",
        "local a: Array<u8> = [1.5]",
        "local a: Array<u8> = [256]",
    ] {
        let (product, window) = source(text, 0);
        let mut rows = ArraySourceLifecycleRows::issue(&product, &window).unwrap();
        let relation = product.expression_source().initializers().next().unwrap();
        rows.consume(relation).unwrap();
        rows.complete(relation).unwrap();
        rows.finish().unwrap();
    }
}

#[test]
fn missing_foreign_spec_and_annotation_drift_reject_without_consumption() {
    let (product, window) = source("local a: Array<i64> = []", 0);
    let relation = product.expression_source().initializers().next().unwrap();
    assert!(ArraySourceLifecycleRows::default()
        .consume(relation)
        .unwrap_err()
        .contains("missing-row"));
    let mut rows = ArraySourceLifecycleRows::issue(&product, &window).unwrap();
    for (text, unit, error) in [
        ("local a: Array<i64> = []", 1, "source-drift"),
        ("local a: Array<u8> = []", 0, "source-drift"),
        ("local a = []", 0, "annotation-drift"),
    ] {
        let (other, _) = source(text, unit);
        assert!(rows
            .consume(other.expression_source().initializers().next().unwrap())
            .unwrap_err()
            .contains(error));
    }
    rows.consume(relation).unwrap();
    rows.complete(relation).unwrap();
    rows.finish().unwrap();
}

#[test]
fn root_completion_preserves_payload_and_checks_last_home_and_commit() {
    for spec in ["i8", "i16", "i32", "i64", "u8", "u16", "u32"] {
        for terminal in ["return 30", "return"] {
            let text = format!("local n = 1\nlocal a: Array<{spec}> = []\nlocal alias = a\nlocal b: Array<{spec}> = [20]\n{terminal}");
            let (product, window) = source(&text, 0);
            let mut rows = ArraySourceLifecycleRows::issue(&product, &window).unwrap();
            rows.require_root().unwrap();
            let relations = product
                .expression_source()
                .initializers()
                .collect::<Vec<_>>();
            assert_eq!(
                rows.terminal.require().unwrap().homes(),
                &[relations[3].binding(), relations[1].binding()]
            );
            assert!(rows.complete(relations[1]).is_err());
            assert!(rows
                .consume(relations[3])
                .unwrap_err()
                .contains("prior-home-not-completed"));
            for relation in &relations {
                rows.consume(relation).unwrap();
                assert!(rows.finish_root().is_err());
                rows.complete(relation).unwrap();
            }
            rows.finish_root().unwrap();
            for coverage in rows.rows.values() {
                let ArraySourceCoverage::Available(row) = coverage else {
                    panic!("source payload retained")
                };
                assert_eq!(row.progress, LocalProgress::Completed);
                assert!(!row.cutpoints.is_empty());
            }
            assert!(rows.complete(relations[1]).is_err());
            rows.rows.clear();
            assert!(rows
                .finish_root()
                .unwrap_err()
                .contains("terminal-home-drift"));
        }
    }
}

#[test]
fn root_unclassified_suffix_or_result_is_not_implicit_unit() {
    for text in [
        "local a: Array<i64> = []",
        "local a: Array<i64> = []\nlocal s = \"x\"\nreturn 30",
        "local a: Array<i64> = []\nreturn true",
        "local a: Array<i64> = []\nlocal alias = a\nreturn alias",
        "local a: Array<i64> = []\na = []\nreturn 30",
    ] {
        let (product, window) = source(text, 0);
        let rows = ArraySourceLifecycleRows::issue(&product, &window).unwrap();
        assert!(rows.require_root().is_err(), "{text}");
        assert!(rows.finish_root().is_err(), "{text}");
    }
}
