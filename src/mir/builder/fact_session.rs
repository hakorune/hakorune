//! FACTSESSION0-S0 private vocabulary for function-scoped transient facts.
//!
//! This module is intentionally disconnected from `MirBuilder` lifecycle
//! entry and completion. It fixes the vocabulary boundary first: one explicit
//! module session issues generations, one function session owns every current
//! ValueId-keyed lane, and a completed draft cannot lose its sealed facts.

use std::collections::BTreeMap;

use crate::mir::builder::function_lowering_state::FunctionValueOriginFactsV1;
use crate::mir::builder::ssa::phi_input_materializer::remat_fact::{
    OpenExactProducerReceiptLedgerV1, SealedExactProducerReceiptLedgerV1,
};
use crate::mir::builder::type_context::TypeContext;
use crate::mir::MirFunction;
#[cfg(test)]
use crate::mir::ValueId;

/// Opaque identity for one function's transient fact session.
///
/// Neither component is a `ValueId`. The module brand comes from the outer
/// lifecycle issuer; the ordinal is local to that exact module session.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::mir::builder) struct FunctionFactGenerationV1 {
    module_brand: u64,
    function_ordinal: u32,
}

impl FunctionFactGenerationV1 {
    #[cfg(test)]
    pub(in crate::mir::builder) const fn for_test(
        module_brand: u64,
        function_ordinal: u32,
    ) -> Self {
        Self {
            module_brand,
            function_ordinal,
        }
    }
}

/// Compiler-lifetime source of distinct module-session brands.
///
/// A later compiler/session entry will own this explicit lifecycle input. It
/// is not a `MirBuilder`, `CoreContext`, `TypeContext`, or metadata field.
#[derive(Debug, Default)]
pub(in crate::mir::builder) struct FactSessionIssuerV1 {
    next_module_brand: u64,
}

impl FactSessionIssuerV1 {
    pub(in crate::mir::builder) fn open_module(
        &mut self,
    ) -> Result<ModuleFactSessionV1, FactSessionIssuerErrorV1> {
        let module_brand = self.next_module_brand;
        self.next_module_brand = self
            .next_module_brand
            .checked_add(1)
            .ok_or(FactSessionIssuerErrorV1::ModuleBrandOverflow)?;
        Ok(ModuleFactSessionV1 {
            module_brand,
            next_function_ordinal: 0,
            completed: BTreeMap::new(),
            _seal: ModuleFactSessionSealV1,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::mir::builder) enum FactSessionIssuerErrorV1 {
    ModuleBrandOverflow,
}

impl std::fmt::Display for FactSessionIssuerErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][fact_session/module_issuer] {self:?}"
        )
    }
}

impl std::error::Error for FactSessionIssuerErrorV1 {}

/// One invocation-local collector and sole opener of function fact sessions.
#[derive(Debug)]
pub(in crate::mir::builder) struct ModuleFactSessionV1 {
    module_brand: u64,
    next_function_ordinal: u32,
    completed: BTreeMap<FunctionFactGenerationV1, CompletedFunctionDraftWithFactsV1>,
    _seal: ModuleFactSessionSealV1,
}

#[derive(Debug)]
struct ModuleFactSessionSealV1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::mir::builder) enum ModuleFactSessionErrorV1 {
    FunctionOrdinalOverflow,
    ForeignCompletedDraft {
        module_brand: u64,
        completed: FunctionFactGenerationV1,
    },
    DuplicateCompletedGeneration(FunctionFactGenerationV1),
}

impl std::fmt::Display for ModuleFactSessionErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][fact_session/module_lifecycle] {self:?}"
        )
    }
}

impl std::error::Error for ModuleFactSessionErrorV1 {}

