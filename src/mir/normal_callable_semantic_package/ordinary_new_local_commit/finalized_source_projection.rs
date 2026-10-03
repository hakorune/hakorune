//! Read-only original source and finished-coordinate loans from the same owner.
use super::super::lexical_instance_call::PreparedBorrowedFormalActualV1;
use super::*;
use crate::mir::normal_callable_semantic_package::LexicalInstanceCallDispositionRowV1;

type Binding = (BasicBlockId, MirInstruction);

impl FinalizedRootSourceHandoffV1 {
    /// Only an exact producer from an original retained node can use this map.
    /// Cleanup/frame membership and a matching finished instruction are insufficient.
    pub(in crate::mir) fn finished_local_call_producer_v1(
        &self,
        owner: FunctionOwnerIdV1,
        group_site: &OwnedExprSiteV1,
        node_site: &OwnedExprSiteV1,
        original: &Binding,
        function: &MirFunction,
    ) -> Result<(BasicBlockId, usize), String> {
        if group_site.owner() != owner || node_site.owner() != owner {
            return Err(freeze("finished-local-call/owner"));
        }
        let mut groups = self
            .local_calls
            .get(&owner)
            .into_iter()
            .flatten()
            .filter(|group| group.site() == group_site);
        let group = groups
            .next()
            .ok_or_else(|| freeze("finished-local-call/group-missing"))?;
        if groups.next().is_some() {
            return Err(freeze("finished-local-call/group-duplicate"));
        }
        let packet = group
            .lexical()
            .ok_or_else(|| freeze("finished-local-call/packet-missing"))?;
        if !packet.has_producer_at(node_site, original) {
            return Err(freeze("finished-local-call/original-producer"));
        }
        let (symbol, finished) = self.ledger.finished_binding_for_owner(owner, original)?;
        find_finished_producer(&symbol, &finished, function)
    }

    /// Read the original Return packet without copying an affine row or issuing source.
    pub(in crate::mir) fn with_terminal_call_packet_v1<T>(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        read: impl FnOnce(&EmittedLexicalCallProjectionV1) -> Result<T, String>,
    ) -> Result<T, String> {
        let extract = |entry: &RootHomeExitEntry| match entry {
            RootHomeExitEntry::Call {
                row: super::super::RootCallDispositionV1::Lexical(packet),
                ..
            } => Ok(Rc::clone(packet)),
            _ => Err(freeze("finished-terminal-call/packet-missing")),
        };
        let packet = if owner == self.owner() {
            extract(
                &self
                    .call_entries
                    .get(exit)
                    .ok_or_else(|| freeze("finished-terminal-call/exit-missing"))?
                    .0,
            )?
        } else {
            let exits = self.ledger.root_exits.borrow();
            match exits.get(&(owner, exit.clone())) {
                Some(RootHomeExitProgress::Emitted { entry, .. }) => extract(entry)?,
                _ => return Err(freeze("finished-terminal-call/child-exit-missing")),
            }
        };
        self.ledger
            .validate_lexical_terminal_packet(owner, exit, &packet)?;
        read(&packet)
    }

    pub(in crate::mir) fn finished_terminal_call_producer_v1(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        node_site: &OwnedExprSiteV1,
        original: &Binding,
        function: &MirFunction,
    ) -> Result<(BasicBlockId, usize), String> {
        if node_site.owner() != owner {
            return Err(freeze("finished-terminal-call/owner"));
        }
        self.with_terminal_call_packet_v1(owner, exit, |packet| {
            if !packet.has_producer_at(node_site, original) {
                return Err(freeze("finished-terminal-call/original-producer"));
            }
            let (symbol, finished) = self.ledger.finished_binding_for_owner(owner, original)?;
            find_finished_producer(&symbol, &finished, function)
        })
    }

