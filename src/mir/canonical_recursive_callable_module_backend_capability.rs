use crate::mir::canonical_recursive_callable_module_capability::{
    CanonicalRecursiveCallableModuleCapabilityV1, CANONICAL_RECURSIVE_CALLABLE_MODULE_CAPABILITY_V1,
};
use crate::mir::MirModule;

pub(crate) const CANONICAL_RECURSIVE_CALLABLE_MODULE_BACKEND_UNSUPPORTED_TAG: &str =
    "[backend/canonical_recursive_callable_module_v1_unsupported]";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CanonicalRecursiveCallableModuleBackendCapabilityReport {
    pub capability_rows: usize,
}

pub(crate) fn inspect(
    module: &MirModule,
) -> CanonicalRecursiveCallableModuleBackendCapabilityReport {
    CanonicalRecursiveCallableModuleBackendCapabilityReport {
        capability_rows: usize::from(
            module
                .metadata
                .canonical_recursive_callable_module_capability
                .is_some(),
        ),
    }
}

pub(crate) fn enforce(module: &MirModule, backend: &str) -> Result<(), String> {
    let marker = module
        .metadata
        .canonical_recursive_callable_module_capability
        .as_ref();
    if marker.is_none() {
        return Ok(());
    }
    CanonicalRecursiveCallableModuleCapabilityV1::verify_required(marker)
        .map_err(str::to_string)?;
    if backend == "mir-interpreter" {
        return Ok(());
    }
    Err(format!(
        "{} backend={} capability_rows=1 require={} silent_fallback_allowed=false",
        CANONICAL_RECURSIVE_CALLABLE_MODULE_BACKEND_UNSUPPORTED_TAG,
        backend,
        CANONICAL_RECURSIVE_CALLABLE_MODULE_CAPABILITY_V1
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::canonical_recursive_callable_module_capability::CanonicalRecursiveCallableModuleCapabilityV1;

    #[test]
    fn absent_marker_is_neutral_and_vm_accepts_exact_marker() {
        let mut module = MirModule::new("recursive-capability".to_string());
        assert_eq!(inspect(&module).capability_rows, 0);
        enforce(&module, "wasm").unwrap();

        CanonicalRecursiveCallableModuleCapabilityV1::install_for_module(
            &mut module
                .metadata
                .canonical_recursive_callable_module_capability,
            true,
        )
        .unwrap();
        assert_eq!(inspect(&module).capability_rows, 1);
        enforce(&module, "mir-interpreter").unwrap();
    }

    #[test]
    fn exact_marker_rejects_every_unsupported_backend_without_fallback() {
        let mut module = MirModule::new("recursive-capability".to_string());
        CanonicalRecursiveCallableModuleCapabilityV1::install_for_module(
            &mut module
                .metadata
                .canonical_recursive_callable_module_capability,
            true,
        )
        .unwrap();
        for backend in ["llvm", "wasm", "wasm-v2", "pyvm-harness"] {
            let error = enforce(&module, backend).unwrap_err();
            assert!(error.contains(CANONICAL_RECURSIVE_CALLABLE_MODULE_BACKEND_UNSUPPORTED_TAG));
            assert!(error.contains("silent_fallback_allowed=false"));
        }
    }
}
