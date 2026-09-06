//! Normal candidate collector drain and final module-publication lifecycle.
//!
//! This owner preserves the selected normal LegacySymbol and CatalogedBoxMethod
//! admission semantics, while binding the collector to the already-issued
//! candidate-session brand. It neither reads source nor opens another
//! module/publication route.

use std::collections::BTreeSet;

use crate::mir::builder::module_invocation_identity::ModuleInvocationBrandV1;
use crate::mir::callable_result_representation::StaticCallResultPublicationOwnerFinishErrorV1;
use crate::mir::function::{
    CanonicalCallableDefinitionPublicationErrorV1, FunctionPublicationErrorV1,
};
use crate::mir::MirModule;
use crate::mir::builder::normal_callable_semantic_lowering_state::construction::RetainedConstructionDrafts;
#[cfg(test)]
use crate::mir::builder::normal_callable_semantic_lowering_state::construction::RetainedConstructionValidation;
#[cfg(test)]
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;

use super::receipt::CollectedDraftReplacementDispositionV1;
use super::{DraftPublicationPolicyV1, FunctionDraftKeyV1, ModuleDraftCollectorV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum NormalCollectorDrainLifecycleErrorV1 {
    BrandMismatch,
    FinalAdmissionDrift { key: FunctionDraftKeyV1 },
    NonLegacyKey { key: FunctionDraftKeyV1 },
    NonLegacyPolicy { key: FunctionDraftKeyV1 },
    SymbolIndexDrift { symbol: String },
    Publication(FunctionPublicationErrorV1),
    CanonicalDefinition(CanonicalCallableDefinitionPublicationErrorV1),
    StaticResultPublicationResidual(StaticCallResultPublicationOwnerFinishErrorV1),
    ObjectDefinitionDestinationOccupied,
    ObjectDefinitionsMissing,
}

impl std::fmt::Display for NormalCollectorDrainLifecycleErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Publication(error) => error.fmt(formatter),
            _ => write!(
                formatter,
                "[freeze:contract][normal-collector-drain] {self:?}"
            ),
        }
    }
}

impl std::error::Error for NormalCollectorDrainLifecycleErrorV1 {}

/// Failure retains every unpublished draft; the outer normal lifecycle owns
/// the candidate session and source disposal.
#[derive(Debug)]
pub(in crate::mir::builder) struct RejectedNormalCollectorDrainLifecycleV1 {
    collector: ModuleDraftCollectorV1,
    error: NormalCollectorDrainLifecycleErrorV1,
}

impl RejectedNormalCollectorDrainLifecycleV1 {
    pub(in crate::mir::builder) fn error(&self) -> &NormalCollectorDrainLifecycleErrorV1 {
        &self.error
    }

    pub(in crate::mir::builder) fn discard(self) {}

    #[cfg(test)]
    fn into_parts(self) -> (ModuleDraftCollectorV1, NormalCollectorDrainLifecycleErrorV1) {
        (self.collector, self.error)
    }
}

#[derive(Debug)]
struct SealedNormalCollectorDrainReceiptV1 {
    brand: ModuleInvocationBrandV1,
    ordered_keys: Box<[FunctionDraftKeyV1]>,
}

/// The target module is borrowed only after all correspondence and collision
/// checks complete. `commit` therefore has no fallible operation.
#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedNormalCollectorDrainLifecycleV1<'module> {
    collector: ModuleDraftCollectorV1,
    receipt: SealedNormalCollectorDrainReceiptV1,
    target: &'module mut MirModule,
    _seal: PreparedNormalCollectorDrainLifecycleSealV1,
}

#[derive(Debug)]
struct PreparedNormalCollectorDrainLifecycleSealV1;

impl PreparedNormalCollectorDrainLifecycleV1<'_> {
    pub(in crate::mir::builder) fn commit(self) -> RetainedConstructionDrafts {
        let Self {
            mut collector,
            receipt,
            target,
            _seal: _,
        } = self;
        debug_assert_eq!(collector.receipt_brand(), Some(receipt.brand));
        let mut construction = Vec::new();
        for key in receipt.ordered_keys.into_vec() {
            let entry = collector
                .drafts
                .remove(&key)
                .expect("prepared normal collector key must own one draft");
            match key {
                FunctionDraftKeyV1::CatalogedConstructor(canonical_key) => {
                    if let Some(validation) = entry.construction {
                        construction.push((canonical_key.clone(), validation));
                    }
                    target
                        .add_cataloged_constructor(canonical_key, entry.draft)
                        .expect("sealed constructor publication must remain valid");
                }
                FunctionDraftKeyV1::CatalogedBoxMethod(canonical_key) => {
                    target
                        .add_cataloged_box_method(canonical_key, entry.draft)
                        .expect("sealed cataloged box-method publication must remain valid");
                }
                _ => target.add_function(entry.draft),
            }
        }
        if let Some(definitions) = collector.object_definitions.take() {
            target.install_object_definitions_preflighted(definitions);
        }
        construction
    }
}

