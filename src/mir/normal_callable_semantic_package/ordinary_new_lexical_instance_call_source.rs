//! Source-target preparation inside the existing lexical-call owner.
//! Each per-need result retains declaration/source order until final issuance.
use super::*;
use crate::mir::normal_callable_semantic_package::original_call_source::OriginalCallSourceCallerV1;
use std::collections::BTreeMap;

pub(in crate::mir::normal_callable_semantic_package) type PreparedLexicalInstanceCallSourceTargetsV1 =
    Result<Vec<Result<Option<LexicalInstanceCallSourceTargetV1>, String>>, String>;

/// Immutable source-target relation shared by preflight and final issuance.
/// Result, completion, ABI adoption and affine consumption are not issued here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LexicalInstanceCallSourceTargetV1 {
    pub(super) call_site: OwnedExprSiteV1,
    pub(super) receiver_site: SourceExprSiteV1,
    pub(super) receiver: LexicalInstanceCallReceiverV1,
    pub(super) target: CanonicalSameModuleCallableKeyV1,
    pub(super) target_batch_slot: u32,
    /// Owner of the callee declaration the target resolves to — the
    /// terminal-relation lookup key for the co-sealed result contract.
    pub(super) callee_owner: FunctionOwnerIdV1,
    /// Exact non-receiver argument sites in source order.
    pub(super) argument_sites: Box<[SourceExprSiteV1]>,
    pub(super) owning_object_input: Option<std::rc::Rc<super::super::result_class_claim::OwningObjectLocalInputSourceLoanV1>>,
    pub(super) borrowed_receiver_source: Option<Box<BorrowedReceiverSourceV1>>,
    pub(super) result_requirement: LexicalCallSourceResultRequirementV1,
}

