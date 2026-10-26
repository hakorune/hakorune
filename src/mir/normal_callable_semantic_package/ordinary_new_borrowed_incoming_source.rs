//! Owned incoming identity preserves the original instance or static source kind.
//! Common coordinates never grant a receiver, object authority or executable ABI.
use super::super::source::{CallTargetReferenceV1, StoredReceiverSourceV1};
use super::super::{LexicalCallSourceResultRequirementV1, LexicalInstanceCallReceiverV1};
use super::*;
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
use std::rc::Rc;

/// Conditional source requirement only. Canonical declaration coordinates do
/// not prove a live receiver, eligibility, incoming payload or executable call.
#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct OriginalStoredIncomingV1 {
    reference: CallTargetReferenceV1,
    receiver: StoredReceiverSourceV1,
    result: LexicalCallSourceResultRequirementV1,
    owning_object_input: Option<Rc<super::super::super::result_class_claim::OwningObjectLocalInputSourceLoanV1>>,
    expected_receiver: LexicalInstanceCallReceiverV1,
}

#[derive(Debug, Clone)]
pub(in crate::mir::normal_callable_semantic_package) enum BorrowedIncomingSourceV1 {
    Instance(super::super::LexicalInstanceCallSourceTargetV1),
    Static(Rc<StaticIncomingSourceV1>),
    PendingStored(Rc<OriginalStoredIncomingV1>),
    SelectedStored {
        original: Rc<OriginalStoredIncomingV1>,
        target: super::super::LexicalInstanceCallSourceTargetV1,
    },
}

