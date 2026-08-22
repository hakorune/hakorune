//! Co-sealed lifecycle observation for selected Dynamic function metadata.

use super::metadata::FunctionMetadata;
use crate::box_callable::provider_admission::DynamicV2AotCallMetadataProjectionV1;
use crate::mir::a_prime_i64_physical_receipt::APrimeI64PhysicalReceiptV1;
use crate::mir::linear_metadata_slot::LinearSlotObservation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DynamicV2MetadataPairObservation<'a> {
    Ordinary,
    Selected {
        receipt: &'a APrimeI64PhysicalReceiptV1,
        admission: &'a DynamicV2AotCallMetadataProjectionV1,
    },
    Scrubbed,
    Partial,
}

impl FunctionMetadata {
    /// Observe the two clone-scrubbing slots as one pair.  Empty ordinary
    /// metadata and a clone-scrubbed candidate are intentionally distinct.
    pub(crate) fn selected_dynamic_metadata_observation(
        &self,
    ) -> DynamicV2MetadataPairObservation<'_> {
        match (
            self.a_prime_i64_physical_receipt.observe(),
            self.dynamic_v2_aot_metadata.observe(),
        ) {
            (LinearSlotObservation::Empty, LinearSlotObservation::Empty) => {
                DynamicV2MetadataPairObservation::Ordinary
            }
            (
                LinearSlotObservation::Occupied(receipt),
                LinearSlotObservation::Occupied(admission),
            ) => DynamicV2MetadataPairObservation::Selected { receipt, admission },
            (LinearSlotObservation::Scrubbed, _) | (_, LinearSlotObservation::Scrubbed) => {
                DynamicV2MetadataPairObservation::Scrubbed
            }
            _ => DynamicV2MetadataPairObservation::Partial,
        }
    }
}

#[cfg(test)]
impl FunctionMetadata {
    pub(crate) fn install_dynamic_v2_aot_metadata_for_test(
        &mut self,
        projection: DynamicV2AotCallMetadataProjectionV1,
    ) -> Result<(), super::dynamic_v2_aot_metadata_slot::DynamicV2AotMetadataSlotRejectV1> {
        self.install_dynamic_v2_aot_metadata(projection)
    }
}
