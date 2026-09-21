use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

use super::{
    issue_normal_callable_semantic_package_v1, LoopBreakSourcePackageLoanV1,
    NormalCallableDynamicProjectionRefV1, NormalCallableSemanticPackageIssueV1,
};

fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("normal callable source");
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("exact callable transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("fixture must remain source-backed")
        };
        source
    })
}

fn issue(
    source: &str,
) -> Result<super::VerifiedNormalCallableSemanticPackageV1, NormalCallableSemanticPackageIssueV1> {
    let mut resolver = FunctionSemanticResolverSessionV1::new(93).unwrap();
    issue_normal_callable_semantic_package_v1(&mut resolver, final_source(source))
}

#[test]
fn parser_scan_source_seals_one_dynamic_candidate_and_all_parameter_contracts() {
    let package = issue(include_str!(
        "../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"
    ))
    .expect("exact parser scan semantic package");

    assert_eq!(package.batch().declarations().len(), 4);
    assert_eq!(
        package.loop_break_source_row_count(),
        package.batch().declarations().len()
    );
    assert_eq!(package.parameter_declaration_count(), 4);
    assert_eq!(package.parameter_count(), 15);
    let NormalCallableDynamicProjectionRefV1::Selected { program } = package.dynamic_projection()
    else {
        panic!("exact Dynamic row must remain selected")
    };
    assert_eq!(
        program.current(),
        crate::mir::compiler::dynamic_full_body_recipe::DynamicInvocationCleanupCurrentDispositionV1::ExactI64TrivialNoEnd
    );
}

#[test]
fn parser_program_source_retains_composite_loopbreak_recipe_candidate() {
    let mut package = issue(include_str!(
        "../../../lang/src/compiler/parser/program/parser_program_box.hako"
    ))
    .expect("parser program composite LoopBreak semantic package");

    assert_eq!(
        package.loop_break_source_row_count(),
        package.batch().declarations().len()
    );
    assert_eq!(package.loop_break_source_candidate_count(), 1);
    let owner = package
        .loop_break_source
        .candidate_owner()
        .expect("composite candidate owner");
    let mut loan = package
        .loop_break_source
        .take_for_owner(owner)
        .expect("owner receives composite candidate");
    assert!(matches!(
        loan,
        LoopBreakSourcePackageLoanV1::CompositeCandidate(_)
    ));
    let sites = match &loan {
        LoopBreakSourcePackageLoanV1::CompositeCandidate(facts) => facts
            .candidates()
            .iter()
            .map(|candidate| candidate.projection().loop_site().clone())
            .collect::<Vec<_>>(),
        _ => panic!("fixture must issue a composite candidate"),
    };
    assert!(
        !sites.is_empty(),
        "composite candidate inventory must be finite"
    );
    for site in sites {
        assert!(loan.take_composite_candidate_for_site(&site).is_some());
    }
    loan.finish_empty()
        .expect("composite candidate is consumed exactly once");
}

#[test]
fn direct_loop_break_source_package_retains_one_candidate_row() {
    let package = issue(
        r#"
static box Main {
    main() {
        local i = 0
        local sum = 0
        loop(i < 3) {
            if i == 3 { break }
            sum = sum + 1
            i = i + 1
        }
        return sum
    }
}
"#,
    )
    .expect("direct LoopBreak semantic package");

    assert_eq!(package.loop_break_source_row_count(), 1);
    assert_eq!(package.loop_break_source_candidate_count(), 1);
}

