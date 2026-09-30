//! Shared physical Home lookup and local installation in the existing ledger.
use super::*;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) enum LocalCommitV1 {
    Ordinary(NewLocalCommitV1),
    Result(NewResultCommitV1),
    Map(MapLocalProgress),
    CallReceived(CallReceivedCommitV1),
}
impl LocalCommitV1 {
    pub(super) fn ordinary(&self) -> Option<&NewLocalCommitV1> {
        match self {
            Self::Ordinary(row) => Some(row),
            Self::Result(_) | Self::Map(_) | Self::CallReceived(_) => None,
        }
    }
    pub(super) fn ordinary_mut(&mut self) -> Option<&mut NewLocalCommitV1> {
        match self {
            Self::Ordinary(row) => Some(row),
            Self::Result(_) | Self::Map(_) | Self::CallReceived(_) => None,
        }
    }
    pub(super) fn result(&self) -> Option<&NewResultCommitV1> {
        match self {
            Self::Result(row) => Some(row),
            Self::Ordinary(_) | Self::Map(_) | Self::CallReceived(_) => None,
        }
    }
    pub(super) fn result_mut(&mut self) -> Option<&mut NewResultCommitV1> {
        match self {
            Self::Result(row) => Some(row),
            Self::Ordinary(_) | Self::Map(_) | Self::CallReceived(_) => None,
        }
    }
    /// Shared emission-state access for ordinary and result rows — the
    /// physical emit/record/complete path is one, only the destination
    /// handling differs.
    pub(super) fn new_emission(&self) -> Option<&NewEmissionProgress> {
        match self {
            Self::Ordinary(row) => Some(&row.emission),
            Self::Result(row) => Some(&row.emission),
            Self::Map(_) | Self::CallReceived(_) => None,
        }
    }
    pub(super) fn new_emission_mut(&mut self) -> Option<&mut NewEmissionProgress> {
        match self {
            Self::Ordinary(row) => Some(&mut row.emission),
            Self::Result(row) => Some(&mut row.emission),
            Self::Map(_) | Self::CallReceived(_) => None,
        }
    }
    pub(super) fn new_box_source(
        &self,
    ) -> Option<&crate::parser::ParserOrdinaryBoxSourceRowV1> {
        match self {
            Self::Ordinary(row) => Some(&row.box_source),
            Self::Result(row) => Some(&row.box_source),
            Self::Map(_) | Self::CallReceived(_) => None,
        }
    }
    pub(super) fn new_argument_rows(
        &self,
    ) -> Option<
        &Result<
            Box<[super::super::OrdinaryNewTrivialArgumentV1]>,
            crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentUnavailableV1,
        >,
    > {
        match self {
            Self::Ordinary(row) => Some(&row.argument_rows),
            Self::Result(row) => Some(&row.argument_rows),
            Self::Map(_) | Self::CallReceived(_) => None,
        }
    }
    pub(super) fn new_children(
        &self,
    ) -> Option<&Option<Box<[super::super::OwnedFieldChildV1]>>> {
        match self {
            Self::Ordinary(row) => Some(&row.children),
            Self::Result(row) => Some(&row.children),
            Self::Map(_) | Self::CallReceived(_) => None,
        }
    }
    pub(super) fn binding(&self) -> Option<BindingRefV1> {
        match self {
            Self::Ordinary(row) => Some(row.binding),
            // A returned construction installs no local destination.
            Self::Result(_) => None,
            Self::Map(row) => row.binding,
            Self::CallReceived(row) => Some(row.binding),
        }
    }
    pub(super) fn owner(&self) -> FunctionOwnerIdV1 {
        match self {
            Self::Ordinary(row) => row.binding.owner(),
            Self::Result(row) => row.site.owner(),
            Self::Map(row) => row.owner,
            Self::CallReceived(row) => row.owner,
        }
    }
    pub(super) fn declaration(&self) -> Option<&SourceBindingSiteV1> {
        match self {
            Self::Ordinary(row) => Some(&row.declaration),
            Self::Result(_) => None,
            Self::Map(row) => row.declaration.as_ref(),
            Self::CallReceived(row) => Some(&row.declaration),
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn is_complete(&self) -> bool {
        match self {
            Self::Ordinary(row) => row.is_complete(),
            Self::Result(row) => row.is_complete(),
            Self::Map(row) => row.is_complete(),
            Self::CallReceived(row) => row.is_complete(),
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn installs(
        &self,
        binding: BindingRefV1,
    ) -> bool {
        match self {
            Self::Ordinary(row) => row.installs(binding),
            Self::Result(_) => false,
            Self::Map(row) => row.binding == Some(binding) && row.local().is_some(),
            Self::CallReceived(row) => {
                row.binding == binding && row.local().is_some()
            }
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
            // `Checked { local }` on a result row marks the emitted object
            // value itself; no physical local binding exists to expose.
            Self::Result(_) => None,
            Self::Map(row) => row.local(),
            Self::CallReceived(row) => row.local(),
        }
    }
    pub(super) fn ordinary_object(&self) -> Option<CanonicalObjectIdV1> {
        self.ordinary().map(NewLocalCommitV1::object)
    }
    pub(super) fn initializer(&self) -> Option<ValueId> {
        match self {
            Self::Ordinary(row) => row.emission.completed_initializer(),
            Self::Result(_) => None,
            Self::Map(row) => row.initializer(),
            Self::CallReceived(row) => row.initializer(),
        }
    }
    pub(super) fn install(&mut self, local: ValueId) {
        match self {
            Self::Ordinary(row) => row.emission.install(local),
            Self::Result(_) => {
                let _ = local;
                unreachable!("result-position `new` claims install no local")
            }
            Self::Map(row) => row.install(local),
            Self::CallReceived(row) => row.install(local),
        }
    }
    pub(super) fn end_available(&self) -> bool {
        match self {
            Self::Ordinary(row) => {
                let destruction_ok = match row.destruction {
                    crate::mir::function::ObjectDestructionDispositionV1::PlainI64NoHook => true,
                    // An owned-field teardown is admissible only with the
                    // sealed per-field residence proof; `None` never degrades
                    // to a plain release.
                    crate::mir::function::ObjectDestructionDispositionV1::OwnedArrayFieldsNoHook
                    | crate::mir::function::ObjectDestructionDispositionV1::OwnedObjectFieldsNoHook => {
                        row.children.is_some()
                    }
                    crate::mir::function::ObjectDestructionDispositionV1::Unavailable(_) => false,
                };
                destruction_ok && matches!(row.emission, NewEmissionProgress::Emitted { .. })
            }
            // Ownership leaves with the caller; the callee never ends the
            // returned object.
            Self::Result(_) => false,
            Self::Map(row) => row.local().is_some(),
            // The caller received an owned handle; it owes one release.
            // `Some(None)` children mean an owned object without residence
            // proof — never releasable here.
            Self::CallReceived(row) => {
                row.local().is_some() && !matches!(row.end_children, Some(None))
            }
        }
    }
    /// Teardown plan in source-issued execution order: owned field
    /// residences in reverse declaration order, then the home's own
    /// release. Each pair carries the root-exit origin subject so prepare
    /// and validation never re-derive the plan from MIR.
    pub(super) fn end_plan(
        &self,
    ) -> Box<[(super::root_home::RootHomeReleaseSubjectV1, InvokeOperation)]> {
        match self {
            Self::Ordinary(row) => row.end_plan(),
            Self::Result(_) => unreachable!("returned object has no caller-side `End` here"),
            Self::Map(row) => {
                let map = row.local().expect("installed Map");
                Box::new([(
                    super::root_home::RootHomeReleaseSubjectV1::Binding(
                        row.binding.expect("installed Map binding"),
                    ),
                    InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::End { map }),
                )])
            }
            Self::CallReceived(row) => row.end_plan(),
        }
    }
    /// The emitted operations alone — fault-unwind chains and map prefixes
    /// consume the same plan without origin subjects.
    pub(super) fn end_operations(&self) -> Vec<InvokeOperation> {
        self.end_plan()
            .into_vec()
            .into_iter()
            .map(|(_, operation)| operation)
            .collect()
    }
    pub(super) fn at_statement(&self, owner: FunctionOwnerIdV1, site: &SourceNodeSiteV1) -> bool {
        self.owner() == owner
            && matches!(self.declaration(),
            Some(SourceBindingSiteV1::Local { statement, .. }) if statement.node() == site)
    }
    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn construction(
        &self,
    ) -> &crate::mir::normal_callable_semantic_package::ConstructionEligibilityV1 {
        match self {
            Self::Ordinary(row) => row.construction(),
            Self::Result(row) => row.construction(),
            Self::Map(_) => panic!("map row has no construction eligibility"),
            Self::CallReceived(_) => {
                panic!("call-received row has no construction eligibility")
            }
        }
    }
}
