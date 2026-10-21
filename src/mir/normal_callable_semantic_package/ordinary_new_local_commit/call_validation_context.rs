//! Borrow the current sole owner of original Call packets after affine moves.
//! This context owns no source facts, packet registry or completion seal.
use super::*;
use crate::mir::normal_callable_semantic_package::LexicalInstanceCallDispositionRowV1;

#[derive(Clone, Copy)]
pub(in crate::mir) enum CallValidationContextV1<'a> {
    Lowering(&'a OrdinaryNewClaimLedgerV1),
    Finalized(&'a FinalizedRootSourceHandoffV1),
}
impl<'a> CallValidationContextV1<'a> {
    /// Source preflight only: no physical execution or completion permission.
    pub(in crate::mir) fn validate_source_loop_bool_frame_v2(self,
        product: &std::rc::Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>, index: usize,
        frame: &std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBoolConditionFrameV1>,
    ) -> Result<(), String> {
        use crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1 as Root;
        use crate::mir::resolved_semantics::home_new_prefix::LifecyclePrefixSourceV1 as Prefix;
        let original = product.schedule().frames().get(index).ok_or_else(|| freeze("bool-condition/loop-frame-index"))?;
        if !matches!(original.prefix(), Prefix::BoolCondition { frame: source, .. } if std::rc::Rc::ptr_eq(source, frame))
            || if index == 0 { !matches!(frame.source().root(), Root::LoopCondition { .. })
                || !std::rc::Rc::ptr_eq(frame, &product.source.condition) }
                else { !matches!(frame.source().root(), Root::IfCondition { .. }) } {
            return Err(freeze("bool-condition/loop-source-purpose"));
        }
        self.require_original_source_loop_v2(product)
    }
    /// The same Loop remains in its existing affine group tree. A child may
    /// retain the immutable product, never an Rc back to its physical parent.
    pub(super) fn require_original_source_loop_v2(self,
        product: &std::rc::Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
    ) -> Result<(), String> {
        self.require_source_loop_parent_v2(product, false)
    }
    /// Borrow the immutable source coordinate through its current affine parent.
    /// Header publication may precede completion; After and all final users may not.
    pub(in crate::mir) fn source_loop_point_product_v2(self,
        source: &crate::mir::resolved_semantics::home_new_prefix::CurrentScalarProducerV1,
    ) -> Result<std::rc::Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>, String> {
        use crate::mir::resolved_semantics::home_new_prefix::CurrentScalarProducerV1 as Producer;
        let (product, after) = match source {
            Producer::LoopHeader(basis) => {
                let product = self.ledger().retained_source_loop_recipe_v2(basis.binding().owner(), basis.source().site())?
                    .ok_or_else(|| freeze("source-loop/header-product-missing"))?;
                if !std::rc::Rc::ptr_eq(&product.source.source, basis.source())
                    || product.source.headers.get(&basis.binding()).is_none_or(|original| !std::rc::Rc::ptr_eq(original, basis)) {
                    return Err(freeze("source-loop/header-basis-drift"));
                }
                (std::rc::Rc::clone(product), false)
            }
            Producer::LoopAfter { product, binding } => {
                let original = self.ledger().retained_source_loop_recipe_v2(binding.owner(), product.source.source.site())?
                    .ok_or_else(|| freeze("source-loop/after-product-missing"))?;
                if !std::rc::Rc::ptr_eq(original, product)
                    || product.source.incoming.as_ref().map_err(|_| freeze("source-loop/after-incoming"))?
                        .iter().filter(|row| row.basis.binding() == *binding).count() != 1 {
                    return Err(freeze("source-loop/after-source-drift"));
                }
                (std::rc::Rc::clone(product), true)
            }
            _ => return Err(freeze("source-loop/point-purpose")),
        };
        self.require_source_loop_parent_v2(&product, after || matches!(self, Self::Finalized(_)))?;
        Ok(product)
    }

    /// Borrow the same source-owned write after the packet pool's affine move.
    /// Membership is safe during read take; full RHS validation is a later phase.
    pub(in crate::mir::normal_callable_semantic_package) fn with_original_field_write_v1<R>(
        self, frame: &std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedFieldWriteFrameV1>,
        check: impl FnOnce(&EmittedBorrowedFieldWriteV1) -> Result<R, String>,
    ) -> Result<R, String> {
        self.with_local_call_group_tree_v2(frame.source().owner(), |groups| {
            let mut found = None;
            for group in groups {
                let Some(packet) = group.borrowed_field_write() else { continue };
                if !std::rc::Rc::ptr_eq(packet.source().frame(), frame) { continue; }
                if group.site() != &packet.source().value_site() || found.replace(packet).is_some() {
                    return Err(freeze("field-write/packet-membership"));
                }
                if matches!(self, Self::Finalized(_)) { group.require_sealed_condition_v1()?; }
            }
            check(found.ok_or_else(|| freeze("field-write/original-packet-missing"))?)
        })
    }
    pub(in crate::mir) fn with_original_rebind_v1<R>(self,
        frame: &std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1>,
        check: impl FnOnce(&EmittedBindingRebindProjectionV1) -> Result<R, String>,
    ) -> Result<R, String> {
        self.with_local_call_group_tree_v2(frame.source().owner(), |groups| {
            let mut found = None;
            for group in groups {
                let Some(packet) = group.rebind() else { continue };
                if !std::rc::Rc::ptr_eq(packet.frame(), frame) { continue; }
                if found.replace(packet).is_some() { return Err(freeze("rebind-expression/packet-duplicate")); }
            }
            check(found.ok_or_else(|| freeze("rebind-expression/packet-missing"))?)
        })
    }
    pub(in crate::mir) fn with_original_initializer_v1<R>(self,
        frame: &std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedInitializerExpressionFrameV1>,
        check: impl FnOnce(&EmittedInitializerExpressionProjectionV1) -> Result<R, String>,
    ) -> Result<R, String> {
        self.with_local_call_group_tree_v2(frame.source().owner(), |groups| {
            let mut found = None;
            for group in groups {
                let Some(packet) = group.initializer() else { continue };
                if !std::rc::Rc::ptr_eq(packet.frame(), frame) { continue; }
                if found.replace(packet).is_some() { return Err(freeze("initializer-expression/packet-duplicate")); }
            }
            check(found.ok_or_else(|| freeze("initializer-expression/packet-missing"))?)
        })
    }
    /// Resolve a Text acquisition through its original source purpose. This
    /// borrows the existing packet and lexical loan; it issues no new Home.
    pub(in crate::mir) fn with_text_residence_v1<R>(self,
        acquisition: &crate::mir::resolved_semantics::home_new_prefix::TextResidenceAcquisitionV1,
        read: &crate::mir::builder::ExactLexicalReadV1,
        check: impl FnOnce(ValueId, &(BasicBlockId, MirInstruction)) -> Result<R, String>,
    ) -> Result<R, String> {
        if let Some(frame) = acquisition.initializer_origin_v1() {
            return self.with_original_initializer_v1(frame, |packet| {
                let home = self.text_home_for_acquisition_v1(acquisition)?;
                let value = read.loan_local_initializer_v1(acquisition.binding())?.0;
                if value != packet.value() || home.local_value() != value {
                    return Err(freeze("expression/residence-original-pair"));
                }
                check(value, packet.root_acquisition_binding_v1()?)
            });
        }
        let frame = acquisition.rebind_origin_v1().ok_or_else(|| freeze("expression/residence-origin"))?;
        self.with_original_rebind_v1(frame, |packet| {
            let home = self.text_home_for_acquisition_v1(acquisition)?;
            let value = read.loan_checked_rebind_v1(frame)?.0;
            if value != packet.value() || home.local_value() != value { return Err(freeze("expression/residence-original-pair")); }
            check(value, packet.root_acquisition_binding_v1()?)
        })
    }
    pub(in crate::mir::normal_callable_semantic_package) fn text_home_for_acquisition_v1(self,
        acquisition: &crate::mir::resolved_semantics::home_new_prefix::TextResidenceAcquisitionV1,
    ) -> Result<TextResidencePacketV1, String> {
        let rows = self.ledger().local_commits.borrow();
        let home = crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1::Text(acquisition.clone());
        let LocalCommitV1::TextResidence(packet) = installed_source_home(&rows, &home)
            .map_err(|_| freeze("expression/residence-published-home"))? else { return Err(freeze("expression/residence-published-home")) };
        self.ledger().validate_text_residence_home_v1(packet, self)?;
        Ok(packet.clone())
    }

    pub(in crate::mir) fn ledger(self) -> &'a OrdinaryNewClaimLedgerV1 {
        match self { Self::Lowering(ledger) => ledger, Self::Finalized(source) => &source.ledger }
    }
    pub(in crate::mir) fn require_ledger(self, ledger: &OrdinaryNewClaimLedgerV1) -> Result<(), String> {
        if !std::ptr::eq(self.ledger(), ledger) { return Err(freeze("call-validation/foreign-ledger")); }
        Ok(())
    }
    /// Source membership follows the exact affine slot state, without retrying pools.
    pub(in crate::mir::normal_callable_semantic_package) fn with_current_instance_source_v1<R>(
        self, site: &OwnedExprSiteV1,
        check: impl FnOnce(&crate::mir::normal_callable_semantic_package::LexicalInstanceCallSourceTargetV1) -> Result<R, String>,
    ) -> Result<R, String> {
        use super::super::lexical_instance_call::LexicalInstanceCallDispositionSlotV1 as Slot;
        let rows = self.ledger().lexical_instance_calls.borrow();
        match rows.get(site) {
            Some(Slot::SourcePending(source)) => check(source),
            Some(Slot::Ready(row)) => check(row.source_target()),
            Some(Slot::Taken) => {
                drop(rows);
                self.with_taken_original_row_v1(site, |row| check(row.source_target()))
            },
            None => Err(freeze("call-validation/original-instance-source-missing")),
        }
    }
    /// Borrow groups from their current affine owner; never retry another pool.
    pub(super) fn with_local_call_groups_v1<R>(
        self, owner: FunctionOwnerIdV1,
        read: impl FnOnce(&[RootLifecyclePrefixBindingGroupV1]) -> Result<R, String>,
    ) -> Result<R, String> {
        let roots = match self {
            Self::Lowering(ledger) => ledger.root_local_call_bindings.borrow().get(&owner).cloned().unwrap_or_default(),
            Self::Finalized(source) => source.local_calls.get(&owner).cloned().unwrap_or_default(),
        };
        read(&roots)
    }

    /// Packet lookup traverses nested groups. Instruction-owner censuses keep
    /// using the root groups, whose completed aggregate owns each binding once.
    pub(super) fn with_local_call_group_tree_v2<R>(self, owner: FunctionOwnerIdV1,
        read: impl FnOnce(&[RootLifecyclePrefixBindingGroupV1]) -> Result<R, String>,
    ) -> Result<R, String> {
        self.with_local_call_groups_v1(owner, |roots| {
            let groups = RootLifecyclePrefixBindingGroupV1::snapshot_tree_v2(roots);
            if matches!(self, Self::Finalized(_)) {
                for group in &groups {
                    if group.source_loop_rc().is_some() { group.require_sealed_condition_v1()?; }
                }
            }
            read(&groups)
        })
    }

    /// Exit bindings move to the finalized root handoff exactly once.
    pub(super) fn visit_exit_bindings_v1(
        self, owner: FunctionOwnerIdV1, mut visit: impl FnMut(&[(BasicBlockId, MirInstruction)]),
    ) -> Result<(), String> {
        for ((exit_owner, site), progress) in self.ledger().root_exits.borrow().iter() {
            if *exit_owner != owner { continue; }
            match progress {
                RootHomeExitProgress::Emitted { bindings, .. } => visit(bindings),
                RootHomeExitProgress::Finalized(_) => {
                    if let Self::Finalized(source) = self {
                        let exit = site.explicit_site().ok_or_else(|| freeze("call-validation/finalized-exit-site"))?;
                        if source.owner != owner { return Err(freeze("call-validation/finalized-exit-owner")); }
                        let (_, bindings) = source.call_entries.get(exit)
                            .ok_or_else(|| freeze("call-validation/finalized-exit-missing"))?;
                        visit(bindings);
                    }
                }
                _ => {},
            }
        }
        Ok(())
    }

    pub(in crate::mir) fn with_taken_original_row_v1<R>(
        self, site: &OwnedExprSiteV1,
        check: impl FnOnce(&LexicalInstanceCallDispositionRowV1) -> Result<R, String>,
    ) -> Result<R, String> {
        self.with_taken_packet_v1(site, |packet| check(packet.original_row()?))
    }

    /// Borrow the payload's SAME producer Normal and original LocalSSA chain.
    /// The lifetime holder never supplies the payload value; no Home is issued.
    pub(in crate::mir) fn require_borrowed_map_receiver_v1(
        self, row: &LexicalInstanceCallDispositionRowV1,
        read: &crate::mir::builder::ExactLexicalReadV1,
    ) -> Result<(), String> {
        let Some(loan) = row.source_target().borrowed_receiver_source_v1() else { return Ok(()); };
        self.ledger().corroborate_unit_discard_v1(row)?;
        let source = self.ledger().borrowed_nullable_call_source_v1(loan.call())
            .ok_or_else(|| freeze("borrowed-return/original-loan"))?;
        if source.borrowed_nullable() != Some(loan)
            || source.local_binding().map(|(_, binding)| binding) != Some(loan.destination()) {
            return Err(freeze("borrowed-return/original-contract"));
        }
        read.value_for(row.call_site().owner(), row.receiver_site().node(), loan.destination())
            .map_err(|_| freeze("borrowed-return/original-contract"))?;
        let (initializer, _) = read.loan_local_initializer_v1(loan.destination())?;
        self.with_taken_packet_v1(loan.call(), |packet| {
            let original = packet.original_row()?;
            let result = crate::mir::instruction::InvokeCallResultKind::BorrowedNullableHandle { support: loan.result_support_v1() };
            if original.source_target().borrowed_nullable_local_source() != Some(loan)
                || self.ledger().corroborate_borrowed_nullable_call_v1(original)?
                    != result
                || packet.value_in_context_v1(source.owner(), source.arguments(), self)? != initializer {
                return Err(freeze("borrowed-return/original-contract"));
            }
            Ok(())
        })
    }

    pub(in crate::mir) fn original_scalar_call_result_v1(self,
        source: &crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1,
    ) -> Result<ValueId, String> {
        if self.ledger().lexical_i64_call_source(source.site()) != Some(source) {
            return Err(freeze("scalar-call/original-source"));
        }
        self.with_local_call_group_tree_v2(source.owner(), |groups| {
            let mut matching = groups.iter().filter(|group| group.site() == source.site());
            let group = matching.next().ok_or_else(|| freeze("scalar-call/group-missing"))?;
            if matching.next().is_some() { return Err(freeze("scalar-call/group-duplicate")); }
            group.scalar_call_value_v1(source, self)
        })
    }

    pub(in crate::mir) fn with_taken_packet_v1<R>(
        self, site: &OwnedExprSiteV1,
        check: impl FnOnce(&EmittedLexicalCallProjectionV1) -> Result<R, String>,
    ) -> Result<R, String> {
        // Select the original source container before consulting any physical
        // state. A missing New packet must never retry the local-group pool.
        let rows = self.ledger().local_commits.borrow();
        let mut selected = None;
        for (new_site, row) in rows.iter().filter(|(new_site, _)| new_site.owner() == site.owner()) {
            let Some(Ok(arguments)) = row.new_argument_rows() else { continue };
            for argument in arguments.iter() {
                let super::super::OrdinaryNewTrivialArgumentKindV1::Expression { evaluation } = argument.kind() else { continue };
                if evaluation.source().contains_call_site_v1(site)
                    && selected.replace((new_site, row, argument)).is_some() {
                    return Err(freeze("call-validation/duplicate-source-node"));
                }
            }
        }
        if let Some((new_site, row, source)) = selected {
            if source.new_site() != new_site || source.owner() != site.owner() {
                return Err(freeze("argument-source-drift"));
            }
            let argument = match row.new_emission() {
                Some(NewEmissionProgress::Emitted { arguments, .. }) => arguments.get(source.ordinal() as usize)
                    .filter(|argument| argument.source() == source),
                _ => None,
            }.ok_or_else(|| freeze("argument-source-unavailable"))?;
            let expression = argument.expression_v1().ok_or_else(|| freeze("argument-source-drift"))?;
            expression.require_source_v1(source, argument.payload())?;
            return expression.with_call_node_v1(site, check);
        }
        drop(rows);
        self.with_local_call_group_tree_v2(site.owner(), |groups| {
            let mut found = None;
            for group in groups {
                for packet in group.lexical_packets() {
                    if let Some(packet) = packet.retained_node_at_v1(site)? {
                        if found.replace(packet).is_some() { return Err(freeze("mixed-result/borrowed-consumed-packet-duplicate")); }
                    }
                }
            }
            check(found.ok_or_else(|| freeze("mixed-result/borrowed-consumed-packet-missing"))?)
        })
    }
}
