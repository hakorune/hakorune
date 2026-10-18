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
    Call(Option<Rc<EmittedLexicalCallProjectionV1>>),
    DirectScalar(Rc<DirectScalarCallProjectionV1>),
    InitializerExpression(Rc<EmittedInitializerExpressionProjectionV1>),
    BoolCondition(Rc<EmittedBoolConditionProjectionV1>),
    PrintExpression(Rc<print_expression_projection::EmittedPrintExpressionProjectionV1>),
    OwnedArrayAppend(Rc<owned_array_append_projection::EmittedOwnedArrayAppendV1>),
    FieldArrayAppend(Rc<field_array_append_projection::EmittedFieldArrayAppendV1>),
    BorrowedFieldWrite(Rc<EmittedBorrowedFieldWriteV1>),
}

impl RootLifecyclePrefixBindingGroupV1 {
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

    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }
    pub(crate) fn bindings(&self) -> &[(BasicBlockId, MirInstruction)] {
        if !self.projected {
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
        for original in self.bindings() {
            let observed = match (self.projected, self.field_array_append()) {
                (false, Some(packet)) => packet.resolve_observed_binding_v1(original, false)?,
                _ => original.clone(),
            };
            let observed = match (self.projected, self.borrowed_field_write()) {
                (false, Some(packet)) => packet.resolve_observed_binding_v1(&observed, false)?, _ => observed,
            };
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
            PrefixPhysicalProofV1::DirectScalar(_) | PrefixPhysicalProofV1::InitializerExpression(_) | PrefixPhysicalProofV1::BorrowedFieldWrite(_) | PrefixPhysicalProofV1::FieldArrayAppend(_) | PrefixPhysicalProofV1::PrintExpression(_) | PrefixPhysicalProofV1::BoolCondition(_) | PrefixPhysicalProofV1::OwnedArrayAppend(_) => None,
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
        self.lexical().into_iter().chain(print.into_iter().flat_map(|packet| packet.calls()))
            .chain(self.initializer().into_iter().flat_map(|packet| packet.calls()))
            .chain(self.bool_condition().into_iter().flat_map(|packet| packet.calls()))
            .chain(self.owned_array_append().and_then(|packet| packet.candidate_packet()))
    }

    pub(super) fn initializer_rc(&self) -> Option<&Rc<EmittedInitializerExpressionProjectionV1>> {
        match &self.proof { PrefixPhysicalProofV1::InitializerExpression(packet) => Some(packet), _ => None }
    }

    pub(in crate::mir) fn initializer(&self) -> Option<&EmittedInitializerExpressionProjectionV1> {
        match &self.proof { PrefixPhysicalProofV1::InitializerExpression(packet) => Some(packet), _ => None }
    }
    pub(super) fn new_initializer(packet: Rc<EmittedInitializerExpressionProjectionV1>, bindings: Vec<(BasicBlockId, MirInstruction)>) -> Result<Self, String> {
        packet.validate_recorded(&bindings, None)?;
        Ok(Self { site: OwnedExprSiteV1::new(packet.frame().source().owner(), packet.frame().source().nodes()[0].site().clone()),
            bindings, proof: PrefixPhysicalProofV1::InitializerExpression(packet), projected: false })
    }

    pub(super) fn bool_condition(&self) -> Option<&EmittedBoolConditionProjectionV1> {
        match &self.proof { PrefixPhysicalProofV1::BoolCondition(packet) => Some(packet), _ => None }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn require_sealed_condition_v1(&self) -> Result<(), String> {
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
    pub(super) fn new_field_append(packet: Rc<field_array_append_projection::EmittedFieldArrayAppendV1>, bindings: Vec<(BasicBlockId, MirInstruction)>) -> Result<Self, String> {
        packet.validate_recorded(&bindings, None)?;
        Ok(Self { site: packet.source().site().clone(), bindings, proof: PrefixPhysicalProofV1::FieldArrayAppend(packet), projected: false })
    }
    pub(in crate::mir::normal_callable_semantic_package) fn matches_source(&self, source: &LifecyclePrefixSourceV1) -> bool {
        if self.site() != source.site() { return false; }
        match (&self.proof, source) {
            (PrefixPhysicalProofV1::InitializerExpression(packet), LifecyclePrefixSourceV1::InitializerExpression { frame, .. }) => Rc::ptr_eq(packet.frame(), frame),
            (PrefixPhysicalProofV1::BorrowedFieldWrite(packet), LifecyclePrefixSourceV1::BorrowedFieldWrite { frame, .. }) =>
                Rc::ptr_eq(packet.source().frame(), frame),
            (PrefixPhysicalProofV1::Call(_) | PrefixPhysicalProofV1::DirectScalar(_), LifecyclePrefixSourceV1::Call { .. }) => true,
            (PrefixPhysicalProofV1::FieldArrayAppend(packet), LifecyclePrefixSourceV1::FieldArrayAppend { proof, index, .. }) =>
                packet.source().matches_original(proof, *index),
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
        match &self.proof {
            PrefixPhysicalProofV1::InitializerExpression(packet) => {
                context.ledger().validate_text_initializer_home_v1(packet, context)?;
                packet.validate_recorded(&self.bindings, coordinates)?;
                packet.validate_paths(context, function, finishing).map(drop)
            }
            PrefixPhysicalProofV1::BorrowedFieldWrite(packet) => {
                packet.validate_recorded(&self.bindings, coordinates)?;
                packet.validate_paths(context, function, finishing)
            },
            PrefixPhysicalProofV1::Call(Some(packet)) => packet.validate_recorded_projected(&self.bindings, coordinates),
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