    /// Copy dependencies belong to the original call node, not its emitted argument group.
    pub(in crate::mir) fn finished_local_call_copy_v1(
        &self,
        owner: FunctionOwnerIdV1,
        group_site: &OwnedExprSiteV1,
        node_site: &OwnedExprSiteV1,
        original: &Binding,
        function: &MirFunction,
    ) -> Result<(BasicBlockId, usize), String> {
        if group_site.owner() != owner || node_site.owner() != owner {
            return Err(freeze("finished-copy/owner"));
        }
        let mut groups = self
            .local_calls
            .get(&owner)
            .into_iter()
            .flatten()
            .filter(|group| group.site() == group_site);
        let group = groups.next().ok_or_else(|| freeze("finished-copy/group"))?;
        if groups.next().is_some() {
            return Err(freeze("finished-copy/group-duplicate"));
        }
        let packet = group
            .lexical()
            .ok_or_else(|| freeze("finished-copy/packet"))?;
        self.finished_packet_copy(owner, packet, node_site, original, function)
    }

    pub(in crate::mir) fn finished_terminal_call_copy_v1(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        node_site: &OwnedExprSiteV1,
        original: &Binding,
        function: &MirFunction,
    ) -> Result<(BasicBlockId, usize), String> {
        if node_site.owner() != owner {
            return Err(freeze("finished-copy/owner"));
        }
        self.with_terminal_call_packet_v1(owner, exit, |packet| {
            self.finished_packet_copy(owner, packet, node_site, original, function)
        })
    }

    fn finished_packet_copy(
        &self,
        owner: FunctionOwnerIdV1,
        packet: &EmittedLexicalCallProjectionV1,
        node_site: &OwnedExprSiteV1,
        original: &Binding,
        function: &MirFunction,
    ) -> Result<(BasicBlockId, usize), String> {
        if !packet
            .copy_dependencies(&self.ledger)?
            .iter()
            .any(|(node, copy)| node == node_site && copy == original)
        {
            return Err(freeze("finished-copy/original-membership"));
        }
        let (symbol, finished) = self.ledger.finished_binding_for_owner(owner, original)?;
        if !matches!(finished.1, MirInstruction::Copy { .. }) {
            return Err(freeze("finished-copy/instruction"));
        }
        let coordinate = find_finished_producer(&symbol, &finished, function)?;
        let dst = finished.1.dst_value().expect("checked Copy");
        if function.params.contains(&dst) {
            return Err(freeze("finished-copy/parameter-collision"));
        }
        if function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .filter(|instruction| instruction.dst_value() == Some(dst))
            .count()
            != 1
        {
            return Err(freeze("finished-copy/destination-duplicate"));
        }
        Ok(coordinate)
    }

    pub(in crate::mir) fn borrowed_call_actuals_v1(
        &self,
        row: &LexicalInstanceCallDispositionRowV1,
    ) -> Result<Option<&[PreparedBorrowedFormalActualV1]>, String> {
        self.ledger.borrowed_call_actuals_v1(row)
    }

    pub(in crate::mir) fn borrowed_ordinary_entry_source_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<super::super::lexical_instance_call::BorrowedOrdinaryEntrySourceRefV1<'_>, String>
    {
        self.ledger
            .finalized_borrowed_ordinary_entry_source_v1(owner)
    }

