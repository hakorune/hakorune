//! The existing source-ordered binding group retains its original physical proof.
//! Sharing a prefix between exits shares the packet, never its affine row.
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use super::*;
use std::rc::Rc;
#[path = "direct_scalar_call.rs"]
mod direct_scalar;
use direct_scalar::DirectScalarCallProjectionV1;

#[derive(Debug, Clone)]
pub(crate) struct RootLifecyclePrefixBindingGroupV1 {
    site: OwnedExprSiteV1,
    bindings: Vec<(BasicBlockId, MirInstruction)>,
    proof: PrefixPhysicalProofV1,
    projected: bool,
}

#[derive(Debug, Clone)]
enum PrefixPhysicalProofV1 {
    Loop(Rc<source_loop_projection::EmittedSourceLoopProjectionV2>),
    Call(Option<Rc<EmittedLexicalCallProjectionV1>>),
    DirectScalar(Rc<DirectScalarCallProjectionV1>),
    EvaluatedBinding(EvaluatedBindingProjectionV1),
    BoolCondition(Rc<EmittedBoolConditionProjectionV1>),
    PrintExpression(Rc<print_expression_projection::EmittedPrintExpressionProjectionV1>),
    OwnedArrayAppend(Rc<owned_array_append_projection::EmittedOwnedArrayAppendV1>),
    FieldArrayAppend(Rc<field_array_append_projection::EmittedFieldArrayAppendV1>),
    BorrowedFieldWrite(Rc<EmittedBorrowedFieldWriteV1>),
}

/// Purpose dispatch over one existing evaluated-expression group; neither arm
/// owns an evaluator, registry or independent traversal of expression semantics.
#[derive(Debug, Clone)]
enum EvaluatedBindingProjectionV1 {
    Initializer(Rc<EmittedInitializerExpressionProjectionV1>),
    Rebind(Rc<binding_rebind_projection::EmittedBindingRebindProjectionV1>),
}