impl ModuleDraftCollectorV1 {
    pub(in crate::mir::builder) fn with_required_object_definitions(
        brand: ModuleInvocationBrandV1,
    ) -> Self {
        Self { object_definitions_required: true, ..Self::with_brand(brand) }
    }

    pub(in crate::mir::builder) fn install_object_definitions_from_package(
        &mut self,
        package: &mut crate::mir::normal_callable_semantic_package::NormalCallableSemanticPackagePortV1<'_>,
        context: &crate::mir::builder::CompilationContext,
        brand: ModuleInvocationBrandV1,
    ) -> Result<(), String> {
        if self.receipt_brand() != Some(brand) {
            return Err("[freeze:contract][mir/object-definitions/foreign-collector]".into());
        }
        if self.object_definitions.is_some() {
            return Err("[freeze:contract][mir/object-definitions/duplicate-install]".into());
        }
        self.object_definitions = Some(package.take_object_definitions(context)?);
        Ok(())
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn install_object_definitions(
        &mut self,
        definitions: Box<[crate::mir::function::CanonicalObjectDefinitionV1]>,
        brand: ModuleInvocationBrandV1,
    ) -> Result<(), String> {
        if self.receipt_brand() != Some(brand) {
            return Err("[freeze:contract][mir/object-definitions/foreign-collector]".into());
        }
        if self.object_definitions.is_some() {
            return Err("[freeze:contract][mir/object-definitions/duplicate-install]".into());
        }
        self.object_definitions = Some(definitions);
        Ok(())
    }

    /// Consume exactly one normal candidate's final LegacySymbol/CatalogedBoxMethod
    /// rows after source-free correspondence, session-brand, and module-collision
    /// preflight.
    pub(in crate::mir::builder) fn prepare_normal_collector_drain<'module>(
        self,
        target: &'module mut MirModule,
        brand: ModuleInvocationBrandV1,
    ) -> Result<
        PreparedNormalCollectorDrainLifecycleV1<'module>,
        RejectedNormalCollectorDrainLifecycleV1,
    > {
        let residual = self
            .static_result_publication_owner
            .as_ref()
            .and_then(|owner| owner.finish_empty().err());
        if let Some(error) = residual {
            return Err(reject(
                self,
                NormalCollectorDrainLifecycleErrorV1::StaticResultPublicationResidual(error),
            ));
        }
        let receipt = match SealedNormalCollectorDrainReceiptV1::seal(&self, target, brand) {
            Ok(receipt) => receipt,
            Err(error) => return Err(reject(self, error)),
        };
        Ok(PreparedNormalCollectorDrainLifecycleV1 {
            collector: self,
            receipt,
            target,
            _seal: PreparedNormalCollectorDrainLifecycleSealV1,
        })
    }
}

