/*!
 * MIR Function and Module - High-level MIR organization
 *
 * Functions contain basic blocks and SSA values, modules contain functions.
 * This parent module keeps the public surface stable while thinner child
 * modules own data definitions, behavior, display helpers, and tests.
 */

mod display;
mod dynamic_metadata_observation;
mod dynamic_v2_aot_metadata_slot;
mod facts;
mod fastmem;
mod function_impl;
mod metadata;
mod module_impl;
mod object_metadata;
mod typed_array_contract;
mod types;

#[cfg(test)]
mod tests;

pub(crate) use dynamic_metadata_observation::DynamicV2MetadataPairObservation;
pub use facts::{
    CountingLoopFact, DirectArrayExtentFact, DirectArrayExtentProofKind, FastPathObligation,
    LoopRangeFact, RangeIndexFact, RangeIndexFactOriginKind, RegionStabilityFact,
    RegionStabilityProofKind, RequiredFastPathRegion, SpanAccessOp, SpanAccessPlan, SpanBorrowFact,
    SpanBorrowMutability, SpanElementType,
};
pub use fastmem::{
    FastMemBlockNextFact, FastMemBlockNextProofKind, FastMemBranchConditionFact,
    FastMemBranchConditionProofKind, FastMemFieldAccessSite, FastMemFreeHeadNonEmptyFact,
    FastMemFreeHeadNonEmptyProofKind, FastMemIndexAccessSite, FastMemLocalFreeNonEmptyFact,
    FastMemLocalFreeNonEmptyProofKind, FastMemRegionMetadata, FastMemRegionOrigin,
    FastMemRemoteOwnerFact, FastMemRemoteOwnerProofKind, FastMemSameOwnerFact,
    FastMemSameOwnerProofKind, FastMemTableLengthFact, FastMemTableLengthPolicyKind,
};
pub use metadata::FunctionMetadata;
pub use object_metadata::{
    ArrayRecordAutoUseEligibilityPlan, ArrayRecordMaterializationBoundaryPlan,
    ArrayRecordPackedAutoUsePilotPlan, ArrayRecordStorageColumnPlan, ArrayRecordStoragePlan,
    DirectStateFieldPlan, DirectStatePlan, HakoAllocAlignedSmallPackedStorePilotPlan,
    HakoAllocHugePagePackedStorePilotPlan, RecordDecl, RecordLayoutFieldPlan, RecordLayoutPlan,
    RecordStateFieldAccessPlan, RecordStateResidenceFieldPlan, RecordStateResidencePlan,
    RecordStateResidenceRejectedFieldPlan, SourcePackedArrayAutoUsePilotPlan,
    SourcePackedArrayDirectReadConsumptionPlan, TypedObjectFieldPlan, TypedObjectFieldStorage,
    TypedObjectPlan, UserBoxFieldDecl, WeakFieldContractSpec, WeakFieldId, WeakFieldWriteContract,
};
pub use typed_array_contract::{
    TypedArrayBoundaryValue, TypedArrayContractBoundary, TypedArrayContractDisposition,
    TypedArrayContractSource, TypedArrayContractSourceIdentity, TypedArrayElementContract,
    TYPED_ARRAY_EXACT_NUMERIC_CAPABILITY,
};
pub use types::{
    ArrayElementWriteWitness, ArrayStateTerm, ArrayStateTermId, ArrayStateTermKind, ClosureBodyId,
    ExactNumericRuntimeCheckContract, ExactNumericRuntimeCheckContractKind,
    FunctionPublicationErrorV1, FunctionSignature, FunctionStats, LocalContractWriteKind,
    LocalIdentityEvidence, LocalSlotContract, MirEnumDecl, MirEnumVariantDecl, MirFunction,
    MirModule, MirParamDecl, ModuleMetadata, ModuleStats, ParameterEntryContract,
    ParameterEntryContractKind, RecordContractDisposition, RecordFieldValueContract,
    RecordValueBoundaryKind, RecordValueContract, ReturnExitContract, ReturnExitContractKind,
    ReturnExitContractOwner, ReturnExitVoidPolicy, StaticDataPlan, StaticElementType,
    StaticTableContractProof, StaticTableContractSpec, StaticTableId, VerifiedStaticTableContract,
};