/// The acquisition origin stays separate from receiver class and execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BorrowedReceiverSourceV1 {
    OrdinaryCall(super::super::result_class_claim::BorrowedNullableLocalSourceLoanV1),
    DirectMapGet(DirectMapReceiverSourceV1),
}
impl BorrowedReceiverSourceV1 {
    pub(super) fn destination(&self) -> BindingRefV1 {
        match self { Self::OrdinaryCall(source) => source.destination(), Self::DirectMapGet(source) => source.binding }
    }
    pub(super) fn acquisition(&self) -> &OwnedExprSiteV1 {
        match self { Self::OrdinaryCall(source) => source.call(), Self::DirectMapGet(source) => &source.initializer }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DirectMapReceiverSourceV1 {
    source: super::super::StoredMapGetSourceLoanV1,
    declaration: crate::mir::resolved_semantics::SourceBindingSiteV1,
    binding: BindingRefV1,
    initializer: OwnedExprSiteV1,
}
impl DirectMapReceiverSourceV1 {
    pub(super) fn issue(
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        binding: BindingRefV1, source: super::super::StoredMapGetSourceLoanV1,
    ) -> Option<Self> {
        if !super::super::result_class_claim::binding_is_unrebound(input.function(), binding) { return None; }
        let row = input.function().expression_source().sole_initializer(binding)?;
        let initializer = row.initializer_site()?.clone();
        if binding.owner() != input.owner()
            || source.get().owner() != input.owner() || source.get().site() != &initializer
            || !input.function().method_call_source_v1(&initializer).is_some_and(|get| std::sync::Arc::ptr_eq(get, source.get()))
            || !input.forest().callable_source_ledger(input.owner()).ok()?.uncaptured_binding(binding) { return None; }
        Some(Self { source, declaration: row.declaration_site().clone(), binding,
            initializer: OwnedExprSiteV1::new(input.owner(), initializer) })
    }
    pub(crate) fn source(&self) -> &super::super::StoredMapGetSourceLoanV1 { &self.source }
    pub(crate) fn matches_original_v1(&self,
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    ) -> bool {
        input.owner() == self.initializer.owner()
            && Self::issue(input, self.binding, self.source.clone()).as_ref() == Some(self)
    }
}
impl LexicalInstanceCallSourceTargetV1 {
    pub(crate) fn has_borrowed_receiver_source_v1(&self) -> bool { self.borrowed_receiver_source.is_some() }
    pub(crate) fn borrowed_call_receiver_source_v1(&self) -> Option<&super::super::result_class_claim::BorrowedNullableLocalSourceLoanV1> {
        match self.borrowed_receiver_source.as_deref()? { BorrowedReceiverSourceV1::OrdinaryCall(source) => Some(source), _ => None }
    }
    pub(crate) fn direct_map_receiver_source_v1(&self) -> Option<&DirectMapReceiverSourceV1> {
        match self.borrowed_receiver_source.as_deref()? { BorrowedReceiverSourceV1::DirectMapGet(source) => Some(source), _ => None }
    }
    pub(crate) fn owning_object_local_input_v1(&self) -> Option<&super::super::result_class_claim::OwningObjectLocalInputSourceLoanV1> {
        self.owning_object_input.as_deref()
    }

    pub(crate) fn mixed_local_source(&self) -> Option<&super::super::result_class_claim::MixedLocalCallSourceLoanV1> {
        self.result_requirement.mixed_local_source()
    }
    pub(crate) fn is_unit_discard_source_v1(&self) -> bool {
        matches!(self.result_requirement, LexicalCallSourceResultRequirementV1::UnitDiscardSource(_))
    }
    pub(crate) fn has_scalar_return_source_v1(&self) -> bool {
        matches!(self.result_requirement, LexicalCallSourceResultRequirementV1::ScalarReturnSource(_))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn uses_existing_borrowed_result_v1(
        &self,
    ) -> bool {
        matches!(
            self.result_requirement,
            LexicalCallSourceResultRequirementV1::ExistingBorrowedResult
            | LexicalCallSourceResultRequirementV1::ScalarReturnSource(_)
        )
    }
    pub(crate) fn has_object_source_requirement(&self) -> bool {
        self.result_requirement.has_object_source_requirement()
    }
    pub(crate) fn borrowed_nullable_local_source(
        &self,
    ) -> Option<&super::super::result_class_claim::BorrowedNullableLocalSourceLoanV1> {
        self.result_requirement.borrowed_nullable_local_source()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn object_producer_dependencies(
        &self,
    ) -> Option<&[super::super::result_class_claim::ObjectReturnCallQualificationV1]> {
        self.result_requirement.object_producer_dependencies()
    }
    pub(crate) fn object_return_sources(
        &self,
    ) -> Option<&[super::super::result_class_claim::ObjectReturnCallQualificationV1]> {
        self.result_requirement.object_return_sources()
    }

    pub(in crate::mir::normal_callable_semantic_package) fn object_source_forwards(
        &self,
    ) -> Option<&[borrowed_formal_result::ForwardIdentityV1]> {
        match &self.result_requirement {
            LexicalCallSourceResultRequirementV1::ObjectReturnSource { forwards, .. } => {
                forwards.as_deref()
            }
            LexicalCallSourceResultRequirementV1::ExistingBorrowedResult
            | LexicalCallSourceResultRequirementV1::UnitDiscardSource(_)
            | LexicalCallSourceResultRequirementV1::ScalarReturnSource(_)
            | LexicalCallSourceResultRequirementV1::BorrowedNullableLocal(_)
            | LexicalCallSourceResultRequirementV1::MixedLocal(_)
            | LexicalCallSourceResultRequirementV1::ObjectProducerDependency(_) => None,
        }
    }

    pub(crate) fn call_site(&self) -> &OwnedExprSiteV1 {
        &self.call_site
    }

    pub(crate) fn receiver_site(&self) -> &SourceExprSiteV1 {
        &self.receiver_site
    }

    /// `me.<name>` coverage row — never the local/parameter lane.
    pub(crate) fn is_self_receiver(&self) -> bool {
        matches!(
            self.receiver,
            LexicalInstanceCallReceiverV1::SelfReceiver(_)
        )
    }

    pub(crate) fn receiver_binding(&self) -> Result<BindingRefV1, String> {
        match self.receiver {
            LexicalInstanceCallReceiverV1::Lexical(binding)
            | LexicalInstanceCallReceiverV1::SelfReceiver(binding) => Ok(binding),
            LexicalInstanceCallReceiverV1::StoredOwnedChild { .. } => Err(freeze(
                "lexical-instance-call/stored-receiver-outside-terminal",
            )),
        }
    }

    pub(crate) fn stored_receiver(
        &self,
    ) -> Option<(
        BindingRefV1,
        &SourceExprSiteV1,
        hakorune_mir_defs::CanonicalFieldRefV1,
        hakorune_mir_defs::CanonicalObjectIdV1,
    )> {
        match &self.receiver {
            LexicalInstanceCallReceiverV1::StoredOwnedChild {
                parent_binding,
                parent_site,
                field,
                child,
            } => Some((*parent_binding, parent_site, *field, *child)),
            LexicalInstanceCallReceiverV1::Lexical(_)
            | LexicalInstanceCallReceiverV1::SelfReceiver(_) => None,
        }
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn matches_receiver(
        &self,
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        call: &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
    ) -> bool {
        if call.receiver_site() != self.receiver_site() {
            return false;
        }
        match &self.receiver {
            LexicalInstanceCallReceiverV1::Lexical(binding)
            | LexicalInstanceCallReceiverV1::SelfReceiver(binding) => call.receiver()
                == ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(*binding)),
            LexicalInstanceCallReceiverV1::StoredOwnedChild { parent_binding, parent_site, .. } => {
                input.body_shape().is_some_and(|shape| {
                    matches!(shape.expression_shape(call.receiver_site()),
                        Some(BodyExpressionShapeV1::FieldAccess { object, .. }) if object == parent_site)
                    && stored_parent_matches_v1(input, parent_site, *parent_binding)
                })
            }
        }
    }

    pub(crate) fn target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.target
    }

    pub(crate) const fn target_batch_slot(&self) -> u32 {
        self.target_batch_slot
    }

    pub(crate) const fn callee_owner(&self) -> FunctionOwnerIdV1 {
        self.callee_owner
    }

    pub(crate) fn argument_sites(&self) -> &[SourceExprSiteV1] {
        &self.argument_sites
    }
}

/// Borrow the sole original target directly from its canonical preparation.
/// Borrowed-input inventory membership is not source-call membership.
pub(in crate::mir::normal_callable_semantic_package) fn original_lexical_target_at_v1<'a>(
    targets: &'a PreparedLexicalInstanceCallSourceTargetsV1, site: &OwnedExprSiteV1,
) -> Result<Option<&'a LexicalInstanceCallSourceTargetV1>, String> {
    let mut found = None;
    for row in targets.as_ref().map_err(Clone::clone)? {
        let Some(target) = row.as_ref().map_err(Clone::clone)? else { continue };
        if target.call_site() != site { continue; }
        if found.replace(target).is_some() { return Err(freeze("original-target/duplicate")); }
    }
    Ok(found)
}

/// A source-backed scalar result keeps its lexical argument contract when a
/// Dynamic candidate shares the call site.
pub(in crate::mir::normal_callable_semantic_package) fn lexical_scalar_result_source_owns_arguments_v1(
    targets: &PreparedLexicalInstanceCallSourceTargetsV1,
    site: &OwnedExprSiteV1,
) -> Result<bool, String> {
    Ok(original_lexical_target_at_v1(targets, site)?
        .is_some_and(LexicalInstanceCallSourceTargetV1::has_scalar_return_source_v1))
}

/// Passive receiver source facts. This is neither an owned-child lease nor inventory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::normal_callable_semantic_package) enum StoredReceiverSourceOriginV1 {
    Ordinary,
    Constructor(crate::mir::normal_callable_semantic_package::instance_construction::ConstructionChildCallSourceLoanV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::normal_callable_semantic_package) struct StoredReceiverSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) origin: StoredReceiverSourceOriginV1,
    pub(in crate::mir::normal_callable_semantic_package) parent_new: Option<(u32, std::rc::Rc<super::super::LocalNewRootSourceV1>)>,
    pub(in crate::mir::normal_callable_semantic_package) parent_binding: BindingRefV1,
    pub(in crate::mir::normal_callable_semantic_package) parent_site: SourceExprSiteV1,
    pub(in crate::mir::normal_callable_semantic_package) parent_class: Box<str>,
    pub(in crate::mir::normal_callable_semantic_package) field_name: Box<str>,
    pub(in crate::mir::normal_callable_semantic_package) child_class: Box<str>,
}

