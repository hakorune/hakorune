//! Selected loop-body scalar source joined to the existing physical signature.
//! The detached packet retains the canonical read and selected scalar actual;
//! publication and executable ABI remain separate boundaries.

use std::rc::Rc;

use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalBindingReadReceiptV1;
use crate::mir::callable_parameter_contract::{
    CallableParameterContractKindV1, CallableParameterDeclarationModeV1,
};
use crate::mir::definitions::MirCall;
use crate::mir::function::MirFunction;
use crate::mir::normal_callable_semantic_package::physical_signature::{
    PhysicalCallableLaneRoleV1, VerifiedCallablePhysicalSignatureCohortV1,
};
use crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1;
use crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind;
use crate::mir::resolved_semantics::{
    BindingRefV1, OwnedExprSiteV1, ResolvedLoopPlacementV1, SourceExprSiteV1, SourceStmtSiteV1,
};

use super::super::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
use super::lexical_instance_call::{
    project_current_owner_loop_scalar_source_v1, BorrowedFormalActualSourceV1,
    PreparedBorrowedFormalActualV1,
};
use super::loop_static_source_loan::LoopStaticSourceCallLoanV1;
use super::static_loop_packet_source::{
    corroborate_unpublished_physical_shape, unique_unpublished_invoke_coordinate,
};
use super::OrdinaryNewClaimLedgerV1;

#[derive(Debug)]
pub(in crate::mir) struct VerifiedLoopStaticBodyScalarSourceV1 {
    original: Rc<StaticIncomingSourceV1>,
    complete_incoming: Box<[Rc<StaticIncomingSourceV1>]>,
    binding: BindingRefV1,
    site: SourceExprSiteV1,
    formal: BindingRefV1,
}

/// Unpublished body packet: one original source cohort and one canonical
/// physical read/Invoke. No Home local call, finished coordinate or ABI grant.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedLoopStaticBodyPrepacketV1 {
    source: VerifiedLoopStaticBodyScalarSourceV1,
    read: CanonicalBindingReadReceiptV1,
    entry: crate::mir::BasicBlockId,
    normal: crate::mir::BasicBlockId,
    result: crate::mir::ValueId,
}

/// One selected body call on DraftSeal's verified detached function. It is
/// not a collector row or a published executable carrier.
#[derive(Debug)]
pub(in crate::mir) struct PreparedLoopStaticBodyDetachedPacketV1 {
    prepacket: VerifiedLoopStaticBodyPrepacketV1,
    actual: PreparedBorrowedFormalActualV1,
    coordinate: (crate::mir::BasicBlockId, usize),
    original_invoke: (crate::mir::BasicBlockId, crate::mir::MirInstruction),
    lifecycle_bindings: Vec<(crate::mir::BasicBlockId, crate::mir::MirInstruction)>,
    header_normal_phis: Option<(
        crate::mir::BasicBlockId,
        crate::mir::BasicBlockId,
        Box<[crate::mir::MirInstruction]>,
    )>,
}

/// A preflighted owner/site slot held across the collector's fallible checks.
pub(crate) struct PreparedLoopStaticBodyRetentionV1<'ledger> {
    ledger: &'ledger OrdinaryNewClaimLedgerV1,
    site: OwnedExprSiteV1,
    packet: PreparedLoopStaticBodyDetachedPacketV1,
}

impl PreparedLoopStaticBodyRetentionV1<'_> {
    #[cfg(test)]
    pub(in crate::mir) fn packet_for_test(&self) -> &PreparedLoopStaticBodyDetachedPacketV1 {
        &self.packet
    }

    pub(crate) fn commit_after_collected(self) {
        self.ledger
            .selected_loop_body_packets
            .borrow_mut()
            .insert(self.site, self.packet);
    }
}

impl OrdinaryNewClaimLedgerV1 {
    #[cfg(test)]
    pub(in crate::mir) fn selected_loop_body_packet_count_for_test(&self) -> usize {
        self.selected_loop_body_packets.borrow().len()
    }

