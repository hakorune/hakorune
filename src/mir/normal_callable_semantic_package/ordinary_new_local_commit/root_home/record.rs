//! Record explicit and implicit exits into the same affine progress.
use super::*;
impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn record_root_home_exit_with_entry(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
        origins: Vec<(RootHomeReleaseOriginV1, BasicBlockId, MirInstruction)>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        entry: RootHomeExitEntry,
    ) -> Result<(), String> {
        self.record_root_home_exit_for_key_v1(owner, &site.clone().into(), origins, bindings, entry)
    }

    pub(super) fn record_root_home_exit_for_key_v1(
        &self, owner: FunctionOwnerIdV1, exit: &RootHomeExitKeyV1,
        origins: Vec<(RootHomeReleaseOriginV1, BasicBlockId, MirInstruction)>,
        bindings: Vec<(BasicBlockId, MirInstruction)>, entry: RootHomeExitEntry,
    ) -> Result<(), String> {
        let mut exits = self.root_exits.borrow_mut();
        let progress = exits
            .get_mut(&(owner, exit.clone()))
            .ok_or_else(|| freeze("root-exit-record-without-prepare"))?;
        if !matches!(*progress, RootHomeExitProgress::Emitting(_)) || bindings.is_empty() {
            return Err(freeze("root-exit-record-without-emission"));
        }
        let RootHomeExitProgress::Emitting(order) = &*progress else {
            unreachable!()
        };
        if order.normal()
            != origins
                .iter()
                .map(|(origin, _, _)| origin.clone())
                .collect::<Vec<_>>()
        {
            return Err(freeze("root-exit-origin-order"));
        }
        if let Some(site) = exit.explicit_site() {
        order.validate_direct_entry(self, owner, site, &entry, &bindings, None)?;
        if self.terminal_relation_index.get(&owner).and_then(|rows| rows.get(site)).is_some_and(|row| {
            matches!(row, TerminalRelationV1::Value(value) if matches!(value.returned(), TerminalReturnedSourceV1::NullLiteral))
        }) && order.null_return_binding().is_none() {
            return Err(freeze("terminal-null/producer-missing"));
        }
        order.validate_null_return(&entry, &bindings, None, None)?;
        self.validate_received_return_producer_v1(owner, site, &entry, &bindings, None, None)?;
        self.validate_fresh_return_producer_v1(owner, site, &entry, &bindings, None, None, None)?;
        } else {
            self.validate_implicit_root_entry_v1(owner, order, &entry, &bindings, None, None)?;
        }
        let RootHomeExitProgress::Emitting(order) =
            std::mem::replace(progress, RootHomeExitProgress::Unprepared)
        else {
            unreachable!()
        };
        let origins = origins
            .into_iter()
            .map(|(origin, block, instruction)| RootHomeReleaseEmissionV1 {
                origin,
                block,
                instruction,
            })
            .collect();
        if let Some(site) = exit.explicit_site() {
        if matches!(self.terminal_relation_for_owner_at(owner, site),
            Some(TerminalRelationV1::IntegerLiteral(_) | TerminalRelationV1::BoolLiteral(_))) {
            self.complete_terminal_literal_return_v1(owner, site, &bindings)?;
        }
        }
        *progress = RootHomeExitProgress::Emitted {
            order,
            origins,
            bindings,
            entry,
        };
        Ok(())
    }

}
