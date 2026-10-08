//! Existing terminal source owns the original immutable lexical packet.
use super::*;

impl OrdinaryNewClaimLedgerV1 {
    /// Lend exact packet-owned reads to the independent whole-function census.
    pub(in crate::mir::normal_callable_semantic_package) fn stored_terminal_receiver_reads_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        let exits = self.root_exits.borrow();
        let mut reads = Vec::new();
        for ((actual_owner, exit), progress) in exits.iter() {
            if *actual_owner != owner {
                continue;
            }
            let RootHomeExitProgress::Emitted {
                entry:
                    RootHomeExitEntry::Call {
                        row: RootCallDispositionV1::Lexical(packet),
                        ..
                    },
                ..
            } = progress
            else {
                continue;
            };
            if let Some(read) = packet.stored_receiver_read_v1() {
                self.validate_lexical_terminal_packet(owner, exit, packet)?;
                reads.push(read.clone());
            }
        }
        Ok(reads)
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_lexical_terminal_packet(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        packet: &EmittedLexicalCallProjectionV1,
    ) -> Result<(), String> {
        let source = self
            .verified_terminal_call_source_v1(owner, exit)?
            .ok_or_else(|| freeze("lexical-terminal/source-missing"))?;
        if packet.call_site().owner() != owner || packet.call_site().site() != source.call_site() {
            return Err(freeze("lexical-terminal/source-identity"));
        }
        if source.legacy_terminal().is_none() {
            source.corroborate_row(packet.original_row()?)?;
        } else if packet.original_source().result() != Some(source.result()) {
            return Err(freeze("lexical-terminal/result-kind"));
        }
        let source = source
            .lexical_arguments()
            .ok_or_else(|| freeze("lexical-terminal/arguments-missing"))?;
        packet.call_with_ledger(owner, source, self)?;
        Ok(())
    }
}
