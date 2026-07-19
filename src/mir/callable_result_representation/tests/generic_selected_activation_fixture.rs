//! Generic selected-call fixture, deliberately independent of Parser source.

use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourcePathSegmentV1};
use crate::mir::source_call_target::VerifiedSourceStaticCallTargetCatalogV1;

use super::super::{
    VerifiedCallableResultActivationPlanV1, VerifiedCallableResultActivationRowsV1,
    VerifiedSameModuleCallableResultCatalogV1,
};
use super::support::{
    declarations, key, qualified_targets, seal_with_targets, site, CallSiteSpecV1,
};

const SOURCE: &str = r#"
    static box Provider {
        step(value) { return value }
    }
    static box Caller {
        run() { return Provider.step(41) }
    }
"#;

pub(crate) fn call_site() -> SourceExprSiteV1 {
    site(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ])
}

pub(crate) fn plan() -> VerifiedCallableResultActivationPlanV1 {
    let declarations = Box::new(declarations(SOURCE));
    let targets = qualified_targets(
        declarations.as_ref(),
        &[],
        &[CallSiteSpecV1 {
            caller_owner: "Caller",
            caller_name: "run",
            caller_arity: 0,
            site: call_site(),
        }],
    );
    let results = seal_with_targets(declarations.as_ref(), &targets);
    let rows =
        VerifiedCallableResultActivationRowsV1::verify(declarations.as_ref(), &targets, &results)
            .expect("generic selected activation rows");
    drop(results);
    drop(targets);
    VerifiedCallableResultActivationPlanV1::seal(declarations, rows)
        .expect("generic selected activation plan")
}

pub(crate) fn caller(
    plan: &VerifiedCallableResultActivationPlanV1,
) -> CanonicalSameModuleCallableKeyV1 {
    key(plan.declaration_catalog(), "Caller", "run", 0)
}

pub(crate) fn with_source_gate_inputs<R>(
    f: impl FnOnce(
        &VerifiedSameModuleCallableDeclarationCatalogV1,
        &CanonicalSameModuleCallableKeyV1,
        &SourceExprSiteV1,
        &VerifiedSourceStaticCallTargetCatalogV1<'_>,
        &VerifiedSameModuleCallableResultCatalogV1<'_, '_>,
    ) -> R,
) -> R {
    let declarations = declarations(SOURCE);
    let call_site = call_site();
    let targets = qualified_targets(
        &declarations,
        &[],
        &[CallSiteSpecV1 {
            caller_owner: "Caller",
            caller_name: "run",
            caller_arity: 0,
            site: call_site.clone(),
        }],
    );
    let results = seal_with_targets(&declarations, &targets);
    let caller = key(&declarations, "Caller", "run", 0);
    f(&declarations, &caller, &call_site, &targets, &results)
}
