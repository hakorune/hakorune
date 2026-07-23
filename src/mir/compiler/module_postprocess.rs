//! CUT0-I0-POST0: disconnected, family-owned module postprocessing.
//!
//! The existing public finish path remains unchanged until atomic CUT0.  This
//! owner records the same stage order for finalized invocation products and
//! keeps the RC/verifier policy derived from the invocation family.

use super::canonical_finalization::{CanonicalFinalizationInputV1, FinalizedModuleInvocationV1};
use crate::mir::function::MirModule;
use crate::mir::module_invocation_identity::{ModuleInvocationBrandV1, ModuleInvocationFamilyV1};
use crate::mir::optimizer::MirOptimizer;
use crate::mir::passes::rc_insertion::insert_rc_instructions;
use crate::mir::semantic_refresh::{
    refresh_and_validate_for_boundary, refresh_module_semantic_metadata, ContractRefreshBoundary,
};
use crate::mir::verification::MirVerifier;
use crate::mir::verification_types::VerificationError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RcInsertionScheduleV1 {
    Run,
    Skip,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum VerificationBarrierV1 {
    ReportPreTransformOnly,
    RequireFinal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct ModulePostprocessScheduleV1 {
    rc: RcInsertionScheduleV1,
    verifier: VerificationBarrierV1,
}

impl ModulePostprocessScheduleV1 {
    pub(in crate::mir) const fn for_family(family: ModuleInvocationFamilyV1) -> Self {
        match family {
            ModuleInvocationFamilyV1::Raw => Self {
                rc: RcInsertionScheduleV1::Run,
                verifier: VerificationBarrierV1::ReportPreTransformOnly,
            },
            ModuleInvocationFamilyV1::CanonicalAPlus => Self {
                rc: RcInsertionScheduleV1::Run,
                verifier: VerificationBarrierV1::RequireFinal,
            },
            ModuleInvocationFamilyV1::BindingSsaTrivial
            | ModuleInvocationFamilyV1::BindingSsaAcyclic
            | ModuleInvocationFamilyV1::BindingSsaRecursive => Self {
                rc: RcInsertionScheduleV1::Skip,
                verifier: VerificationBarrierV1::RequireFinal,
            },
        }
    }

    pub(in crate::mir) const fn rc(self) -> RcInsertionScheduleV1 {
        self.rc
    }

    pub(in crate::mir) const fn verifier(self) -> VerificationBarrierV1 {
        self.verifier
    }
}

#[derive(Debug)]
pub(in crate::mir) enum ModuleVerificationEvidenceV1 {
    Canonical {
        pre_transform: Result<(), Box<[VerificationError]>>,
        final_verified: CanonicalFinalVerificationSealV1,
    },
}

#[derive(Debug)]
pub(in crate::mir) struct CanonicalFinalVerificationSealV1 {
    _seal: CanonicalFinalVerificationSealInnerV1,
}

#[derive(Debug)]
struct CanonicalFinalVerificationSealInnerV1;

#[derive(Debug)]
pub(in crate::mir) enum ModulePostprocessErrorV1 {
    OptimizerDiagnostics { count: usize },
    ContractRefresh(String),
    FinalVerification(Box<[VerificationError]>),
}

#[derive(Debug)]
pub(in crate::mir) struct PostprocessedModuleInvocationV1<'a> {
    pub(in crate::mir) input: CanonicalFinalizationInputV1<'a>,
    pub(in crate::mir) schedule: ModulePostprocessScheduleV1,
    pub(in crate::mir) verification: ModuleVerificationEvidenceV1,
}

impl<'a> PostprocessedModuleInvocationV1<'a> {
    pub(in crate::mir) const fn brand(&self) -> ModuleInvocationBrandV1 {
        match &self.input {
            CanonicalFinalizationInputV1::Single(input) => input.token.brand(),
            CanonicalFinalizationInputV1::Callable(input) => input.token.brand(),
        }
    }

    pub(in crate::mir) const fn family(&self) -> ModuleInvocationFamilyV1 {
        match &self.input {
            CanonicalFinalizationInputV1::Single(input) => input.token.family(),
            CanonicalFinalizationInputV1::Callable(input) => input.token.family(),
        }
    }

    pub(in crate::mir) fn module(&self) -> &MirModule {
        match &self.input {
            CanonicalFinalizationInputV1::Single(input) => &input.physical.module,
            CanonicalFinalizationInputV1::Callable(input) => &input.physical.module,
        }
    }
}

pub(in crate::mir) struct ModulePostprocessOwnerV1<'a> {
    verifier: &'a mut MirVerifier,
    optimize: bool,
}

