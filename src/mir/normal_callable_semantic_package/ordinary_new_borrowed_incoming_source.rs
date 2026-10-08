//! Owned incoming identity preserves the original instance or static source kind.
//! Common coordinates never grant a receiver, object authority or executable ABI.
use super::*;
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::QualifiedStaticIncomingSourceV1;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(in crate::mir::normal_callable_semantic_package) enum BorrowedIncomingSourceV1 {
    Instance(super::super::LexicalInstanceCallSourceTargetV1),
    QualifiedStatic(Rc<QualifiedStaticIncomingSourceV1>),
}

impl BorrowedIncomingSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn as_loan(
        &self,
    ) -> BorrowedCallSourceLoanV1<'_> {
        match self {
            Self::Instance(row) => BorrowedCallSourceLoanV1::Instance(row),
            Self::QualifiedStatic(row) => BorrowedCallSourceLoanV1::QualifiedStatic(row),
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn instance(
        &self,
    ) -> Option<&super::super::LexicalInstanceCallSourceTargetV1> {
        match self {
            Self::Instance(row) => Some(row),
            Self::QualifiedStatic(_) => None,
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
            Self::Instance(row) => row,
            Self::QualifiedStatic(_) => panic!("Instance mutation requires Instance source"),
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn call_site(&self) -> &OwnedExprSiteV1 {
        match self {
            Self::Instance(row) => row.call_site(),
            Self::QualifiedStatic(row) => row.call_site(),
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn target(
        &self,
    ) -> &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1 {
        match self {
            Self::Instance(row) => row.target(),
            Self::QualifiedStatic(row) => row.target(),
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn callee_owner(
        &self,
    ) -> FunctionOwnerIdV1 {
        match self {
            Self::Instance(row) => row.callee_owner(),
            Self::QualifiedStatic(row) => row.callee_owner(),
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn target_batch_slot(&self) -> u32 {
        match self {
            Self::Instance(row) => row.target_batch_slot(),
            Self::QualifiedStatic(row) => row.target_batch_slot(),
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn argument_sites(
        &self,
    ) -> &[SourceExprSiteV1] {
        match self {
            Self::Instance(row) => row.argument_sites(),
            Self::QualifiedStatic(row) => row.argument_sites(),
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