impl StoredReceiverSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn local_new_root_in_input_v1(
        &self, input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>, slot: u32,
    ) -> Result<Option<&std::rc::Rc<super::super::LocalNewRootSourceV1>>, String> {
        let Some((original_slot, root)) = &self.parent_new else { return Ok(None); };
        if !matches!(self.origin, StoredReceiverSourceOriginV1::Ordinary) || *original_slot != slot
            || root.binding() != self.parent_binding || root.class() != self.parent_class.as_ref() {
            return Err(freeze("stored-child/local-new-source-drift"));
        }
        root.corroborate_input(input, &self.parent_site)?;
        Ok(Some(root))
    }
    /// Original constructor identity only. Canonical field/child inventory and
    /// executable eligibility remain obligations of the existing consumers.
    pub(in crate::mir::normal_callable_semantic_package) fn corroborate_constructor_origin_v1(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
        site: &OwnedExprSiteV1,
    ) -> Result<Option<(hakorune_mir_defs::CanonicalFieldRefV1, hakorune_mir_defs::CanonicalObjectIdV1)>, String> {
        let StoredReceiverSourceOriginV1::Constructor(loan) = &self.origin else { return Ok(None); };
        let reject = || freeze("stored-child/constructor-source-drift");
        if self.parent_new.is_some() { return Err(reject()); }
        let (source_id, owner) = loan.source().constructor().ok_or_else(reject)?;
        let mut rows = constructors.rows().iter().filter(|row| row.source_id().same_as(source_id));
        let row = rows.next().ok_or_else(reject)?;
        let input = row.lowering_input(batch.source_ast())?;
        let request = loan.call();
        let call = input.function().method_call(site.site()).ok_or_else(reject)?;
        let key = OriginalCallSourceCallerV1::Constructor { row, input }
            .constructor_key_for_call(site, call)?.ok_or_else(reject)?;
        if rows.next().is_some() || *owner != site.owner()
            || !row.construction_source().as_ref().is_ok_and(|current| std::rc::Rc::ptr_eq(current, loan.source()))
            || loan.source().object() != row.object()
            || loan.source().child_call_source_v1(site)?.as_ref() != Some(loan)
            || request.site() != site || request.receiver_site() != call.receiver_site()
            || request.receiver_binding() != self.parent_binding
            || request.class() != self.child_class.as_ref() || key.owner() != self.parent_class.as_ref()
            || request.field().object() != row.object()
            || request.arguments().iter().ne(call.arguments().iter().map(|argument| argument.site()))
        { return Err(reject()); }
        let shape = input.body_shape().ok_or_else(reject)?;
        if !matches!(shape.expression_shape(call.receiver_site()),
            Some(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { object, field, .. })
                if object == &self.parent_site && field == &self.field_name)
            || !matches!(shape.expression_shape(&self.parent_site),
                Some(crate::mir::resolved_semantics::BodyExpressionShapeV1::Me {
                    receiver: crate::mir::resolved_semantics::BodyMeReceiverV1::Lexical(binding), .. })
                    if binding == &self.parent_binding)
        { return Err(reject()); }
        Ok(Some((request.field(), request.child())))
    }
}

