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
}
impl PartialEq for ReceivedFieldReadSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Object { request: a, loan: al }, Self::Object { request: b, loan: bl }) =>
                a == b && al.source() == bl.source() && al.result_class() == bl.result_class()
                    && al.roots().len() == bl.roots().len()
                    && al.roots().iter().zip(bl.roots()).all(|(a, b)| Rc::ptr_eq(a, b)),
            (Self::Mixed { request: a, target: at }, Self::Mixed { request: b, target: bt }) => a == b && at == bt,
            _ => false,
        }
    }
}
impl Eq for ReceivedFieldReadSourceV1 {}
impl ReceivedFieldReadSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn issue(
        (batch, selected, classes, targets, _): ReceivedFieldContextV1<'_>,
        ingress: &PreparedBorrowedFormalIngressV1,
        request: &LocalFieldReadRequestV1,
    ) -> Result<Self, String> {
        let original = request.received.as_ref().ok_or_else(|| fault("received/source-missing"))?;
        if request.nullable || request.formal || request.alias_class.is_some()
            || request.home != request.receiver || request.site.owner() != request.receiver.owner()
        { return Err(fault("received/provenance-overlap")); }
        let acquisition = match original {
            ReceivedFieldReadRequestV1::StoredMapGet { .. } => return Err(fault("received/provenance-overlap")),
            ReceivedFieldReadRequestV1::Object { acquisition, .. } => acquisition,
            ReceivedFieldReadRequestV1::Mixed(loan) => loan.call(),
        };
        let target = original_lexical_target_at_v1(targets, acquisition)?
            .ok_or_else(|| fault("received/original-target-missing"))?;
        let mut callers = batch.declarations().filter(|row| row.owner() == request.site.owner());
        let caller = callers.next().ok_or_else(|| fault("received/caller-missing"))?;
        if callers.next().is_some() { return Err(fault("received/caller-ambiguous")); }
        let exact = batch.with_lowering_input(caller.batch_slot(), |input| {
            let mut rows = input.function().expression_source().initializers()
                .filter(|row| row.binding() == request.receiver);
            rows.next().is_some_and(|row| row.initializer_site() == Some(acquisition.site())) && rows.next().is_none()
        }).map_err(|_| fault("received/caller-input"))?;
        if !exact || acquisition.owner() != request.site.owner() { return Err(fault("received/initializer")); }
        match original {
            ReceivedFieldReadRequestV1::StoredMapGet { .. } => Err(fault("received/provenance-overlap")),
            ReceivedFieldReadRequestV1::Object { nullable, .. } => {
                let loan = classes.object_call_source_v1(batch, selected, target).map_err(fault)?;
                if !matches!((loan.result_class(), *nullable),
                    (OrdinaryNewResultClassV1::Object(_), false)
                    | (OrdinaryNewResultClassV1::NullableObject(_), true)) {
                    return Err(fault("received/object-result-kind"));
                }
                Ok(Self::Object { request: original.clone(), loan })
            }
            ReceivedFieldReadRequestV1::Mixed(loan) => {
                if loan.destination() != request.receiver
                    || ingress.mixed_received_actual_source_v1(request.receiver)? != Some(loan.as_ref())
                    || ingress.original_mixed_input_source_v1(target)? != Some(loan.as_ref())
                { return Err(fault("received/mixed-source")); }
                classes.recheck_mixed_local_roots_v1(loan).map_err(fault)?;
                Ok(Self::Mixed { request: original.clone(), target: target.clone() })
            }
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn request(&self) -> &ReceivedFieldReadRequestV1 {
        match self { Self::Object { request, .. } | Self::Mixed { request, .. } => request }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn class(&self) -> &str {
        match self {
            Self::Object { loan, .. } => loan.class(),
            Self::Mixed { request: ReceivedFieldReadRequestV1::Mixed(loan), .. } => loan.fresh_class(),
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
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn mixed(&self) -> Option<&MixedLocalCallSourceLoanV1> {
        match self.request() { ReceivedFieldReadRequestV1::Mixed(loan) => Some(loan), _ => None }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn target(&self) -> &LexicalInstanceCallSourceTargetV1 {
        match self { Self::Object { loan, .. } => loan.source(), Self::Mixed { target, .. } => target }
    }
    /// Borrow the original target from its current sole owner: the issuer
    /// during source selection, or the retained Call packet after affine moves.
    pub(in crate::mir::normal_callable_semantic_package) fn recheck_issuing_target_v1(
        &self, classes: &OrdinaryNewResultClassClaimsV1, ingress: &PreparedBorrowedFormalIngressV1,
        target: &LexicalInstanceCallSourceTargetV1, request: &LocalFieldReadRequestV1,
    ) -> Result<(), String> {
        if request.received.as_ref() != Some(self.request()) || request.nullable || request.formal
            || request.alias_class.is_some() || request.receiver != request.home || target != self.target()
        { return Err(fault("received/source-drift")); }
        match self {
            Self::Object { loan, .. } => classes.recheck_object_call_roots_v1(self.target(), loan).map_err(fault),
            Self::Mixed { .. } => {
                let loan = self.mixed().ok_or_else(|| fault("received/mixed-source"))?;
                if loan.destination() != request.receiver
                    || ingress.mixed_received_actual_source_v1(request.receiver)? != Some(loan)
                    || ingress.original_mixed_input_source_v1(self.target())? != Some(loan)
                { return Err(fault("received/mixed-source-drift")); }
                classes.recheck_mixed_local_roots_v1(loan).map_err(fault)
            }
        }
    }
}