    #[cfg(test)]
    pub(in crate::mir) fn with_selected_loop_body_packet_for_test<R>(
        &self,
        visit: impl FnOnce(&OwnedExprSiteV1, &PreparedLoopStaticBodyDetachedPacketV1) -> R,
    ) -> R {
        let packets = self.selected_loop_body_packets.borrow();
        assert_eq!(packets.len(), 1, "one selected LoopBody packet");
        let (site, packet) = packets.iter().next().unwrap();
        visit(site, packet)
    }

    pub(crate) fn prepare_selected_loop_body_retention(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        packet: PreparedLoopStaticBodyDetachedPacketV1,
    ) -> Result<PreparedLoopStaticBodyRetentionV1<'_>, String> {
        let site = packet.source_site().clone();
        if site.owner() != owner || self.selected_loop_body_packets.borrow().contains_key(&site) {
            return Err("[freeze:contract][callable-loop/body-retention-site-drift]".into());
        }
        Ok(PreparedLoopStaticBodyRetentionV1 {
            ledger: self,
            site,
            packet,
        })
    }
}

impl PreparedLoopStaticBodyDetachedPacketV1 {
    pub(in crate::mir) fn corroborates_retained_scalar(&self) -> bool {
        self.corroborates_selected_source(self.source_site(), self.prepacket.read.binding())
    }

    pub(in crate::mir) fn source_site(&self) -> &OwnedExprSiteV1 {
        self.prepacket.source.original.call_site()
    }

    pub(in crate::mir) fn original_invoke_binding(
        &self,
    ) -> &(crate::mir::BasicBlockId, crate::mir::MirInstruction) {
        &self.original_invoke
    }

    pub(in crate::mir) fn lifecycle_bindings(
        &self,
    ) -> &[(crate::mir::BasicBlockId, crate::mir::MirInstruction)] {
        &self.lifecycle_bindings
    }

    /// Lent only after the original publication has checked this Normal landing.
    pub(in crate::mir) fn header_normal_phi_role(
        &self,
    ) -> Option<(
        crate::mir::BasicBlockId,
        crate::mir::BasicBlockId,
        &[crate::mir::MirInstruction],
    )> {
        self.header_normal_phis
            .as_ref()
            .map(|(block, predecessor, phis)| (*block, *predecessor, phis.as_ref()))
    }

    pub(in crate::mir) fn corroborates_publication_batch(
        &self,
        brand: &crate::mir::builder::SameModuleCallableCatalogBrandV1,
        caller: &crate::mir::builder::CanonicalSameModuleCallableKeyV1,
        body: &(
            SourceExprSiteV1,
            crate::mir::builder::CanonicalSameModuleCallableKeyV1,
        ),
    ) -> bool {
        let original = &self.prepacket.source.original;
        original.catalog_brand().is_same(brand)
            && original.caller() == caller
            && original.call_site().site() == &body.0
            && original.target() == &body.1
    }