/// Structure only: the original incoming obligation separately proves the root.
pub(super) fn stored_parent_matches_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1, binding: BindingRefV1,
) -> bool {
    binding.owner() == input.owner() && (input.function().variable_ref(site) == Some(ResolvedLexicalRefV1::Local(binding))
        || input.body_shape().is_some_and(|shape| matches!(shape.expression_shape(site),
            Some(BodyExpressionShapeV1::Me { receiver: BodyMeReceiverV1::Lexical(actual), .. }) if *actual == binding)))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) struct CallTargetReferenceV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) call_site: OwnedExprSiteV1,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) receiver_site: SourceExprSiteV1,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) target: CanonicalSameModuleCallableKeyV1,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) target_batch_slot: u32,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) callee_owner: FunctionOwnerIdV1,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) argument_sites: Box<[SourceExprSiteV1]>,
}

impl CallTargetReferenceV1 {
    /// Corroboration only: canonical preparation still owns target selection.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn matches_original_call_v1(
        &self, input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        call: &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
    ) -> bool {
        self.call_site.owner() == input.owner() && call.owner() == input.owner()
            && self.call_site.site() == call.site()
            && input.function().method_call(call.site()) == Some(call)
            && self.receiver_site == *call.receiver_site()
            && self.target.namespace() == SameModuleCallableNamespaceV1::InstanceBoxMethod
            && self.target.name() == call.selector() && self.target.arity() == call.arity()
            && self.argument_sites.len() == call.arguments().len()
            && self.argument_sites.iter().zip(call.arguments()).enumerate()
                .all(|(ordinal, (site, argument))| argument.ordinal() as usize == ordinal
                    && site == argument.site())
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn from_target(row: &LexicalInstanceCallSourceTargetV1) -> Self {
        Self {
            call_site: row.call_site.clone(), receiver_site: row.receiver_site.clone(),
            target: row.target.clone(), target_batch_slot: row.target_batch_slot,
            callee_owner: row.callee_owner, argument_sites: row.argument_sites.clone(),
        }
    }
}

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) enum PreparedSourceCallNeedV1 {
    Lexical(LexicalInstanceCallSourceTargetV1),
    Stored {
        reference: CallTargetReferenceV1,
        receiver: StoredReceiverSourceV1,
        owning_object_input: Option<std::rc::Rc<super::super::result_class_claim::OwningObjectLocalInputSourceLoanV1>>,
        result_requirement: LexicalCallSourceResultRequirementV1,
    },
}

