//! Final-root validation is separate from local New emission state changes.

use super::*;

impl OrdinaryNewClaimLedgerV1 {
    /// Called on the exact physical root after all module finalization passes.
    /// Script-only packages never register a callable root; an empty New set
    /// does not erase a registered root's validation obligation.
    pub(crate) fn validate_finalized_new_root(
        &self,
        function: &MirFunction,
    ) -> Result<RootOrdinaryNewObservation, String> {
        let mut state = self.root_validation.borrow_mut();
        let owner = match *state {
            RootNewValidation::Unregistered => return Ok(RootOrdinaryNewObservation::NotIssued),
            RootNewValidation::Pending(owner) => owner,
            RootNewValidation::Checked(_) | RootNewValidation::FinishingChecked => {
                return Err(freeze("duplicate-root-validation"));
            }
        };
        self.validate_new_emissions(owner, function)?;
        self.validate_terminal_unit_return(owner, function)?;
        self.validate_root_home_exit(function)?;
        self.validate_field_reads(owner, function)?;
        self.validate_terminal_i64_add_return(owner, function)?;
        self.validate_terminal_integer_literal_return(owner, function)?;
        let observation = self.finalized_root_observation(owner);
        *state = RootNewValidation::Checked(owner);
        Ok(observation)
    }

    /// Recheck the same retained source obligations after compiler finishing.
    /// The early observation cannot authorize a modified function. No source
    /// products are reconstructed from the final CFG or its metadata.
    pub(crate) fn validate_after_compiler_finishing(
        &self,
        function: &MirFunction,
    ) -> Result<(), String> {
        let mut state = self.root_validation.borrow_mut();
        let owner = match *state {
            RootNewValidation::Unregistered => return Ok(()),
            RootNewValidation::Checked(owner) => owner,
            RootNewValidation::Pending(_) => return Err(freeze("root-before-draft-validation")),
            RootNewValidation::FinishingChecked => {
                return Err(freeze("duplicate-finishing-validation"));
            }
        };
        self.validate_new_emissions(owner, function)?;
        self.validate_terminal_unit_return(owner, function)?;
        self.validate_root_home_exit(function)?;
        self.validate_field_reads(owner, function)?;
        self.validate_terminal_i64_add_return(owner, function)?;
        self.validate_terminal_integer_literal_return(owner, function)?;
        if self.finalized_root_observation(owner) != function.root_ordinary_new_observation() {
            return Err(freeze("root-observation-drift"));
        }
        *state = RootNewValidation::FinishingChecked;
        Ok(())
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn validate_terminal_unit_return(&self, owner: FunctionOwnerIdV1, function: &MirFunction) -> Result<(), String> {
        let Some(relation) = self.terminal_unit_return.as_ref() else { return Ok(()); };
        if relation.owner() != owner { return Err(freeze("unit-return-owner-drift")); }
        let count = function.blocks.values().flat_map(|block| block.all_instructions())
            .filter(|instruction| matches!(instruction, MirInstruction::Return { value: None }))
            .count();
        if count != 1 { return Err(freeze("unit-return-control-drift")); }
        Ok(())
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn validate_terminal_integer_literal_return(&self, owner: FunctionOwnerIdV1, function: &MirFunction) -> Result<(), String> {
        let Some(relation) = self.terminal_integer_literal.as_ref() else { return Ok(()); };
        if relation.owner() != owner { return Err(freeze("literal-owner-drift")); }
        let Some(value) = *self.terminal_integer_literal_value.borrow() else { return Err(freeze("literal-unconsumed")); };
        let exact = function.blocks.values().flat_map(|block| block.all_instructions()).any(|instruction| matches!(instruction, MirInstruction::Const { dst, value: crate::mir::ConstValue::Integer(actual) } if *dst == value && *actual == relation.value()));
        let returned = function.blocks.values().flat_map(|block| block.all_instructions()).any(|instruction| matches!(instruction, MirInstruction::Return { value: Some(actual) } if *actual == value));
        (exact && returned).then_some(()).ok_or_else(|| freeze("literal-physical-drift"))
    }
}
