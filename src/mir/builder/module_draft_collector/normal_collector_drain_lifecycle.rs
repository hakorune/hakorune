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
    pub(in crate::mir::builder) fn commit(self) {
        let Self {
            mut collector,
            receipt,
            target,
            _seal: _,
        } = self;
        debug_assert_eq!(collector.receipt_brand(), Some(receipt.brand));
        for key in receipt.ordered_keys.into_vec() {
            let entry = collector
                .drafts
                .remove(&key)
                .expect("prepared normal collector key must own one draft");
            match key {
                FunctionDraftKeyV1::CatalogedConstructor(canonical_key) => {
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
mod tests {
    use super::*;
    use crate::mir::builder::module_draft_collector::CompletedDraftSignatureViewV1;
    use crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationOwnerV1;
    use crate::mir::resolved_semantics::{SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1};
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};

    fn brand() -> ModuleInvocationBrandV1 {
        ModuleInvocationBrandV1::test_with_ordinal(701)
    }

    fn draft(symbol: &str) -> MirFunction {
        draft_with_arity(symbol, 0)
    }

    fn draft_with_arity(symbol: &str, arity: usize) -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: symbol.to_owned(),
                params: vec![MirType::Integer; arity],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    fn collect(collector: &mut ModuleDraftCollectorV1, symbol: &str) {
        collector
            .prepare_admission(
                FunctionDraftKeyV1::LegacySymbol(symbol.to_owned()),
                symbol.to_owned(),
                0,
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            )
            .unwrap()
            .seal(draft(symbol))
            .unwrap()
            .collect();
    }

    fn collect_cataloged(collector: &mut ModuleDraftCollectorV1) {
        let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
            "ParserScanLoopBox",
            "skip_while",
            4,
        );
        collector
            .prepare_admission(
                FunctionDraftKeyV1::CatalogedBoxMethod(key),
                "ParserScanLoopBox.skip_while/4".to_owned(),
                4,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .unwrap()
            .seal(draft_with_arity("ParserScanLoopBox.skip_while/4", 4))
            .unwrap()
            .collect();
    }

    #[test]
    fn object_definitions_move_once_with_drafts_and_reject_partial_publication() {
        use crate::mir::function::{CanonicalObjectDefinitionV1, UserBoxFieldDecl};
        use hakorune_mir_defs::{CanonicalObjectIdV1, CanonicalFieldRefV1};
        let payload = || vec![CanonicalObjectDefinitionV1::from_source_declaration(
            "Page".into(), vec![UserBoxFieldDecl {
                name: "value".into(), declared_type_name: Some("i64".into()), is_weak: false,
            }].into_boxed_slice(), Ok(()),
        )].into_boxed_slice();
        let id = CanonicalObjectIdV1::from_declaration_index(0).unwrap();
        let mut required = ModuleDraftCollectorV1::with_required_object_definitions(brand());
        collect(&mut required, "must_not_publish");
        let mut missing = MirModule::new("missing".into());
        assert!(matches!(required.prepare_normal_collector_drain(&mut missing, brand())
            .unwrap_err().error(), NormalCollectorDrainLifecycleErrorV1::ObjectDefinitionsMissing));
        assert!(missing.functions.is_empty());
        let mut empty = ModuleDraftCollectorV1::with_required_object_definitions(brand());
        empty.install_object_definitions(Box::new([]), brand()).unwrap();
        empty.prepare_normal_collector_drain(&mut missing, brand()).unwrap().commit();
        assert!(missing.preflight_object_definition_install().is_err(), "empty transfer is installed");
        let mut collector = ModuleDraftCollectorV1::with_brand(brand());
        assert!(collector.install_object_definitions(payload(),
            ModuleInvocationBrandV1::test_with_ordinal(702)).is_err());
        collector.install_object_definitions(payload(), brand()).unwrap();
        assert!(collector.install_object_definitions(payload(), brand()).is_err());
        collect(&mut collector, "helper");
        let mut module = MirModule::new("objects".into());
        collector.prepare_normal_collector_drain(&mut module, brand()).unwrap().commit();
        assert!(module.functions.contains_key("helper"));
        assert_eq!(module.canonical_object_definition(id).unwrap().diagnostic_name(), "Page");
        assert_eq!(module.canonical_field_definition(
            CanonicalFieldRefV1::from_declaration_ordinal(id, 0).unwrap()).unwrap().name, "value");
        assert!(module.canonical_field_definition(
            CanonicalFieldRefV1::from_declaration_ordinal(id, 1).unwrap()).is_none());
        assert!(module.canonical_object_definition(
            CanonicalObjectIdV1::from_declaration_index(1).unwrap()).is_none());

        let mut repeated = ModuleDraftCollectorV1::with_brand(brand());
        repeated.install_object_definitions(payload(), brand()).unwrap();
        collect(&mut repeated, "must_not_publish");
        assert!(repeated.prepare_normal_collector_drain(&mut module, brand()).is_err());
        assert!(!module.functions.contains_key("must_not_publish"));

        let mut rejected = ModuleDraftCollectorV1::with_brand(brand());
        rejected.install_object_definitions(payload(), brand()).unwrap();
        collect(&mut rejected, "bad");
        rejected.key_by_symbol.clear();
        let mut empty = MirModule::new("rejected".into());
        assert!(rejected.prepare_normal_collector_drain(&mut empty, brand()).is_err());
        assert!(empty.functions.is_empty());
        assert!(empty.canonical_object_definition(id).is_none());
    }

    #[test]
    fn birth_definition_survives_atomic_drain_with_n_plus_one_abi() {
        use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
        let key = CanonicalSameModuleCallableKeyV1::birth_constructor("Page", 2);
        let symbol = key.mir_symbol_projection();
        let mut collector = ModuleDraftCollectorV1::with_brand(brand());
        collector
            .prepare_admission(
                FunctionDraftKeyV1::CatalogedConstructor(key.clone()),
                symbol.clone(),
                3,
                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            )
            .unwrap()
            .seal(draft_with_arity(&symbol, 3))
            .unwrap()
            .collect();
        let mut module = MirModule::new("birth".into());
        assert!(matches!(
            module.preflight_cataloged_constructor(&key, &symbol, 2),
            Err(CanonicalCallableDefinitionPublicationErrorV1::KeyArityMismatch { .. })
        ));
        assert!(matches!(
            module.preflight_cataloged_box_method(&key, &symbol, 3),
            Err(CanonicalCallableDefinitionPublicationErrorV1::KeyNamespaceMismatch { .. })
        ));
        assert!(matches!(
            module.preflight_cataloged_constructor(&key, "Wrong.birth/2", 3),
            Err(CanonicalCallableDefinitionPublicationErrorV1::KeySymbolMismatch { .. })
        ));
        assert_eq!(module.canonical_callable_definition_count(), 0);
        collector
            .prepare_normal_collector_drain(&mut module, brand())
            .unwrap()
            .commit();
        assert_eq!(
            module.canonical_callable_definition_symbol(&key),
            Some(symbol.as_str())
        );
        assert_eq!(module.canonical_callable_definition_count(), 1);
        assert!(matches!(
            module.add_cataloged_constructor(key, draft_with_arity(&symbol, 3)),
            Err(CanonicalCallableDefinitionPublicationErrorV1::DuplicateKey { .. })
        ));
        assert_eq!(module.canonical_callable_definition_count(), 1);
    }

    #[test]
    fn mixed_normal_drain_accepts_legacy_and_cataloged_rows() {
        let mut collector = ModuleDraftCollectorV1::with_brand(brand());
        collect(&mut collector, "legacy/0");
        collect_cataloged(&mut collector);
        let mut target = MirModule::new("normal".to_owned());

        collector
            .prepare_normal_collector_drain(&mut target, brand())
            .unwrap()
            .commit();

        assert_eq!(
            target.function_names(),
            vec!["ParserScanLoopBox.skip_while/4", "legacy/0"]
        );
        let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
            "ParserScanLoopBox",
            "skip_while",
            4,
        );
        assert_eq!(target.canonical_callable_definition_count(), 1);
        assert_eq!(
            target.canonical_callable_definition_symbol(&key),
            Some("ParserScanLoopBox.skip_while/4")
        );
    }

    #[test]
    fn normal_drain_rejects_cataloged_legacy_policy_without_publication() {
        let key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
            "ParserScanLoopBox",
            "skip_while",
            4,
        );
        let mut collector = ModuleDraftCollectorV1::with_brand(brand());
        collector
            .prepare_admission(
                FunctionDraftKeyV1::CatalogedBoxMethod(key),
                "ParserScanLoopBox.skip_while/4".to_owned(),
                4,
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            )
            .unwrap()
            .seal(draft_with_arity("ParserScanLoopBox.skip_while/4", 4))
            .unwrap()
            .collect();
        let mut target = MirModule::new("normal".to_owned());

        let rejected = collector
            .prepare_normal_collector_drain(&mut target, brand())
            .unwrap_err();
        assert!(matches!(
            rejected.error(),
            NormalCollectorDrainLifecycleErrorV1::FinalAdmissionDrift {
                key: FunctionDraftKeyV1::CatalogedBoxMethod(_)
            }
        ));
        let (collector, _) = rejected.into_parts();
        assert_eq!(collector.symbol_count(), 1);
        assert!(target.function_names().is_empty());
        assert_eq!(target.canonical_callable_definition_count(), 0);
    }

    #[test]
    fn prepared_normal_drain_preserves_final_legacy_key_order_until_commit() {
        let mut collector = ModuleDraftCollectorV1::with_brand(brand());
        collect(&mut collector, "Zeta.run/0");
        collect(&mut collector, "Alpha.run/0");
        let mut target = MirModule::new("normal".to_owned());

        collector
            .prepare_normal_collector_drain(&mut target, brand())
            .unwrap()
            .commit();

        assert_eq!(target.function_names(), vec!["Alpha.run/0", "Zeta.run/0"]);
    }

    #[test]
    fn empty_normal_collector_prepares_and_commits_without_publication() {
        let collector = ModuleDraftCollectorV1::with_brand(brand());
        let mut target = MirModule::new("normal".to_owned());

        collector
            .prepare_normal_collector_drain(&mut target, brand())
            .unwrap()
            .commit();

        assert!(target.function_names().is_empty());
    }

    #[test]
    fn normal_drain_rejection_retains_collector_and_leaves_target_unchanged() {
        let mut collector = ModuleDraftCollectorV1::with_brand(brand());
        collect(&mut collector, "same/0");
        let mut target = MirModule::new("normal".to_owned());
        target.add_function(draft("same/0"));

        let rejected = collector
            .prepare_normal_collector_drain(&mut target, brand())
            .unwrap_err();
        assert!(matches!(
            rejected.error(),
            NormalCollectorDrainLifecycleErrorV1::Publication(_)
        ));
        let (collector, _) = rejected.into_parts();
        assert_eq!(collector.symbol_count(), 1);
        assert_eq!(target.function_names(), vec!["same/0"]);
    }

    #[test]
    fn normal_drain_rejects_foreign_or_missing_session_brand_without_consuming_rows() {
        let mut collector = ModuleDraftCollectorV1::with_brand(brand());
        collect(&mut collector, "same/0");
        let mut target = MirModule::new("normal".to_owned());

        let rejected = collector
            .prepare_normal_collector_drain(
                &mut target,
                ModuleInvocationBrandV1::test_with_ordinal(702),
            )
            .unwrap_err();
        assert!(matches!(
            rejected.error(),
            NormalCollectorDrainLifecycleErrorV1::BrandMismatch
        ));
        let (collector, _) = rejected.into_parts();
        assert_eq!(collector.symbol_count(), 1);
        assert!(target.function_names().is_empty());
    }

    #[test]
    fn normal_drain_rejects_nonlegacy_final_rows_without_consuming_them() {
        let mut collector = ModuleDraftCollectorV1::with_brand(brand());
        collector
            .prepare_admission(
                FunctionDraftKeyV1::Main,
                "main".to_owned(),
                0,
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            )
            .unwrap()
            .seal(draft("main"))
            .unwrap()
            .collect();
        let mut target = MirModule::new("normal".to_owned());

        let rejected = collector
            .prepare_normal_collector_drain(&mut target, brand())
            .unwrap_err();
        assert!(matches!(
            rejected.error(),
            NormalCollectorDrainLifecycleErrorV1::NonLegacyKey { .. }
        ));
        let (collector, _) = rejected.into_parts();
        assert_eq!(collector.symbol_count(), 1);
        assert!(target.function_names().is_empty());
    }

    #[test]
    fn normal_drain_rejects_unconsumed_static_publication_before_commit() {
        let mut collector = ModuleDraftCollectorV1::with_brand(brand());
        collect(&mut collector, "same/0");
        let caller = crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
            "Owner", "caller", 0,
        );
        let target = crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
            "Owner", "target", 0,
        );
        let site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(0),
        ]));
        collector
            .install_static_result_publication_owner(
                VerifiedStaticCallResultPublicationOwnerV1::target_only_for_test(
                    caller, site, target,
                ),
            )
            .unwrap();
        let mut module = MirModule::new("normal".to_owned());

        let rejected = collector
            .prepare_normal_collector_drain(&mut module, brand())
            .unwrap_err();
        assert!(matches!(
            rejected.error(),
            NormalCollectorDrainLifecycleErrorV1::StaticResultPublicationResidual(
                StaticCallResultPublicationOwnerFinishErrorV1::UnconsumedTargetOnly { .. }
            )
        ));
        let (collector, _) = rejected.into_parts();
        assert_eq!(collector.symbol_count(), 1);
        assert!(module.function_names().is_empty());
    }
}