impl SealedNormalCollectorDrainReceiptV1 {
    fn seal(
        collector: &ModuleDraftCollectorV1,
        target: &MirModule,
        brand: ModuleInvocationBrandV1,
    ) -> Result<Self, NormalCollectorDrainLifecycleErrorV1> {
        if collector.receipt_brand() != Some(brand) {
            return Err(NormalCollectorDrainLifecycleErrorV1::BrandMismatch);
        }
        if collector.drafts.len() != collector.key_by_symbol.len() {
            let symbol = collector
                .key_by_symbol
                .keys()
                .next()
                .cloned()
                .unwrap_or_default();
            return Err(NormalCollectorDrainLifecycleErrorV1::SymbolIndexDrift { symbol });
        }

        if collector.object_definitions_required && collector.object_definitions.is_none() {
            return Err(NormalCollectorDrainLifecycleErrorV1::ObjectDefinitionsMissing);
        }
        if collector.object_definitions.is_some() {
            target.preflight_object_definition_install()
                .map_err(|_| NormalCollectorDrainLifecycleErrorV1::ObjectDefinitionDestinationOccupied)?;
        }
        let mut symbols = BTreeSet::new();
        let mut canonical_keys = BTreeSet::new();
        let mut ordered_keys = Vec::with_capacity(collector.drafts.len());
        for (key, entry) in &collector.drafts {
            let admission = &entry.admission;
            if entry.construction.is_some()
                && (!matches!(key, FunctionDraftKeyV1::CatalogedConstructor(_))
                    || admission.policy != DraftPublicationPolicyV1::CanonicalRejectDuplicate)
            {
                return Err(NormalCollectorDrainLifecycleErrorV1::FinalAdmissionDrift { key: key.clone() });
            }
            if admission.key != *key
                || admission.symbol.as_ref() != entry.draft.signature.name
                || admission.arity != entry.draft.signature.params.len()
            {
                return Err(NormalCollectorDrainLifecycleErrorV1::FinalAdmissionDrift {
                    key: key.clone(),
                });
            }
            match key {
                FunctionDraftKeyV1::LegacySymbol(symbol)
                    if symbol == admission.symbol.as_ref()
                        && admission.policy == DraftPublicationPolicyV1::LegacyReplaceWholePair
                        && matches!(
                            &admission.replacement,
                            CollectedDraftReplacementDispositionV1::Inserted
                                | CollectedDraftReplacementDispositionV1::ReplacedWholePair { .. }
                        ) => {}
                FunctionDraftKeyV1::CatalogedBoxMethod(canonical_key)
                | FunctionDraftKeyV1::CatalogedConstructor(canonical_key)
                    if admission.policy == DraftPublicationPolicyV1::CanonicalRejectDuplicate
                        && matches!(
                            &admission.replacement,
                            CollectedDraftReplacementDispositionV1::Inserted
                        ) =>
                {
                    if !canonical_keys.insert(canonical_key.clone()) {
                        return Err(NormalCollectorDrainLifecycleErrorV1::CanonicalDefinition(
                            CanonicalCallableDefinitionPublicationErrorV1::DuplicateKey {
                                key: canonical_key.clone(),
                            },
                        ));
                    }
                    let preflight = if matches!(key, FunctionDraftKeyV1::CatalogedConstructor(_)) {
                        target.preflight_cataloged_constructor(
                            canonical_key,
                            admission.symbol.as_ref(),
                            admission.arity,
                        )
                    } else {
                        target.preflight_cataloged_box_method(
                            canonical_key,
                            admission.symbol.as_ref(),
                            admission.arity,
                        )
                    };
                    preflight.map_err(NormalCollectorDrainLifecycleErrorV1::CanonicalDefinition)?;
                }
                _ => {
                    return Err(match key {
                        FunctionDraftKeyV1::LegacySymbol(_) => {
                            NormalCollectorDrainLifecycleErrorV1::NonLegacyPolicy {
                                key: key.clone(),
                            }
                        }
                        FunctionDraftKeyV1::CatalogedBoxMethod(_)
                        | FunctionDraftKeyV1::CatalogedConstructor(_) => {
                            NormalCollectorDrainLifecycleErrorV1::FinalAdmissionDrift {
                                key: key.clone(),
                            }
                        }
                        _ => {
                            NormalCollectorDrainLifecycleErrorV1::NonLegacyKey { key: key.clone() }
                        }
                    });
                }
            }
            if collector.key_by_symbol.get(admission.symbol.as_ref()) != Some(key)
                || !symbols.insert(admission.symbol.as_ref())
            {
                return Err(NormalCollectorDrainLifecycleErrorV1::SymbolIndexDrift {
                    symbol: admission.symbol.to_string(),
                });
            }
            ordered_keys.push(key.clone());
        }

        target
            .preflight_add_function_symbols(symbols.into_iter())
            .map_err(NormalCollectorDrainLifecycleErrorV1::Publication)?;
        Ok(Self {
            brand,
            ordered_keys: ordered_keys.into_boxed_slice(),
        })
    }
}

fn reject(
    collector: ModuleDraftCollectorV1,
    error: NormalCollectorDrainLifecycleErrorV1,
) -> RejectedNormalCollectorDrainLifecycleV1 {
    RejectedNormalCollectorDrainLifecycleV1 { collector, error }
}

#[cfg(test)]
#[path = "normal_collector_drain_lifecycle_tests.rs"]
mod tests;