impl ModuleFactSessionV1 {
    /// Opens the sole transient fact owner for one physical function lowering.
    pub(in crate::mir::builder) fn open_function(
        &mut self,
    ) -> Result<OpenFunctionFactSessionV1, ModuleFactSessionErrorV1> {
        let ordinal = self.next_function_ordinal;
        self.next_function_ordinal = self
            .next_function_ordinal
            .checked_add(1)
            .ok_or(ModuleFactSessionErrorV1::FunctionOrdinalOverflow)?;
        let generation = FunctionFactGenerationV1 {
            module_brand: self.module_brand,
            function_ordinal: ordinal,
        };
        Ok(OpenFunctionFactSessionV1 {
            generation,
            transient: FunctionTransientFactLanesV1::default(),
            receipts: OpenExactProducerReceiptLedgerV1::new(generation),
            _seal: OpenFunctionFactSessionSealV1,
        })
    }

    /// Collects a sealed draft only when it belongs to this exact module run.
    pub(in crate::mir::builder) fn collect_completed(
        &mut self,
        completed: CompletedFunctionDraftWithFactsV1,
    ) -> Result<(), ModuleFactSessionErrorV1> {
        let generation = completed.generation();
        if generation.module_brand != self.module_brand {
            return Err(ModuleFactSessionErrorV1::ForeignCompletedDraft {
                module_brand: self.module_brand,
                completed: generation,
            });
        }
        if self.completed.contains_key(&generation) {
            return Err(ModuleFactSessionErrorV1::DuplicateCompletedGeneration(
                generation,
            ));
        }
        self.completed.insert(generation, completed);
        Ok(())
    }

    /// Moves all completed drafts into an eventual module candidate owner.
    pub(in crate::mir::builder) fn seal(self) -> SealedModuleFactSessionV1 {
        SealedModuleFactSessionV1 {
            module_brand: self.module_brand,
            completed: self.completed,
            _seal: SealedModuleFactSessionSealV1,
        }
    }
}

/// The full transient ValueId surface that moves with one function.
#[derive(Debug, Default)]
pub(in crate::mir::builder) struct FunctionTransientFactLanesV1 {
    type_context: TypeContext,
    diagnostic_origins: FunctionValueOriginFactsV1,
}

impl FunctionTransientFactLanesV1 {
    #[cfg(test)]
    fn test_insert_all_lanes(&mut self, value: ValueId) {
        use crate::mir::value_kind::MirValueKind;
        use crate::mir::MirType;

        self.type_context
            .value_types
            .insert(value, MirType::Integer);
        self.type_context
            .value_kinds
            .insert(value, MirValueKind::Temporary);
        self.type_context
            .value_origin_newbox
            .insert(value, "Owner".to_string());
        self.type_context
            .string_literals
            .insert(value, "literal".to_string());
        self.type_context
            .map_value_types
            .insert(value, MirType::Integer);
        self.type_context
            .map_literal_value_types
            .insert((value, "key".to_string()), MirType::Integer);
        self.diagnostic_origins
            .record_span(value, crate::ast::Span::unknown());
        self.diagnostic_origins
            .record_caller(value, std::panic::Location::caller());
    }

    #[cfg(test)]
    fn test_lane_counts(&self) -> [usize; 8] {
        [
            self.type_context.value_types.len(),
            self.type_context.value_kinds.len(),
            self.type_context.value_origin_newbox.len(),
            self.type_context.string_literals.len(),
            self.type_context.map_value_types.len(),
            self.type_context.map_literal_value_types.len(),
            self.diagnostic_origins.value_origin_spans.len(),
            self.diagnostic_origins.value_origin_callers.len(),
        ]
    }
}

/// Open only during one physical function-lowering transaction.
#[derive(Debug)]
pub(in crate::mir::builder) struct OpenFunctionFactSessionV1 {
    generation: FunctionFactGenerationV1,
    transient: FunctionTransientFactLanesV1,
    receipts: OpenExactProducerReceiptLedgerV1,
    _seal: OpenFunctionFactSessionSealV1,
}

#[derive(Debug)]
struct OpenFunctionFactSessionSealV1;

impl OpenFunctionFactSessionV1 {
    pub(in crate::mir::builder) const fn generation(&self) -> FunctionFactGenerationV1 {
        self.generation
    }

    /// Aborting is consumption: it cannot leak receipts or fact lanes.
    pub(in crate::mir::builder) fn abort(self) {}

