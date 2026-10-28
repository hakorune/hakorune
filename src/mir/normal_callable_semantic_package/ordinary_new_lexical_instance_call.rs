//! Lexical-receiver instance-call disposition for the bounded ordinary-New
//! cohort.
//!
//! The resolver owns the MethodCall, initializer, variable-ref and binding
//! facts. This issuer joins those facts with the already selected instance
//! declarations and their physical signatures. It admits three receiver
//! provenance shapes only:
//!
//! * claim-local receivers: `local x = new C()` then `x.m(...)` — the sole
//!   initializer carries the ordinary-new claim and therefore the class;
//! * parameter receivers: `m(self_box)` where every selector+arity candidate
//!   edge in the package passes an argument that carries exactly one
//!   ordinary-new claim class (the `map_argument_edge` caller->callee
//!   argument-provenance precedent);
//! * `me` receivers: `me.m(...)` resolves to a method on the caller's own
//!   box — the caller declaration's selected key is the sole authority for
//!   that box name, never an inference.
//!
//! It never resolves a name from MIR or C input, never consumes Dynamic
//! products, and never guesses a receiver class from declared-type spellings.

use super::OrdinaryNewClaimLedgerV1;
use crate::mir::builder::SelectedNormalCallableKeyV1;
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::instruction::InvokeCallResultKind;
use crate::mir::normal_callable_semantic_package::physical_signature::{
    PhysicalCallableLaneRoleV1, VerifiedCallablePhysicalSignatureCohortV1,
};
use crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingRefV1, BodyExpressionShapeV1, BodyMeReceiverV1, FunctionOwnerIdV1,
    OwnedExprSiteV1, ResolvedAssignmentTargetV1, ResolvedLexicalRefV1,
    ResolvedMethodCallReceiverSourceV1, SourceExprSiteV1,
};
use hakorune_mir_defs::{CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1};

/// Closed original receiver spelling; a stored child lends no owned Home.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LexicalInstanceCallReceiverV1 {
    Lexical(BindingRefV1),
    /// `me.<name>` — the receiver is the caller's own `Receiver` binding.
    /// ExistingBorrowedResult keeps its coverage/receiver protocol. Completed
    /// Object local acquisition rows route lifecycle groups through the same
    /// receiver-call owner and retain their original input/result packet.
    SelfReceiver(BindingRefV1),
    StoredOwnedChild {
        parent_binding: BindingRefV1,
        parent_site: SourceExprSiteV1,
        field: hakorune_mir_defs::CanonicalFieldRefV1,
        child: hakorune_mir_defs::CanonicalObjectIdV1,
    },
}

#[cfg(test)]
#[path = "ordinary_new_stored_child_receiver_tests.rs"]
mod stored_child_receiver_tests;

/// Source requirement only; result execution still needs its final handoff seal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LexicalCallSourceResultRequirementV1 {
    ExistingBorrowedResult,
    /// Passive original field Return request, retained even if proof fails.
    ScalarReturnSource(borrowed_formal_result::original::OriginalReturnSourceLoanV1),
    /// Original parameterless statement demand; source is not execution proof.
    UnitDiscardSource(borrowed_formal_result::original::OriginalReturnSourceLoanV1),
    /// Original borrowed LOCAL identity; never an owning Object requirement.
    BorrowedNullableLocal(Box<super::result_class_claim::BorrowedNullableLocalSourceLoanV1>),
    /// All original Fresh/Forward/Null outcomes; no owning-only permission.
    MixedLocal(Box<super::result_class_claim::MixedLocalCallSourceLoanV1>),
    ObjectProducerDependency(Box<[super::result_class_claim::ObjectReturnCallQualificationV1]>),
    ObjectReturnSource {
        qualifications: Box<[super::result_class_claim::ObjectReturnCallQualificationV1]>,
        /// Original source identities survive execution-profile selection.
        forwards: Option<Box<[borrowed_formal_result::ForwardIdentityV1]>>,
    },
}