impl<'a> ModulePostprocessOwnerV1<'a> {
    pub(in crate::mir) fn new(verifier: &'a mut MirVerifier, optimize: bool) -> Self {
        Self { verifier, optimize }
    }

    pub(in crate::mir) fn run(
        self,
        finalized: FinalizedModuleInvocationV1<'a>,
    ) -> Result<PostprocessedModuleInvocationV1<'a>, ModulePostprocessErrorV1> {
        let FinalizedModuleInvocationV1 { input, .. } = finalized;
        let family = match &input {
            CanonicalFinalizationInputV1::Single(input) => input.token.family(),
            CanonicalFinalizationInputV1::Callable(input) => input.token.family(),
        };
        let schedule = ModulePostprocessScheduleV1::for_family(family);
        process_input(input, schedule, self.verifier, self.optimize)
    }
}

fn process_input<'a>(
    mut input: CanonicalFinalizationInputV1<'a>,
    schedule: ModulePostprocessScheduleV1,
    verifier: &mut MirVerifier,
    optimize: bool,
) -> Result<PostprocessedModuleInvocationV1<'a>, ModulePostprocessErrorV1> {
    let module = match &mut input {
        CanonicalFinalizationInputV1::Single(input) => &mut input.physical.module,
        CanonicalFinalizationInputV1::Callable(input) => &mut input.physical.module,
    };

    crate::mir::rune_plan_refresh::refresh_module_rune_plans(module);
    if optimize {
        let stats = MirOptimizer::new().optimize_module(module);
        if (crate::config::env::opt_diag_fail() || crate::config::env::opt_diag_forbid_legacy())
            && stats.diagnostics_reported > 0
        {
            return Err(ModulePostprocessErrorV1::OptimizerDiagnostics {
                count: stats.diagnostics_reported,
            });
        }
    }
    refresh_and_validate_for_boundary(module, ContractRefreshBoundary::Verifier)
        .map_err(|error| ModulePostprocessErrorV1::ContractRefresh(format!("{error:?}")))?;
    let pre_transform = verifier
        .verify_module(module)
        .map(|()| ())
        .map_err(|errors| errors.into_boxed_slice());
    if matches!(schedule.rc(), RcInsertionScheduleV1::Run) {
        insert_rc_instructions(module);
    }
    refresh_module_semantic_metadata(module);
    let changed = crate::mir::passes::callsite_canonicalize::canonicalize_for_site(
        module,
        crate::mir::passes::callsite_canonicalize::CallsiteCanonicalizeScheduleSite::MirCompilerPostRc,
    );
    if changed > 0 {
        refresh_module_semantic_metadata(module);
    }
    let final_verified = match schedule.verifier() {
        VerificationBarrierV1::ReportPreTransformOnly => CanonicalFinalVerificationSealV1 {
            _seal: CanonicalFinalVerificationSealInnerV1,
        },
        VerificationBarrierV1::RequireFinal => {
            let errors = verifier.verify_module(module);
            if let Err(errors) = errors {
                return Err(ModulePostprocessErrorV1::FinalVerification(
                    errors.into_boxed_slice(),
                ));
            }
            CanonicalFinalVerificationSealV1 {
                _seal: CanonicalFinalVerificationSealInnerV1,
            }
        }
    };
    Ok(PostprocessedModuleInvocationV1 {
        input,
        schedule,
        verification: ModuleVerificationEvidenceV1::Canonical {
            pre_transform,
            final_verified,
        },
    })
}
