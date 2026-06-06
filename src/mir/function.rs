/*!
 * MIR Function and Module - High-level MIR organization
 *
 * Functions contain basic blocks and SSA values, modules contain functions.
 * This parent module keeps the public surface stable while thinner child
 * modules own data definitions, behavior, display helpers, and tests.
 */

mod display;
mod function_impl;
mod module_impl;
mod types;

#[cfg(test)]
mod tests;

pub use types::{
    ArrayRecordAutoUseEligibilityPlan, ArrayRecordMaterializationBoundaryPlan,
    ArrayRecordPackedAutoUsePilotPlan, ArrayRecordStorageColumnPlan, ArrayRecordStoragePlan,
    ClosureBodyId, CountingLoopFact, DirectArrayExtentFact, DirectArrayExtentProofKind,
    DirectStateFieldPlan, DirectStatePlan, ExactNumericRuntimeCheckContract,
    ExactNumericRuntimeCheckContractKind, FastMemBlockNextFact, FastMemBlockNextProofKind,
    FastMemLocalFreeNonEmptyFact, FastMemLocalFreeNonEmptyProofKind, FastMemRegionMetadata,
    FastMemRegionOrigin, FastMemSameOwnerFact, FastMemSameOwnerProofKind, FastMemTableLengthFact,
    FastMemTableLengthPolicyKind, FastPathObligation, FunctionMetadata, FunctionSignature,
    FunctionStats, HakoAllocAlignedSmallPackedStorePilotPlan,
    HakoAllocHugePagePackedStorePilotPlan, LoopRangeFact, MirEnumDecl, MirEnumVariantDecl,
    MirFunction, MirModule, MirParamDecl, ModuleMetadata, ModuleStats, RangeIndexFact,
    RangeIndexFactOriginKind, RecordDecl, RecordLayoutFieldPlan, RecordLayoutPlan,
    RecordStateFieldAccessPlan, RecordStateResidenceFieldPlan, RecordStateResidencePlan,
    RecordStateResidenceRejectedFieldPlan, RegionStabilityFact, RegionStabilityProofKind,
    RequiredFastPathRegion, SourcePackedArrayAutoUsePilotPlan,
    SourcePackedArrayDirectReadConsumptionPlan, SpanAccessOp, SpanAccessPlan, SpanBorrowFact,
    SpanBorrowMutability, SpanElementType, StaticDataPlan, TypedObjectFieldPlan,
    TypedObjectFieldStorage, TypedObjectPlan, UserBoxFieldDecl,
};
