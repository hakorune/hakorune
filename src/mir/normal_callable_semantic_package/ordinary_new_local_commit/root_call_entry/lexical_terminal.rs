//! Existing terminal source owns the original immutable lexical packet.
use super::*;

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_lexical_terminal_packet(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        packet: &EmittedLexicalCallProjectionV1,
    ) -> Result<(), String> {
        let (completion, terminal) = self
            .call_source_completion_for_owner_at(owner, exit)
            .ok_or_else(|| freeze("lexical-terminal/source-missing"))?;
        if completion.owner() != owner
            || terminal.return_site() != exit
            || packet.call_site().owner() != owner
            || packet.call_site().site() != terminal.call_site()
        {
            return Err(freeze("lexical-terminal/source-identity"));
        }
        let source = self
            .borrowed_terminal_arguments_v1(owner, exit)?
            .ok_or_else(|| freeze("lexical-terminal/arguments-missing"))?;
        packet.call_with_ledger(owner, &source, self)?;
        Ok(())
    }
}
