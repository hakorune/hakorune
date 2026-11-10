//! One owned semantic package for the final parser callable source.
//!
//! The issuer resolves the complete callable batch once, projects parameter
//! demands, and publishes either a valid-unselected or exact selected Dynamic
//! projection before Builder effects begin. It owns no CFG, Completion
//! consumption, physical ABI, or fallback route.

mod core_method_source;
pub(crate) use core_method_source::{ExpressionCoreSourceLoanV1, FieldMapReceiverSourceV1};
#[cfg(test)]
mod core_method_source_tests;
#[cfg(test)]
pub(crate) mod named_array_emission_tests;
#[cfg(test)]
pub(crate) use core_method_source::test_witness::unconditional_test_rows;
pub(crate) use core_method_source::{
    validate_named_array_coverage, EmittedNamedArrayRequirementV1, FieldArrayAppendSourceLoanV1,
    NamedArrayEmissionCollectorV1, NamedArrayWriteEmissionPortV1, SelectedSourceCoreMethodCallV1,
};
mod declared_instance_locator;
#[cfg(test)]
mod direct_call_lifecycle_tests;
mod direct_call_loan;
#[cfg(test)]
mod direct_call_owner_loan_tests;
mod disposition_slot;
mod dynamic_admission;
mod install;
mod original_call_source;
pub(crate) use install::{
    OwnedArrayAppendSourceLoanV1, OwnedArrayLifecycleSourceLoanV1, OwnedArrayReadSourceLoanV1,
    PreparedOwnedArrayConstructionV1,
};
mod instance_construction;
mod instance_constructor_initialization;
mod instance_constructor_loan;
mod instance_constructor_non_escape;
pub(crate) use instance_constructor_initialization::{
    ConstructionInitKeyV1, ConstructionInitOriginV1,
};
mod instance_constructor_semantic;
pub(crate) use instance_constructor_semantic::provider_source::{
    PreparedProviderStaticArgumentV1, ProviderStaticArgumentSourceV1,
};
mod instance_entry_home;
pub(crate) use instance_construction::{
    ConstructionEligibilityV1, ConstructionPlanV1, ConstructionSourceStepV1,
    ConstructionSourceStructureV1, ConstructionStoreRhsV1, ConstructionStoreV1,
    ConstructionUnavailableV1, ProviderConstructionChildV1,
};
pub(crate) use instance_constructor_semantic::{
    BirthFormalContractV1, BirthFormalDeclarationClassV1, BirthFormalPhysicalDispositionV1,
    BirthFormalUseCoverageV1, VerifiedInstanceConstructorSemanticBatchV1,
};
pub(in crate::mir) use instance_entry_home::SourceEntryHomeIssueV1;
mod issuer;
mod loop_break_source;
pub(in crate::mir) use loop_break_source::LoopBreakSourcePackageLoanV1;
mod map_lifecycle_undertaking;
#[cfg(test)]
mod map_lifecycle_undertaking_tests;
mod map_read_fact;
#[cfg(test)]
mod map_read_fact_tests;
mod model;
mod ordinary_new_coseal;
pub(crate) use ordinary_new_coseal::OrdinaryResultSourceLoanV1;
pub(crate) use ordinary_new_coseal::RetainedSourceLoopProductV1;
pub(in crate::mir) use ordinary_new_coseal::SourceValueClassFactsV1;
pub(crate) use ordinary_new_coseal::{
    EntryFieldWriteSourceV1, OriginalFieldWriteSourceLoanV1, OwnedLocalFieldWriteSourceV1,
};
pub(in crate::mir) use ordinary_new_coseal::{MainArgumentKindV1, MainArgumentsSourceV1};
mod callee_result_kind;
mod physical_header;
mod physical_signature;
mod qualified_static_call_claim;
pub(in crate::mir) use qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
mod result_contract;
pub(crate) use result_contract::VerifiedCallableResultContractCohortV1;
mod s6c_child;
mod s6c_effects;
mod s6c_storage_header;
mod selected_mapping;

/// Physical source selection belongs to the requested artifact, not to a
/// parsed App Main found in a reusable package.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum PackagePhysicalSelectionV1 {
    AllSource,
    ClosedAppExecutable,
}

/// Opaque capability created only by the package-owned install bridge.  The
/// type name is visible to the Builder bridge, while its constructor and
/// fields remain private to this package family.
pub(in crate::mir) struct BuilderInstallTokenV1 {
    _private: (),
}

impl BuilderInstallTokenV1 {
    fn issue() -> Self {
        Self { _private: () }
    }
}