#[derive(Debug)]
pub(crate) struct LexicalInstanceCallDispositionRowV1 {
    source: LexicalInstanceCallSourceTargetV1,
    // Outcome authorization remains on the final affine disposition only.
    result: Option<InvokeCallResultKind>,
    // Exact result facts retained by the original scalar terminal, not a new ABI grant.
    scalar_result: Option<BorrowedScalarResultSourceLoanV1>,
    unit_completion: Option<std::rc::Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>>,
    borrowed_completion: Option<std::rc::Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>>,
    object_packet: Option<object_slot::ObjectPacketSealV1>,
    mixed_result: Option<std::rc::Rc<super::completion_index::VerifiedMixedCallResultV1>>,
}

impl LexicalCallSourceResultRequirementV1 {
    pub(crate) fn mixed_local_source(&self) -> Option<&super::result_class_claim::MixedLocalCallSourceLoanV1> {
        if let Self::MixedLocal(loan) = self { Some(loan) } else { None }
    }
    pub(crate) fn has_object_source_requirement(&self) -> bool {
        matches!(
            self,
            LexicalCallSourceResultRequirementV1::ObjectProducerDependency(_)
                | LexicalCallSourceResultRequirementV1::ObjectReturnSource { .. }
        )
    }

    pub(crate) fn borrowed_nullable_local_source(
        &self,
    ) -> Option<&super::result_class_claim::BorrowedNullableLocalSourceLoanV1> {
        match self {
            LexicalCallSourceResultRequirementV1::BorrowedNullableLocal(loan) => Some(loan),
            _ => None,
        }
    }

    pub(in crate::mir::normal_callable_semantic_package) fn object_producer_dependencies(
        &self,
    ) -> Option<&[super::result_class_claim::ObjectReturnCallQualificationV1]> {
        match self {
            LexicalCallSourceResultRequirementV1::ObjectProducerDependency(rows) => Some(rows),
            _ => None,
        }
    }

    pub(crate) fn object_return_sources(
        &self,
    ) -> Option<&[super::result_class_claim::ObjectReturnCallQualificationV1]> {
        match self {
            LexicalCallSourceResultRequirementV1::ObjectReturnSource { qualifications, .. } => {
                Some(qualifications)
            }
            LexicalCallSourceResultRequirementV1::ExistingBorrowedResult
            | LexicalCallSourceResultRequirementV1::UnitDiscardSource(_)
            | LexicalCallSourceResultRequirementV1::ScalarReturnSource(_)
            | LexicalCallSourceResultRequirementV1::BorrowedNullableLocal(_)
            | LexicalCallSourceResultRequirementV1::MixedLocal(_)
            | LexicalCallSourceResultRequirementV1::ObjectProducerDependency(_) => None,
        }
    }
}

impl LexicalInstanceCallDispositionRowV1 {
    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn with_result_for_test(
        mut self,
        result: Option<InvokeCallResultKind>,
    ) -> Self {
        self.result = result;
        self
    }

    pub(crate) const fn source_target(&self) -> &LexicalInstanceCallSourceTargetV1 {
        &self.source
    }
    pub(crate) fn call_site(&self) -> &OwnedExprSiteV1 {
        self.source_target().call_site()
    }
    pub(crate) fn receiver_site(&self) -> &SourceExprSiteV1 {
        self.source_target().receiver_site()
    }
    pub(crate) fn receiver_binding(&self) -> Result<BindingRefV1, String> {
        self.source_target().receiver_binding()
    }
    pub(crate) fn target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        self.source_target().target()
    }
    pub(crate) const fn target_batch_slot(&self) -> u32 {
        self.source_target().target_batch_slot()
    }
    pub(crate) const fn callee_owner(&self) -> FunctionOwnerIdV1 {
        self.source_target().callee_owner()
    }
    pub(crate) fn argument_sites(&self) -> &[SourceExprSiteV1] {
        self.source_target().argument_sites()
    }
    pub(crate) const fn result(&self) -> Option<InvokeCallResultKind> {
        self.result
    }
    pub(crate) fn scalar_result_source_v1(&self) -> Option<&BorrowedScalarResultSourceLoanV1> {
        self.scalar_result.as_ref()
    }
}

