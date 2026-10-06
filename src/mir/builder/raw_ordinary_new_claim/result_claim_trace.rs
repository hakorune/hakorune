//! Opt-in observations of existing affine results; never takes or issues claims.
use crate::mir::normal_callable_semantic_package::OrdinaryNewResultClaimV1;
use crate::mir::resolved_semantics::OwnedExprSiteV1;

pub(super) fn missing(reason: &str) {
    if crate::config::env::builder_mir_compile_trace() {
        eprintln!("[mir-compile/result-new-take] state=unavailable reason={reason}");
    }
}

pub(super) fn take<E: std::fmt::Debug>(
    site: &OwnedExprSiteV1,
    class: &str,
    arity: usize,
    result: &Result<Option<OrdinaryNewResultClaimV1>, E>,
) {
    if !crate::config::env::builder_mir_compile_trace() {
        return;
    }
    match result {
        Ok(Some(claim)) => eprintln!(
            "[mir-compile/result-new-take] site={site:?} class={class} arity={arity} state=claimed prefix_error={:?} argument_error={:?} construction_error={:?}",
            claim.home_prefix().err(), claim.argument_rows().err(),
            claim.construction().as_ref().err()
        ),
        Ok(None) => eprintln!(
            "[mir-compile/result-new-take] site={site:?} class={class} arity={arity} state=absent"
        ),
        Err(error) => eprintln!(
            "[mir-compile/result-new-take] site={site:?} class={class} arity={arity} state=rejected error={error:?}"
        ),
    }
}

pub(super) fn prepare(claim: &OrdinaryNewResultClaimV1, result: &Result<bool, String>) {
    if crate::config::env::builder_mir_compile_trace() {
        eprintln!(
            "[mir-compile/result-new-prepare] site={:?} class={} selected={result:?}",
            claim.site(),
            claim.class()
        );
    }
}
