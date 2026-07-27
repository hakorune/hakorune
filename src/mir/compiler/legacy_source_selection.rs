//! Disconnected whole-source selection for the bounded pre-loop Stage-B row.
//!
//! This module owns the only 0/1/many policy. It runs before Builder effects
//! and has no compile, install, retry, or fallback terminal.

use crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::preloop_stageb_carrier::{
    inventory_preloop_stageb_candidates_v1, seal_preloop_stageb_candidate_selection_v1,
    PreloopStageBCandidateSelectionErrorV1, PreloopStageBSourceInventoryErrorV1,
    RejectedPreloopStageBCandidateSelectionV1, VerifiedPreloopStageBAmbiguousCandidatesV1,
    VerifiedPreloopStageBCandidateSelectionV1, VerifiedPreloopStageBCarrierActivationPlanV1,
};
use crate::mir::source_call_target::{
    StaticImportAliasViewErrorV1, VerifiedWholeSourceStaticCallTargetInventoryV1,
    WholeSourceStaticCallTargetInventoryErrorV1,
};

use super::legacy_whole_source_request::LegacyWholeSourceCompileRequestV1;
use super::lowering_input::LegacyModuleOriginV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum PreloopStageBUnavailableDispositionV1 {
    ProfileExcluded { origin: LegacyModuleOriginV1 },
    NoExactCandidate,
}

#[derive(Debug)]
pub(super) struct PreparedOrdinaryLegacyWholeSourceV1 {
    request: LegacyWholeSourceCompileRequestV1,
    disposition: PreloopStageBUnavailableDispositionV1,
}

impl PreparedOrdinaryLegacyWholeSourceV1 {
    pub(super) const fn disposition(&self) -> &PreloopStageBUnavailableDispositionV1 {
        &self.disposition
    }

    pub(super) fn diagnostic_source_hint(&self) -> Option<&str> {
        self.request.diagnostic_source_hint()
    }

    pub(super) fn discard(self) {
        self.request.discard();
    }
}

#[derive(Debug)]
pub(super) struct PreparedSelectedPreloopStageBWholeSourceV1 {
    request: LegacyWholeSourceCompileRequestV1,
    activation: VerifiedPreloopStageBCarrierActivationPlanV1,
}

impl PreparedSelectedPreloopStageBWholeSourceV1 {
    pub(super) const fn activation(&self) -> &VerifiedPreloopStageBCarrierActivationPlanV1 {
        &self.activation
    }

    pub(super) fn diagnostic_source_hint(&self) -> Option<&str> {
        self.request.diagnostic_source_hint()
    }

    pub(super) fn prepare_module_activation(
        self,
        builder: &crate::mir::builder::MirBuilder,
    ) -> Result<
        super::legacy_module_activation::PreparedPreloopStageBModuleActivationV1,
        super::legacy_module_activation::RejectedPreloopStageBModuleActivationV1,
    > {
        super::legacy_module_activation::prepare_preloop_stageb_module_activation_v1(self, builder)
    }

    pub(super) fn import_expectation(&self) -> (bool, usize, impl Iterator<Item = (&str, &str)>) {
        (
            self.request.imports_are_explicit(),
            self.request.import_count(),
            self.request.import_entries(),
        )
    }

    pub(super) fn discard(self) {
        let Self {
            request,
            activation,
        } = self;
        request.discard();
        let _ = activation;
    }
}

