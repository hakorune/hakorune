//! Read-only source Return consult over the one original draft partition.
use super::*;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::home_new_prefix::BorrowedViewUseRequestV1;

impl PreparedBorrowedFormalIngressV1 {
    /// Select the existing verified Completion/Home walk for its exact source terminal.
    /// This never fabricates Home flow or grants executable entry to source-only rows.
    pub(in crate::mir::normal_callable_semantic_package) fn integer_return_target(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> bool {
        self.source_definition_for(owner).is_some_and(|draft| {
            draft
                .uses
                .iter()
                .any(|row| matches!(row.kind, BorrowedFormalUseDraftKindV1::IntegerReturn { .. }))
        })
    }

    pub(in crate::mir::normal_callable_semantic_package) fn consult_view_use_v1(
        &self,
        input: ResolvedFunctionLoweringInputV1<'_>,
        site: &OwnedExprSiteV1,
        request: BorrowedViewUseRequestV1<'_>,
    ) -> Result<bool, String> {
        if site.owner() != input.owner() {
            return Err(freeze("borrowed-return/source-owner"));
        }
        let Some(draft) = self.source_definition_for(input.owner()) else {
            return Ok(false);
        };
        match request {
            BorrowedViewUseRequestV1::Operand
                if draft
                    .mul_operand_at(input, site)
                    .map_err(|_| freeze("borrowed-mul/source-identity"))?
                    .is_some() =>
            {
                Ok(self.dominated_view_use_at(input.owner(), site))
            }
            BorrowedViewUseRequestV1::Operand => Ok(self
                .dominated_view_use_at(input.owner(), site)
                && draft.uses.iter().any(|row| {
                    &row.site == site
                        && matches!(
                            row.kind,
                            BorrowedFormalUseDraftKindV1::ArrayElementValue { .. }
                                | BorrowedFormalUseDraftKindV1::AddOperand { .. }
                                | BorrowedFormalUseDraftKindV1::NewArgument { .. }
                        )
                })),
            BorrowedViewUseRequestV1::IntegerReturn { exit, binding } => {
                let Some(row) = draft
                    .integer_return_at(input, site)
                    .map_err(|_| freeze("borrowed-return/source-identity"))?
                else {
                    return Ok(false);
                };
                let BorrowedFormalUseDraftKindV1::IntegerReturn { exit: original, .. } = &row.kind
                else {
                    return Err(freeze("borrowed-return/source-kind"));
                };
                if original != exit || row.binding != binding {
                    return Err(freeze("borrowed-return/exit-binding-identity"));
                }
                Ok(true)
            }
        }
    }
}
