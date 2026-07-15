use crate::mir::canonical_direct_static_call_capability::CANONICAL_DIRECT_STATIC_CALL_CAPABILITY_V1;
use crate::mir::MirModule;

pub(crate) const CANONICAL_DIRECT_STATIC_CALL_BACKEND_UNSUPPORTED_TAG: &str =
    "[backend/canonical_direct_static_call_v1_unsupported]";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CanonicalDirectStaticCallBackendCapabilityReport {
    pub capability_rows: usize,
}

pub(crate) fn inspect(module: &MirModule) -> CanonicalDirectStaticCallBackendCapabilityReport {
    CanonicalDirectStaticCallBackendCapabilityReport {
        capability_rows: module
            .functions
            .values()
            .map(|function| {
                function
                    .metadata
                    .canonical_direct_static_call_capabilities
                    .len()
            })
            .sum(),
    }
}

pub(crate) fn enforce(module: &MirModule, backend: &str) -> Result<(), String> {
    let report = inspect(module);
    if report.capability_rows == 0 || backend == "mir-interpreter" {
        return Ok(());
    }
    Err(format!(
        "{} backend={} capability_rows={} require={} silent_fallback_allowed=false",
        CANONICAL_DIRECT_STATIC_CALL_BACKEND_UNSUPPORTED_TAG,
        backend,
        report.capability_rows,
        CANONICAL_DIRECT_STATIC_CALL_CAPABILITY_V1
    ))
}

#[cfg(test)]
#[path = "canonical_direct_static_call_backend_capability_tests.rs"]
mod tests;