#[derive(Debug)]
pub(super) enum PreloopStageBWholeSourceDispositionV1 {
    Ordinary(PreparedOrdinaryLegacyWholeSourceV1),
    Selected(PreparedSelectedPreloopStageBWholeSourceV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PreloopStageBWholeSourceSelectionStageV1 {
    DeclarationCatalog,
    ImportAliases,
    WholeSourceInventory,
    CandidateInventory,
    CandidateSelection,
    CandidateCardinality,
}

#[derive(Debug)]
pub(super) enum PreloopStageBWholeSourceSelectionErrorV1 {
    DeclarationCatalog { detail: Box<str> },
    ImportAliases(StaticImportAliasViewErrorV1),
    WholeSourceInventory(WholeSourceStaticCallTargetInventoryErrorV1),
    CandidateInventory(PreloopStageBSourceInventoryErrorV1),
    CandidateSelection(PreloopStageBCandidateSelectionErrorV1),
    AmbiguousCandidates { count: usize },
}

#[derive(Debug)]
enum RetainedPreloopStageBWholeSourceSelectionOwnerV1 {
    Request(LegacyWholeSourceCompileRequestV1),
    CandidateSelection {
        request: LegacyWholeSourceCompileRequestV1,
        rejected: RejectedPreloopStageBCandidateSelectionV1,
    },
    Ambiguous {
        request: LegacyWholeSourceCompileRequestV1,
        evidence: VerifiedPreloopStageBAmbiguousCandidatesV1,
    },
}

#[derive(Debug)]
pub(super) struct RejectedPreloopStageBWholeSourceSelectionV1 {
    owner: RetainedPreloopStageBWholeSourceSelectionOwnerV1,
    stage: PreloopStageBWholeSourceSelectionStageV1,
    cause: PreloopStageBWholeSourceSelectionErrorV1,
}

impl RejectedPreloopStageBWholeSourceSelectionV1 {
    pub(super) const fn stage(&self) -> PreloopStageBWholeSourceSelectionStageV1 {
        self.stage
    }

    pub(super) const fn cause(&self) -> &PreloopStageBWholeSourceSelectionErrorV1 {
        &self.cause
    }

    pub(super) fn bounded_report(&self) -> Box<str> {
        format!(
            "[mir/preloop-stageb/source-selection/{:?}] {:?}",
            self.stage, self.cause
        )
        .into_boxed_str()
    }

    pub(super) fn discard(self) {
        match self.owner {
            RetainedPreloopStageBWholeSourceSelectionOwnerV1::Request(request) => {
                request.discard();
            }
            RetainedPreloopStageBWholeSourceSelectionOwnerV1::CandidateSelection {
                request,
                rejected,
            } => {
                request.discard();
                rejected.discard();
            }
            RetainedPreloopStageBWholeSourceSelectionOwnerV1::Ambiguous { request, evidence } => {
                request.discard();
                evidence.discard();
            }
        }
    }
}

pub(super) struct PreloopStageBWholeSourceProducerV1;

impl PreloopStageBWholeSourceProducerV1 {
    pub(super) fn select(
        request: LegacyWholeSourceCompileRequestV1,
    ) -> Result<PreloopStageBWholeSourceDispositionV1, RejectedPreloopStageBWholeSourceSelectionV1>
    {
        if request.origin() != LegacyModuleOriginV1::BareAst {
            let origin = request.origin();
            return Ok(PreloopStageBWholeSourceDispositionV1::Ordinary(
                PreparedOrdinaryLegacyWholeSourceV1 {
                    request,
                    disposition: PreloopStageBUnavailableDispositionV1::ProfileExcluded { origin },
                },
            ));
        }

        let declarations =
            match VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(request.ast()) {
                Ok(declarations) => Box::new(declarations),
                Err(cause) => {
                    return Err(reject_request(
                        request,
                        PreloopStageBWholeSourceSelectionStageV1::DeclarationCatalog,
                        PreloopStageBWholeSourceSelectionErrorV1::DeclarationCatalog {
                            detail: format!("{cause:?}").into_boxed_str(),
                        },
                    ));
                }
            };
        let aliases = match request.verify_alias_view(declarations.as_ref()) {
            Ok(aliases) => aliases,
            Err(cause) => {
                return Err(reject_request(
                    request,
                    PreloopStageBWholeSourceSelectionStageV1::ImportAliases,
                    PreloopStageBWholeSourceSelectionErrorV1::ImportAliases(cause),
                ));
            }
        };
        let calls = match VerifiedWholeSourceStaticCallTargetInventoryV1::verify(
            declarations.as_ref(),
            &aliases,
        ) {
            Ok(calls) => calls,
            Err(cause) => {
                return Err(reject_request(
                    request,
                    PreloopStageBWholeSourceSelectionStageV1::WholeSourceInventory,
                    PreloopStageBWholeSourceSelectionErrorV1::WholeSourceInventory(cause),
                ));
            }
        };
        let candidates = match inventory_preloop_stageb_candidates_v1(&calls) {
            Ok(candidates) => candidates,
            Err(cause) => {
                return Err(reject_request(
                    request,
                    PreloopStageBWholeSourceSelectionStageV1::CandidateInventory,
                    PreloopStageBWholeSourceSelectionErrorV1::CandidateInventory(cause),
                ));
            }
        };
        drop(calls);
        drop(aliases);

        let selection = match seal_preloop_stageb_candidate_selection_v1(declarations, candidates) {
            Ok(selection) => selection,
            Err(rejected) => {
                let cause = rejected.cause().clone();
                return Err(RejectedPreloopStageBWholeSourceSelectionV1 {
                    owner: RetainedPreloopStageBWholeSourceSelectionOwnerV1::CandidateSelection {
                        request,
                        rejected,
                    },
                    stage: PreloopStageBWholeSourceSelectionStageV1::CandidateSelection,
                    cause: PreloopStageBWholeSourceSelectionErrorV1::CandidateSelection(cause),
                });
            }
        };
        match selection {
            VerifiedPreloopStageBCandidateSelectionV1::Zero(receipt) => {
                receipt.discard();
                Ok(PreloopStageBWholeSourceDispositionV1::Ordinary(
                    PreparedOrdinaryLegacyWholeSourceV1 {
                        request,
                        disposition: PreloopStageBUnavailableDispositionV1::NoExactCandidate,
                    },
                ))
            }
            VerifiedPreloopStageBCandidateSelectionV1::One(selected) => {
                Ok(PreloopStageBWholeSourceDispositionV1::Selected(
                    PreparedSelectedPreloopStageBWholeSourceV1 {
                        request,
                        activation: selected.into_activation(),
                    },
                ))
            }
            VerifiedPreloopStageBCandidateSelectionV1::Many(evidence) => {
                let count = evidence.candidate_count();
                Err(RejectedPreloopStageBWholeSourceSelectionV1 {
                    owner: RetainedPreloopStageBWholeSourceSelectionOwnerV1::Ambiguous {
                        request,
                        evidence,
                    },
                    stage: PreloopStageBWholeSourceSelectionStageV1::CandidateCardinality,
                    cause: PreloopStageBWholeSourceSelectionErrorV1::AmbiguousCandidates { count },
                })
            }
        }
    }
}

fn reject_request(
    request: LegacyWholeSourceCompileRequestV1,
    stage: PreloopStageBWholeSourceSelectionStageV1,
    cause: PreloopStageBWholeSourceSelectionErrorV1,
) -> RejectedPreloopStageBWholeSourceSelectionV1 {
    RejectedPreloopStageBWholeSourceSelectionV1 {
        owner: RetainedPreloopStageBWholeSourceSelectionOwnerV1::Request(request),
        stage,
        cause,
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1;
    use crate::mir::preloop_stageb_carrier::{
        inventory_preloop_stageb_candidates_v1, seal_preloop_stageb_candidate_selection_v1,
        PreloopStageBCandidateSelectionErrorV1,
    };
    use crate::mir::source_call_target::{
        VerifiedStaticImportAliasViewV1, VerifiedWholeSourceStaticCallTargetInventoryV1,
    };
    use crate::parser::NyashParser;

    use super::super::legacy_static_import_snapshot::CompilerSuppliedStaticImportSnapshotV1;
    use super::super::legacy_whole_source_request::LegacyWholeSourceCompileRequestV1;
    use super::super::lowering_input::LegacyModuleLoweringInputV1;
    use super::{
        PreloopStageBUnavailableDispositionV1, PreloopStageBWholeSourceDispositionV1,
        PreloopStageBWholeSourceProducerV1, PreloopStageBWholeSourceSelectionErrorV1,
        PreloopStageBWholeSourceSelectionStageV1,
    };

    const ZERO: &str = r#"
static box Carrier {
  keep(left, right) { return right }
}
"#;

    const ONE_DIRECT: &str = r#"
static box Carrier {
  keep(left, right) { return right }
}
box Caller {
  inner(value) { return 1 }
  run(text, pos) { pos = Carrier.keep(text, me.inner(pos)) }
}
"#;

    const ONE_ALIAS: &str = r#"
static box Carrier {
  keep(left, right) { return right }
}
box Caller {
  inner(value) { return 1 }
  run(text, pos) { pos = Alias.keep(text, me.inner(pos)) }
}
"#;

    const MANY: &str = r#"
static box Carrier {
  keep(left, right) { return right }
}
box Caller {
  inner(value) { return 1 }
  first(text, pos) { pos = Carrier.keep(text, me.inner(pos)) }
  second(text, pos) { pos = Carrier.keep(text, me.inner(pos)) }
}
"#;

    const UNRELATED: &str = r#"
static box Carrier {
  keep(left, right) { return left }
}
box Caller {
  inner(value) { return 1 }
  run(text, pos) { pos = Carrier.keep(text, me.inner(pos)) }
}
"#;

    const GENERAL_RESULT_ALREADY_AVAILABLE: &str = r#"
static box Carrier {
  keep(left, right) { return right }
}
static box Caller {
  exact(value) { return Carrier.keep(1, value) }
}
"#;

    fn request(
        source: &str,
        imports: CompilerSuppliedStaticImportSnapshotV1,
    ) -> LegacyWholeSourceCompileRequestV1 {
        let ast = NyashParser::parse_from_string(source).expect("selection fixture");
        LegacyWholeSourceCompileRequestV1::new(
            LegacyModuleLoweringInputV1::bare_ast(ast),
            imports,
            Some("selection-fixture.hako".into()),
        )
    }

    #[test]
    fn zero_one_and_many_have_distinct_terminal_products() {
        let zero = PreloopStageBWholeSourceProducerV1::select(request(
            ZERO,
            CompilerSuppliedStaticImportSnapshotV1::none(),
        ))
        .unwrap();
        let PreloopStageBWholeSourceDispositionV1::Ordinary(zero) = zero else {
            panic!("zero complete candidates must be explicit Ordinary");
        };
        assert_eq!(
            zero.disposition(),
            &PreloopStageBUnavailableDispositionV1::NoExactCandidate
        );
        assert_eq!(
            zero.diagnostic_source_hint(),
            Some("selection-fixture.hako")
        );
        zero.discard();

        let one = PreloopStageBWholeSourceProducerV1::select(request(
            ONE_DIRECT,
            CompilerSuppliedStaticImportSnapshotV1::none(),
        ))
        .unwrap();
        let PreloopStageBWholeSourceDispositionV1::Selected(one) = one else {
            panic!("one complete candidate must be Selected");
        };
        assert_eq!(one.activation().row().caller().owner(), "Caller");
        assert_eq!(one.activation().row().caller().name(), "run");
        one.discard();

        let many = PreloopStageBWholeSourceProducerV1::select(request(
            MANY,
            CompilerSuppliedStaticImportSnapshotV1::none(),
        ))
        .unwrap_err();
        assert_eq!(
            many.stage(),
            PreloopStageBWholeSourceSelectionStageV1::CandidateCardinality
        );
        assert!(matches!(
            many.cause(),
            PreloopStageBWholeSourceSelectionErrorV1::AmbiguousCandidates { count: 2 }
        ));
        let super::RetainedPreloopStageBWholeSourceSelectionOwnerV1::Ambiguous { evidence, .. } =
            &many.owner
        else {
            panic!("many rejection must retain the complete ambiguity owner");
        };
        assert!(evidence.is_branded_by_exact_catalog());
        assert_eq!(evidence.candidate_identities().count(), 2);
        assert!(many.bounded_report().contains("AmbiguousCandidates"));
        many.discard();
    }

    #[test]
    fn supplied_alias_can_select_but_missing_owner_is_a_typed_rejection() {
        let selected = PreloopStageBWholeSourceProducerV1::select(request(
            ONE_ALIAS,
            CompilerSuppliedStaticImportSnapshotV1::explicit([(
                "Alias".to_owned(),
                "Carrier".to_owned(),
            )])
            .unwrap(),
        ))
        .unwrap();
        assert!(matches!(
            selected,
            PreloopStageBWholeSourceDispositionV1::Selected(_)
        ));

        let rejected = PreloopStageBWholeSourceProducerV1::select(request(
            ONE_ALIAS,
            CompilerSuppliedStaticImportSnapshotV1::explicit([(
                "Alias".to_owned(),
                "Missing".to_owned(),
            )])
            .unwrap(),
        ))
        .unwrap_err();
        assert_eq!(
            rejected.stage(),
            PreloopStageBWholeSourceSelectionStageV1::ImportAliases
        );
        rejected.discard();
    }

    #[test]
    fn compatibility_and_non_program_inputs_remain_explicit_ordinary() {
        let syntax = ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        };
        let invalid_for_any_catalog = || {
            CompilerSuppliedStaticImportSnapshotV1::explicit([(
                "Alias".to_owned(),
                "Missing".to_owned(),
            )])
            .unwrap()
        };
        let compatibility = LegacyWholeSourceCompileRequestV1::new(
            LegacyModuleLoweringInputV1::program_v0_compatibility(syntax.clone()),
            invalid_for_any_catalog(),
            None,
        );
        let compatibility = PreloopStageBWholeSourceProducerV1::select(compatibility).unwrap();
        let PreloopStageBWholeSourceDispositionV1::Ordinary(compatibility) = compatibility else {
            panic!("ProgramV0 compatibility must be profile-excluded Ordinary");
        };
        assert!(matches!(
            compatibility.disposition(),
            PreloopStageBUnavailableDispositionV1::ProfileExcluded { .. }
        ));

        let repl = LegacyWholeSourceCompileRequestV1::new(
            LegacyModuleLoweringInputV1::repl_compatibility(syntax.clone()),
            invalid_for_any_catalog(),
            None,
        );
        let repl = PreloopStageBWholeSourceProducerV1::select(repl).unwrap();
        assert!(matches!(
            repl,
            PreloopStageBWholeSourceDispositionV1::Ordinary(_)
        ));

        let non_program = LegacyWholeSourceCompileRequestV1::new(
            LegacyModuleLoweringInputV1::bare_ast(syntax),
            CompilerSuppliedStaticImportSnapshotV1::none(),
            None,
        );
        let non_program = PreloopStageBWholeSourceProducerV1::select(non_program).unwrap();
        let PreloopStageBWholeSourceDispositionV1::Ordinary(non_program) = non_program else {
            panic!("non-Program BareAst must remain candidate-zero Ordinary");
        };
        assert_eq!(
            non_program.disposition(),
            &PreloopStageBUnavailableDispositionV1::NoExactCandidate
        );
    }

    #[test]
    fn unrelated_and_general_result_shapes_remain_complete_zero_candidates() {
        for source in [UNRELATED, GENERAL_RESULT_ALREADY_AVAILABLE] {
            let disposition = PreloopStageBWholeSourceProducerV1::select(request(
                source,
                CompilerSuppliedStaticImportSnapshotV1::none(),
            ))
            .unwrap();
            let PreloopStageBWholeSourceDispositionV1::Ordinary(ordinary) = disposition else {
                panic!("existing non-Stage-B result authority must not become selected");
            };
            assert_eq!(
                ordinary.disposition(),
                &PreloopStageBUnavailableDispositionV1::NoExactCandidate
            );
        }
    }

    #[test]
    fn equal_looking_foreign_catalog_is_rejected_before_cardinality() {
        let ast = NyashParser::parse_from_string(ONE_DIRECT).unwrap();
        let primary =
            Box::new(VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&ast).unwrap());
        let imports =
            VerifiedStaticImportAliasViewV1::seal(primary.as_ref(), std::iter::empty()).unwrap();
        let calls =
            VerifiedWholeSourceStaticCallTargetInventoryV1::verify(primary.as_ref(), &imports)
                .unwrap();
        let candidates = inventory_preloop_stageb_candidates_v1(&calls).unwrap();
        drop(calls);
        drop(imports);

        let foreign =
            Box::new(VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&ast).unwrap());
        let rejected = seal_preloop_stageb_candidate_selection_v1(foreign, candidates).unwrap_err();
        assert_eq!(
            rejected.cause(),
            &PreloopStageBCandidateSelectionErrorV1::CatalogAllocationMismatch
        );
        rejected.discard();
        let _ = primary;
    }
}
