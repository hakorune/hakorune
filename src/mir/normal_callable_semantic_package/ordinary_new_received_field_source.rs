//! Received-field source on the existing affine field descriptor, not a registry.
use super::*;
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::result_class_claim::OrdinaryNewResultClassClaimsV1;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::result_class_claim::ObjectCallSourceLoanV1;
use crate::mir::normal_callable_semantic_package::MixedLocalCallSourceLoanV1;
use crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use crate::mir::resolved_semantics::home_new_prefix::{LocalFieldReadRequestV1, ReceivedFieldReadRequestV1};
use super::super::lexical_instance_call::{PreparedBorrowedFormalIngressV1, LexicalInstanceCallSourceTargetV1, PreparedLexicalInstanceCallSourceTargetsV1, original_lexical_target_at_v1};

pub(in crate::mir::normal_callable_semantic_package) type ReceivedFieldContextV1<'a> = (
    &'a VerifiedResolvedCallableSemanticBatchV1,
    &'a VerifiedSelectedCallableBatchMapV1,
    &'a OrdinaryNewResultClassClaimsV1,
    &'a PreparedLexicalInstanceCallSourceTargetsV1,
    &'a super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
);

#[derive(Debug, Clone)]
pub(in crate::mir::normal_callable_semantic_package) enum ReceivedFieldReadSourceV1 {
    Object { request: ReceivedFieldReadRequestV1, loan: ObjectCallSourceLoanV1 },
    Mixed { request: ReceivedFieldReadRequestV1, target: LexicalInstanceCallSourceTargetV1 },
    Borrowed { request: ReceivedFieldReadRequestV1, targets: Vec<LexicalInstanceCallSourceTargetV1> },
}
impl PartialEq for ReceivedFieldReadSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Object { request: a, loan: al }, Self::Object { request: b, loan: bl }) =>
                a == b && al.source() == bl.source() && al.result_class() == bl.result_class()
                    && al.roots().len() == bl.roots().len()
                    && al.roots().iter().zip(bl.roots()).all(|(a, b)| Rc::ptr_eq(a, b)),
            (Self::Mixed { request: a, target: at }, Self::Mixed { request: b, target: bt }) => a == b && at == bt,
            (Self::Borrowed { request: a, targets: at }, Self::Borrowed { request: b, targets: bt }) => a == b && at == bt,
            _ => false,
        }
    }
}
impl Eq for ReceivedFieldReadSourceV1 {}
impl ReceivedFieldReadSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn issue(
        (batch, selected, classes, targets_context, _): ReceivedFieldContextV1<'_>,
        ingress: &PreparedBorrowedFormalIngressV1,
        request: &LocalFieldReadRequestV1,
    ) -> Result<Self, String> {
        let original = request.received.as_ref().ok_or_else(|| fault("received/source-missing"))?;
        if request.nullable || request.formal || request.alias_class.is_some()
            || request.home != request.receiver || request.site.owner() != request.receiver.owner()
        { return Err(fault("received/provenance-overlap")); }
        let acquisitions: Vec<_> = match original {
            ReceivedFieldReadRequestV1::StoredMapGet { .. } => return Err(fault("received/provenance-overlap")),
            ReceivedFieldReadRequestV1::Object { acquisition, .. } => vec![acquisition],
            ReceivedFieldReadRequestV1::Mixed(loan) => vec![loan.call()],
            ReceivedFieldReadRequestV1::Borrowed { loan, .. } => loan.loans().map(|leaf| leaf.call()).collect(),
        };
        if acquisitions.is_empty() { return Err(fault("received/original-target-missing")); }
        let mut targets = Vec::with_capacity(acquisitions.len());
        for acquisition in &acquisitions {
            let target = original_lexical_target_at_v1(targets_context, acquisition)?;
            targets.push(target.ok_or_else(|| fault("received/original-target-missing"))?.clone());
        }
        let caller = batch.unique_declaration_for_owner(request.site.owner())
            .map_err(|_| fault("received/caller-ambiguous"))?
            .ok_or_else(|| fault("received/caller-missing"))?;
        let exact = batch.with_lowering_input(caller.batch_slot(), |input| {
            if let ReceivedFieldReadRequestV1::Borrowed { loan, exclusion } = original {
                return loan.binding() == request.receiver && loan.read().site() == &request.receiver_site
                    && loan.corroborates_v1(input) && exclusion.as_ref().is_none_or(|exclusion|
                        exclusion.corroborates_result_read_v1(input, loan));
            }
            let mut rows = input.function().expression_source().initializers().filter(|row| row.binding() == request.receiver);
            rows.next().is_some_and(|row| row.initializer_site() == Some(acquisitions[0].site())) && rows.next().is_none()
        }).map_err(|_| fault("received/caller-input"))?;
        if !exact || acquisitions.iter().any(|acquisition| acquisition.owner() != request.site.owner()) { return Err(fault("received/initializer")); }
        match original {
            ReceivedFieldReadRequestV1::StoredMapGet { .. } => Err(fault("received/provenance-overlap")),
            ReceivedFieldReadRequestV1::Object { nullable, .. } => {
                let loan = classes.object_call_source_v1(batch, selected, &targets[0]).map_err(fault)?;
                if !matches!((loan.result_class(), *nullable),
                    (OrdinaryNewResultClassV1::Object(_), false)
                    | (OrdinaryNewResultClassV1::NullableObject(_), true)) {
                    return Err(fault("received/object-result-kind"));
                }
                Ok(Self::Object { request: original.clone(), loan })
            }
            ReceivedFieldReadRequestV1::Borrowed { .. } => {
                let source = Self::Borrowed { request: original.clone(), targets };
                for target in source.targets_v1() { source.recheck_issuing_target_v1(classes, ingress, target, request)?; }
                Ok(source)
            }
            ReceivedFieldReadRequestV1::Mixed(loan) => {
                if loan.destination() != request.receiver
                    || ingress.mixed_received_actual_source_v1(request.receiver)? != Some(loan.as_ref())
                    || ingress.original_mixed_input_source_v1(&targets[0])? != Some(loan.as_ref())
                { return Err(fault("received/mixed-source")); }
                classes.recheck_mixed_local_roots_v1(loan).map_err(fault)?;
                Ok(Self::Mixed { request: original.clone(), target: targets[0].clone() })
            }
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn request(&self) -> &ReceivedFieldReadRequestV1 {
        match self { Self::Object { request, .. } | Self::Mixed { request, .. } | Self::Borrowed { request, .. } => request }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn class(&self) -> &str {
        match self {
            Self::Object { loan, .. } => loan.class(),
            Self::Mixed { request: ReceivedFieldReadRequestV1::Mixed(loan), .. } => loan.fresh_class(),
            Self::Borrowed { request: ReceivedFieldReadRequestV1::Borrowed { loan, .. }, .. } =>
                loan.declared_type(),
            _ => unreachable!("received Mixed source retains original Mixed request"),
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn object_result_kind_v1(&self)
        -> Option<crate::mir::instruction::InvokeCallResultKind> {
        match self {
            Self::Object { loan, .. } => match loan.result_class() {
                OrdinaryNewResultClassV1::Object(_) => Some(crate::mir::instruction::InvokeCallResultKind::Handle),
                OrdinaryNewResultClassV1::NullableObject(_) => Some(crate::mir::instruction::InvokeCallResultKind::NullableHandle),
                _ => None,
            },
            Self::Mixed { .. } => None,
            Self::Borrowed { request: ReceivedFieldReadRequestV1::Borrowed { loan, .. }, .. } =>
                loan.single().map(|read| read.source().result_kind_v1()),
            Self::Borrowed { .. } => None,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn mixed(&self) -> Option<&MixedLocalCallSourceLoanV1> {
        match self.request() { ReceivedFieldReadRequestV1::Mixed(loan) => Some(loan), _ => None }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn borrowed(&self)
        -> Option<&crate::mir::normal_callable_semantic_package::BorrowedNullableLocalSourceLoanV1> {
        match self.request() { ReceivedFieldReadRequestV1::Borrowed { loan, .. } => loan.single().map(|read| read.source()), _ => None }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn borrowed_read_v1(&self)
        -> Option<&crate::mir::resolved_semantics::home_new_prefix::DeclaredReadValueV1> {
        match self.request() { ReceivedFieldReadRequestV1::Borrowed { loan, .. } => Some(loan), _ => None }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn borrowed_guard(&self) -> Option<(
        &crate::mir::resolved_semantics::home_new_prefix::DeclaredReadValueV1,
        &Rc<crate::mir::resolved_semantics::home_new_prefix::SourceNullExclusionV1>,
    )> {
        match self.request() {
            ReceivedFieldReadRequestV1::Borrowed { loan, exclusion: Some(exclusion) }
                if exclusion.root_binding() == loan.original_binding() => Some((loan, exclusion)),
            _ => None,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn targets_v1(&self) -> impl Iterator<Item = &LexicalInstanceCallSourceTargetV1> {
        let single = match self { Self::Object { loan, .. } => Some(loan.source()), Self::Mixed { target, .. } => Some(target), _ => None };
        let multiple = match self { Self::Borrowed { targets, .. } => targets.as_slice(), _ => &[] };
        single.into_iter().chain(multiple.iter())
    }
    pub(in crate::mir::normal_callable_semantic_package) fn target(&self) -> Result<&LexicalInstanceCallSourceTargetV1, String> {
        let mut targets = self.targets_v1();
        let target = targets.next().ok_or_else(|| fault("received/original-target-missing"))?;
        if targets.next().is_some() { return Err(fault("received/multiple-call-target")); }
        Ok(target)
    }
    /// Borrow the original target from its current sole owner: the issuer
    /// during source selection, or the retained Call packet after affine moves.
    pub(in crate::mir::normal_callable_semantic_package) fn recheck_issuing_target_v1(
        &self, classes: &OrdinaryNewResultClassClaimsV1, ingress: &PreparedBorrowedFormalIngressV1,
        target: &LexicalInstanceCallSourceTargetV1, request: &LocalFieldReadRequestV1,
    ) -> Result<(), String> {
        if request.received.as_ref() != Some(self.request()) || request.nullable || request.formal
            || request.alias_class.is_some() || request.receiver != request.home || !self.targets_v1().any(|original| original == target)
        { return Err(fault("received/source-drift")); }
        match self {
            Self::Object { loan, .. } => classes.recheck_object_call_roots_v1(self.target()?, loan).map_err(fault),
            Self::Borrowed { .. } => {
                let view = self.borrowed_read_v1().ok_or_else(|| fault("received/source-drift"))?;
                let mut matching = view.loans().filter(|loan| loan.call() == target.call_site());
                let loan = matching.next().ok_or_else(|| fault("received/source-drift"))?;
                if matching.next().is_some() { return Err(fault("received/source-drift")); }
                if self.borrowed_read_v1().is_none_or(|view| view.binding() != request.receiver || view.read().site() != &request.receiver_site) || !loan.is_declared_read_v1() || (!view.payload_nonnull() && self.borrowed_guard().is_none())
                    || loan.lifetime_anchor_v1().is_none()
                    || target.borrowed_nullable_local_source() != Some(loan)
                    || !ingress.corroborated_original_v1() {
                    return Err(fault("received/source-drift"));
                }
                let roots = classes.checked_original_source_roots_v1(loan.key(), loan.callee_owner()).map_err(fault)?;
                classes.check_borrowed_nullable_roots_v1(loan, &roots).map_err(fault)
            }
            Self::Mixed { .. } => {
                let loan = self.mixed().ok_or_else(|| fault("received/mixed-source"))?;
                if loan.destination() != request.receiver
                    || ingress.mixed_received_actual_source_v1(request.receiver)? != Some(loan)
                    || ingress.original_mixed_input_source_v1(self.target()?)? != Some(loan)
                { return Err(fault("received/mixed-source-drift")); }
                classes.recheck_mixed_local_roots_v1(loan).map_err(fault)
            }
        }
    }
}
