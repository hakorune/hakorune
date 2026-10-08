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
    Legacy {
        terminal: &'a TerminalI64CallReturnV1,
        arguments: Option<Box<[LocalCallArgumentV1]>>,
    },
}
impl VerifiedTerminalCallSourceV1<'_> {
    pub(crate) fn call_site(&self) -> &SourceExprSiteV1 {
        match self {
            Self::Direct(view) => view.target().call_site().site(),
            Self::Legacy { terminal, .. } => terminal.call_site(),
        }
    }
    pub(crate) fn result(&self) -> InvokeCallResultKind {
        match self {
            Self::Direct(view) => view.result(),
            Self::Legacy { .. } => InvokeCallResultKind::I64,
        }
    }
    pub(crate) fn lexical_arguments(&self) -> Option<&[LocalCallArgumentV1]> {
        match self {
            Self::Direct(view) => Some(view.arguments()),
            Self::Legacy { arguments, .. } => arguments.as_deref(),
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn legacy_terminal(
        &self,
    ) -> Option<&TerminalI64CallReturnV1> {
        match self {
            Self::Legacy { terminal, .. } => Some(terminal),
            Self::Direct(_) => None,
        }
    }
    pub(crate) fn corroborate_row(
        &self,
        row: &LexicalInstanceCallDispositionRowV1,
    ) -> Result<(), String> {
        if row.call_site().site() != self.call_site() || row.result() != Some(self.result()) {
            return Err(freeze("row-identity"));
        }
        if let Self::Direct(view) = self {
            if row.source_target() != view.target() {
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
