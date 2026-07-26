//! Typed request boundary for the default-off canonical-core reference lane.
//!
//! This owner selects neither a runner nor a backend implementation. It only
//! seals the one fixed normal-file profile before any file I/O.

use super::cli_profile::{seal_reference_cli_profile, ReferenceCliProfileErrorV1};
use super::normal_file_vm_frontdoor::{NormalFileRequestV1, NormalFileVmFrontDoorV1};
use crate::cli::CliConfig;
use std::path::PathBuf;

const NORMAL_FILE_CANONICAL_CORE_VM_REFERENCE_BACKEND: &str =
    "normal-file-canonical-core-vm-reference";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NormalFileCanonicalCoreVmReferenceProfileErrorV1 {
    Common(ReferenceCliProfileErrorV1),
    NonDefaultOptimizationRequested,
}

impl NormalFileCanonicalCoreVmReferenceProfileErrorV1 {
    pub(crate) const fn code(&self) -> &'static str {
        match self {
            Self::Common(error) => error.code(),
            Self::NonDefaultOptimizationRequested => "non-default-optimization-requested",
        }
    }
}

/// One explicit request owns one existing canonical-core front-door request.
#[derive(Debug)]
pub(crate) struct NormalFileCanonicalCoreVmReferenceProductionRequestV1 {
    request: NormalFileRequestV1,
    _seal: NormalFileCanonicalCoreVmReferenceProductionRequestSealV1,
}

#[derive(Debug)]
struct NormalFileCanonicalCoreVmReferenceProductionRequestSealV1;

impl NormalFileCanonicalCoreVmReferenceProductionRequestV1 {
    pub(crate) const fn backend_name() -> &'static str {
        NORMAL_FILE_CANONICAL_CORE_VM_REFERENCE_BACKEND
    }

    /// Selection is I/O-free and has no optimization/backend reconstruction.
    pub(crate) fn try_from_selected_cli(
        config: &CliConfig,
    ) -> Result<Self, NormalFileCanonicalCoreVmReferenceProfileErrorV1> {
        let source_file = seal_reference_cli_profile(config)
            .map_err(NormalFileCanonicalCoreVmReferenceProfileErrorV1::Common)?
            .into_source_file();
        if config.no_optimize {
            return Err(
                NormalFileCanonicalCoreVmReferenceProfileErrorV1::NonDefaultOptimizationRequested,
            );
        }
        Ok(Self {
            request: NormalFileVmFrontDoorV1::file_canonical_core_request(PathBuf::from(
                String::from(source_file),
            )),
            _seal: NormalFileCanonicalCoreVmReferenceProductionRequestSealV1,
        })
    }

    pub(crate) fn into_frontdoor_request(self) -> NormalFileRequestV1 {
        self.request
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn selected_config() -> CliConfig {
        let mut config = CliConfig::default();
        config.backend =
            NormalFileCanonicalCoreVmReferenceProductionRequestV1::backend_name().to_owned();
        config.file = Some("canonical-core.hako".to_owned());
        config
    }

    #[test]
    fn rejects_non_default_optimization_before_front_door_io() {
        let mut config = selected_config();
        config.no_optimize = true;
        assert!(matches!(
            NormalFileCanonicalCoreVmReferenceProductionRequestV1::try_from_selected_cli(&config),
            Err(NormalFileCanonicalCoreVmReferenceProfileErrorV1::NonDefaultOptimizationRequested)
        ));
    }
}