#[derive(Debug)]
pub(crate) enum LexicalInstanceCallDispositionSlotV1 {
    SourcePending(LexicalInstanceCallSourceTargetV1),
    Ready(LexicalInstanceCallDispositionRowV1),
    Taken,
}

#[path = "ordinary_new_lexical_object_slot.rs"]
mod object_slot;
#[path = "ordinary_new_lexical_borrowed_result.rs"]
mod borrowed_result;
#[path = "ordinary_new_lexical_mixed_result.rs"]
mod mixed_result;
pub(super) use object_slot::has_original_forwarding_receiver_input_source_v1;

/// One callee-side call site whose lexical receiver still needs a class.
struct LexicalInstanceCallNeedV1 {
    owner: FunctionOwnerIdV1,
    /// Batch slot of the function that contains the call site.
    callee_slot: u32,
    call_site: SourceExprSiteV1,
    receiver_site: SourceExprSiteV1,
    receiver_binding: BindingRefV1,
    /// `Some(index)` for parameter receivers; `None` for claim-local
    /// receivers whose sole initializer proves the class.
    parameter_index: Option<u32>,
    /// `me` receivers: the class is the caller's own box, read from the
    /// caller declaration's selected key — never inferred.
    self_receiver: bool,
    selector: Box<str>,
    arity: u32,
    argument_sites: Box<[SourceExprSiteV1]>,
    /// Whether the receiver binding is rebound anywhere in the callee.
    rebound: bool,
}

#[path = "ordinary_new_lexical_instance_call_provenance.rs"]
mod provenance;

#[path = "ordinary_new_lexical_instance_call_source.rs"]
mod source;
pub(in crate::mir::normal_callable_semantic_package) use source::{
    lexical_scalar_result_source_owns_arguments_v1, original_lexical_target_at_v1,
    StoredReceiverSourceOriginV1, StoredReceiverSourceV1,
};
pub(crate) use source::LexicalInstanceCallSourceTargetV1;
pub(super) use source::{BorrowedReceiverSourceV1, DirectMapReceiverSourceV1};
pub(super) use source::{
    prepare_lexical_source_targets_v1, PreparedLexicalInstanceCallSourceTargetsV1,
    PreparedSourceCallNeedV1, CallTargetReferenceV1,
};

#[path = "ordinary_new_borrowed_static_argument.rs"]
mod borrowed_static_argument;

#[path = "ordinary_new_borrowed_formal_uses.rs"]
mod borrowed_formal_uses;

#[path = "ordinary_new_borrowed_formal_source.rs"]
mod borrowed_formal_source;
pub(in crate::mir::normal_callable_semantic_package) use borrowed_formal_source::ClosedOriginalCoreSourceV1;
pub(in crate::mir) use borrowed_formal_source::{SourceValueSumV1, SourceValueAlternativeV1};
pub(in crate::mir) use borrowed_formal_source::SourceValueClassFactsV1;

#[path = "ordinary_new_borrowed_formal_actuals.rs"]
mod borrowed_formal_actuals;
pub(in crate::mir::normal_callable_semantic_package) use borrowed_formal_actuals::recheck_pending_array_source_actuals_v1;
pub(in crate::mir::normal_callable_semantic_package) use borrowed_formal_actuals::prepare_dynamic_invocation_argument_actuals_v1;

#[path = "ordinary_new_borrowed_formal_result.rs"]
mod borrowed_formal_result;
pub(crate) use borrowed_formal_result::ResultPathFieldSourceLoanV1;
pub(super) use borrowed_formal_result::{
    required_scalar_source_walk_v1, source_scalar_result_arguments_v1, BorrowedI64ResultSourceV1,
};
pub(in crate::mir::normal_callable_semantic_package) use borrowed_formal_result::BorrowedResultCohortV1;
pub(crate) use borrowed_formal_result::{
    BorrowedScalarResultSourceLoanV1, CallableScalarResultSourceLoanV1, CallableResultSourceV1,
};