    pub(in crate::mir::builder) fn seal(self) -> SealedFunctionFactSessionV1 {
        SealedFunctionFactSessionV1 {
            generation: self.generation,
            transient: self.transient,
            receipts: self.receipts.seal(),
            _seal: SealedFunctionFactSessionSealV1,
        }
    }

    pub(in crate::mir::builder) fn seal_with_draft(
        self,
        draft: MirFunction,
    ) -> CompletedFunctionDraftWithFactsV1 {
        CompletedFunctionDraftWithFactsV1 {
            draft,
            facts: self.seal(),
            _seal: CompletedFunctionDraftWithFactsSealV1,
        }
    }

    #[cfg(test)]
    fn test_insert_all_lanes(&mut self, value: ValueId) {
        self.transient.test_insert_all_lanes(value);
    }
}

/// Immutable fact/session transport paired with one completed function draft.
#[derive(Debug)]
pub(in crate::mir::builder) struct SealedFunctionFactSessionV1 {
    generation: FunctionFactGenerationV1,
    transient: FunctionTransientFactLanesV1,
    #[allow(dead_code)] // S0 retains the sealed ledger for a later candidate consumer.
    receipts: SealedExactProducerReceiptLedgerV1,
    _seal: SealedFunctionFactSessionSealV1,
}

#[derive(Debug)]
struct SealedFunctionFactSessionSealV1;

/// Single-use completed draft that cannot lose its matching sealed facts.
#[derive(Debug)]
pub(in crate::mir::builder) struct CompletedFunctionDraftWithFactsV1 {
    #[allow(dead_code)] // S0 seals, but does not yet publish, the paired draft.
    draft: MirFunction,
    facts: SealedFunctionFactSessionV1,
    _seal: CompletedFunctionDraftWithFactsSealV1,
}

#[derive(Debug)]
struct CompletedFunctionDraftWithFactsSealV1;

impl CompletedFunctionDraftWithFactsV1 {
    fn generation(&self) -> FunctionFactGenerationV1 {
        self.facts.generation
    }
}

/// Sealed input for the eventual per-function module candidate collection.
#[derive(Debug)]
pub(in crate::mir::builder) struct SealedModuleFactSessionV1 {
    #[allow(dead_code)] // S0's later candidate consumer will verify this brand.
    module_brand: u64,
    completed: BTreeMap<FunctionFactGenerationV1, CompletedFunctionDraftWithFactsV1>,
    _seal: SealedModuleFactSessionSealV1,
}

#[derive(Debug)]
struct SealedModuleFactSessionSealV1;

/// Test-only lifecycle adapter for FACTSESSION0-P0.
///
/// This deliberately owns only the disconnected session vocabulary. It does
/// not borrow `MirBuilder`, move Builder maps, or stand in for a production
/// completion boundary; P0 uses it beside existing Builder sessions solely to
/// observe lifecycle ordering before I0 selects a live connection.
#[cfg(test)]
pub(super) mod p0_test_support {
    use super::{
        FactSessionIssuerErrorV1, FactSessionIssuerV1, FunctionFactGenerationV1,
        ModuleFactSessionErrorV1, ModuleFactSessionV1,
    };
    use crate::mir::{MirFunction, ValueId};

    #[allow(dead_code)] // P0-S0 exposes the typed test-only failure surface before P0 probes it.
    #[derive(Debug)]
    pub(in crate::mir::builder) enum FactSessionP0HarnessErrorV1 {
        Issuer(FactSessionIssuerErrorV1),
        Module(ModuleFactSessionErrorV1),
    }

    impl From<FactSessionIssuerErrorV1> for FactSessionP0HarnessErrorV1 {
        fn from(error: FactSessionIssuerErrorV1) -> Self {
            Self::Issuer(error)
        }
    }

    impl From<ModuleFactSessionErrorV1> for FactSessionP0HarnessErrorV1 {
        fn from(error: ModuleFactSessionErrorV1) -> Self {
            Self::Module(error)
        }
    }

