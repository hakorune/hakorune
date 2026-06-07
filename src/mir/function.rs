/*!
 * MIR Function and Module - High-level MIR organization
 *
 * Functions contain basic blocks and SSA values, modules contain functions.
 * This parent module keeps the public surface stable while thinner child
 * modules own data definitions, behavior, display helpers, and tests.
 */

mod display;
mod fastmem;
mod function_impl;
mod module_impl;
mod object_metadata;
mod types;

#[cfg(test)]
mod tests;

pub use fastmem::{
    FastMemBlockNextFact, FastMemBlockNextProofKind, FastMemFreeHeadNonEmptyFact,
    FastMemFreeHeadNonEmptyProofKind, FastMemLocalFreeNonEmptyFact,
    FastMemLocalFreeNonEmptyProofKind, FastMemRegionMetadata, FastMemRegionOrigin,
    FastMemRemoteOwnerFact, FastMemRemoteOwnerProofKind, FastMemSameOwnerFact,
    FastMemSameOwnerProofKind, FastMemTableLengthFact, FastMemTableLengthPolicyKind,
};
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
    ClosureBodyId, CountingLoopFact, DirectArrayExtentFact, DirectArrayExtentProofKind,
    ExactNumericRuntimeCheckContract, ExactNumericRuntimeCheckContractKind, FastPathObligation,
    FunctionMetadata, FunctionSignature, FunctionStats, LoopRangeFact, MirEnumDecl,
    MirEnumVariantDecl, MirFunction, MirModule, MirParamDecl, ModuleMetadata, ModuleStats,
    RangeIndexFact, RangeIndexFactOriginKind, RegionStabilityFact, RegionStabilityProofKind,
    RequiredFastPathRegion, SpanAccessOp, SpanAccessPlan, SpanBorrowFact, SpanBorrowMutability,
    SpanElementType, StaticDataPlan,
};