pub(in crate::mir::normal_callable_semantic_package) use borrowed_formal_result::{lend_integer_field_v1, IntegerFieldSourceLoanV1};
pub(in crate::mir::normal_callable_semantic_package) use borrowed_formal_result::{ExpressionFieldSourceLoanV1, ClosedNumericFieldSourceLoanV1, ExpressionObjectFieldSourceLoanV1};

#[path = "ordinary_new_borrowed_formal_profile.rs"]
mod profile;
pub(super) use profile::prepare_borrowed_profile_v1;
#[path = "ordinary_new_borrowed_nullable_local_arguments.rs"]
mod borrowed_nullable_local_arguments;
pub(super) use borrowed_nullable_local_arguments::arguments_v1 as borrowed_nullable_local_arguments_v1;
pub(super) use borrowed_nullable_local_arguments::mixed_arguments_v1 as mixed_local_arguments_v1;

#[path = "ordinary_new_borrowed_formal_entry.rs"]
mod borrowed_formal_entry;
pub(crate) use borrowed_formal_actuals::OrdinaryResultSourceLoanV1;
pub(super) use borrowed_formal_actuals::{
    corroborate_received_object_receiver_v1, issue_original_static_forwarded_actual_v1,
    prepare_borrowed_call_actuals_v1, bind_argument_sources_v1, complete_argument_expression_v1, stage_observed_call_actuals_v1, observed_argument_source_v1, observed_carrier_array_append_argument_source_v1, lend_expression_core_source_v1, lend_dynamic_entry_read_v1, lend_finite_entry_read_v1, lend_text_entry_read_v1, lend_field_array_text_entry_read_v1, project_current_owner_loop_scalar_source_v1,
    project_pending_borrowed_i64_arguments_v1,
    project_pending_current_owner_static_source_arguments_v1,
    project_pending_i64_result_arguments_v1, project_pending_instance_source_arguments_v1,
    project_pending_object_arguments_v1, project_pending_static_source_arguments_v1, source_ordinary_result_arguments_v1,
    project_pending_object_target_arguments_v1,
    received_field_alias_producer_arguments_v1, received_producer_arguments_v1, original_owned_call_target_v1,
    reject_borrowed_actuals_for_owner_v1,
    PendingBorrowedFormalActualsV1,
    VerifiedStaticForwardedActualV1,
};
pub(in crate::mir) use borrowed_formal_actuals::{
    BorrowedFormalActualSourceV1, PreparedBorrowedFormalActualV1,
};
pub(in crate::mir) use borrowed_formal_entry::BorrowedCompareCarrierOperandLoanV1;
pub(crate) use borrowed_formal_entry::BorrowedCompareIntegerLiteralLoanV1;
pub(in crate::mir) use borrowed_formal_entry::BorrowedCompareIntegerLiteralMaterializationV1;
pub(in crate::mir) use borrowed_formal_entry::BorrowedCompareMaterializationV1;
pub(crate) use borrowed_formal_entry::BorrowedCompareSourceLoanV1;
pub(super) use borrowed_formal_entry::BorrowedOrdinaryEntryPhysicalV1;
pub(in crate::mir) use borrowed_formal_entry::{BorrowedOrdinaryEntrySourceRefV1, BorrowedFormalDomainV1};
pub(crate) use borrowed_formal_entry::field_write_source::{BorrowedFieldWriteSourceLoanV1, FieldWriteSourceLoanV1};
pub(in crate::mir) use borrowed_formal_entry::array_set_source::BorrowedArraySetSourceLoanV1;
pub(in crate::mir) use borrowed_formal_entry::{
    BorrowedMulMaterializationV1, BorrowedMulSourceLoanV1,
};
#[cfg(test)]
pub(super) use {borrowed_formal_source::prepare_borrowed_formal_ingress_v1,
    borrowed_formal_actuals::{lend_argument_expression_source_v1, stage_borrowed_call_actuals_v1}};