#[cfg(test)]
mod brand_catalog_tests;
#[cfg(test)]
pub(in crate::mir) use brand_catalog_tests::{
    issue_with_brand_catalog as issue_source_package_fixture_v1, STORED_BORROWED_LOCAL_SOURCE,
};
#[cfg(test)]
mod declared_instance_locator_tests;
#[cfg(test)]
mod instance_entry_home_tests;
#[cfg(test)]
#[path = "loop_break_source_tests.rs"]
mod loop_break_source_tests;
#[cfg(test)]
mod main_static_child_tests;
#[cfg(test)]
mod ordinary_new_bin_size_words_tests;
#[cfg(test)]
mod ordinary_new_emission_validation_tests;
#[cfg(test)]
#[path = "ordinary_new_result_claim_tests.rs"]
mod ordinary_new_result_claim_tests;
#[cfg(test)]
mod physical_header_tests;
#[cfg(test)]
mod physical_signature_tests;
#[cfg(test)]
mod resolved_selected_handoff_tests;
#[cfg(test)]
mod resolver_deferred_tests;
#[cfg(test)]
mod result_origin_tests;
#[cfg(test)]
mod s6c_child_tests;
#[cfg(test)]
mod tests;

pub(in crate::mir) use declared_instance_locator::{
    DeclaredInstanceCallLocatorScopeV1, DeclaredInstanceCallLocatorViewV1,
};
pub(crate) use direct_call_loan::{DirectCallDispositionLoansV1, DirectCallDispositionRowV1};
pub(in crate::mir) use install::LoopBreakSourcePackageTakeHandle;
pub(in crate::mir) use install::SelectedCallableSemanticRefV1;
pub(crate) use install::{
    InstalledNormalCallableSemanticPackageV1, NormalCallableSemanticPackageInstallIssueV1,
    NormalCallableSemanticPackagePortV1, ResolvedCallablePhysicalSignatureLoanV1,
    S6CCommonV2PreSessionLoanRefV1, SelectedCallableLoweringInputRefV1,
    SelectedCatalogedCallableLoweringInputV1,
};
pub(in crate::mir) use issuer::issue_normal_callable_semantic_package_with_brand_catalog_and_loop_policy_v1;
pub(in crate::mir) use issuer::issue_normal_callable_semantic_package_with_selection_v1;
#[cfg(test)]
pub(in crate::mir) use issuer::NormalCallableSemanticPackageIssueV1;
#[cfg(test)]
pub(in crate::mir) use issuer::{
    issue_normal_callable_semantic_package_v1,
    issue_normal_callable_semantic_package_with_brand_catalog_v1,
};
pub(in crate::mir) use map_lifecycle_undertaking::{
    MapLifecycleConsumerCapabilityV1, MapLifecycleOperationV1, MapLifecycleUndertakingV1,
};
pub(crate) use map_read_fact::{
    MapReadFactV1, MapReadFactsV1, MapReadOperandV1, MapReadOperationV1, MapReadResultClassV1,
};
#[cfg(test)]
pub(in crate::mir) use model::NormalCallableDynamicProjectionRefV1;
pub(crate) use model::{
    QualifiedReceiverCatalogTakeV1, VerifiedNormalCallableSemanticPackageV1,
    VerifiedQualifiedReceiverCatalogRelationV1,
};
pub(in crate::mir) use ordinary_new_coseal::BorrowedArraySetSourceLoanV1;
pub(in crate::mir) use ordinary_new_coseal::BorrowedCompareCarrierOperandLoanV1;
pub(in crate::mir) use ordinary_new_coseal::BorrowedCompareIntegerLiteralMaterializationV1;
pub(in crate::mir) use ordinary_new_coseal::BorrowedCompareMaterializationV1;
pub(crate) use ordinary_new_coseal::CanonicalArrayFieldReadSourceV1;
pub(in crate::mir) use ordinary_new_coseal::LoopEntryStaticI64SourceLoanV1;
pub(in crate::mir) use ordinary_new_coseal::LoopStaticSourceCallLoanV1;
pub(in crate::mir) use ordinary_new_coseal::LoopTailStaticI64SourceLoanV1;
pub(in crate::mir) use ordinary_new_coseal::TextFieldValueSourceV1;
pub(in crate::mir) use ordinary_new_coseal::VerifiedStaticLoopI64ResultSourceV1;
pub(in crate::mir) use ordinary_new_coseal::VerifiedStaticLoopPacketSourceV1;
pub(in crate::mir) use ordinary_new_coseal::VerifiedStaticLoopTaggedEntrySourceV1;
pub(crate) use ordinary_new_coseal::{
    BirthAbiHandoffV1, BorrowedCallResultSourceLoanV1, BorrowedCallSourceLoanV1,
    BorrowedCompareIntegerLiteralLoanV1, BorrowedCompareSourceLoanV1, BorrowedLifetimeAnchorV1,
    BorrowedNullableLocalSourceLoanV1, FinalizedBirthActualsV1, FinalizedRootResultAbiV1,
    FinalizedRootSourceHandoffV1, LexicalInstanceCallDispositionRowV1,
    LexicalInstanceCallSourceTargetV1, MapStoredResultSourceV1, MixedLocalCallSourceLoanV1,
    NewEmissionSelectionV1, NewEmissionUnavailableV1, ObjectReturnCallQualificationV1,
    OrdinaryNewAdmissionClaimV1, OrdinaryNewClaimLedgerV1, OrdinaryNewConstructorDispositionV1,
    OrdinaryNewResultClaimV1, OrdinaryNewResultClassV1, OrdinaryNewTrivialArgumentKindV1,
    OrdinaryNewTrivialArgumentV1, OwnedFieldChildKindV1, OwnedFieldChildV1,
    PreparedTerminalMapGetReturnV1, QualifiedFieldWriteReceiverV1,
    QualifiedStaticCallArgumentKindV1, ReceiverCallClassObservationV1, ResultExitOriginV1,
    ResultOriginWitnessV1, ResultValueOriginV1, ResultWitnessStepV1, RootCallDispositionV1,
    RootInstanceCallDispositionRowV1, StoredMapGetReturnSourceLoanV1, StoredMapGetSourceLoanV1,
    TerminalNullReturnSourceLoanV1, VerifiedOrdinaryNewBirthRecipeV1,
};
pub(in crate::mir) use ordinary_new_coseal::{
    BorrowedMulMaterializationV1, BorrowedMulSourceLoanV1,
};
pub(in crate::mir) use ordinary_new_coseal::{
    FinalizedObjectReturnOriginV1, FinalizedObjectReturnOutcomeV1,
};
pub(in crate::mir) use ordinary_new_coseal::{
    LocalFieldReadSourceV1, OriginalScalarAppendV1, StoredChildFieldSourceV1,
};
pub(crate) use ordinary_new_coseal::{
    PendingChildPhysicalValidationV1, PendingRootHomeExitV1, PendingSelectedStaticLoopViewV1,
    PreparedLoopStaticBodyRetentionV1, PreparedRootHomeGraphPlanV1, PreparedRootHomeOrderV1,
    PreparedRootLexicalCallBindingGroupV1, PreparedStaticTerminalPacketV1,
    PreparedStaticTerminalRootRouteV1, RootCleanupActionV1, RootHomeFaultSequencesV1,
    RootHomeReleaseOriginV1,
};
pub(in crate::mir) use ordinary_new_coseal::{
    PreparedLoopStaticBodyDetachedPacketV1, VerifiedLoopStaticBodyPrepacketV1,
    VerifiedLoopStaticBodyScalarSourceV1,
};
pub(crate) use physical_header::CallablePhysicalHeaderRefV1;
pub(crate) use physical_signature::{
    PhysicalCallableLaneRoleV1, PhysicalCallableLaneV1, PhysicalCallableSignatureRowRefV1,
};
pub(crate) use s6c_effects::VerifiedS6CPhysicalFunctionEffectsV1;

