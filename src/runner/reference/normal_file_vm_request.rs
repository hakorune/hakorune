//! Typed request boundary for the default-off NormalFileNoImportVmReferenceV1 lane.

use super::cli_profile::{seal_reference_cli_profile, ReferenceCliProfileErrorV1};
use super::normal_file_vm_frontdoor::{NormalFileRequestV1, NormalFileVmFrontDoorV1};
use crate::cli::CliConfig;
use std::path::PathBuf;

const NORMAL_FILE_VM_REFERENCE_BACKEND: &str = "normal-file-vm-reference";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NormalFileVmReferenceProfileErrorV1 {
    Common(ReferenceCliProfileErrorV1),
    NonDefaultOptimizationRequested,
}

impl NormalFileVmReferenceProfileErrorV1 {
    pub(crate) const fn code(&self) -> &'static str {
        match self {
            Self::Common(error) => error.code(),
            Self::NonDefaultOptimizationRequested => "non-default-optimization-requested",
        }
    }
}

#[derive(Debug)]
pub(crate) struct NormalFileVmReferenceProductionRequestV1 {
    request: NormalFileRequestV1,
    _seal: NormalFileVmReferenceProductionRequestSealV1,
}

#[derive(Debug)]
struct NormalFileVmReferenceProductionRequestSealV1;

impl NormalFileVmReferenceProductionRequestV1 {
    pub(super) const fn backend_name() -> &'static str {
        NORMAL_FILE_VM_REFERENCE_BACKEND
    }

    /// Seal a selected route without reading the source file or rebuilding a
    /// downstream Raw profile.
    pub(super) fn try_from_selected_cli(
        config: &CliConfig,
    ) -> Result<Self, NormalFileVmReferenceProfileErrorV1> {
        let source_file = seal_reference_cli_profile(config)
            .map_err(NormalFileVmReferenceProfileErrorV1::Common)?
            .into_source_file();
        if config.no_optimize {
            return Err(NormalFileVmReferenceProfileErrorV1::NonDefaultOptimizationRequested);
        }
        Ok(Self {
            request: NormalFileVmFrontDoorV1::file_no_import_request(PathBuf::from(
                String::from(source_file),
            )),
            _seal: NormalFileVmReferenceProductionRequestSealV1,
        })
    }

    pub(super) fn into_frontdoor_request(self) -> NormalFileRequestV1 {
        self.request
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn selected_config() -> CliConfig {
        let mut config = CliConfig::default();
        config.backend = NormalFileVmReferenceProductionRequestV1::backend_name().to_owned();
        config.file = Some("normal.hako".to_owned());
        config
    }

    #[test]
    fn rejects_non_default_optimization_before_front_door_io() {
        let mut config = selected_config();
        config.no_optimize = true;
        assert!(matches!(
            NormalFileVmReferenceProductionRequestV1::try_from_selected_cli(&config),
            Err(NormalFileVmReferenceProfileErrorV1::NonDefaultOptimizationRequested)
        ));
    }
}
