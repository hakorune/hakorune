//! Scoped accessors for one installed selected-callable input.

use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{BindingRefV1, VerifiedResolvedBlockExpressionExpectationV1};
use crate::parser::CallableMethodSourceObservationV1;

impl<'loan> super::SelectedCallableLoweringInputRefV1<'loan> {
    pub(crate) fn source(&self) -> ResolvedFunctionLoweringInputV1<'loan> {
        self.source
    }

    pub(crate) fn parameter_contracts(
        &self,
    ) -> impl ExactSizeIterator<Item = (u32, BindingRefV1, CallableParameterContractKindV1)> + '_
    {
        self.parameter_contracts
            .iter()
            .map(|row| (row.ordinal, row.binding, row.kind))
    }

    /// Borrow the resolver-owned BlockExpr expectation from the same batch
    /// row. This is transport only: no count is recomputed and the receipt is
    /// neither cloned nor reissued by the installed package.
    pub(crate) fn block_expr_expectation(&self) -> &VerifiedResolvedBlockExpressionExpectationV1 {
        self.block_expr_expectation
    }

    pub(crate) fn physical_header(&self) -> Option<super::CallablePhysicalHeaderRefV1<'_>> {
        self.physical_header
    }

    pub(in crate::mir) fn semantic(&self) -> super::SelectedCallableSemanticRefV1<'loan> {
        self.semantic
    }

    pub(crate) fn method_source_observation(&self) -> Option<&CallableMethodSourceObservationV1> {
        self.source_identity.method_source_observation()
    }

    pub(crate) fn source_identity(
        &self,
    ) -> &crate::mir::callable_semantic_batch::VerifiedResolvedCallableSourceIdentityV1 {
        &self.source_identity
    }

    pub(crate) fn selected_key(&self) -> &crate::mir::builder::SelectedNormalCallableKeyV1 {
        &self.selected_key
    }
}
