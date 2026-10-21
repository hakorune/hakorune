//! The existing New emission keeps exact checked read/local Copy/actual loans.
use super::*;
impl EmittedNewArgumentV1 {
    pub(super) fn validate_field_array_read(
        &self, function: &MirFunction, projection: Option<&physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        match (self.source.field_array_read(), &self.field_array_read, self.payload()) {
            (None, None, _) => Ok(()),
            (Some(source), Some(read), crate::mir::instruction::BirthArgumentV1::Value(value)) if read.matches(source, *value) => {
                read.validate(function, *value, |original| match projection {
                    Some(projection) => projection.relation_binding(function, original),
                    None => Ok(Some(original.clone())),
                })
            }
            _ => Err(freeze("argument-read-record-drift")),
        }
    }
}

#[cfg(test)]
impl FinalizedRootSourceHandoffV1 {
    pub(in crate::mir) fn validate_new_argument_reads_for_test(
        &self, owner: FunctionOwnerIdV1, function: &MirFunction,
    ) -> Result<(), String> {
        self.ledger.with_finished_projection(owner, |symbol, projection| {
            if symbol != function.signature.name { return Err(freeze("argument-read-function-drift")); }
            self.ledger.validate_new_emissions_projected(owner, function, Some(projection))
        })
    }
}