impl PreparedSourceCallNeedV1 {
    pub(super) fn owning_object_input_v1(&self) -> Option<&super::super::result_class_claim::OwningObjectLocalInputSourceLoanV1> {
        match self { Self::Lexical(row) => row.owning_object_local_input_v1(),
            Self::Stored { owning_object_input, .. } => owning_object_input.as_deref() }
    }
    pub(super) fn set_owning_object_input_v1(&mut self, loan: std::rc::Rc<super::super::result_class_claim::OwningObjectLocalInputSourceLoanV1>) {
        match self { Self::Lexical(row) => row.owning_object_input = Some(loan),
            Self::Stored { owning_object_input, .. } => *owning_object_input = Some(loan) }
    }
    pub(super) fn reference(&self) -> CallTargetReferenceV1 {
        match self {
            Self::Lexical(row) => CallTargetReferenceV1::from_target(row),
            Self::Stored { reference, .. } => reference.clone(),
        }
    }
    pub(super) fn result_requirement(&self) -> &LexicalCallSourceResultRequirementV1 {
        match self {
            Self::Lexical(row) => &row.result_requirement,
            Self::Stored {
                result_requirement, ..
            } => result_requirement,
        }
    }
    pub(super) fn set_result_requirement(
        &mut self,
        requirement: LexicalCallSourceResultRequirementV1,
    ) {
        match self {
            Self::Lexical(row) => row.result_requirement = requirement,
            Self::Stored {
                result_requirement, ..
            } => *result_requirement = requirement,
        }
    }
    pub(super) fn stored(&self) -> Option<&StoredReceiverSourceV1> {
        match self {
            Self::Stored { receiver, .. } => Some(receiver),
            _ => None,
        }
    }
}

