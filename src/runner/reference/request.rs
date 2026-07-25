//! One typed selector for explicit reference runner requests.

use super::normal_file_vm_request::{
    NormalFileVmReferenceProductionRequestV1, NormalFileVmReferenceProfileErrorV1,
};
use super::raw_vm_reference_request::{
    RawVmReferenceProductionRequestV1, RawVmReferenceProfileErrorV1,
};
use crate::cli::CliConfig;

#[derive(Debug)]
pub(crate) enum ExplicitReferenceRunnerRequestV1 {
    RawVmReference(RawVmReferenceProductionRequestV1),
    NormalFileVmReference(NormalFileVmReferenceProductionRequestV1),
}

#[derive(Debug)]
pub(crate) enum ExplicitReferenceRunnerSelectionV1 {
    NotSelected,
    Selected(ExplicitReferenceRunnerRequestV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExplicitReferenceRunnerSelectionErrorV1 {
    RawVmReference(RawVmReferenceProfileErrorV1),
    NormalFileVmReference(NormalFileVmReferenceProfileErrorV1),
}

impl ExplicitReferenceRunnerSelectionErrorV1 {
    pub(crate) const fn code(&self) -> &'static str {
        match self {
            Self::RawVmReference(error) => error.code(),
            Self::NormalFileVmReference(error) => error.code(),
        }
    }
}

/// Select the explicit reference route exactly once.  A non-reference backend
/// falls through unchanged and no route reads a source file at this boundary.
pub(crate) fn select_from_cli(
    config: &CliConfig,
) -> Result<ExplicitReferenceRunnerSelectionV1, ExplicitReferenceRunnerSelectionErrorV1> {
    match config.backend.as_str() {
        backend if backend == RawVmReferenceProductionRequestV1::backend_name() => {
            RawVmReferenceProductionRequestV1::try_from_selected_cli(config)
                .map(ExplicitReferenceRunnerRequestV1::RawVmReference)
                .map(ExplicitReferenceRunnerSelectionV1::Selected)
                .map_err(ExplicitReferenceRunnerSelectionErrorV1::RawVmReference)
        }
        backend if backend == NormalFileVmReferenceProductionRequestV1::backend_name() => {
            NormalFileVmReferenceProductionRequestV1::try_from_selected_cli(config)
                .map(ExplicitReferenceRunnerRequestV1::NormalFileVmReference)
                .map(ExplicitReferenceRunnerSelectionV1::Selected)
                .map_err(ExplicitReferenceRunnerSelectionErrorV1::NormalFileVmReference)
        }
        _ => Ok(ExplicitReferenceRunnerSelectionV1::NotSelected),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_only_explicit_reference_backends() {
        let mut config = CliConfig::default();
        config.backend = NormalFileVmReferenceProductionRequestV1::backend_name().to_owned();
        config.file = Some("normal.hako".to_owned());
        assert!(matches!(
            select_from_cli(&config),
            Ok(ExplicitReferenceRunnerSelectionV1::Selected(
                ExplicitReferenceRunnerRequestV1::NormalFileVmReference(_)
            ))
        ));

        config.backend = "mir".to_owned();
        assert!(matches!(
            select_from_cli(&config),
            Ok(ExplicitReferenceRunnerSelectionV1::NotSelected)
        ));
    }

    #[test]
    fn preserves_raw_no_optimize_but_rejects_it_for_normal() {
        let mut config = CliConfig::default();
        config.file = Some("profile.hako".to_owned());
        config.no_optimize = true;

        config.backend = RawVmReferenceProductionRequestV1::backend_name().to_owned();
        assert!(matches!(
            select_from_cli(&config),
            Ok(ExplicitReferenceRunnerSelectionV1::Selected(
                ExplicitReferenceRunnerRequestV1::RawVmReference(_)
            ))
        ));

        config.backend = NormalFileVmReferenceProductionRequestV1::backend_name().to_owned();
        assert!(matches!(
            select_from_cli(&config),
            Err(ExplicitReferenceRunnerSelectionErrorV1::NormalFileVmReference(
                NormalFileVmReferenceProfileErrorV1::NonDefaultOptimizationRequested
            ))
        ));
    }
}
