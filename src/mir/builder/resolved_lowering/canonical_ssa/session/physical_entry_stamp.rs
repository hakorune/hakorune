//! Session-owned physical-entry cohort stamp transport.

use super::CanonicalSsaFunctionSessionV2;
use crate::mir::compiler::common_v2_physical_function_skeleton::PhysicalFunctionEntryCohortStampV1;

impl CanonicalSsaFunctionSessionV2<'_> {
    pub(in crate::mir::builder::resolved_lowering) fn attach_physical_entry_stamp(
        &mut self,
        stamp: PhysicalFunctionEntryCohortStampV1,
    ) -> Result<(), String> {
        if self.physical_entry_stamp.is_some() {
            return Err("canonical session already owns a physical entry stamp".to_owned());
        }
        self.physical_entry_stamp = Some(stamp);
        Ok(())
    }

    pub(in crate::mir::builder::resolved_lowering) fn physical_entry_stamp(
        &self,
    ) -> Result<&PhysicalFunctionEntryCohortStampV1, String> {
        self.physical_entry_stamp
            .as_ref()
            .ok_or_else(|| "canonical session has no physical entry stamp".to_owned())
    }
}