    /// Check the original publication batch and this same detached draft;
    /// callers cannot append arbitrary instructions as ownership evidence.
    pub(in crate::mir) fn retain_header_bindings(
        &mut self,
        publication: &crate::mir::builder::resolved_lowering::SelectedStaticLoopPublicationBatchV1,
        function: &MirFunction,
        header: crate::mir::BasicBlockId,
        header_normal: crate::mir::BasicBlockId,
        tail: crate::mir::BasicBlockId,
        tail_normal: crate::mir::BasicBlockId,
    ) -> Result<(), String> {
        self.prepacket.corroborate_unpublished_function(function)?;
        let bindings = publication.corroborate_detached_zeroarg_invokes(
            self,
            function,
            header,
            header_normal,
            tail,
            tail_normal,
        )?;
        if self.lifecycle_bindings.len() != 3
            || bindings
                .iter()
                .any(|binding| self.lifecycle_bindings.contains(binding))
        {
            return Err("[freeze:contract][callable-loop/header-retention-drift]".into());
        }
        let block = function
            .blocks
            .get(&header_normal)
            .ok_or_else(|| "[freeze:contract][callable-loop/header-normal-missing]".to_owned())?;
        let predecessors: Vec<_> = function
            .blocks
            .values()
            .flat_map(|block| {
                block
                    .out_edges()
                    .into_iter()
                    .map(move |edge| (block.id, edge.target))
            })
            .filter_map(|(from, to)| (to == header_normal).then_some(from))
            .collect();
        if predecessors.as_slice() != [header] {
            return Err("[freeze:contract][callable-loop/header-normal-predecessor-drift]".into());
        }
        let phis: Vec<_> = block
            .instructions
            .iter()
            .take_while(|row| matches!(row, crate::mir::MirInstruction::Phi { .. }))
            .cloned()
            .collect();
        if phis.iter().any(|row| {
            !matches!(row,
            crate::mir::MirInstruction::Phi { inputs, .. }
                if inputs.len() == 1 && inputs[0].0 == header)
        }) {
            return Err("[freeze:contract][callable-loop/header-normal-phi-drift]".into());
        }
        self.header_normal_phis = Some((header_normal, header, phis.into_boxed_slice()));
        self.lifecycle_bindings.extend(bindings);
        Ok(())
    }

    pub(in crate::mir) fn corroborates_selected_source(
        &self,
        site: &OwnedExprSiteV1,
        binding: BindingRefV1,
    ) -> bool {
        let source = &self.prepacket.source;
        source.original.call_site() == site
            && self.coordinate.0 == self.prepacket.entry
            && self.actual.ordinal == 0
            && self.actual.site == source.site
            && self.actual.formal == source.formal
            && self.prepacket.read.binding() == binding
            && matches!(&self.actual.source,
                BorrowedFormalActualSourceV1::Scalar {
                    binding: actual_binding,
                    kind: SourceScalarKind::Integer,
                } if *actual_binding == binding)
    }
}

impl VerifiedLoopStaticBodyPrepacketV1 {
    pub(in crate::mir) fn corroborate_unpublished_function(
        &self,
        function: &MirFunction,
    ) -> Result<(), String> {
        let expected = self.source.materialize_unpublished_call(
            self.read.binding(),
            &self.source.site,
            self.read.physical_value(),
        )?;
        if self.read.owner() != self.source.original.call_site().owner()
            || self.read.physical_block() != self.entry
        {
            return Err("[freeze:contract][callable-loop/body-prepacket-read-drift]".into());
        }
        corroborate_unpublished_physical_shape(
            &expected,
            self.read.physical_value(),
            function,
            self.entry,
            self.normal,
            self.result,
        )
    }

    pub(in crate::mir) fn prepare_detached_packet(
        self,
        function: &MirFunction,
    ) -> Result<PreparedLoopStaticBodyDetachedPacketV1, String> {
        self.corroborate_unpublished_function(function)?;
        let coordinate = unique_unpublished_invoke_coordinate(function, self.entry)?;
        let invoke = function
            .blocks
            .get(&self.entry)
            .and_then(|block| block.terminator.as_ref())
            .expect("unique Invoke coordinate has its original terminator")
            .clone();
        let crate::mir::MirInstruction::Invoke {
            fault_frame,
            fault_landing,
            ..
        } = &invoke
        else {
            return Err("[freeze:contract][callable-loop/body-detached-invoke-missing]".into());
        };
        let projection = function
            .blocks
            .get(&self.normal)
            .and_then(|block| block.instructions.first())
            .ok_or_else(|| {
                "[freeze:contract][callable-loop/body-detached-projection-missing]".to_owned()
            })?;
        let fault_return = function
            .blocks
            .get(fault_landing)
            .and_then(|block| block.terminator.as_ref())
            .ok_or_else(|| {
                "[freeze:contract][callable-loop/body-detached-fault-missing]".to_owned()
            })?;
        if !matches!(fault_return, crate::mir::MirInstruction::ReturnFault { fault_frame: returned } if returned == fault_frame)
        {
            return Err("[freeze:contract][callable-loop/body-detached-fault-drift]".into());
        }
        let lifecycle_bindings = vec![
            (self.entry, invoke.clone()),
            (self.normal, projection.clone()),
            (*fault_landing, fault_return.clone()),
        ];
        let actual = PreparedBorrowedFormalActualV1 {
            ordinal: 0,
            site: self.source.site.clone(),
            formal: self.source.formal,
            source: BorrowedFormalActualSourceV1::Scalar {
                binding: self.source.binding,
                kind: SourceScalarKind::Integer,
            },
        };
        Ok(PreparedLoopStaticBodyDetachedPacketV1 {
            coordinate,
            prepacket: self,
            actual,
            original_invoke: (coordinate.0, invoke),
            lifecycle_bindings,
            header_normal_phis: None,
        })
    }
}

