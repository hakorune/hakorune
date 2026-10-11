//! Read-only wire projection of the signature owner's existing carrier rows.
use super::FunctionMetadata;
use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1;

impl FunctionMetadata {
    pub(crate) fn physical_param_carrier_transport_tags_v1(&self) -> Option<Vec<&'static str>> {
        self.physical_param_carriers.as_ref().map(|rows| {
            rows.iter()
                .map(|row| match row {
                    PhysicalCallableLaneCarrierV1::ExistingCallableI64 => "existing_callable_i64",
                    PhysicalCallableLaneCarrierV1::U64BitsOnI64 => "u64_bits_on_i64",
                    PhysicalCallableLaneCarrierV1::CheckedMapStorage => "checked_map_storage",
                    PhysicalCallableLaneCarrierV1::BorrowedTaggedValue => "borrowed_tagged_value",
                })
                .collect()
        })
    }
}
