use super::*;
use crate::mir::MirFunction;

mod a_prime_i64_entry;
mod block;
mod diagnostics;
mod exact_numeric_ops;
mod exact_numeric_value_checker;
mod frame_transaction;
#[cfg(test)]
mod frame_transaction_tests;
mod function_loop;
mod local_contracts;
mod numeric_contracts;
#[cfg(test)]
mod ownership_forwarding_tests;
mod parameter_contracts;
mod phi;
mod record_contracts;
mod return_contracts;
mod trace;
mod typed_array_contracts;
#[cfg(test)]
mod weak_field_contracts;

pub(crate) use block::BlockOutcome;

impl MirInterpreter {
    #[inline]
    fn trace_enabled(&self) -> bool {
        self.vm_trace_enabled
    }

    pub(super) fn exec_function_inner(
        &mut self,
        function: &MirFunction,
        arguments: Option<&[VMValue]>,
    ) -> Result<VMValue, VMError> {
        frame_transaction::with_function_frame(self, function, arguments, |interpreter| {
            interpreter.execute_installed_function(function)
        })
    }

    // SSA-RC-A1b is a disconnected semantic-oracle entry until SSA-I1.
    #[allow(dead_code)]
    pub(crate) fn exec_function_inner_with_verified_ownership(
        &mut self,
        function: &MirFunction,
        arguments: Vec<VMValue>,
        expected_owner: crate::mir::ownership_ssa::OwnershipFunctionOwnerV1,
        witness: crate::mir::ownership_ssa::VerifiedOwnershipSsaV1,
    ) -> Result<VMValue, VMError> {
        frame_transaction::with_verified_ownership_function_frame(
            self,
            function,
            arguments,
            expected_owner,
            witness,
            |interpreter| interpreter.execute_installed_function(function),
        )
    }
}
