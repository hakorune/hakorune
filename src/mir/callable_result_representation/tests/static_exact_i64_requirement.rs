use crate::mir::resolved_semantics::{SourceNodeSiteV1, SourcePathSegmentV1};

use super::super::{project_static_exact_i64_requirement_v1, StaticExactI64RequirementErrorV1};
use super::support::{
    declarations as seal_declarations, key, qualified_targets, seal_with_targets, site,
    CallSiteSpecV1,
};

const SOURCE: &str = r#"
static box Helper {
  identity(x) { return x }
  text(x) { return "text" }
}

static box Caller {
  exact(x) { return Helper.identity(x) }
  unavailable(x) { return Helper.text(x) }
}
"#;

fn return_site() -> crate::mir::resolved_semantics::SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ])
}

fn build_targets(
    declarations: &crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1,
) -> crate::mir::source_call_target::VerifiedSourceStaticCallTargetCatalogV1<'_> {
    qualified_targets(
        declarations,
        &[],
        &[
            CallSiteSpecV1 {
                caller_owner: "Caller",
                caller_name: "exact",
                caller_arity: 1,
                site: return_site(),
            },
            CallSiteSpecV1 {
                caller_owner: "Caller",
                caller_name: "unavailable",
                caller_arity: 1,
                site: return_site(),
            },
        ],
    )
}

#[test]
fn ordinary_exact_call_row_is_not_reclassified_as_the_bounded_requirement() {
    let declarations = seal_declarations(SOURCE);
    let targets = build_targets(&declarations);
    let results = seal_with_targets(&declarations, &targets);
    let caller = key(&declarations, "Caller", "exact", 1);

    assert_eq!(
        project_static_exact_i64_requirement_v1(
            &declarations,
            &caller,
            &return_site(),
            &targets,
            &results,
        )
        .unwrap_err(),
        StaticExactI64RequirementErrorV1::GeneralCallResultAlreadyAvailable
    );
}

#[test]
fn unavailable_static_result_rejects_before_any_bounded_bridge() {
    let declarations = seal_declarations(SOURCE);
    let targets = build_targets(&declarations);
    let results = seal_with_targets(&declarations, &targets);
    let caller = key(&declarations, "Caller", "unavailable", 1);

    assert_eq!(
        project_static_exact_i64_requirement_v1(
            &declarations,
            &caller,
            &return_site(),
            &targets,
            &results,
        )
        .unwrap_err(),
        StaticExactI64RequirementErrorV1::TargetResultUnavailable
    );
}

#[test]
fn missing_source_target_and_foreign_catalogs_fail_closed() {
    let declarations = seal_declarations(SOURCE);
    let targets = build_targets(&declarations);
    let results = seal_with_targets(&declarations, &targets);
    let caller = key(&declarations, "Caller", "exact", 1);
    let missing = crate::mir::resolved_semantics::SourceExprSiteV1::from_node(
        SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(7),
            SourcePathSegmentV1::Value,
        ]),
    );
    assert_eq!(
        project_static_exact_i64_requirement_v1(
            &declarations,
            &caller,
            &missing,
            &targets,
            &results,
        )
        .unwrap_err(),
        StaticExactI64RequirementErrorV1::SourceTargetUnavailable
    );

    let foreign_declarations = seal_declarations(SOURCE);
    let foreign_caller = key(&foreign_declarations, "Caller", "exact", 1);
    assert_eq!(
        project_static_exact_i64_requirement_v1(
            &foreign_declarations,
            &foreign_caller,
            &return_site(),
            &targets,
            &results,
        )
        .unwrap_err(),
        StaticExactI64RequirementErrorV1::TargetCatalogBrandMismatch
    );

    let second_targets = build_targets(&declarations);
    assert_eq!(
        project_static_exact_i64_requirement_v1(
            &declarations,
            &caller,
            &return_site(),
            &second_targets,
            &results,
        )
        .unwrap_err(),
        StaticExactI64RequirementErrorV1::ResultCatalogBrandMismatch
    );
}