#[test]
fn loop_break_source_candidate_is_taken_once_by_its_owner() {
    let mut package = issue(
        r#"
static box Main {
    main() {
        local i = 0
        local sum = 0
        loop(i < 3) {
            if i == 3 { break }
            sum = sum + 1
            i = i + 1
        }
        return sum
    }
}
"#,
    )
    .expect("direct LoopBreak semantic package");
    let owner = package
        .loop_break_source
        .candidate_owner()
        .expect("candidate owner");

    let loan = package
        .loop_break_source
        .take_for_owner(owner)
        .expect("owner receives the sealed candidate row");
    assert!(matches!(loan, LoopBreakSourcePackageLoanV1::Candidate(_)));
    let duplicate = package.loop_break_source.take_for_owner(owner);
    assert!(duplicate
        .expect_err("a consumed owner row cannot be taken again")
        .contains("owner-row-duplicate-take"));
}

#[test]
fn loop_break_source_candidate_rejects_finish_with_a_residual_row() {
    let mut package = issue(
        r#"
static box Main {
    main() {
        local i = 0
        local sum = 0
        loop(i < 3) {
            if i == 3 { break }
            sum = sum + 1
            i = i + 1
        }
        return sum
    }
}
"#,
    )
    .expect("direct LoopBreak semantic package");
    let owner = package
        .loop_break_source
        .candidate_owner()
        .expect("candidate owner");
    let loan = package
        .loop_break_source
        .take_for_owner(owner)
        .expect("owner receives the candidate row");
    let error = loan
        .finish_empty()
        .expect_err("an unconsumed candidate must not disappear at finish");
    assert!(error.contains("residual-candidates"));
}

#[test]
fn loop_break_source_candidate_finishes_after_exact_site_take() {
    let mut package = issue(
        r#"
static box Main {
    main() {
        local i = 0
        local sum = 0
        loop(i < 3) {
            if i == 3 { break }
            sum = sum + 1
            i = i + 1
        }
        return sum
    }
}
"#,
    )
    .expect("direct LoopBreak semantic package");
    let owner = package
        .loop_break_source
        .candidate_owner()
        .expect("candidate owner");
    let mut loan = package
        .loop_break_source
        .take_for_owner(owner)
        .expect("owner receives the candidate row");
    let site = match &loan {
        LoopBreakSourcePackageLoanV1::Candidate(facts) => facts
            .candidates()
            .first()
            .expect("candidate")
            .projection()
            .loop_site()
            .clone(),
        LoopBreakSourcePackageLoanV1::SupportedNonCandidate { .. } => {
            panic!("fixture must issue a candidate")
        }
        LoopBreakSourcePackageLoanV1::CompositeCandidate(_) => {
            panic!("direct fixture must not issue a composite candidate")
        }
    };
    assert!(loan.take_candidate_for_site(&site).is_some());
    loan.finish_empty()
        .expect("an exact candidate take leaves an empty loan");
}

#[test]
fn loop_break_source_package_retains_typed_absence_for_unsupported_shape() {
    let mut package = issue(
        r#"
static box Main {
    main() {
        local i = 0
        loop(i < 3) {
            i = i + 1
        }
        return i
    }
}
"#,
    )
    .expect("unsupported loop shape remains a valid semantic package");

    assert_eq!(package.loop_break_source_row_count(), 1);
    assert_eq!(package.loop_break_source_candidate_count(), 0);
    let owner = package
        .loop_break_source
        .first_owner()
        .expect("typed absence owner");
    let loan = package
        .loop_break_source
        .take_for_owner(owner)
        .expect("owner receives typed absence");
    loan.finish_empty()
        .expect("typed absence must finish without a residual candidate");
}

#[test]
fn loop_break_source_package_retains_typed_absence_for_specialized_topology() {
    let package = issue(
        r#"
static box Main {
    main() {
        local s = "123"
        local i = 0
        local acc = ""
        loop(true) {
            local ch = s.substring(i, i + 1)
            if ch == "" { break }
            if ch >= "0" && ch <= "9" {
                acc = acc + ch
                i = i + 1
            } else {
                break
            }
        }
        return acc
    }
}
"#,
    )
    .expect("specialized LoopBreak remains a valid semantic package");

    assert_eq!(package.loop_break_source_row_count(), 1);
    assert_eq!(package.loop_break_source_candidate_count(), 1);
}