impl BorrowedIncomingSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn as_loan(
        &self,
    ) -> Result<BorrowedCallSourceLoanV1<'_>, String> {
        match self {
            Self::Instance(row) | Self::SelectedStored { target: row, .. } => {
                Ok(BorrowedCallSourceLoanV1::Instance(row))
            }
            Self::Static(row) => Ok(BorrowedCallSourceLoanV1::Static(row)),
            Self::PendingStored(_) => Err(super::super::freeze(
                "borrowed-formal/incoming-stored-pending",
            )),
        }
    }
    /// Original coordinates also exist before Stored execution selection.
    /// This lends no instance target, receiver, argument frame or result ABI.
    pub(in crate::mir::normal_callable_semantic_package) fn original_instance_reference_v1(&self) -> Option<CallTargetReferenceV1> {
        match self {
            Self::Instance(row) | Self::SelectedStored { target: row, .. } => Some(CallTargetReferenceV1::from_target(row)),
            Self::PendingStored(row) => Some(row.reference.clone()),
            Self::Static(_) => None,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn original_owning_object_input_v1(&self)
        -> Option<&super::super::super::result_class_claim::OwningObjectLocalInputSourceLoanV1> {
        match self {
            Self::PendingStored(row) | Self::SelectedStored { original: row, .. } => row.owning_object_input.as_deref(),
            Self::Instance(row) => row.owning_object_local_input_v1(),
            Self::Static(_) => None,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn instance(
        &self,
    ) -> Option<&super::super::LexicalInstanceCallSourceTargetV1> {
        match self {
            Self::Instance(row) | Self::SelectedStored { target: row, .. } => Some(row),
            Self::Static(_) | Self::PendingStored(_) => None,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn require_instance(
        &self,
    ) -> Result<&super::super::LexicalInstanceCallSourceTargetV1, String> {
        self.instance()
            .ok_or_else(|| super::super::freeze("borrowed-formal/instance-source-required"))
    }
    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn instance_mut(
        &mut self,
    ) -> &mut super::super::LexicalInstanceCallSourceTargetV1 {
        match self {
            Self::Instance(row) | Self::SelectedStored { target: row, .. } => row,
            Self::Static(_) | Self::PendingStored(_) => {
                panic!("Instance mutation requires Instance source")
            }
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn call_site(&self) -> &OwnedExprSiteV1 {
        match self {
            Self::Instance(row) | Self::SelectedStored { target: row, .. } => row.call_site(),
            Self::Static(row) => row.call_site(),
            Self::PendingStored(row) => &row.reference.call_site,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn target(
        &self,
    ) -> &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1 {
        match self {
            Self::Instance(row) | Self::SelectedStored { target: row, .. } => row.target(),
            Self::Static(row) => row.target(),
            Self::PendingStored(row) => &row.reference.target,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn callee_owner(
        &self,
    ) -> FunctionOwnerIdV1 {
        match self {
            Self::Instance(row) | Self::SelectedStored { target: row, .. } => row.callee_owner(),
            Self::Static(row) => row.callee_owner(),
            Self::PendingStored(row) => row.reference.callee_owner,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn target_batch_slot(&self) -> u32 {
        match self {
            Self::Instance(row) | Self::SelectedStored { target: row, .. } => {
                row.target_batch_slot()
            }
            Self::Static(row) => row.target_batch_slot(),
            Self::PendingStored(row) => row.reference.target_batch_slot,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn argument_sites(
        &self,
    ) -> &[SourceExprSiteV1] {
        match self {
            Self::Instance(row) | Self::SelectedStored { target: row, .. } => row.argument_sites(),
            Self::Static(row) => row.argument_sites(),
            Self::PendingStored(row) => &row.reference.argument_sites,
        }
    }
    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn reissue_stored_origin_for_test(
        &mut self,
    ) {
        let original = match self {
            Self::SelectedStored { original, .. } | Self::PendingStored(original) => original,
            _ => panic!("original Stored source"),
        };
        *original = Rc::new(OriginalStoredIncomingV1 {
            reference: original.reference.clone(),
            receiver: original.receiver.clone(),
            result: original.result.clone(),
            owning_object_input: original.owning_object_input.clone(),
            expected_receiver: original.expected_receiver.clone(),
        });
    }
    pub(in crate::mir::normal_callable_semantic_package) fn is_pending(&self) -> bool {
        matches!(self, Self::PendingStored(_))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn same_source(
        &self,
        other: &Self,
    ) -> bool {
        match (self, other) {
            (Self::Instance(a), Self::Instance(b)) => a == b,
            (Self::Static(a), Self::Static(b)) => Rc::ptr_eq(a, b),
            (Self::PendingStored(a), Self::PendingStored(b)) => Rc::ptr_eq(a, b),
            (
                Self::SelectedStored {
                    original: a,
                    target: at,
                },
                Self::SelectedStored {
                    original: b,
                    target: bt,
                },
            ) => Rc::ptr_eq(a, b) && at == bt,
            _ => false,
        }
    }
    /// Source-phase identity across the checked Pending -> Selected binding.
    /// This does not replace the selected target equality required for execution.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn same_original_source_v1(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::SelectedStored { original, target }, Self::PendingStored(pending))
            | (Self::PendingStored(pending), Self::SelectedStored { original, target }) => {
                Rc::ptr_eq(original, pending) && self.bind_selected_v1(target).is_ok()
            }
            _ => self.same_source(other),
        }
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn pending_stored_v1(
        batch: &crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
        reference: &CallTargetReferenceV1,
        receiver: &StoredReceiverSourceV1,
        result: &LexicalCallSourceResultRequirementV1,
        owning_object_input: Option<Rc<super::super::super::result_class_claim::OwningObjectLocalInputSourceLoanV1>>,
    ) -> Result<Self, String> {
        let coverage = batch.ordinary_box_coverage();
        let parent = coverage
            .row_for(&receiver.parent_class)
            .ok()
            .flatten()
            .ok_or_else(|| super::super::freeze("borrowed-formal/incoming-parent-source"))?;
        let child = coverage
            .row_for(&receiver.child_class)
            .ok()
            .flatten()
            .ok_or_else(|| super::super::freeze("borrowed-formal/incoming-child-source"))?;
        let (field, _) = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::terminal_home::source_declared_field(
            constructors, parent, &reference.call_site, &receiver.field_name,
        ).map_err(|_| super::super::freeze("borrowed-formal/incoming-field-source"))?
            .ok_or_else(|| super::super::freeze("borrowed-formal/incoming-field-source"))?;
        let child = constructors
            .object_for(child)
            .map_err(|_| super::super::freeze("borrowed-formal/incoming-child-source"))?;
        if receiver.parent_new.as_ref().is_some_and(|(_, root)| !root.matches_definition(parent, field.object())) {
            return Err(super::super::freeze("borrowed-formal/incoming-local-new-source"));
        }
        if receiver.corroborate_constructor_origin_v1(batch, constructors, &reference.call_site)?
            .is_some_and(|pair| pair != (field, child)) {
            return Err(super::super::freeze("borrowed-formal/incoming-constructor-field-drift"));
        }
        Ok(Self::PendingStored(Rc::new(OriginalStoredIncomingV1 {
            reference: reference.clone(),
            receiver: receiver.clone(),
            result: result.clone(),
            owning_object_input,
            expected_receiver: LexicalInstanceCallReceiverV1::StoredOwnedChild {
                parent_binding: receiver.parent_binding,
                parent_site: receiver.parent_site.clone(),
                field,
                child,
            },
        })))
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn bind_selected_v1(
        &self,
        target: &super::super::LexicalInstanceCallSourceTargetV1,
    ) -> Result<Self, String> {
        let original = match self {
            Self::PendingStored(original) | Self::SelectedStored { original, .. } => original,
            _ => {
                return if self.instance() == Some(target) {
                    Ok(self.clone())
                } else {
                    Err(super::super::freeze(
                        "borrowed-formal/incoming-selected-drift",
                    ))
                }
            }
        };
        if CallTargetReferenceV1::from_target(target) != original.reference
            || target.receiver != original.expected_receiver
            || target.owning_object_input != original.owning_object_input
            || target.result_requirement != original.result
            || target.target().owner() != original.receiver.child_class.as_ref()
        {
            return Err(super::super::freeze(
                "borrowed-formal/incoming-selected-drift",
            ));
        }
        Ok(Self::SelectedStored {
            original: Rc::clone(original),
            target: target.clone(),
        })
    }
    /// Borrow the SAME original constructor step after exact stored selection.
    /// This proves source correspondence, not field liveness or construction completion.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn constructor_unit_source_v1(
        &self,
        target: &super::super::LexicalInstanceCallSourceTargetV1,
    ) -> Result<Option<&crate::mir::normal_callable_semantic_package::instance_construction::ConstructionChildCallSourceLoanV1>, String> {
        let original = match self {
            Self::SelectedStored { original, .. } => original,
            _ => return Ok(None),
        };
        let super::super::source::StoredReceiverSourceOriginV1::Constructor(loan) = &original.receiver.origin
            else { return Ok(None); };
        self.bind_selected_v1(target)?;
        let request = loan.call();
        let same_receiver = matches!(&target.receiver,
            LexicalInstanceCallReceiverV1::StoredOwnedChild { parent_binding, field, child, .. }
            if *parent_binding == request.receiver_binding() && *field == request.field() && *child == request.child());
        if !target.is_unit_discard_source_v1() || !same_receiver
            || request.site() != target.call_site() || request.receiver_site() != target.receiver_site()
            || !request.arguments().is_empty() || !target.argument_sites().is_empty()
            || loan.source().child_call_source_v1(target.call_site())?.as_ref() != Some(loan)
            || loan.source().constructor().is_none_or(|(_, owner)| *owner != target.call_site().owner()) {
            return Err(super::super::freeze("unit-call/constructor-source-drift"));
        }
        Ok(Some(loan))
    }

    /// An execution consumer may borrow only the already-selected original root.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn local_new_root_source_v1(
        &self, target: &super::super::LexicalInstanceCallSourceTargetV1,
    ) -> Result<Option<(u32, &Rc<crate::mir::normal_callable_semantic_package::ordinary_new_coseal::LocalNewRootSourceV1>)>, String> {
        let Self::SelectedStored { original, .. } = self else {
            return Err(super::super::freeze("stored-child/original-selected-source-missing"));
        };
        self.bind_selected_v1(target)?;
        let Some((slot, root)) = &original.receiver.parent_new else { return Ok(None); };
        if !matches!(original.receiver.origin, super::super::source::StoredReceiverSourceOriginV1::Ordinary)
            || root.binding() != original.receiver.parent_binding || root.class() != original.receiver.parent_class.as_ref()
            || target.stored_receiver().is_none_or(|(binding, site, field, _)| binding != root.binding()
                || site != &original.receiver.parent_site || field.object() != root.object()) {
            return Err(super::super::freeze("stored-child/original-root-drift"));
        }
        Ok(Some((*slot, root)))
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn unit_discard_source_v1(
        &self,
    ) -> Option<&super::super::borrowed_formal_result::original::OriginalReturnSourceLoanV1> {
        let requirement = match self {
            Self::Instance(row) | Self::SelectedStored { target: row, .. } => &row.result_requirement,
            Self::PendingStored(row) => &row.result,
            Self::Static(_) => return None,
        };
        match requirement {
            LexicalCallSourceResultRequirementV1::UnitDiscardSource(source) => Some(source),
            _ => None,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn object_return_sources(&self) -> Option<&[crate::mir::normal_callable_semantic_package::ordinary_new_coseal::result_class_claim::ObjectReturnCallQualificationV1]>{
        self.instance().and_then(|row| row.object_return_sources())
    }
    pub(in crate::mir::normal_callable_semantic_package) fn object_source_forwards(
        &self,
    ) -> Option<&[super::super::borrowed_formal_result::ForwardIdentityV1]> {
        self.instance().and_then(|row| row.object_source_forwards())
    }
    pub(in crate::mir::normal_callable_semantic_package) fn object_producer_dependencies(&self) -> Option<&[crate::mir::normal_callable_semantic_package::ordinary_new_coseal::result_class_claim::ObjectReturnCallQualificationV1]>{
        self.instance()
            .and_then(|row| row.object_producer_dependencies())
    }
    pub(in crate::mir::normal_callable_semantic_package) fn stored_receiver(
        &self,
    ) -> Option<(
        BindingRefV1,
        &SourceExprSiteV1,
        hakorune_mir_defs::CanonicalFieldRefV1,
        hakorune_mir_defs::CanonicalObjectIdV1,
    )> {
        self.instance().and_then(|row| row.stored_receiver())
    }
}