pub(crate) use borrowed_formal_source::NamedArrayHolderSourceLoanV1;
pub(in crate::mir::normal_callable_semantic_package) use borrowed_formal_source::{ArrayElementSourceLoanV1, ArrayHolderLifecycleRecipeV1};
pub(super) use borrowed_formal_source::PreparedBorrowedFormalIngressV1;

pub(in crate::mir::normal_callable_semantic_package) use borrowed_formal_result::source_unit_discard_v1;

impl OrdinaryNewClaimLedgerV1 {
    /// Join lexical receiver provenance to selected `InstanceBoxMethod`
    /// targets for every method call in the batch. Parameter receivers are
    /// proven by caller edges whose argument carries exactly one claim class;
    /// claim-local receivers are proven by their sole initializer's claim.
    /// Missing, ambiguous, or contradictory evidence simply leaves the call
    /// unarmed — outside an armed loop the existing dynamic path still owns
    /// it, and inside one the route coverage names the uncovered site. Only
    /// structural corruption (batch loan failure, duplicate issuance) is a
    /// hard freeze here.
    pub(in crate::mir::normal_callable_semantic_package) fn issue_lexical_instance_call_dispositions(
        &mut self,
        _batch: &VerifiedResolvedCallableSemanticBatchV1,
        _selected: &VerifiedSelectedCallableBatchMapV1,
        signatures: &VerifiedCallablePhysicalSignatureCohortV1,
        results: &crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1,
    ) -> Result<(), String> {
        let prepared = self
            .lexical_source_targets
            .take()
            .ok_or_else(|| freeze("lexical-instance-call/missing-source-preparation"))??;
        if let Some(Ok(borrowed)) = &self.borrowed_formal_source {
            borrowed.corroborate_source_targets(&prepared)?;
        }
        // All callee completions precede dependency checks and affine issuance.
        // Pending result errors are still demanded by their selected profile.
        self.corroborate_borrowed_result_cohort_v1(&prepared, results);
        for source in prepared {
            let Some(source) = source? else {
                continue;
            };
            let target_batch_slot = source.target_batch_slot();
            let target = source.target().clone();
            let call_site = source.call_site().clone();
            let callee_owner = source.callee_owner();
            let borrowed_nullable = source.borrowed_nullable_local_source().is_some();
            let mixed = source.mixed_local_source().is_some();
            // Slot result law and holder/key lifetime must precede a packet.
            // A Map element receiver of a scalar-result call carries no result slot
            // law of its own: the callee's numeric-only field writes are checked
            // here, then the ordinary scalar disposition below applies.
            let direct_scalar = source.direct_map_receiver_source_v1().is_some()
                && self.lexical_i64_call_source(&call_site).is_some()
                && self.check_direct_map_numeric_effects_v1(&source).is_ok();
            let borrowed_receiver = source.has_borrowed_receiver_source_v1() && !direct_scalar;
            if borrowed_nullable || mixed || borrowed_receiver || self.requires_object_packet_v1(&source)? {
                // A selected Object local remains on its canonical route while
                // its packet is pending. The affine take owns that refusal;
                // missing readiness must not send it to a plain Call writer.
                if borrowed_nullable || mixed || borrowed_receiver
                    || (!source.is_self_receiver()
                        && (self.handle_call_source(&call_site).is_some()
                            || self.nullable_call_source(&call_site).is_some())) {
                    self.record_lifecycle_local_call_site(call_site.owner(), call_site.clone());
                }
                if self
                    .lexical_instance_calls
                    .borrow_mut()
                    .insert(
                        call_site,
                        LexicalInstanceCallDispositionSlotV1::SourcePending(source),
                    )
                    .is_some()
                {
                    return Err(freeze("lexical-instance-call/duplicate"));
                }
                continue;
            }
            let terminal_selected = self.terminal_lexical_call_selected_v1(&call_site);
            if terminal_selected {
                self.corroborate_terminal_lexical_result_v1(&source, results)?;
            }
            let Some(signature) = signatures.row(target_batch_slot) else {
                if terminal_selected {
                    return Err(freeze("lexical-instance-call/terminal-signature-missing"));
                }
                continue;
            };
            if signature.mode()
                != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod
                || signature.source_logical_arity() != target.arity()
                || signature.receiver_lane_count() != 1
                || signature.lanes().first().is_none_or(|lane| {
                    lane.index() != 0
                        || lane.role() != PhysicalCallableLaneRoleV1::InstanceReceiver
                })
            {
                if terminal_selected {
                    return Err(freeze("lexical-instance-call/terminal-signature-mismatch"));
                }
                continue;
            }
            // The caller-side scan and this co-seal are independent proofs
            // over the same sealed facts: a caller-minted Handle local-call
            // observation must be matched by a callee-side `Value(
            // Construction)` terminal, an unannotated declared result, and a
            // `callable_result_classes` claim; a caller-minted i64
            // observation must be matched by literal-return relations that
            // uniformly classify `I64`; a caller-minted nullable
            // observation must be matched by the callee's `NullableObject`
            // claim and an unannotated declared result — its mixed
            // null/construction exits never classify to one scalar kind.
            // Any half-sealed edge freezes.
            if source.is_unit_discard_source_v1() {
                self.issue_unit_discard_v1(source, results)?;
                continue;
            }
            // Canonical Completion owns the absence of a value result.
            // Never let an empty terminal set manufacture an I64 projection
            // for a Unit source used in a receiving/value context.
            if self.completion_for_owner(callee_owner).is_some_and(|completion| !completion.returns_value()) {
                return Err(freeze("lexical-instance-call/unit-value-use"));
            }
            let source_bool = self
                .borrowed_i64_results
                .get(&callee_owner)
                .and_then(|row| row.as_ref().ok())
                .is_some_and(|row| {
                    row.class == borrowed_formal_result::BorrowedResultClassV1::Bool
                });
            let retained_scalar = self.retained_terminal_scalar_result_v1(&source)?;
            let source_text = self.checked_local_instance_text_result_source_v1(&source)?;
            let source_nullable_text = self.checked_local_instance_nullable_text_result_source_v1(&source)?;
            let callee_result = if source_text {
                Some(InvokeCallResultKind::OwnedString)
            } else if source_nullable_text {
                Some(InvokeCallResultKind::DynamicCarrier)
            } else if source_bool {
                retained_scalar.as_ref().filter(|loan| loan.kind() == crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Bool)
                    .map(|_| InvokeCallResultKind::Bool)
            } else {
                super::super::direct_call_loan::lifecycle::uniform_call_result_kind(
                    self.terminal_relations_for_owner(callee_owner).into_iter(),
                )
            };
            if callee_result == Some(InvokeCallResultKind::Bool) {
                let loan = retained_scalar.as_ref()
                    .filter(|loan| loan.kind() == crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Bool)
                    .ok_or_else(|| freeze("lexical-instance-call/terminal-scalar-source-missing"))?;
                borrowed_formal_result::corroborate_source_scalar_result_loan_v1(
                    self.borrowed_formal_source
                        .as_ref()
                        .ok_or_else(|| freeze("scalar-loan/source-missing"))?,
                    &self.borrowed_i64_results,
                    loan,
                    &source,
                    results,
                )?;
            }
            let handle_observation = self.handle_call_source(&call_site).is_some();
            let scalar_observation = self.lexical_i64_call_source(&call_site).is_some();
            let nullable_observation = self.nullable_call_source(&call_site).is_some();
            let result = match (handle_observation, scalar_observation, callee_result) {
                (true, false, Some(InvokeCallResultKind::Handle))
                    if results
                        .row(target_batch_slot)
                        .and_then(|row| row.result())
                        .is_none()
                        && self.callable_result_class(&target).is_some() =>
                {
                    Some(InvokeCallResultKind::Handle)
                }
                (true, ..) => return Err(freeze("lexical-instance-call/handle-result-mismatch")),
                // The existing routing set also owns source-scalar
                // self calls. Other self receiver protocols keep their owner.
                (false, true, Some(kind @ (InvokeCallResultKind::I64 | InvokeCallResultKind::Bool))) => {
                    let self_scalar = source.is_self_receiver()
                        && self.lexical_i64_call_source(&call_site)
                            .and_then(|observation| observation.borrowed_scalar())
                            .is_some_and(|loan| loan.source() == &source
                                && retained_scalar.as_ref() == Some(loan)
                                && match loan.kind() {
                                    crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer => kind == InvokeCallResultKind::I64,
                                    crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Bool => kind == InvokeCallResultKind::Bool,
                                });
                    if self_scalar {
                        borrowed_formal_result::corroborate_source_scalar_result_loan_v1(
                            self.borrowed_formal_source.as_ref()
                                .ok_or_else(|| freeze("scalar-loan/source-missing"))?,
                            &self.borrowed_i64_results,
                            retained_scalar.as_ref().expect("checked same scalar loan"),
                            &source, results,
                        )?;
                    }
                    if !source.is_self_receiver() || self_scalar {
                        self.record_lifecycle_local_call_site(call_site.owner(), call_site.clone());
                    }
                    Some(kind)
                }
                (false, true, _) => {
                    return Err(freeze("lexical-instance-call/i64-result-mismatch"))
                }
                (false, false, _)
                    if nullable_observation
                        && results
                            .row(target_batch_slot)
                            .and_then(|row| row.result())
                            .is_none()
                        && matches!(
                            self.callable_result_classes.get(&target),
                            Some(
                                crate::mir::normal_callable_semantic_package::OrdinaryNewResultClassV1::NullableObject(_)
                            )
                        ) =>
                {
                    // The checked-release lane owes the lifecycle invoke:
                    // the `CallReceived` commit owns the received Home and
                    // the checked release, while the routed binding group
                    // keeps the producer inventory the finalized-call
                    // visitor and the exit cleanup both claim.
                    // A `me.<name>` row is coverage-only — the receiver-call
                    // lane already owns `local x = me.m(..)` emission and
                    // its own `CallReceived` bookkeeping.
                    if !source.is_self_receiver() {
                        self.record_lifecycle_local_call_site(
                            call_site.owner(),
                            call_site.clone(),
                        );
                    }
                    Some(InvokeCallResultKind::NullableHandle)
                }
                (false, false, _) if nullable_observation => {
                    return Err(freeze("lexical-instance-call/nullable-result-mismatch"))
                }
                (false, false, other) => other,
            };
            let scalar_result = retained_scalar;
            let mut rows = self.lexical_instance_calls.borrow_mut();
            if rows
                .insert(
                    call_site.clone(),
                    LexicalInstanceCallDispositionSlotV1::Ready(
                        LexicalInstanceCallDispositionRowV1 {
                            source,
                            result,
                            scalar_result,
                            unit_completion: None,
                            borrowed_completion: None,
                            object_packet: None, mixed_result: None,
                        },
                    ),
                )
                .is_some()
            {
                return Err(freeze("lexical-instance-call/duplicate"));
            }
        }
        self.validate_terminal_lexical_ready_v1()?;
        Ok(())
    }

