//! Prepared process projection while retaining the physical source carrier.

use super::source_entry_physical::PhysicalSourceEntryCarrierV1;
use super::source_entry_result::{
    ProcessExitProfileV1, ProcessExitProjectionErrorV1, ProcessExitProjectionV1,
    ProcessTerminationV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum SourceEntryProjectionStageV1 {
    Profile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum SourceEntryProjectionErrorV1 {
    LegacyProfileDisconnected,
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedSourceEntryProjectionV1 {
    carrier: PhysicalSourceEntryCarrierV1,
    stage: SourceEntryProjectionStageV1,
    error: SourceEntryProjectionErrorV1,
}

#[derive(Debug)]
pub(in crate::mir) struct PreparedSourceEntryProjectionV1 {
    carrier: PhysicalSourceEntryCarrierV1,
    termination: ProcessTerminationV1,
}

#[derive(Debug)]
pub(in crate::mir) struct ProjectedSourceEntryV1 {
    carrier: PhysicalSourceEntryCarrierV1,
    termination: ProcessTerminationV1,
    _seal: ProjectedSourceEntrySealV1,
}

#[derive(Debug)]
struct ProjectedSourceEntrySealV1;

impl PhysicalSourceEntryCarrierV1 {
    pub(in crate::mir) fn prepare_process_projection(
        self,
        profile: ProcessExitProfileV1,
    ) -> Result<PreparedSourceEntryProjectionV1, RejectedSourceEntryProjectionV1> {
        let termination = match ProcessExitProjectionV1::project_borrowed(self.result(), profile)
        {
            Ok(termination) => termination,
            Err(ProcessExitProjectionErrorV1::LegacyProfileDisconnected) => {
                return Err(RejectedSourceEntryProjectionV1 {
                    carrier: self,
                    stage: SourceEntryProjectionStageV1::Profile,
                    error: SourceEntryProjectionErrorV1::LegacyProfileDisconnected,
                });
            }
        };
        Ok(PreparedSourceEntryProjectionV1 {
            carrier: self,
            termination,
        })
    }
}

impl PreparedSourceEntryProjectionV1 {
    pub(in crate::mir) fn project(self) -> ProjectedSourceEntryV1 {
        ProjectedSourceEntryV1 {
            carrier: self.carrier,
            termination: self.termination,
            _seal: ProjectedSourceEntrySealV1,
        }
    }
}

impl RejectedSourceEntryProjectionV1 {
    pub(in crate::mir) const fn stage(&self) -> SourceEntryProjectionStageV1 {
        self.stage
    }

    pub(in crate::mir) const fn error(&self) -> SourceEntryProjectionErrorV1 {
        self.error
    }

    pub(in crate::mir) fn carrier(&self) -> &PhysicalSourceEntryCarrierV1 {
        &self.carrier
    }

    pub(in crate::mir) fn discard(self) {}
}

impl ProjectedSourceEntryV1 {
    pub(in crate::mir) fn termination(&self) -> &ProcessTerminationV1 {
        &self.termination
    }

    pub(in crate::mir) fn carrier(&self) -> &PhysicalSourceEntryCarrierV1 {
        &self.carrier
    }
}

#[cfg(test)]
mod tests {
    use super::super::raw_root_environment_manifest::RawRootEnvironmentManifestV1;
    use super::super::raw_root_source_facts::RawRootSourceRouteV1;
    use super::super::source_entry_physical::PhysicalEntryRoleV1;
    use super::super::source_entry_result::{
        CanonicalProcessExitV1, ProcessExitCodeV1, ProcessExitProfileV1, ProcessFaultV1,
        ProcessTerminationV1, SealedSourceFaultV1, SourceEntryResultV1, UnitOriginV1,
    };
    use super::super::source_entry_selection::select_source_entry;
    use super::*;

    fn carrier(route: RawRootSourceRouteV1, result: SourceEntryResultV1) -> PhysicalSourceEntryCarrierV1 {
        select_source_entry(RawRootEnvironmentManifestV1::from_test(route))
            .begin_thunk()
            .complete(result)
            .into_physical()
    }

    fn canonical() -> ProcessExitProfileV1 {
        ProcessExitProfileV1::Canonical(CanonicalProcessExitV1::V1)
    }

    #[test]
    fn projection_retains_script_carrier_and_unit_termination() {
        let projected = carrier(
            RawRootSourceRouteV1::Script,
            SourceEntryResultV1::Unit(UnitOriginV1::EmptyBody),
        )
        .prepare_process_projection(canonical())
        .expect("canonical profile")
        .project();
        assert_eq!(projected.carrier().role(), PhysicalEntryRoleV1::SourceResultThunk);
        assert_eq!(projected.carrier().route(), super::super::source_entry_selection::SelectedSourceEntryRouteV1::Script);
        assert_eq!(
            projected.termination(),
            &ProcessTerminationV1::Exit(ProcessExitCodeV1::zero())
        );
    }

    #[test]
    fn projection_retains_app_carrier_and_fault_termination() {
        let projected = carrier(
            RawRootSourceRouteV1::App,
            SourceEntryResultV1::Fault(SealedSourceFaultV1::new("fault", "detail".into())),
        )
        .prepare_process_projection(canonical())
        .expect("canonical profile")
        .project();
        assert_eq!(projected.carrier().route(), super::super::source_entry_selection::SelectedSourceEntryRouteV1::AppMain0);
        assert!(matches!(
            projected.termination(),
            ProcessTerminationV1::Fault(ProcessFaultV1::SourceFault { .. })
        ));
    }

    #[test]
    fn legacy_profile_rejects_with_exact_carrier() {
        let mut rejected = carrier(
            RawRootSourceRouteV1::Script,
            SourceEntryResultV1::Integer(7),
        )
        .prepare_process_projection(ProcessExitProfileV1::LegacyRunnerExitProjectionV1)
        .expect_err("legacy profile is disconnected");
        assert_eq!(rejected.stage(), SourceEntryProjectionStageV1::Profile);
        assert_eq!(
            rejected.error(),
            SourceEntryProjectionErrorV1::LegacyProfileDisconnected
        );
        assert_eq!(rejected.carrier().route(), super::super::source_entry_selection::SelectedSourceEntryRouteV1::Script);
        rejected.discard();
    }
}
