//! Read-only source Return consult over the one original draft partition.
use super::*;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::home_new_prefix::BorrowedViewUseRequestV1;

impl PreparedBorrowedFormalIngressV1 {
    /// Preserve only the canonical terminal matching the original result-owner loan.
    /// Generic Handle returns retain their existing child selection boundary.
    pub(in crate::mir::normal_callable_semantic_package) fn retains_borrowed_formal_terminal_v1(
        &self, terminal: &crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1,
    ) -> bool {
        use crate::mir::resolved_semantics::home_new_prefix::{TerminalRelationV1, TerminalReturnedSourceV1};
        let TerminalRelationV1::Value(value) = terminal else { return false; };
        let TerminalReturnedSourceV1::Handle(root) = value.returned() else { return false; };
        self.source_definition_for(value.owner()).is_some_and(|draft| draft.uses.iter().any(|row|
            row.formal == *root && draft.origins.get(&row.binding) == Some(root)
                && matches!(&row.kind, BorrowedFormalUseDraftKindV1::BorrowedFormalReturn { exit, source }
                    if exit == value.return_site() && source.formal() == *root
                        && source.value() == &row.site && row.site.owner() == value.owner()
                        && row.site.site() == value.value_site())))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn borrowed_formal_return_target_v1(
        &self, owner: FunctionOwnerIdV1,
    ) -> bool {
        self.source_definition_for(owner).is_some_and(|draft| draft.uses.iter().any(|row|
            matches!(row.kind, BorrowedFormalUseDraftKindV1::BorrowedFormalReturn { .. })))
    }
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

    /// Coordinates select the existing walk; the actual consult validates the
    /// original product. A malformed selected row must not disappear as false.
    pub(in crate::mir::normal_callable_semantic_package) fn integer_mul_return_target(
        &self,
        input: ResolvedFunctionLoweringInputV1<'_>,
    ) -> bool {
        let Some(sites) = input
            .body_shape()
            .and_then(|shape| super::super::super::verified_value_return_sites(input, shape))
        else {
            return false;
        };
        self.source_definition_for(input.owner())
            .is_some_and(|draft| {
                draft.uses.iter().any(|row| {
                    matches!(&row.kind,
                BorrowedFormalUseDraftKindV1::MulOperand { binary, .. }
                if binary.owner() == input.owner() && sites.contains(binary.site()))
                })
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
            BorrowedViewUseRequestV1::ArrayElementFieldIndexValue { call, index, binding, root } => {
                Ok(draft.uses.iter().any(|row| &row.site == site
                    && draft.origins.get(&row.binding) == Some(&row.formal)
                    && draft.origins.get(&binding) == Some(&root)
                    && matches!(&row.kind, BorrowedFormalUseDraftKindV1::ArrayElementFieldIndexValue { call: original, index: required, reach, .. }
                        if original.owner() == input.owner() && original.site() == call
                            && required.matches_source(input, index, binding, root)
                            && reach.operand() == (row.binding, row.formal, site) && reach.corroborates(input)
                            && input.function().method_call(call).is_some_and(|call|
                                call.selector() == "set" && call.arity() == 2
                                    && call.arguments().first().is_some_and(|arg| arg.ordinal() == 0 && arg.site() == index)
                                    && call.arguments().get(1).is_some_and(|arg| arg.ordinal() == 1 && arg.site() == site.site())))))
            }
            BorrowedViewUseRequestV1::FormalFieldWriteReceiver { target, binding, root } => {
                Ok(draft.uses.iter().any(|row| {
                    &row.site == site && row.binding == binding && row.formal == root
                        && draft.origins.get(&binding) == Some(&root)
                        && matches!(&row.kind, BorrowedFormalUseDraftKindV1::FieldWriteReceiver { source, .. }
                            if source.write().owner() == input.owner()
                                && source.write().assignment().target_site() == target
                                && source.write().receiver_site() == site.site()
                                && input.body_shape().is_some_and(|shape|
                                    shape.assignment_sources().contains(source.write().assignment())))
                }))
            }
            BorrowedViewUseRequestV1::FormalFieldWriteValue { target } => {
                Ok(draft.uses.iter().any(|row| {
                    &row.site == site && draft.origins.get(&row.binding) == Some(&row.formal)
                        && matches!(&row.kind, BorrowedFormalUseDraftKindV1::FieldWriteValue { source, reach }
                            if source.write().owner() == input.owner()
                                && source.write().assignment().target_site() == target
                                && source.write().assignment().value_site() == site.site()
                                && input.body_shape().is_some_and(|shape|
                                    shape.assignment_sources().contains(source.write().assignment()))
                                && reach.operand() == (row.binding, row.formal, site)
                                && reach.corroborates(input))
                }))
            }
            // The package consult owns retained field-array result facts.
            BorrowedViewUseRequestV1::IntegerFieldArrayReturn { .. }
            | BorrowedViewUseRequestV1::IntegerNestedFieldReturn { .. } => Ok(false),
            BorrowedViewUseRequestV1::CheckedAddOperand { binary } => Ok(self
                .dominated_view_use_at(input.owner(), site)
                && draft.uses.iter().any(|row| {
                    &row.site == site
                        && matches!(&row.kind,
                                BorrowedFormalUseDraftKindV1::AddOperand { binary: retained }
                                if retained == binary)
                })),
            BorrowedViewUseRequestV1::CheckedCompareOperand { binary } => {
                Ok(draft.uses.iter().any(|row| {
                    &row.site == site
                        && matches!(&row.kind,
                        BorrowedFormalUseDraftKindV1::CompareOperand {
                            binary: retained,
                            source,
                        } if retained == binary && {
                            let (_, left, right, _) = source.comparison_parts();
                            left == site || right == site
                        })
                }))
            }
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
            BorrowedViewUseRequestV1::IntegerMulReturn { exit } => {
                use crate::mir::resolved_semantics::{SourcePathSegmentV1, SourcePathV1};
                let value = SourcePathV1::from_node(exit.node())
                    .child(SourcePathSegmentV1::Value)
                    .expr();
                if &value != site.site()
                    || !input
                        .body_shape()
                        .and_then(|shape| {
                            super::super::super::verified_value_return_sites(input, shape)
                        })
                        .is_some_and(|sites| sites.contains(site.site()))
                {
                    return Err(freeze("borrowed-mul-return/exit-value-identity"));
                }
                draft
                    .mul_source_at(input, site)
                    .map(|source| source.is_some())
                    .map_err(|_| freeze("borrowed-mul-return/source-identity"))
            }
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
