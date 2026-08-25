use super::super::dynamic_v2_aot_metadata_slot::DynamicV2AotMetadataSlotRejectV1;
use super::FunctionMetadata;
use crate::mir::a_prime_i64_physical_receipt::{
    APrimeI64PhysicalReceiptSlotRejectV1, APrimeI64PhysicalReceiptV1,
};

impl FunctionMetadata {
    /// Borrow the candidate-only Dynamic AOT projection for JSON observation.
    pub(crate) fn dynamic_v2_aot_metadata(
        &self,
    ) -> Option<&crate::box_callable::provider_admission::DynamicV2AotCallMetadataProjectionV1>
    {
        self.dynamic_v2_aot_metadata.borrow()
    }

    pub(in crate::mir) fn install_dynamic_v2_aot_metadata(
        &mut self,
        projection: crate::box_callable::provider_admission::DynamicV2AotCallMetadataProjectionV1,
    ) -> Result<(), DynamicV2AotMetadataSlotRejectV1> {
        self.dynamic_v2_aot_metadata.install(projection)
    }

    /// Borrow the transport receipt for JSON observation only. The live
    /// physical consumer must use `take_a_prime_i64_physical_receipt`.
    pub(crate) fn a_prime_i64_physical_receipt(&self) -> Option<&APrimeI64PhysicalReceiptV1> {
        self.a_prime_i64_physical_receipt.borrow()
    }

    /// Install after the last cloneable metadata/prepared-draft snapshot.
    pub(in crate::mir) fn install_a_prime_i64_physical_receipt(
        &mut self,
        receipt: APrimeI64PhysicalReceiptV1,
    ) -> Result<(), APrimeI64PhysicalReceiptSlotRejectV1> {
        self.a_prime_i64_physical_receipt.install(receipt)
    }

    pub(in crate::mir) fn take_a_prime_i64_physical_receipt(
        &mut self,
    ) -> Result<APrimeI64PhysicalReceiptV1, APrimeI64PhysicalReceiptSlotRejectV1> {
        self.a_prime_i64_physical_receipt.take_once()
    }

    #[cfg(test)]
    pub(crate) fn install_a_prime_i64_physical_receipt_for_test(
        &mut self,
        receipt: APrimeI64PhysicalReceiptV1,
    ) -> Result<(), APrimeI64PhysicalReceiptSlotRejectV1> {
        self.a_prime_i64_physical_receipt.install_for_test(receipt)
    }
}