    /// Whether a lexical instance-call row is still armed for one exact call
    /// site. The route probe uses this to classify covered loop items without
    /// consuming the disposition.
    pub(crate) fn lexical_instance_call_covered(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceExprSiteV1,
    ) -> bool {
        self.lexical_instance_calls
            .borrow()
            .get(&OwnedExprSiteV1::new(owner, site.clone()))
            .is_some_and(|slot| matches!(slot, LexicalInstanceCallDispositionSlotV1::Ready(_)))
    }

    /// Consume the armed disposition for one exact call site. Emission is
    /// one-shot per source site, mirroring the locator contract.
    pub(crate) fn take_lexical_instance_call(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceExprSiteV1,
    ) -> Result<Option<LexicalInstanceCallDispositionRowV1>, String> {
        let key = OwnedExprSiteV1::new(owner, site.clone());
        {
            let rows = self.lexical_instance_calls.borrow();
            if let Some(LexicalInstanceCallDispositionSlotV1::Ready(row)) = rows.get(&key) {
                if row.source_target().mixed_local_source().is_some() { self.corroborate_mixed_call_v1(row)?; }
            }
        }
        let mut rows = self.lexical_instance_calls.borrow_mut();
        let Some(slot) = rows.get_mut(&key) else {
            return Ok(None);
        };
        if let LexicalInstanceCallDispositionSlotV1::SourcePending(source) = slot {
            return Err(format!(
                "{}: call={:?}, target={:?}, callee={:?}",
                freeze("lexical-instance-call/source-pending"),
                source.call_site(),
                source.target(),
                source.callee_owner()
            ));
        }
        if let LexicalInstanceCallDispositionSlotV1::Ready(row) = slot {
            if row.result() == Some(InvokeCallResultKind::Unit) { self.corroborate_unit_discard_v1(row)?; }
            if row.source_target().borrowed_nullable_local_source().is_some() { self.corroborate_borrowed_nullable_call_v1(row)?; }
        }
        match std::mem::replace(slot, LexicalInstanceCallDispositionSlotV1::Taken) {
            LexicalInstanceCallDispositionSlotV1::Ready(row) => Ok(Some(row)),
            LexicalInstanceCallDispositionSlotV1::Taken => {
                Err(freeze("lexical-instance-call/already-taken"))
            }
            LexicalInstanceCallDispositionSlotV1::SourcePending(_) => {
                unreachable!("pending checked before consumption")
            }
        }
    }
}

