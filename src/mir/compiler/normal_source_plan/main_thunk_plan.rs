//! Exact source-Main to canonical physical-entry relation.
//!
//! This is a semantic plan only. It emits no MIR and opens no Builder state.

use crate::mir::builder::{canonical_normal_main_entry_target, CanonicalNormalMainEntryTargetV1};
use crate::mir::compiler::capability::{
    ResolvedOwnerHeaderSealErrorV1, VerifiedResolvedOwnerHeaderV1,
};
use crate::mir::resolved_control_flow::{FunctionUnitOriginV1, SealedFunctionExitDispositionV1};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::resolved_value_profile::product::{
    TrivialRepresentationV1, TrivialTerminalProfileV1,
};

use super::main_function_plan::VerifiedNormalMainFunctionPlanV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VerifiedNormalMainThunkResultV1 {
    Unit { origin: FunctionUnitOriginV1 },
    Integer,
    Bool,
    Float,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum NormalMainThunkPlanErrorV1 {
    Header(ResolvedOwnerHeaderSealErrorV1),
    SourceArityMismatch {
        actual: usize,
    },
    UnsupportedResultCarrier {
        representation: TrivialRepresentationV1,
    },
    CompletionRepresentationMismatch,
    EntryRelationMismatch,
}

#[derive(Debug)]
pub(crate) struct VerifiedNormalMainEntryRelationV1 {
    source_owner: FunctionOwnerIdV1,
    physical: CanonicalNormalMainEntryTargetV1,
    _seal: VerifiedNormalMainEntryRelationSealV1,
}

/// Reusable source-Main to physical-entry relation. It contains only facts
/// already sealed by the F1 plan and never owns a source-lowering plan.
#[derive(Debug)]
pub(crate) struct VerifiedNormalMainPhysicalRelationV1 {
    source_header: VerifiedResolvedOwnerHeaderV1,
    source_result: VerifiedNormalMainThunkResultV1,
    entry: VerifiedNormalMainEntryRelationV1,
    _seal: VerifiedNormalMainPhysicalRelationSealV1,
}

#[derive(Debug)]
struct VerifiedNormalMainPhysicalRelationSealV1;

impl VerifiedNormalMainPhysicalRelationV1 {
    pub(crate) fn source_header(&self) -> &VerifiedResolvedOwnerHeaderV1 {
        &self.source_header
    }

    pub(crate) const fn source_result(&self) -> VerifiedNormalMainThunkResultV1 {
        self.source_result
    }

    pub(crate) fn entry(&self) -> &VerifiedNormalMainEntryRelationV1 {
        &self.entry
    }

    pub(in crate::mir) fn into_parts(
        self,
    ) -> (
        VerifiedResolvedOwnerHeaderV1,
        VerifiedNormalMainThunkResultV1,
        VerifiedNormalMainEntryRelationV1,
    ) {
        (self.source_header, self.source_result, self.entry)
    }
}

/// Seals the sole Main-to-physical relation from already-verified F1 facts.
pub(crate) fn seal_normal_main_physical_relation_v1(
    source_header: VerifiedResolvedOwnerHeaderV1,
    completion: &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
    terminal: &TrivialTerminalProfileV1,
) -> Result<VerifiedNormalMainPhysicalRelationV1, NormalMainThunkPlanErrorV1> {
    seal_relation(
        source_header,
        completion,
        terminal,
        canonical_normal_main_entry_target(),
    )
}

#[derive(Debug)]
struct VerifiedNormalMainEntryRelationSealV1;

impl VerifiedNormalMainEntryRelationV1 {
    pub(crate) const fn source_owner(&self) -> FunctionOwnerIdV1 {
        self.source_owner
    }

    pub(crate) fn physical_symbol(&self) -> &str {
        self.physical.symbol()
    }

    pub(crate) const fn physical_arity(&self) -> usize {
        self.physical.arity()
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedNormalMainThunkPlanV1<'unit> {
    source: VerifiedNormalMainFunctionPlanV1<'unit>,
    source_header: VerifiedResolvedOwnerHeaderV1,
    source_result: VerifiedNormalMainThunkResultV1,
    entry: VerifiedNormalMainEntryRelationV1,
    _seal: VerifiedNormalMainThunkPlanSealV1,
}

#[derive(Debug)]
struct VerifiedNormalMainThunkPlanSealV1;

impl<'unit> VerifiedNormalMainThunkPlanV1<'unit> {
    pub(crate) fn seal(
        source: VerifiedNormalMainFunctionPlanV1<'unit>,
    ) -> Result<Self, RejectedNormalMainThunkPlanV1<'unit>> {
        let source_header = match source.seal_source_header() {
            Ok(header) => header,
            Err(error) => {
                return Err(RejectedNormalMainThunkPlanV1 {
                    owner: source,
                    error: NormalMainThunkPlanErrorV1::Header(error),
                })
            }
        };
        match seal_relation(
            source_header,
            source.completion(),
            source.terminal_profile(),
            canonical_normal_main_entry_target(),
        ) {
            Ok(relation) => {
                let (source_header, source_result, entry) = relation.into_parts();
                Ok(Self {
                    source,
                    source_header,
                    source_result,
                    entry,
                    _seal: VerifiedNormalMainThunkPlanSealV1,
                })
            }
            Err(error) => Err(RejectedNormalMainThunkPlanV1 {
                owner: source,
                error,
            }),
        }
    }

    pub(crate) fn source_header(&self) -> &VerifiedResolvedOwnerHeaderV1 {
        &self.source_header
    }

    pub(crate) const fn source_result(&self) -> VerifiedNormalMainThunkResultV1 {
        self.source_result
    }

    pub(crate) fn entry(&self) -> &VerifiedNormalMainEntryRelationV1 {
        &self.entry
    }

    pub(crate) fn into_source(self) -> VerifiedNormalMainFunctionPlanV1<'unit> {
        self.source
    }

    pub(in crate::mir) fn into_parts(
        self,
    ) -> (
        VerifiedNormalMainFunctionPlanV1<'unit>,
        VerifiedResolvedOwnerHeaderV1,
        VerifiedNormalMainThunkResultV1,
        VerifiedNormalMainEntryRelationV1,
    ) {
        (
            self.source,
            self.source_header,
            self.source_result,
            self.entry,
        )
    }
}