    /// One test-only module invocation boundary.
    #[derive(Debug)]
    pub(in crate::mir::builder) struct FactSessionP0HarnessV1 {
        module: ModuleFactSessionV1,
    }

    impl FactSessionP0HarnessV1 {
        pub(in crate::mir::builder) fn open(
            issuer: &mut FactSessionIssuerV1,
        ) -> Result<Self, FactSessionP0HarnessErrorV1> {
            Ok(Self {
                module: issuer.open_module()?,
            })
        }

        /// Exercises only the success sequence: open, seed, seal, collect.
        pub(in crate::mir::builder) fn collect_success(
            &mut self,
            draft: MirFunction,
            value: ValueId,
        ) -> Result<FunctionFactGenerationV1, FactSessionP0HarnessErrorV1> {
            let mut function = self.module.open_function()?;
            let generation = function.generation();
            function.test_insert_all_lanes(value);
            self.module
                .collect_completed(function.seal_with_draft(draft))?;
            Ok(generation)
        }

        /// Exercises abort as a consuming terminal state without collection.
        pub(in crate::mir::builder) fn abort_seeded(
            &mut self,
            value: ValueId,
        ) -> Result<FunctionFactGenerationV1, FactSessionP0HarnessErrorV1> {
            let mut function = self.module.open_function()?;
            let generation = function.generation();
            function.test_insert_all_lanes(value);
            function.abort();
            Ok(generation)
        }

        pub(in crate::mir::builder) fn completed_count(&self) -> usize {
            self.module.completed.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FactSessionIssuerV1, FunctionFactGenerationV1, ModuleFactSessionErrorV1};
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType, ValueId};

    fn draft(name: &str) -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: name.to_string(),
                params: Vec::new(),
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    #[test]
    fn module_sessions_keep_reused_value_ids_generation_distinct() {
        let mut issuer = FactSessionIssuerV1::default();
        let mut first_module = issuer.open_module().unwrap();
        let first = first_module.open_function().unwrap();
        let mut second_module = issuer.open_module().unwrap();
        let second = second_module.open_function().unwrap();

        assert_ne!(first.generation(), second.generation());
        assert_eq!(first.generation().function_ordinal, 0);
        assert_eq!(second.generation().function_ordinal, 0);
        assert_ne!(
            first.generation().module_brand,
            second.generation().module_brand
        );
    }

    #[test]
    fn function_seal_moves_every_transient_lane_with_its_draft() {
        let mut issuer = FactSessionIssuerV1::default();
        let mut module = issuer.open_module().unwrap();
        let mut function = module.open_function().unwrap();
        function.test_insert_all_lanes(ValueId::new(1));
        let completed = function.seal_with_draft(draft("a/0"));

        assert_eq!(completed.facts.transient.test_lane_counts(), [1; 8]);
        assert_eq!(completed.facts.generation, completed.generation());
        module.collect_completed(completed).unwrap();
        assert_eq!(module.seal().completed.len(), 1);
    }

    #[test]
    fn foreign_completed_draft_is_rejected_before_collection() {
        let mut issuer = FactSessionIssuerV1::default();
        let mut first_module = issuer.open_module().unwrap();
        let completed = first_module
            .open_function()
            .unwrap()
            .seal_with_draft(draft("first/0"));
        let mut second_module = issuer.open_module().unwrap();

        match second_module.collect_completed(completed) {
            Err(ModuleFactSessionErrorV1::ForeignCompletedDraft {
                module_brand,
                completed,
            }) => {
                assert_eq!(module_brand, 1);
                assert_eq!(completed, FunctionFactGenerationV1::for_test(0, 0));
            }
            other => panic!("expected foreign completed draft, got {other:?}"),
        }
    }

    #[test]
    fn abort_is_consuming_and_does_not_create_a_completed_draft() {
        let mut issuer = FactSessionIssuerV1::default();
        let mut module = issuer.open_module().unwrap();
        module.open_function().unwrap().abort();
        assert!(module.seal().completed.is_empty());
    }
}
