//! Root exit progress for the readable-Map terminal.
//!
//! The MapGet entry retains the already-emitted checked read exactly like a
//! Call ingress: the source relation owns the key and receiver class; this
//! file only records and re-verifies physical coordinates.
use super::*;

impl OrdinaryNewClaimLedgerV1 {
    /// Move the pending local-call binding groups into the MapGet entry and
    /// seal the emitted read, mirroring `record_root_call_exit`.
    pub(crate) fn record_root_map_get_exit(
        &self,
        owner: FunctionOwnerIdV1,
        invoke: (BasicBlockId, MirInstruction),
        projection: (BasicBlockId, MirInstruction),
        frame: (BasicBlockId, MirInstruction),
        origins: Vec<(RootHomeReleaseOriginV1, BasicBlockId, MirInstruction)>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        let mut pending = self.root_local_call_bindings.borrow_mut();
        let pending_groups = pending.get(&owner).map(Vec::as_slice).unwrap_or(&[]);
        self.validate_local_call_binding_groups(owner, pending_groups)?;
        let local_bindings = pending.remove(&owner).unwrap_or_default();
        drop(pending);
        self.record_root_home_exit_with_entry(
            owner,
            origins,
            bindings,
            RootHomeExitEntry::MapGet {
                local_bindings,
                invoke,
                projection,
                frame,
            },
        )
    }

    /// Artifact seal evidence: the owner's exit is an emitted MapGet entry.
    /// Child completeness is separately enforced by finalized child
    /// validation; this flag only answers the seal-time root question.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn terminal_map_get_return_emitted(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> bool {
        matches!(
            self.root_exits.borrow().get(&owner),
            Some(RootHomeExitProgress::Emitted {
                entry: RootHomeExitEntry::MapGet { .. },
                ..
            })
        )
    }

    /// Re-verify one emitted MapGet entry against its source relation and
    /// the finished function. Mirrors `validate_call_entry`'s Call arm:
    /// relation presence, operation shape, frame identity, binding coverage,
    /// local-call groups, and the shared cleanup ingress graph.
    pub(super) fn validate_map_get_entry(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
        local_bindings: &[(OwnedExprSiteV1, Vec<(BasicBlockId, MirInstruction)>)],
        invoke: &(BasicBlockId, MirInstruction),
        projection: &(BasicBlockId, MirInstruction),
        frame: &(BasicBlockId, MirInstruction),
        cleanup: &[(BasicBlockId, MirInstruction)],
    ) -> Result<(), String> {
        let relation = self
            .terminal_map_get_return_for_owner(owner)
            .ok_or_else(|| freeze("map-get-source-missing"))?;
        let MirInstruction::Invoke {
            operation:
                InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::CheckedGetI64 {
                    utf8,
                    ..
                }),
            fault_frame,
            ..
        } = &invoke.1
        else {
            return Err(freeze("map-get-shape"));
        };
        if utf8.as_str() != relation.key() {
            return Err(freeze("map-get-key-drift"));
        }
        // The recorded instruction pins the mode — `RootOwned` for the root,
        // `Borrowed` for a cataloged child like the readable-Map callee —
        // matching `map.rs`'s call row; `check_binding` below re-verifies it.
        if !matches!(&frame.1,
            MirInstruction::FaultFrameEnter { dst, .. } if dst == fault_frame)
        {
            return Err(freeze("map-get-frame-drift"));
        }
        for (id, instruction) in [invoke, projection, frame] {
            if !super::super::physical_boundary::check_binding(
                function,
                finishing,
                *id,
                instruction,
            )? {
                return Err(freeze("map-get-binding-drift"));
            }
        }
        self.check_local_call_binding_groups(owner, function, finishing, local_bindings)?;
        let mapped = |binding: &(BasicBlockId, MirInstruction)| match finishing {
            Some(p) => p
                .binding(binding.0, &binding.1)?
                .ok_or_else(|| freeze("map-get-binding-missing")),
            None => Ok(binding.clone()),
        };
        let cleanup = match finishing {
            Some(p) => p.bindings(cleanup)?,
            None => cleanup.to_vec(),
        };
        super::super::root_cleanup_graph::call::ingress(
            function,
            &cleanup,
            &mapped(invoke)?,
            &mapped(projection)?,
        )?;
        Ok(())
    }
}