/// The unique selected `InstanceBoxMethod` target for
/// (`owner_box`, `selector`, `arity`), when exactly one exists.
pub(in crate::mir::normal_callable_semantic_package) fn unique_instance_target(
    selected: &VerifiedSelectedCallableBatchMapV1,
    owner: &str,
    selector: &str,
    arity: u32,
) -> Option<(CanonicalSameModuleCallableKeyV1, u32)> {
    let matches = selected
        .keys()
        .filter_map(|selected_key| {
            let SelectedNormalCallableKeyV1::Cataloged(key) = selected_key else {
                return None;
            };
            (key.namespace() == SameModuleCallableNamespaceV1::InstanceBoxMethod
                && key.owner() == owner
                && key.name() == selector
                && key.arity() == arity)
                .then(|| {
                    selected
                        .batch_slot(selected_key)
                        .map(|slot| (key.clone(), slot))
                })
                .flatten()
        })
        .collect::<Vec<_>>();
    let [pair] = matches.as_slice() else {
        return None;
    };
    Some(pair.clone())
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/{reason}]")
}

#[cfg(test)]
#[path = "ordinary_new_lexical_source_requirement_tests.rs"]
mod source_requirement_tests;

#[path = "ordinary_new_borrowed_app_main_source.rs"]
mod app_main_source;
pub(in crate::mir) use app_main_source::{MainArgumentsSourceV1, MainArgumentKindV1};
pub(in crate::mir::normal_callable_semantic_package) use app_main_source::{
    borrow_app_main_source_v1, AppMainSourceRefV1, BorrowedAppMainSourceLoanV1,
};

pub(super) use borrowed_formal_entry::original_borrowed_instance_input_completion_needed_v1;

pub(crate) use borrowed_formal_actuals::{ArgumentExpressionSourceLoanV1, DynamicEntryReadV1, FiniteEntryReadV1};

pub(crate) use borrowed_formal_entry::BorrowedTextEntryReadRequirementV1;
pub(crate) use borrowed_formal_entry::BorrowedFormalEntryReadRequirementV1;