impl RootLifecyclePrefixBindingGroupV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn snapshot_tree_v2(roots: &[Self]) -> Vec<Self> {
        source_loop_projection::snapshot_group_tree_v2(roots)
    }
    pub(super) fn new_loop_pending(packet: Rc<source_loop_projection::EmittedSourceLoopProjectionV2>) -> Self {
        let source = packet.product().source.condition.source();
        Self { site: OwnedExprSiteV1::new(source.owner(), source.nodes()[0].site().clone()),
            bindings: Vec::new(), proof: PrefixPhysicalProofV1::Loop(packet), projected: false }
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn source_loop_rc(&self) -> Option<&Rc<source_loop_projection::EmittedSourceLoopProjectionV2>> {
        match &self.proof { PrefixPhysicalProofV1::Loop(packet) => Some(packet), _ => None }
    }

    pub(in crate::mir::normal_callable_semantic_package) fn new(
        site: OwnedExprSiteV1,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        lexical: Option<Rc<EmittedLexicalCallProjectionV1>>,
    ) -> Result<Self, String> {
        if bindings.is_empty() {
            return Err(freeze("local-call-bindings-empty"));
        }
        if let Some(packet) = &lexical {
            if packet.call_site() != &site {
                return Err(freeze("local-call-proof-site"));
            }
            packet.validate_recorded(&bindings)?;
        }
        Ok(Self {
            site,
            bindings,
            proof: PrefixPhysicalProofV1::Call(lexical),
            projected: false,
        })
    }

    /// Shared commit/group references count one physical packet only when
    /// source identity, original packet and full instruction inventory agree.
    pub(in crate::mir::normal_callable_semantic_package) fn corroborate_received_packet_v1(
        &self, site: &OwnedExprSiteV1, packet: &EmittedLexicalCallProjectionV1,
        bindings: &[(BasicBlockId, MirInstruction)],
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        let projected;
        let expected = if self.projected {
            projected = finishing.ok_or_else(|| freeze("received-call/shared-packet-projection-missing"))?
                .bindings(bindings)?;
            projected.as_slice()
        } else { bindings };
        if self.site() != site || packet.call_site() != site
            || self.lexical().is_none_or(|original| !std::ptr::eq(original, packet))
            || self.bindings() != expected
        { return Err(freeze("received-call/shared-packet-drift")); }
        packet.validate_recorded(bindings)
    }

    pub(in crate::mir) fn check_raw_effect_v2(&self, context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.validate_recorded_in_context(context, function, None)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn same_effect_packet_v2(&self, other: &Self) -> bool {
        self.site == other.site && self.bindings == other.bindings && self.projected == other.projected
            && match (&self.proof, &other.proof) {
                (PrefixPhysicalProofV1::OwnedArrayAppend(a), PrefixPhysicalProofV1::OwnedArrayAppend(b)) => Rc::ptr_eq(a, b),
                (PrefixPhysicalProofV1::Call(Some(a)), PrefixPhysicalProofV1::Call(Some(b))) => Rc::ptr_eq(a, b),
                (PrefixPhysicalProofV1::DirectScalar(a), PrefixPhysicalProofV1::DirectScalar(b)) => Rc::ptr_eq(a, b),
                _ => false,
            }
    }

    /// Match the original Call's actual continuation after argument cleanup.
    /// A discarded value has a packet, but grants no receiving Local binding.
    pub(in crate::mir) fn corroborate_loop_call_v2(
        &self, source: &crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1,
        entry: BasicBlockId, end: BasicBlockId, value: Option<ValueId>,
        context: CallValidationContextV1<'_>, function: &MirFunction, check_coordinates: bool,
    ) -> Result<(), String> {
        if self.site() != source.site() || source.local_binding().is_some() != value.is_some() {
            return Err(freeze("source-loop/call-source"));
        }
        if let Some(value) = value {
            if self.scalar_call_value_v1(source, context)? != value { return Err(freeze("source-loop/call-result")); }
        }
        match &self.proof {
            PrefixPhysicalProofV1::Call(Some(packet)) => packet.corroborate_loop_continuation_v2(
                source, entry, end, context, function, check_coordinates),
            PrefixPhysicalProofV1::DirectScalar(packet) => {
                packet.value(source, context)?;
                packet.corroborate_loop_continuation_v2(entry, end, function, check_coordinates)
            }
            _ => Err(freeze("source-loop/call-packet")),
        }
    }

    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }
    pub(crate) fn bindings(&self) -> &[(BasicBlockId, MirInstruction)] {
        if !self.projected {
            if let Some(packet) = self.source_loop_rc() { return packet.bindings(); }
            if let Some(packet) = self.field_array_append() { return packet.bindings(); }
            if let Some(packet) = self.borrowed_field_write() { return packet.bindings(); }
        }
        &self.bindings
    }

    pub(in crate::mir::normal_callable_semantic_package) fn seal_physical_observations_v1(&self) -> Result<(), String> {
        if let Some(packet) = self.field_array_append() { packet.seal_bindings_v1()?; }
        if let Some(packet) = self.borrowed_field_write() { packet.seal_bindings_v1()?; }
        Ok(())
    }

    /// Intermediate checks borrow the same writer loans; the final census is
    /// separately sealed before capture. Projected bindings remain immutable.
    pub(in crate::mir::normal_callable_semantic_package) fn check_physical_bindings_v1(
        &self, function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        if let (Some(packet), Some(projection)) = (self.source_loop_rc(), finishing) {
            for original in packet.bindings() {
                if let Some(dst) = original.1.dst_value() {
                    if projection.captured_definition(dst)? != original { return Err(freeze("source-loop/captured-write")); }
                } else if matches!(original.1, MirInstruction::KeepAlive { .. }) {
                    projection.validate_metadata_correspondence_v2(function, original, false)?;
                } else if !super::super::physical_boundary::check_binding(function, Some(projection), original.0, &original.1)? {
                    return Err(freeze("source-loop/projected-effect"));
                }
            }
            return Ok(());
        }
        for original in self.bindings() {
            let observed = match (self.projected, self.field_array_append()) {
                (false, Some(packet)) => packet.resolve_observed_binding_v1(original, false)?,
                _ => original.clone(),
            };
            let observed = match (self.projected, self.borrowed_field_write()) {
                (false, Some(packet)) => packet.resolve_observed_binding_v1(&observed, false)?, _ => observed,
            };
            if let (Some(packet), Some(boundary)) = (self.initializer(), finishing) {
                if packet.validates_receipt_copy_v1(&observed, function, boundary, self.projected)? { continue; }
            }
            if !super::super::physical_boundary::check_binding(function, finishing, observed.0, &observed.1)? {
                return Err(format!("{} function={} site={:?} projected={} finishing={} binding={observed:?}",
                    freeze("local-call-binding-drift"), function.signature.name, self.site, self.projected, finishing.is_some()));
            }
        }
        Ok(())
    }
    pub(in crate::mir::normal_callable_semantic_package) fn scalar_call_value_v1(
        &self, source: &crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1,
        context: CallValidationContextV1<'_>,
    ) -> Result<ValueId, String> {
        if self.site() != source.site() { return Err(freeze("scalar-call/group-site")); }
        match &self.proof {
            PrefixPhysicalProofV1::Call(Some(packet)) => packet.value_in_context_v1(source.owner(), source.arguments(), context),
            PrefixPhysicalProofV1::DirectScalar(packet) => packet.value(source, context),
            _ => Err(freeze("scalar-call/source-packet-missing")),
        }
    }
    pub(in crate::mir) fn lexical(&self) -> Option<&EmittedLexicalCallProjectionV1> {
        match &self.proof {
            PrefixPhysicalProofV1::Call(packet) => packet.as_deref(),
            PrefixPhysicalProofV1::Loop(_) | PrefixPhysicalProofV1::DirectScalar(_) | PrefixPhysicalProofV1::EvaluatedBinding(_) | PrefixPhysicalProofV1::BorrowedFieldWrite(_) | PrefixPhysicalProofV1::FieldArrayAppend(_) | PrefixPhysicalProofV1::PrintExpression(_) | PrefixPhysicalProofV1::BoolCondition(_) | PrefixPhysicalProofV1::OwnedArrayAppend(_) => None,
        }
    }

    pub(super) fn new_print(
        packet: Rc<print_expression_projection::EmittedPrintExpressionProjectionV1>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<Self, String> {
        packet.validate_recorded(&bindings, None)?;
        let source = packet.frame().source();
        Ok(Self {
            site: OwnedExprSiteV1::new(source.owner(), source.nodes()[0].site().clone()),
            bindings,
            proof: PrefixPhysicalProofV1::PrintExpression(packet),
            projected: false,
        })
    }

    pub(in crate::mir) fn lexical_packets(&self) -> impl Iterator<Item = &EmittedLexicalCallProjectionV1> {
        let print = match &self.proof {
            PrefixPhysicalProofV1::PrintExpression(packet) => Some(packet),
            _ => None,
        };
        let direct = match &self.proof { PrefixPhysicalProofV1::DirectScalar(packet) => Some(packet), _ => None };
        self.lexical().into_iter().chain(direct.into_iter().flat_map(|packet| packet.argument_calls_v1()))
            .chain(print.into_iter().flat_map(|packet| packet.calls()))
            .chain(self.rebind().into_iter().flat_map(|packet| packet.calls()))
            .chain(self.initializer().into_iter().flat_map(|packet| packet.calls()))
            .chain(self.bool_condition().into_iter().flat_map(|packet| packet.calls()))
            .chain(self.owned_array_append().and_then(|packet| packet.candidate_packet()))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn text_residence_v1(&self) -> Option<TextResidencePacketV1> {
        match &self.proof {
            PrefixPhysicalProofV1::EvaluatedBinding(EvaluatedBindingProjectionV1::Initializer(packet))
                if packet.frame().is_text_residence() => Some(TextResidencePacketV1::Initializer(Rc::clone(packet))),
            PrefixPhysicalProofV1::EvaluatedBinding(EvaluatedBindingProjectionV1::Rebind(packet))
                if packet.frame().is_text_residence() => Some(TextResidencePacketV1::Rebind(Rc::clone(packet))),
            _ => None,
        }
    }

    pub(super) fn initializer_rc(&self) -> Option<&Rc<EmittedInitializerExpressionProjectionV1>> {
        match &self.proof { PrefixPhysicalProofV1::EvaluatedBinding(EvaluatedBindingProjectionV1::Initializer(packet)) => Some(packet), _ => None }
    }

    pub(in crate::mir) fn initializer(&self) -> Option<&EmittedInitializerExpressionProjectionV1> {
        match &self.proof { PrefixPhysicalProofV1::EvaluatedBinding(EvaluatedBindingProjectionV1::Initializer(packet)) => Some(packet), _ => None }
    }
    pub(super) fn new_initializer(packet: Rc<EmittedInitializerExpressionProjectionV1>, bindings: Vec<(BasicBlockId, MirInstruction)>) -> Result<Self, String> {
        packet.validate_recorded(&bindings, None)?;
        Ok(Self { site: OwnedExprSiteV1::new(packet.frame().source().owner(), packet.frame().source().nodes()[0].site().clone()),
            bindings, proof: PrefixPhysicalProofV1::EvaluatedBinding(EvaluatedBindingProjectionV1::Initializer(packet)), projected: false })
    }

    pub(in crate::mir) fn rebind(&self) -> Option<&binding_rebind_projection::EmittedBindingRebindProjectionV1> {
        match &self.proof { PrefixPhysicalProofV1::EvaluatedBinding(EvaluatedBindingProjectionV1::Rebind(packet)) => Some(packet), _ => None }
    }
    pub(super) fn new_rebind(packet: Rc<binding_rebind_projection::EmittedBindingRebindProjectionV1>, bindings: Vec<(BasicBlockId, MirInstruction)>) -> Result<Self, String> {
        packet.validate_recorded(&bindings, None)?;
        Ok(Self { site: OwnedExprSiteV1::new(packet.frame().source().owner(), packet.frame().source().nodes()[0].site().clone()),
            bindings, proof: PrefixPhysicalProofV1::EvaluatedBinding(EvaluatedBindingProjectionV1::Rebind(packet)), projected: false })
    }

    pub(super) fn bool_condition(&self) -> Option<&EmittedBoolConditionProjectionV1> {
        match &self.proof { PrefixPhysicalProofV1::BoolCondition(packet) => Some(packet), _ => None }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn require_sealed_condition_v1(&self) -> Result<(), String> {
        if let Some(packet) = self.source_loop_rc() { packet.require_completed()?; }
        if let Some(packet) = self.bool_condition() { packet.require_sealed()?; }
        if let Some(packet) = self.field_array_append() { packet.require_sealed_v1()?; }
        if let Some(packet) = self.borrowed_field_write() { packet.require_sealed_v1()?; }
        Ok(())
    }
    pub(super) fn new_bool_pending(packet: Rc<EmittedBoolConditionProjectionV1>) -> Result<Self, String> {
        packet.validate_prefix_recorded(packet.prefix())?;
        let source = packet.frame().source();
        Ok(Self { site: OwnedExprSiteV1::new(source.owner(), source.nodes()[0].site().clone()),
            bindings: packet.prefix().to_vec(), proof: PrefixPhysicalProofV1::BoolCondition(packet), projected: false })
    }

    pub(in crate::mir) fn owned_array_append(&self) -> Option<&owned_array_append_projection::EmittedOwnedArrayAppendV1> {
        match &self.proof { PrefixPhysicalProofV1::OwnedArrayAppend(packet) => Some(packet), _ => None }
    }
    pub(super) fn new_append(
        packet: Rc<owned_array_append_projection::EmittedOwnedArrayAppendV1>, bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<Self, String> {
        packet.validate_recorded(&bindings, None)?;
        Ok(Self { site: packet.source().site().clone(), bindings,
            proof: PrefixPhysicalProofV1::OwnedArrayAppend(packet), projected: false })
    }

    pub(in crate::mir) fn borrowed_field_write(&self) -> Option<&EmittedBorrowedFieldWriteV1> {
        match &self.proof { PrefixPhysicalProofV1::BorrowedFieldWrite(packet) => Some(packet), _ => None }
    }
    pub(super) fn new_borrowed_field_write(packet: Rc<EmittedBorrowedFieldWriteV1>) -> Result<Self, String> {
        packet.validate_recorded(packet.bindings(), None)?;
        Ok(Self { site: packet.source().value_site(), bindings: packet.bindings().to_vec(),
            proof: PrefixPhysicalProofV1::BorrowedFieldWrite(packet), projected: false })
    }

    pub(in crate::mir) fn field_array_append(&self) -> Option<&field_array_append_projection::EmittedFieldArrayAppendV1> {
        match &self.proof { PrefixPhysicalProofV1::FieldArrayAppend(packet) => Some(packet), _ => None }
    }
    pub(super) fn field_array_append_packet_v1(&self) -> Option<&Rc<field_array_append_projection::EmittedFieldArrayAppendV1>> {
        match &self.proof { PrefixPhysicalProofV1::FieldArrayAppend(packet) => Some(packet), _ => None }
    }
    pub(super) fn new_field_append(packet: Rc<field_array_append_projection::EmittedFieldArrayAppendV1>, bindings: Vec<(BasicBlockId, MirInstruction)>) -> Result<Self, String> {
        packet.validate_recorded(&bindings, None)?;
        Ok(Self { site: packet.source().site().clone(), bindings, proof: PrefixPhysicalProofV1::FieldArrayAppend(packet), projected: false })
    }
    pub(in crate::mir::normal_callable_semantic_package) fn matches_source(&self, source: &LifecyclePrefixSourceV1) -> bool {
        if self.site() != source.site() { return false; }
        match (&self.proof, source) {
            (PrefixPhysicalProofV1::Loop(packet), LifecyclePrefixSourceV1::Loop { product, .. }) => Rc::ptr_eq(packet.product(), product),
            (PrefixPhysicalProofV1::EvaluatedBinding(EvaluatedBindingProjectionV1::Initializer(packet)), LifecyclePrefixSourceV1::InitializerExpression { frame, .. }) => Rc::ptr_eq(packet.frame(), frame),
            (PrefixPhysicalProofV1::EvaluatedBinding(EvaluatedBindingProjectionV1::Rebind(packet)), LifecyclePrefixSourceV1::BindingRebind { frame, .. }) => Rc::ptr_eq(packet.frame(), frame),
            (PrefixPhysicalProofV1::BorrowedFieldWrite(packet), LifecyclePrefixSourceV1::BorrowedFieldWrite { frame, .. }) =>
                Rc::ptr_eq(packet.source().frame(), frame),
            (PrefixPhysicalProofV1::Call(_) | PrefixPhysicalProofV1::DirectScalar(_), LifecyclePrefixSourceV1::Call { .. }) => true,
            (PrefixPhysicalProofV1::FieldArrayAppend(packet), LifecyclePrefixSourceV1::FieldArrayAppend { proof, index, .. }) =>
                packet.source().matches_original(proof, *index),
            (PrefixPhysicalProofV1::FieldArrayAppend(packet), LifecyclePrefixSourceV1::FieldTextAppend { frame, .. }) => packet.source().matches_text_frame(frame),
            (PrefixPhysicalProofV1::BoolCondition(packet), LifecyclePrefixSourceV1::BoolCondition { frame, .. }) =>
                Rc::ptr_eq(packet.frame(), frame),
            (PrefixPhysicalProofV1::PrintExpression(packet), LifecyclePrefixSourceV1::PrintExpression { frame, .. }) =>
                Rc::ptr_eq(packet.frame(), frame),
            (PrefixPhysicalProofV1::OwnedArrayAppend(packet), LifecyclePrefixSourceV1::OwnedArrayAppend { holder, candidate, .. }) =>
                packet.source().matches_original_v1(holder, candidate),
            _ => false,
        }
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_recorded(
        &self, ledger: &OrdinaryNewClaimLedgerV1, function: &MirFunction,
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        self.validate_recorded_in_context(CallValidationContextV1::Lowering(ledger), function, finishing)
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_recorded_in_context(
        &self, context: CallValidationContextV1<'_>, function: &MirFunction,
        finishing: Option<&super::super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        let coordinates = if self.projected { finishing } else { None };
        if let Some(home) = self.text_residence_v1() { context.ledger().validate_text_residence_home_v1(&home, context)?; }
        match &self.proof {
            PrefixPhysicalProofV1::Loop(packet) =>
                packet.validate_completion_v2(context, function, self.bindings(), coordinates, finishing),
            PrefixPhysicalProofV1::EvaluatedBinding(EvaluatedBindingProjectionV1::Rebind(packet)) => {
                packet.validate_recorded(&self.bindings, coordinates)?;
                packet.validate_paths(context, function, finishing).map(drop)
            }
            PrefixPhysicalProofV1::EvaluatedBinding(EvaluatedBindingProjectionV1::Initializer(packet)) => {
                packet.validate_recorded(&self.bindings, coordinates)?;
                packet.validate_paths(context, function, finishing).map(drop)
            }
            PrefixPhysicalProofV1::BorrowedFieldWrite(packet) => {
                packet.validate_recorded(&self.bindings, coordinates)?;
                packet.validate_paths(context, function, finishing)
            },
            PrefixPhysicalProofV1::Call(Some(packet)) => {
                packet.validate_recorded_projected(&self.bindings, coordinates)?;
                packet.validate_expression_arguments_v1(context, function, finishing, &self.bindings, coordinates).map(drop)
            },
            PrefixPhysicalProofV1::Call(None) => Ok(()),
            PrefixPhysicalProofV1::DirectScalar(packet) => packet.validate(context, function, finishing, &self.bindings, coordinates),
            PrefixPhysicalProofV1::BoolCondition(packet) =>
                packet.validate_recorded(&self.bindings, context, function, coordinates, finishing),
            PrefixPhysicalProofV1::PrintExpression(packet) => {
                packet.validate_recorded(&self.bindings, coordinates)?;
                packet.validate_paths_in_context(context, function, finishing).map(drop)
            },
            PrefixPhysicalProofV1::FieldArrayAppend(packet) => {
                packet.validate_recorded(self.bindings(), coordinates)?;
                packet.validate_paths(context, function, finishing)
            },
            PrefixPhysicalProofV1::OwnedArrayAppend(packet) => {
                packet.validate_recorded(&self.bindings, coordinates)?;
                packet.validate_paths_in_context(context, function, finishing)
            },
        }
    }

    // Only flat bindings are rebound. The shared original packet remains
    // immutable even when several exits cover the same source prefix.
    pub(in crate::mir::normal_callable_semantic_package) fn with_bindings(
        &self,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Self {
        Self {
            site: self.site.clone(),
            bindings,
            proof: self.proof.clone(),
            projected: true,
        }
    }
}