pub(super) type PreparedSourceNeedsV1 =
    Result<Vec<Result<Option<PreparedSourceCallNeedV1>, String>>, String>;

enum SourceReceiverNeedV1 {
    Lexical(LexicalInstanceCallNeedV1),
    Stored(Result<Option<PreparedSourceCallNeedV1>, String>),
}

pub(in crate::mir::normal_callable_semantic_package) fn prepare_lexical_source_targets_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    new_classes: &BTreeMap<OwnedExprSiteV1, Box<str>>,
    ordinary_box_names: &[Box<str>],
    field_write_claims: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    callable_result_classes: &super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    parameters: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    app_main: Option<AppMainSourceRefV1<'_>>,
    stored_receiver: &mut impl FnMut(
        OriginalCallSourceCallerV1<'_>,
        crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
    ) -> Result<Option<StoredReceiverSourceV1>, String>,
    constructors: Option<&crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1>,
    entries: Option<&crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1>,
) -> PreparedSourceNeedsV1 {
    let source = provenance::LexicalReceiverClassSourceV1::prepared(
        new_classes,
        ordinary_box_names,
        field_write_claims,
        callable_result_classes, parameters, app_main, constructors, entries,
    );
    let mut needs = Vec::new();
    let mut collect = |caller: OriginalCallSourceCallerV1<'_>,
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>| -> Result<(), String> {
                let owner = input.owner();
                for (site, call) in input.function().method_calls() {
                    let ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(
                        binding,
                    )) = call.receiver()
                    else {
                        let row = stored_receiver(caller, input, call).map(|proof| {
                            let receiver = proof?;
                            let (target, target_batch_slot) = unique_instance_target(
                                selected,
                                receiver.child_class.as_ref(),
                                call.selector(),
                                call.arity(),
                            )?;
                            let callee_owner = batch
                                .declarations()
                                .find(|row| row.batch_slot() == target_batch_slot)?
                                .owner();
                            Some(PreparedSourceCallNeedV1::Stored {
                                reference: CallTargetReferenceV1 {
                                    call_site: OwnedExprSiteV1::new(owner, site.clone()),
                                    receiver_site: call.receiver_site().clone(),
                                    target,
                                    target_batch_slot,
                                    callee_owner,
                                    argument_sites: call
                                        .arguments()
                                        .iter()
                                        .map(|argument| argument.site().clone())
                                        .collect(),
                                },
                                receiver,
                                owning_object_input: None,
                                result_requirement:
                                    LexicalCallSourceResultRequirementV1::ExistingBorrowedResult,
                            })
                        });
                        needs.push(SourceReceiverNeedV1::Stored(row));
                        continue;
                    };
                    let Some(slot) = caller.declaration_slot() else { continue; };
                    if binding.owner() != owner {
                        continue;
                    }
                    let Some(record) = input.function().binding(binding) else {
                        continue;
                    };
                    let (parameter_index, self_receiver) = match record.kind() {
                        BindingKindV1::Parameter { index } => (Some(index), false),
                        BindingKindV1::Local { .. } => (None, false),
                        // `me.<name>`: the receiver resolves to this
                        // callable's own box through the selected key —
                        // the same exact-target shape as a lexical local.
                        BindingKindV1::Receiver => (None, true),
                        _ => continue,
                    };
                    let rebound = input.function().assignment_targets().any(|(_, target)| {
                        matches!(
                            target,
                            ResolvedAssignmentTargetV1::BindingRebind(rebound)
                                if *rebound == binding
                        )
                    });
                    needs.push(SourceReceiverNeedV1::Lexical(LexicalInstanceCallNeedV1 {
                        owner,
                        callee_slot: slot,
                        call_site: site.clone(),
                        receiver_site: call.receiver_site().clone(),
                        receiver_binding: binding,
                        parameter_index,
                        self_receiver,
                        selector: call.selector().into(),
                        arity: call.arity(),
                        argument_sites: call
                            .arguments()
                            .iter()
                            .map(|argument| argument.site().clone())
                            .collect(),
                        rebound,
                    }));
                }
        Ok(())
    };
    for declaration in batch.declarations() {
        let slot = declaration.batch_slot();
        batch.with_lowering_input(slot, |input|
            collect(OriginalCallSourceCallerV1::Declaration { batch_slot: slot }, input))
            .map_err(|_| freeze("lexical-instance-call/batch-loan"))??;
    }
    if let Some(constructors) = constructors {
        for row in constructors.rows().iter().filter(|row| row.kind() == crate::parser::ConstructorSourceKindV1::Birth) {
            let input = row.lowering_input(batch.source_ast())?;
            collect(OriginalCallSourceCallerV1::Constructor { row, input }, input)?;
        }
    }

    Ok(needs
        .into_iter()
        .map(|need| {
            let SourceReceiverNeedV1::Lexical(need) = need else {
                let SourceReceiverNeedV1::Stored(row) = need else {
                    unreachable!()
                };
                return row;
            };
            (|| -> Result<Option<PreparedSourceCallNeedV1>, String> {
                let mut borrowed_receiver_source = None;
                let class = if need.self_receiver {
                    // `me.<name>` resolves to a method on the caller's own
                    // box — the caller declaration's selected key is the
                    // only authority for that box name.
                    let Some(SelectedNormalCallableKeyV1::Cataloged(key)) =
                        selected.key_for_batch_slot(need.callee_slot)
                    else {
                        return Ok(None);
                    };
                    if key.namespace() != SameModuleCallableNamespaceV1::InstanceBoxMethod {
                        return Ok(None);
                    }
                    key.owner().into()
                } else {
                    match need.parameter_index {
                        Some(index) => {
                            match source.prove_parameter_class(batch, selected, &need, index)? {
                                Some(class) => class,
                                None => return Ok(None),
                            }
                        }
                        None => match source.claim_local_class(batch, selected, &need)? {
                            Some((class, borrowed)) => { borrowed_receiver_source = borrowed; class },
                            None => return Ok(None),
                        },
                    }
                };
                // Missing or contradictory evidence leaves the call unarmed:
                // outside an armed loop it keeps the existing dynamic path, and
                // inside one the route coverage names the uncovered site. Only
                // structural corruption (batch loan, duplicate issuance) is a
                // hard freeze.
                if need.rebound {
                    return Ok(None);
                }
                let Some((target, target_batch_slot)) = unique_instance_target(
                    selected,
                    class.as_ref(),
                    need.selector.as_ref(),
                    need.arity,
                ) else {
                    return Ok(None);
                };
                let call_site = OwnedExprSiteV1::new(need.owner, need.call_site.clone());
                let Some(callee_owner) = batch
                    .declarations()
                    .find(|declaration| declaration.batch_slot() == target_batch_slot)
                    .map(|declaration| declaration.owner())
                else {
                    return Ok(None);
                };

                if let Some(loan) = &borrowed_receiver_source {
                    let interval = source.borrowed_receiver_interval_v1(batch, &need, target_batch_slot, loan)?;
                    if !interval {
                        return Ok(None);
                    }
                }
                let receiver = if need.self_receiver {
                    LexicalInstanceCallReceiverV1::SelfReceiver(need.receiver_binding)
                } else {
                    LexicalInstanceCallReceiverV1::Lexical(need.receiver_binding)
                };
                Ok(Some(PreparedSourceCallNeedV1::Lexical(
                    LexicalInstanceCallSourceTargetV1 {
                        call_site,
                        receiver_site: need.receiver_site,
                        receiver,
                        target,
                        target_batch_slot,
                        callee_owner,
                        argument_sites: need.argument_sites,
                        owning_object_input: None,
                        borrowed_receiver_source,
                        result_requirement:
                            LexicalCallSourceResultRequirementV1::ExistingBorrowedResult,
                    },
                )))
            })()
        })
        .collect())
}