#[derive(Debug)]
pub(crate) struct RejectedNormalMainThunkPlanV1<'unit> {
    owner: VerifiedNormalMainFunctionPlanV1<'unit>,
    error: NormalMainThunkPlanErrorV1,
}

impl<'unit> RejectedNormalMainThunkPlanV1<'unit> {
    pub(crate) fn error(&self) -> &NormalMainThunkPlanErrorV1 {
        &self.error
    }

    pub(crate) fn discard(self) {
        drop(self);
    }

    pub(in crate::mir) fn into_parts(
        self,
    ) -> (VerifiedNormalMainFunctionPlanV1<'unit>, NormalMainThunkPlanErrorV1) {
        (self.owner, self.error)
    }

    #[cfg(test)]
    fn owner_for_test(&self) -> &VerifiedNormalMainFunctionPlanV1<'_> {
        &self.owner
    }
}

fn seal_relation(
    source_header: VerifiedResolvedOwnerHeaderV1,
    completion: &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
    terminal: &TrivialTerminalProfileV1,
    physical: CanonicalNormalMainEntryTargetV1,
) -> Result<VerifiedNormalMainPhysicalRelationV1, NormalMainThunkPlanErrorV1> {
    if source_header.arity() != 0 {
        return Err(NormalMainThunkPlanErrorV1::SourceArityMismatch {
            actual: source_header.arity(),
        });
    }
    let contract = completion.function_exit_contract();
    if source_header.owner() != contract.owner() {
        return Err(NormalMainThunkPlanErrorV1::EntryRelationMismatch);
    }
    let source_result = seal_result(contract.disposition(), terminal)?;
    if !physical.is_main() || physical.arity() != 0 {
        return Err(NormalMainThunkPlanErrorV1::EntryRelationMismatch);
    }
    let entry = VerifiedNormalMainEntryRelationV1 {
        source_owner: source_header.owner(),
        physical,
        _seal: VerifiedNormalMainEntryRelationSealV1,
    };
    Ok(VerifiedNormalMainPhysicalRelationV1 {
        source_header,
        source_result,
        entry,
        _seal: VerifiedNormalMainPhysicalRelationSealV1,
    })
}

fn seal_result(
    disposition: &SealedFunctionExitDispositionV1,
    terminal: &TrivialTerminalProfileV1,
) -> Result<VerifiedNormalMainThunkResultV1, NormalMainThunkPlanErrorV1> {
    match (disposition, terminal) {
        (
            SealedFunctionExitDispositionV1::ExplicitUnit { origin, .. },
            TrivialTerminalProfileV1::ExplicitNoValue { .. },
        ) => Ok(VerifiedNormalMainThunkResultV1::Unit { origin: *origin }),
        (
            SealedFunctionExitDispositionV1::ImplicitUnit { origin, .. },
            TrivialTerminalProfileV1::ImplicitNoValue { .. },
        ) => Ok(VerifiedNormalMainThunkResultV1::Unit { origin: *origin }),
        (
            SealedFunctionExitDispositionV1::ExplicitValue { .. },
            TrivialTerminalProfileV1::ExplicitValue { representation, .. },
        ) => match representation {
            TrivialRepresentationV1::InlineI64 => Ok(VerifiedNormalMainThunkResultV1::Integer),
            TrivialRepresentationV1::InlineBool => Ok(VerifiedNormalMainThunkResultV1::Bool),
            TrivialRepresentationV1::InlineF64 => Ok(VerifiedNormalMainThunkResultV1::Float),
            representation @ (TrivialRepresentationV1::ExplicitVoidValue
            | TrivialRepresentationV1::NullSentinel) => {
                Err(NormalMainThunkPlanErrorV1::UnsupportedResultCarrier {
                    representation: *representation,
                })
            }
        },
        _ => Err(NormalMainThunkPlanErrorV1::CompletionRepresentationMismatch),
    }
}

#[cfg(test)]
pub(super) fn prepare_with_physical_for_test<'unit>(
    source: VerifiedNormalMainFunctionPlanV1<'unit>,
    physical: CanonicalNormalMainEntryTargetV1,
) -> Result<VerifiedNormalMainThunkPlanV1<'unit>, RejectedNormalMainThunkPlanV1<'unit>> {
    let source_header = match source.seal_source_header() {
        Ok(header) => header,
        Err(error) => {
            return Err(RejectedNormalMainThunkPlanV1 {
                owner: source,
                error: NormalMainThunkPlanErrorV1::Header(error),
            })
        }
    };
    match seal_relation(
        source_header,
        source.completion(),
        source.terminal_profile(),
        physical,
    ) {
        Ok(relation) => {
            let (source_header, source_result, entry) = relation.into_parts();
            Ok(VerifiedNormalMainThunkPlanV1 {
                source,
                source_header,
                source_result,
                entry,
                _seal: VerifiedNormalMainThunkPlanSealV1,
            })
        }
        Err(error) => Err(RejectedNormalMainThunkPlanV1 {
            owner: source,
            error,
        }),
    }
}

#[cfg(test)]
#[path = "main_thunk_plan_tests.rs"]
mod tests;