impl VerifiedLoopStaticBodyScalarSourceV1 {
    /// Only the original selected call may lend a target to the unpublished
    /// canonical body. The per-iteration ValueId remains the SSA owner's.
    pub(in crate::mir) fn materialize_unpublished_call(
        &self,
        binding: BindingRefV1,
        site: &SourceExprSiteV1,
        actual: crate::mir::ValueId,
    ) -> Result<MirCall, String> {
        let reject = || "[freeze:contract][callable-loop/body-scalar-call-drift]".to_owned();
        if self.binding != binding
            || &self.site != site
            || self
                .complete_incoming
                .iter()
                .filter(|row| Rc::ptr_eq(row, &self.original))
                .count()
                != 1
        {
            return Err(reject());
        }
        let target = self
            .original
            .target()
            .canonical_global_target_v1()
            .map_err(|_| reject())?;
        Ok(MirCall::global(None, target, vec![actual]))
    }

    pub(in crate::mir) fn prepare_unpublished_physical_packet(
        self,
        read: CanonicalBindingReadReceiptV1,
        function: &MirFunction,
        entry: crate::mir::BasicBlockId,
        normal: crate::mir::BasicBlockId,
        result: crate::mir::ValueId,
    ) -> Result<VerifiedLoopStaticBodyPrepacketV1, String> {
        let packet = VerifiedLoopStaticBodyPrepacketV1 {
            source: self,
            read,
            entry,
            normal,
            result,
        };
        packet.corroborate_unpublished_function(function)?;
        Ok(packet)
    }

