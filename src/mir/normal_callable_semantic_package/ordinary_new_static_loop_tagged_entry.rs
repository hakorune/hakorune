//! Source authority for the selected Static Loop's checked tagged formal.
//! It issues no physical carrier, entry value, actual, or call packet.

use std::rc::Rc;

use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_parameter_contract::{
    CallableParameterContractKindV1, CallableParameterDeclarationModeV1,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingRefV1, ResolvedLexicalRefV1, SourceStmtSiteV1,
};

use super::super::model::OwnedCallableParameterContractDeclarationV1;
use super::super::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
use super::lexical_instance_call::PreparedBorrowedFormalIngressV1;
use super::loop_static_source_loan::LoopEntryStaticI64SourceLoanV1;

#[derive(Debug)]
pub(in crate::mir) struct VerifiedStaticLoopTaggedEntrySourceV1 {
    loop_site: SourceStmtSiteV1,
    formal: BindingRefV1,
    original: Rc<StaticIncomingSourceV1>,
}

impl VerifiedStaticLoopTaggedEntrySourceV1 {
    pub(in crate::mir) fn formal(&self) -> BindingRefV1 {
        self.formal
    }

    pub(in crate::mir) fn corroborates(&self, entry: &LoopEntryStaticI64SourceLoanV1) -> bool {
        self.loop_site == *entry.loop_site() && Rc::ptr_eq(&self.original, entry.original())
    }
}

pub(super) fn issue_static_loop_tagged_entry_source_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    caller: &CanonicalSameModuleCallableKeyV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    incoming: &PreparedBorrowedFormalIngressV1,
    entry: &LoopEntryStaticI64SourceLoanV1,
) -> Result<VerifiedStaticLoopTaggedEntrySourceV1, String> {
    let reject = || "[freeze:contract][callable-loop/tagged-entry-source-unavailable]".to_owned();
    let mut rows = contracts.iter().filter(|row| row.owner == input.owner());
    let contract = rows.next().ok_or_else(reject)?;
    if rows.next().is_some()
        || contract.mode != CallableParameterDeclarationModeV1::StaticBoxMethod
        || caller.arity() != 1
        || contract.parameters.len() != 1
        || entry.original().caller() != caller
        || entry.original().argument_sites().len() != 1
    {
        return Err(reject());
    }
    let formal = &contract.parameters[0];
    let argument = &entry.original().argument_sites()[0];
    if formal.ordinal != 0
        || formal.kind != CallableParameterContractKindV1::OpaqueHandle
        || !incoming.checked_static_input(formal.binding)
        || input.function().variable_ref(argument)
            != Some(ResolvedLexicalRefV1::Local(formal.binding))
        || !matches!(
            input
                .function()
                .binding(formal.binding)
                .map(|row| row.kind()),
            Some(BindingKindV1::Parameter { index: 0 })
        )
    {
        return Err(reject());
    }
    Ok(VerifiedStaticLoopTaggedEntrySourceV1 {
        loop_site: entry.loop_site().clone(),
        formal: formal.binding,
        original: Rc::clone(entry.original()),
    })
}