    pub(in crate::mir) fn borrowed_ordinary_entry_source_for_function_v1(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<super::super::lexical_instance_call::BorrowedOrdinaryEntrySourceRefV1<'_>, String>
    {
        let source = self.borrowed_ordinary_entry_source_v1(owner)?;
        self.ledger.with_finished_projection(owner, |symbol, _| {
            if function.signature.name != symbol {
                return Err(freeze("borrowed-entry/finished-function"));
            }
            Ok(())
        })?;
        Ok(source)
    }

    /// Same original alias materialization, demanded against the finished owner.
    /// Boundary mapping and residual-use checking remain consumer obligations.
    pub(in crate::mir) fn with_borrowed_ordinary_alias_copies_v1(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        visit: impl FnMut(
            &OwnedExprSiteV1,
            &SourceBindingSiteV1,
            BindingRefV1,
            BindingRefV1,
            ValueId,
            &[(BasicBlockId, MirInstruction)],
        ) -> Result<(), String>,
    ) -> Result<(), String> {
        self.borrowed_ordinary_entry_source_for_function_v1(owner, function)?;
        self.ledger
            .with_borrowed_ordinary_alias_copies_v1(owner, visit)
    }

    /// Original alias tuple -> the same held Boundary -> exact final coordinate.
    /// None is allowed only by the original optional Copy cone, never a scan.
    pub(in crate::mir) fn borrowed_ordinary_alias_copy_coordinate_v1(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        original: &(BasicBlockId, MirInstruction),
    ) -> Result<Option<(BasicBlockId, usize)>, String> {
        self.borrowed_ordinary_entry_source_for_function_v1(owner, function)?;
        self.ledger.with_finished_projection(owner, |_, projection| {
            projection.borrowed_copy_coordinate(function, original)
        })
    }

    pub(in crate::mir) fn borrowed_ordinary_entry_values_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<Box<[(u32, BindingRefV1, ValueId)]>, String> {
        self.ledger.borrowed_ordinary_entry_values_v1(owner)
    }

    /// Same finalized entry proof, projecting only the admitted checked-
    /// compare operand uses: `(binding, formal, binary site)` triples.
    /// The finished-function check runs through the same owner/function
    /// binding as the alias copy projection.
    pub(in crate::mir) fn borrowed_ordinary_compare_uses_v1(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<Box<[(BindingRefV1, BindingRefV1, OwnedExprSiteV1)]>, String> {
        let source = self.borrowed_ordinary_entry_source_for_function_v1(owner, function)?;
        Ok(source
            .compare_uses()
            .map(|(binding, formal, binary)| (binding, formal, binary.clone()))
            .collect())
    }

    /// Same finalized entry proof, projecting only the admitted dominated
    /// `+` operand uses: `(binding, formal, binary site)` triples. The
    /// finished-function check runs through the same owner/function binding.
    pub(in crate::mir) fn borrowed_ordinary_add_uses_v1(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<Box<[(BindingRefV1, BindingRefV1, OwnedExprSiteV1)]>, String> {
        let source = self.borrowed_ordinary_entry_source_for_function_v1(owner, function)?;
        Ok(source
            .add_uses()
            .map(|(binding, formal, binary)| (binding, formal, binary.clone()))
            .collect())
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn finished_binding_for_owner(
        &self,
        owner: FunctionOwnerIdV1,
        original: &Binding,
    ) -> Result<(String, Binding), String> {
        self.with_finished_projection(owner, |symbol, projection| {
            project_recorded(symbol, projection, original)
        })
    }

    fn with_finished_projection<T>(
        &self,
        owner: FunctionOwnerIdV1,
        read: impl FnOnce(&str, &physical_boundary::FinishedBindings) -> Result<T, String>,
    ) -> Result<T, String> {
        let root = self.root_validation.borrow();
        if let RootNewValidation::ArtifactFinalized {
            owner: root_owner,
            symbol,
            projection,
        } = &*root
        {
            if *root_owner == owner {
                return read(symbol, projection);
            }
        } else {
            return Err(freeze("finished-local-call/root-not-finalized"));
        }
        let children = self.child_physical_validation.borrow();
        match children.get(&owner) {
            Some(ChildPhysicalValidation::FinishingChecked { symbol, projection }) => {
                read(symbol, projection)
            }
            _ => Err(freeze("finished-local-call/child-not-finished")),
        }
    }
}

fn project_recorded(
    symbol: &str,
    projection: &physical_boundary::FinishedBindings,
    original: &Binding,
) -> Result<(String, Binding), String> {
    let finished = projection
        .binding(original.0, &original.1)?
        .ok_or_else(|| freeze("finished-local-call/producer-removed"))?;
    if !projection.recorded().contains(&finished) {
        return Err(freeze("finished-local-call/producer-unrecorded"));
    }
    Ok((symbol.to_owned(), finished))
}

fn find_finished_producer(
    symbol: &str,
    finished: &Binding,
    function: &MirFunction,
) -> Result<(BasicBlockId, usize), String> {
    if function.signature.name != symbol {
        return Err(freeze("finished-local-call/function"));
    }
    let block = function
        .blocks
        .get(&finished.0)
        .ok_or_else(|| freeze("finished-local-call/block-missing"))?;
    let mut matches = block
        .all_instructions()
        .enumerate()
        .filter(|(_, instruction)| *instruction == &finished.1);
    let (index, _) = matches
        .next()
        .ok_or_else(|| freeze("finished-local-call/producer-missing"))?;
    if matches.next().is_some() {
        return Err(freeze("finished-local-call/producer-duplicate"));
    }
    Ok((finished.0, index))
}