#[cfg(test)]
mod lexical_i64_local_call_tests;
#[cfg(test)]
mod local_field_read_claim_tests;
#[cfg(test)]
mod map_call_argument_flow_tests;
#[cfg(test)]
mod map_contained_descendant_flow_tests;
#[cfg(test)]
mod map_entry_value_flow_tests;
#[cfg(test)]
mod map_get_terminal_tests;
#[cfg(test)]
mod map_home_flow_tests;
#[cfg(test)]
mod map_value_completion_tests;
#[cfg(test)]
#[path = "provider_static_call_argument_tests.rs"]
mod provider_static_call_argument_tests;
#[cfg(test)]
mod qualified_static_call_claim_tests;
#[cfg(test)]
mod scalar_expression_claim_tests;
#[cfg(test)]
mod terminal_value_return_tests;

#[cfg(test)]
mod map_physical_dependency_tests;

pub(in crate::mir) use ordinary_new_coseal::{
    BorrowedFormalActualSourceV1, BorrowedFormalDomainV1, CallPacketSourceLoanV1,
    CallPacketSourceV1, EmittedLexicalCallProjectionV1, LexicalCallArgumentProjectionV1,
    PreparedBorrowedFormalActualV1, PreparedLexicalCallProjectionV1,
    PreparedSelectedStaticLoopCallProjectionV1, StoredReceiverFaultV1, StoredReceiverReadV1,
};

