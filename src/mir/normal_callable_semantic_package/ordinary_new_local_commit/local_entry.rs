//! Shared physical Home lookup and local installation in the existing ledger.
use super::*;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) enum LocalCommitV1 {
    Ordinary(NewLocalCommitV1),
    Map(MapLocalProgress),
}
impl LocalCommitV1 {
    pub(super) fn ordinary(&self) -> Option<&NewLocalCommitV1> {
        match self {
            Self::Ordinary(row) => Some(row),
            Self::Map(_) => None,
        }
    }
    pub(super) fn ordinary_mut(&mut self) -> Option<&mut NewLocalCommitV1> {
        match self {
            Self::Ordinary(row) => Some(row),
            Self::Map(_) => None,
        }
    }
    pub(super) fn binding(&self) -> BindingRefV1 {
        match self {
            Self::Ordinary(row) => row.binding,
            Self::Map(row) => row.binding,
        }
    }
    pub(super) fn declaration(&self) -> &SourceBindingSiteV1 {
        match self {
            Self::Ordinary(row) => &row.declaration,
            Self::Map(row) => &row.declaration,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn is_complete(&self) -> bool {
        match self {
            Self::Ordinary(row) => row.is_complete(),
            Self::Map(row) => row.is_complete(),
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn installs(
        &self,
        binding: BindingRefV1,
    ) -> bool {
        match self {
            Self::Ordinary(row) => row.installs(binding),
            Self::Map(row) => row.binding == binding && row.local().is_some(),
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn installs_ordinary(
        &self,
        binding: BindingRefV1,
    ) -> bool {
        self.ordinary().is_some_and(|row| row.installs(binding))
    }
    pub(super) fn local(&self) -> Option<ValueId> {
        match self {
            Self::Ordinary(row) => row.emission.local(),
            Self::Map(row) => row.local(),
        }
    }
    pub(super) fn ordinary_object(&self) -> Option<CanonicalObjectIdV1> {
        self.ordinary().map(NewLocalCommitV1::object)
    }
    pub(super) fn initializer(&self) -> Option<ValueId> {
        match self {
            Self::Ordinary(row) => row.emission.completed_initializer(),
            Self::Map(row) => row.initializer(),
        }
    }
    pub(super) fn install(&mut self, local: ValueId) {
        match self {
            Self::Ordinary(row) => row.emission.install(local),
            Self::Map(row) => row.install(local),
        }
    }
    pub(super) fn end_available(&self) -> bool {
        match self {
            Self::Ordinary(row) => {
                row.destruction
                    == crate::mir::function::ObjectDestructionDispositionV1::PlainI64NoHook
                    && matches!(row.emission, NewEmissionProgress::Emitted { .. })
            }
            Self::Map(row) => row.local().is_some(),
        }
    }
    pub(super) fn end_operation(&self) -> InvokeOperation {
        match self {
            Self::Ordinary(row) => row.end_operation(),
            Self::Map(row) => {
                InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::End {
                    map: row.local().expect("installed Map"),
                })
            }
        }
    }
    pub(super) fn at_statement(&self, owner: FunctionOwnerIdV1, site: &SourceNodeSiteV1) -> bool {
        self.binding().owner() == owner
            && matches!(self.declaration(),
            SourceBindingSiteV1::Local { statement, .. } if statement.node() == site)
    }
    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn construction(
        &self,
    ) -> &crate::mir::normal_callable_semantic_package::ConstructionEligibilityV1 {
        self.ordinary().expect("ordinary test row").construction()
    }
}
