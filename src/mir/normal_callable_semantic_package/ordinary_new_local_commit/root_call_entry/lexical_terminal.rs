//! Existing terminal source owns the original immutable lexical packet.
use super::*;
use crate::mir::normal_callable_semantic_package::CallPacketSourceLoanV1;
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1;

impl OrdinaryNewClaimLedgerV1 {
    /// Lend the argument authority retained by this original packet protocol.
    /// Object observations carry flow only; their sealed inputs own arguments.
    pub(in crate::mir::normal_callable_semantic_package) fn original_local_packet_arguments_v1<
        'a,
    >(
        &'a self,
        packet: &'a EmittedLexicalCallProjectionV1,
    ) -> Result<&'a [LocalCallArgumentV1], String> {
        match packet.original_source() {
            CallPacketSourceLoanV1::Instance(row)
                if self.uses_object_packet_protocol_v1(row)? =>
            {
                self.object_packet_arguments_v1(row)
            }
            CallPacketSourceLoanV1::Instance(_) => {
                let source = self
                    .local_call_for_owner(packet.call_site().owner(), packet.call_site().site())
                    .ok_or_else(|| freeze("local-packet/source-missing"))?;
                if source.site() != packet.call_site()
                    || source.owner() != packet.call_site().owner()
                {
                    return Err(freeze("local-packet/source-drift"));
                }
                Ok(source.arguments())
            }
            CallPacketSourceLoanV1::Static { observation, .. }
            | CallPacketSourceLoanV1::SelectedStaticLoop { observation, .. } => {
                if observation.site() != packet.call_site()
                    || observation.owner() != packet.call_site().owner()
                {
                    return Err(freeze("local-packet/static-source-drift"));
                }
                Ok(observation.arguments())
            }
        }
    }

    /// Lend terminal and original local-pool packet reads to the same census.
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
            let exit = exit
                .explicit_site()
                .ok_or_else(|| freeze("implicit-exit/terminal-call-drift"))?;
            if let Some(read) = packet.stored_receiver_read_v1() {
                self.validate_lexical_terminal_packet(owner, exit, packet)?;
                reads.push(read.clone());
            }
        }
        let groups = self.root_local_call_bindings.borrow();
        let mut sites = std::collections::BTreeSet::new();
        for group in groups.get(&owner).into_iter().flatten() {
            for packet in group.lexical_packets() {
                if packet.call_site().owner() != owner {
                    return Err(freeze("local-stored-receiver/source-drift"));
                }
                let arguments = self.original_local_packet_arguments_v1(packet)?;
                packet.visit_original_nodes_v1(
                    owner,
                    arguments,
                    self,
                    &mut |node, arguments| {
                        if let Some(read) = node.stored_receiver_read_v1() {
                            if !sites.insert(node.call_site().clone()) {
                                return Err(freeze("local-stored-receiver/duplicate-source"));
                            }
                            node.call_with_ledger(owner, arguments, self)?;
                            node.validate_recorded(group.bindings())?;
                            reads.push(read.clone());
                        }
                        Ok(())
                    },
                )?;
            }
        }
        Ok(reads)
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_pending_static_terminal_packet(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        pending: &PendingStaticCallProjectionV1,
    ) -> Result<(), String> {
        let source = self
            .verified_terminal_call_source_v1(owner, exit)?
            .ok_or_else(|| freeze("lexical-terminal/source-missing"))?;
        let packet = pending.packet();
        let original = packet.original();
        let observation = packet.observation();
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
        self.verify_original_static_packet_source_v1(original)?;
        self.borrowed_static_packet_actuals_v1(original)?
            .ok_or_else(|| freeze("static-packet/actuals-missing"))?;
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_lexical_terminal_packet(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        packet: &EmittedLexicalCallProjectionV1,
    ) -> Result<Vec<LocalCallArgumentV1>, String> {
        self.validate_lexical_terminal_packet_in_context(
            owner,
            exit,
            packet,
            CallValidationContextV1::Lowering(self),
        )
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_lexical_terminal_packet_in_context(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        packet: &EmittedLexicalCallProjectionV1,
        context: CallValidationContextV1<'_>,
    ) -> Result<Vec<LocalCallArgumentV1>, String> {
        context.require_ledger(self)?;
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
            packet.call_in_context(owner, arguments, context)?;
            return Ok(arguments.to_vec());
        }
        source.corroborate_row(packet.original_row()?)?;
        let source = source
            .lexical_arguments()
            .ok_or_else(|| freeze("lexical-terminal/arguments-missing"))?;
        packet.call_in_context(owner, source, context)?;
        Ok(source.to_vec())
    }
}