pub(in crate::mir) use ordinary_new_coseal::FinalizedLexicalCallContextV1;

pub(crate) use ordinary_new_coseal::NamedArrayHolderSourceLoanV1;
pub(crate) use ordinary_new_coseal::{
    BorrowedScalarResultSourceLoanV1, CallableResultSourceV1, CallableScalarResultSourceLoanV1,
    ResultPathFieldSourceLoanV1,
};

pub(crate) use core_method_source::{
    EmittedFieldArrayReadArgumentV1, EmittedFieldArrayReadV1, FieldArrayReadRecipeV1,
    FieldKeysCountRecipeV1,
};
pub(crate) use ordinary_new_coseal::{
    ArgumentFieldArrayReadSourceV1, CheckedFieldArrayAccessSourceV1,
};

pub(in crate::mir) use ordinary_new_coseal::EmittedDynamicInvocationCallProjectionV1;
#[cfg(test)]
pub(in crate::mir) use ordinary_new_coseal::ExpressionPhysicalProjectionRefV1;
pub(crate) use ordinary_new_coseal::{
    checked_expression_core_operation_v1, checked_scalar_invoke_operation_v1,
    FrameFreeExpressionReadV1,
};
pub(in crate::mir) use ordinary_new_coseal::{
    validate_expression_acquisition_prefix_v1, validate_expression_acquisition_scope_v1,
    BranchReadPhiObservationV1, BranchScalarPhiObservationV1, BranchTextPhiObservationV1,
    EvaluatedExpressionValueV1, ExpressionCallArgumentSourceV1, ExpressionCallTransientPrefixV1,
    ExpressionReferenceReadV1,
};
pub(crate) use ordinary_new_coseal::{
    EmittedBindingRebindProjectionV1, EmittedFieldArrayAppendV1,
    EmittedInitializerExpressionProjectionV1, EmittedOwnedArrayAppendV1,
    EmittedPrintExpressionProjectionV1, FieldAppendTextProjectionV1, OwnedArrayAppendCandidateV1,
    RootLifecyclePrefixBindingGroupV1,
};
pub(crate) use ordinary_new_coseal::{
    MapFieldInstallEmissionV1, PreparedMapFieldInstallV1, StoredMapGetLocalEmissionV1,
    StoredMapGetReadEmissionV1, StoredMapGetReturnEmissionV1,
};

pub(crate) use ordinary_new_coseal::EmittedOwnedArrayReadV1;
pub(crate) use ordinary_new_coseal::{EmittedReadArgumentV1, ReadActualSourceLoanV1};

pub(crate) use ordinary_new_coseal::EmittedOwnedArrayHolderV1;

pub(in crate::mir) use ordinary_new_coseal::{
    borrowed_source_loop_phi_relations_v2, BoolConditionCursorV1, BoolConditionNodeProjectionV1,
    BoolConditionSealV1, BoolConditionValueProofV1, EmittedBoolConditionProjectionV1,
    SourceLoopCleanupPlanV2, SourceLoopPhysicalProofV2,
};

pub(in crate::mir) use ordinary_new_coseal::CallValidationContextV1;

pub(crate) use ordinary_new_coseal::{BorrowedFieldWriteSourceLoanV1, FieldWriteSourceLoanV1};

pub(crate) use ordinary_new_coseal::{CheckedFieldWriteGraphV1, EmittedBorrowedFieldWriteV1};
pub(in crate::mir) use ordinary_new_coseal::{
    EmittedFieldWriteRhsV1, FieldWriteRhsNodeV1, FieldWriteRhsProofV1, FieldWriteValueV1,
};

pub(crate) use ordinary_new_coseal::{
    ArgumentExpressionSourceLoanV1, DynamicEntryReadV1, FiniteEntryReadV1,
};

pub(crate) use ordinary_new_coseal::BorrowedFormalEntryReadRequirementV1;
pub(crate) use ordinary_new_coseal::BorrowedTextEntryReadRequirementV1;

pub(crate) use ordinary_new_coseal::{
    displaced_replacement_fault_sequences_v2, fault_indices_after_normal_step_v2,
};

pub(in crate::mir) use ordinary_new_coseal::{SourceValueAlternativeV1, SourceValueSumV1};

pub(in crate::mir) use ordinary_new_coseal::{
    EmittedNewArgumentV1, EmittedNewExpressionV1, NewArgumentCleanupV1,
};

pub(in crate::mir) use ordinary_new_coseal::FinalizedBorrowedCallPacketV1;
