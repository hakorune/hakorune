/*!
 * MIR Function and Module - High-level MIR organization
 *
 * Functions contain basic blocks and SSA values, modules contain functions.
 * This parent module keeps the public surface stable while thinner child
 * modules own data definitions, behavior, display helpers, and tests.
 */

mod display;
mod facts;
mod fastmem;
mod function_impl;
mod metadata;
mod module_impl;
mod object_metadata;
mod types;

#[cfg(test)]
mod tests;

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
    TypedObjectPlan, UserBoxFieldDecl,
};
pub use types::{
    ClosureBodyId, ExactNumericRuntimeCheckContract, ExactNumericRuntimeCheckContractKind,
    FunctionSignature, FunctionStats, MirEnumDecl, MirEnumVariantDecl, MirFunction, MirModule,
    MirParamDecl, ModuleMetadata, ModuleStats, StaticDataPlan,
};
