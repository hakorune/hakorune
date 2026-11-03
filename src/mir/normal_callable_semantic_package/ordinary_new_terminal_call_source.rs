//! One source loan for the existing lexical terminal selector and validators.
//! Received acquisitions remain value returns; this lends no physical permission.
use super::super::{
    lexical_instance_call::LexicalInstanceCallDispositionRowV1, OrdinaryNewClaimLedgerV1,
};
use super::direct_return_source::VerifiedDirectObjectReturnSourceV1;
use crate::mir::instruction::InvokeCallResultKind;
use crate::mir::resolved_semantics::home_new_prefix::{
    LocalCallArgumentV1, TerminalI64CallReturnV1,
};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceExprSiteV1, SourceStmtSiteV1};

pub(crate) enum VerifiedTerminalCallSourceV1<'a> {
    Direct(VerifiedDirectObjectReturnSourceV1<'a>),
    Borrowed {
        source: &'a crate::mir::normal_callable_semantic_package::BorrowedCallSourceLoanV1,
        target: &'a super::super::lexical_instance_call::LexicalInstanceCallSourceTargetV1,
        arguments: &'a [LocalCallArgumentV1],
    },
    Legacy {
        terminal: &'a TerminalI64CallReturnV1,
        arguments: Option<Box<[LocalCallArgumentV1]>>,
    },
}
impl VerifiedTerminalCallSourceV1<'_> {
    pub(crate) fn call_site(&self) -> &SourceExprSiteV1 {
        match self {
            Self::Direct(view) => view.target().call_site().site(),
            Self::Borrowed { source, .. } => source.call().site(),
            Self::Legacy { terminal, .. } => terminal.call_site(),
        }
    }
    pub(crate) fn result(&self) -> InvokeCallResultKind {
        match self {
            Self::Direct(view) => view.result(),
            Self::Borrowed { source, .. } => source.result_kind_v1(),
            Self::Legacy { terminal, .. } => match terminal.scalar_kind() {
                crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer => InvokeCallResultKind::I64,
                crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Bool => InvokeCallResultKind::Bool,
            },
        }
    }
    pub(crate) fn lexical_arguments(&self) -> Option<&[LocalCallArgumentV1]> {
        match self {
            Self::Direct(view) => Some(view.arguments()),
            Self::Borrowed { arguments, .. } => Some(arguments),
            Self::Legacy { arguments, .. } => arguments.as_deref(),
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn legacy_terminal(
        &self,
    ) -> Option<&TerminalI64CallReturnV1> {
        match self {
            Self::Legacy { terminal, .. } => Some(terminal),
            Self::Direct(_) | Self::Borrowed { .. } => None,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn borrowed_source(&self)
        -> Option<&crate::mir::normal_callable_semantic_package::BorrowedCallSourceLoanV1> {
        match self { Self::Borrowed { source, .. } => Some(source), _ => None }
    }
    pub(crate) fn corroborate_row(
        &self,
        row: &LexicalInstanceCallDispositionRowV1,
    ) -> Result<(), String> {
        if row.call_site().site() != self.call_site() || row.result() != Some(self.result()) {
            return Err(freeze("row-identity"));
        }
        if let Self::Legacy { terminal, .. } = self {
            let loan = terminal.source_scalar();
            if loan != row.scalar_result_source_v1()
                || loan.is_some_and(|loan| loan.source() != row.source_target())
            {
                return Err(freeze("scalar-source-identity"));
            }
        }
        if let Self::Direct(view) = self {
            if row.source_target() != view.target() {
                return Err(freeze("direct-target-identity"));
            }
        }
        if let Self::Borrowed { source, target, .. } = self {
            if row.source_target() != *target || target.borrowed_nullable_call_source() != Some(source) {
                return Err(freeze("direct-target-identity"));
            }
        }
        Ok(())
    }
}
impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn verified_terminal_call_source_v1(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
    ) -> Result<Option<VerifiedTerminalCallSourceV1<'_>>, String> {
        if let Some(view) = self.verified_direct_object_return_source_v1(owner, exit)? {
            return Ok(Some(VerifiedTerminalCallSourceV1::Direct(view)));
        }
        if let Some(crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::Value(value)) =
            self.terminal_relation_for_owner_at(owner, exit) {
            if let crate::mir::resolved_semantics::home_new_prefix::TerminalReturnedSourceV1::BorrowedNullableLocal(returned) = value.returned() {
                if let Some(source) = returned.direct_v1() {
                    let checked = self.checked_borrowed_nullable_return_source_v1(owner, exit)?
                        .ok_or_else(|| freeze("direct-target-identity"))?;
                    let observed = self.borrowed_nullable_call_source_v1(source.call())
                        .ok_or_else(|| freeze("direct-target-identity"))?;
                    let target = self.borrowed_formal_source.as_ref().ok_or_else(|| freeze("direct-target-identity"))?
                        .as_ref().map_err(Clone::clone)?.object_source_target_at_v1(source.call())?
                        .ok_or_else(|| freeze("direct-target-identity"))?;
                    if checked != &**source || !source.is_declared_read_v1()
                        || target.borrowed_nullable_call_source() != Some(source)
                        || observed.borrowed_nullable_call() != Some(source) || !observed.is_expression_value() {
                        return Err(freeze("direct-target-identity"));
                    }
                    return Ok(Some(VerifiedTerminalCallSourceV1::Borrowed { source, target, arguments: observed.arguments() }));
                }
            }
        }
        let Some((completion, terminal)) = self.call_source_completion_for_owner_at(owner, exit)
        else {
            return Ok(None);
        };
        // Keep the original lexical return-site/argument failure ordering.
        let arguments = self.borrowed_terminal_arguments_v1(owner, exit)?;
        if completion.owner() != owner
            || terminal.owner() != owner
            || terminal.return_site() != exit
        {
            return Err(freeze("legacy-source-identity"));
        }
        Ok(Some(VerifiedTerminalCallSourceV1::Legacy {
            terminal,
            arguments,
        }))
    }
}
fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/terminal-call-source/{reason}]")
}

#[cfg(test)]
#[path = "ordinary_new_terminal_call_source_tests.rs"]
mod tests;
