//! Existing terminal source owns the original immutable lexical packet.
use super::*;
use crate::mir::normal_callable_semantic_package::CallPacketSourceLoanV1;
use crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1;

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
    ) -> Result<Vec<LocalCallArgumentV1>, String> {
        let source = self
            .verified_terminal_call_source_v1(owner, exit)?
            .ok_or_else(|| freeze("lexical-terminal/source-missing"))?;
        if packet.call_site().owner() != owner || packet.call_site().site() != source.call_site() {
            return Err(freeze("lexical-terminal/source-identity"));
        }
        if let CallPacketSourceLoanV1::Static {
            original,
            observation,
            ..
        } = packet.original_source()
        {
            let terminal = source
                .legacy_terminal()
                .ok_or_else(|| freeze("static-terminal/legacy-source-missing"))?;
            let selected = self
                .selected_static_local_source_v1(original.call_site())?
                .ok_or_else(|| freeze("static-terminal/original-source-missing"))?;
            if !std::rc::Rc::ptr_eq(&selected, original)
                || terminal.owner() != owner
                || terminal.return_site() != exit
                || terminal.call_site() != original.call_site().site()
                || !terminal.arguments().is_empty()
                || source.result() != crate::mir::instruction::InvokeCallResultKind::I64
                || observation.owner() != owner
                || observation.site() != original.call_site()
            {
                return Err(freeze("static-terminal/packet-source-drift"));
            }
            packet.original_source().validate_static(self)?;
            let arguments = observation.arguments();
            packet.call_with_ledger(owner, arguments, self)?;
            return Ok(arguments.to_vec());
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
        Ok(source.to_vec())
    }
}