    pub(in crate::mir) fn corroborates(
        &self,
        loan: &LoopStaticSourceCallLoanV1,
        binding: BindingRefV1,
        site: &SourceExprSiteV1,
    ) -> bool {
        Rc::ptr_eq(&self.original, loan.original())
            && self
                .complete_incoming
                .iter()
                .filter(|row| Rc::ptr_eq(row, &self.original))
                .count()
                == 1
            && self.binding == binding
            && &self.site == site
            && self.original.argument_sites() == [site.clone()]
            && self.original.parameters().len() == 1
            && self.original.parameters()[0].binding == self.formal
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn retain_loop_static_body_scalar_sources_v1(
        &mut self,
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &VerifiedCallableResultContractCohortV1,
    ) -> Result<(), String> {
        let mut rows = std::collections::BTreeMap::new();
        for ((loop_site, site), loan) in self.loop_static_source_loans.borrow().iter() {
            let Ok(loan) = loan else { continue };
            if loan.placement() != &ResolvedLoopPlacementV1::Body
                || loan.argument_sites().len() != 1
            {
                continue;
            }
            if rows
                .insert(
                    (loop_site.clone(), site.clone()),
                    issue_scalar_source(self, signatures, results, loan),
                )
                .is_some()
            {
                return Err("[freeze:contract][callable-loop/body-scalar-duplicate]".into());
            }
        }
        for (owner, product) in self.preissued_static_i64_products.borrow().iter() {
            let Ok(product) = product else { continue };
            let semantic = product.semantic();
            let body = semantic.source_calls().2;
            let key = (body.loop_site().clone(), body.call_site().clone());
            if body.call_site().owner() != *owner
                || semantic.roles().loop_site != *body.loop_site()
                || body.placement() != &ResolvedLoopPlacementV1::Body
                || body.argument_sites().len() != 1
            {
                return Err(
                    "[freeze:contract][callable-loop/body-scalar-source-unavailable]".into(),
                );
            }
            if rows
                .insert(key, issue_scalar_source(self, signatures, results, body))
                .is_some()
            {
                return Err("[freeze:contract][callable-loop/body-scalar-duplicate]".into());
            }
        }
        *self.loop_static_body_scalar_sources.get_mut() = rows;
        Ok(())
    }

    pub(in crate::mir) fn take_loop_static_body_scalar_source_v1(
        &self,
        loop_site: &SourceStmtSiteV1,
        site: &OwnedExprSiteV1,
    ) -> Option<Result<VerifiedLoopStaticBodyScalarSourceV1, String>> {
        self.loop_static_body_scalar_sources
            .borrow_mut()
            .remove(&(loop_site.clone(), site.clone()))
    }
}

fn issue_scalar_source(
    ledger: &OrdinaryNewClaimLedgerV1,
    signatures: &VerifiedCallablePhysicalSignatureCohortV1,
    results: &VerifiedCallableResultContractCohortV1,
    loan: &LoopStaticSourceCallLoanV1,
) -> Result<VerifiedLoopStaticBodyScalarSourceV1, String> {
    let reject = || "[freeze:contract][callable-loop/body-scalar-source-unavailable]".to_owned();
    let original = loan.original();
    let incoming = ledger.borrowed_formal_source.as_ref().ok_or_else(reject)?;
    let Some((candidate_source, binding, site)) = project_current_owner_loop_scalar_source_v1(
        incoming,
        &ledger.borrowed_formal_actuals,
        loan.call_site(),
        loan.claim(),
    )?
    else {
        return Err(reject());
    };
    let signature = signatures
        .row(original.target_batch_slot())
        .ok_or_else(reject)?;
    let source = incoming.as_ref().map_err(Clone::clone)?;
    let completion = ledger
        .completion_for_owner(original.callee_owner())
        .ok_or_else(reject)?;
    let result = results
        .row(original.target_batch_slot())
        .ok_or_else(reject)?;
    let result_ref = result.borrow();
    let [formal] = original.parameters() else {
        return Err(reject());
    };
    let [lane] = signature.lanes() else {
        return Err(reject());
    };
    if !Rc::ptr_eq(&candidate_source, original)
        || original.argument_sites() != [site.clone()]
        || signature.owner() != original.callee_owner()
        || signature.mode() != CallableParameterDeclarationModeV1::StaticBoxMethod
        || signature.source_logical_arity() != 1
        || signature.receiver_lane_count() != 0
        || signature.physical_formal_lane_count() != 1
        || signature.physical_callable_lane_count() != 1
        || lane.index() != 0
        || lane.role() != PhysicalCallableLaneRoleV1::OrdinaryScalar
        || lane.logical_ordinal() != Some(0)
        || lane.binding() != formal.binding
        || formal.ordinal != 0
        || formal.kind != CallableParameterContractKindV1::OpaqueHandle
        || !source.source_only_formal_origin(formal.binding)
        || !source.checked_static_input(formal.binding)
        || completion.owner() != original.callee_owner()
        || !completion.returns_value()
        || result.owner() != original.callee_owner()
        || !std::ptr::eq(completion, result_ref.completion())
        || !result_ref.declared_result_agrees_with_i64_source()
    {
        return Err(reject());
    }
    // This is the source cohort, not executable transport. A mixed
    // CurrentOwner/qualified callee must keep every original incoming row.
    let complete_incoming = source.static_incoming_cohort_v1(original)?;
    Ok(VerifiedLoopStaticBodyScalarSourceV1 {
        original: Rc::clone(original),
        complete_incoming,
        binding,
        site,
        formal: formal.binding,
    })
}
