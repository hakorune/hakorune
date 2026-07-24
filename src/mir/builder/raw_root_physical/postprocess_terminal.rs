//! POST-CARRIER: private Raw module mutation capability.
//!
//! The compiler owns postprocess stage order, while this Builder-side owner
//! keeps the finalized module opaque.  Only named stage operations cross the
//! module boundary; no `MirModule` accessor or caller mutation closure exists.

use super::super::module_invocation_identity::{ModuleInvocationBrandV1, ModuleInvocationTokenV1};
use super::super::module_invocation_session::PreparedBuilderModuleSessionV1;
use super::drain_terminal::{RawDrainWitnessV1, RawFinalizedModuleV1};
use super::finalization_terminal::{RawFinalizationParitySealV1, RawFinalizedPhysicalV1};
use crate::mir::optimizer_stats::OptimizationStats;
use crate::mir::verification::MirVerifier;
use crate::mir::verification_types::VerificationError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawPostprocessCarrierParityErrorV1 {
    ModuleNameMismatch {
        expected: Box<str>,
        actual: Box<str>,
    },
    MissingFunction {
        symbol: Box<str>,
    },
    SignatureNameMismatch {
        key: Box<str>,
        signature: Box<str>,
    },
    SurplusFunction {
        symbol: Box<str>,
    },
    FunctionArityMismatch {
        symbol: Box<str>,
        expected: usize,
        actual: usize,
    },
    FunctionCountMismatch {
        expected: usize,
        actual: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawPostprocessProgressV1 {
    Ready,
    RunePlansRefreshed,
    Optimized,
    ContractsRefreshed,
    PreTransformObserved,
    RcInserted,
    SemanticMetadataRefreshed,
    CallsitesCanonicalized,
    ParitySealed,
}

#[derive(Debug)]
pub(in crate::mir) struct RawPostprocessModuleLoanV1 {
    module: RawFinalizedModuleV1,
    progress: RawPostprocessProgressV1,
    _seal: RawPostprocessModuleLoanSealV1,
}

#[derive(Debug)]
struct RawPostprocessModuleLoanSealV1;

#[derive(Debug)]
pub(in crate::mir) struct RawPostprocessParitySealV1 {
    brand: ModuleInvocationBrandV1,
    function_count: usize,
    _seal: RawPostprocessParitySealInnerV1,
}

#[derive(Debug)]
struct RawPostprocessParitySealInnerV1;

#[derive(Debug)]
pub(in crate::mir) struct RawPostprocessPhysicalOwnerV1 {
    token: ModuleInvocationTokenV1,
    builder: PreparedBuilderModuleSessionV1,
    loan: RawPostprocessModuleLoanV1,
    witness: RawDrainWitnessV1,
    finalization_parity: RawFinalizationParitySealV1,
    _seal: RawPostprocessPhysicalOwnerSealV1,
}

#[derive(Debug)]
struct RawPostprocessPhysicalOwnerSealV1;

#[derive(Debug)]
pub(in crate::mir) struct RawPostprocessedModuleV1 {
    module: RawPostprocessModuleLoanV1,
    _seal: RawPostprocessedModuleSealV1,
}

#[derive(Debug)]
struct RawPostprocessedModuleSealV1;

#[derive(Debug)]
pub(in crate::mir) struct RawPostprocessedPhysicalV1 {
    token: ModuleInvocationTokenV1,
    builder: PreparedBuilderModuleSessionV1,
    module: RawPostprocessedModuleV1,
    witness: RawDrainWitnessV1,
    finalization_parity: RawFinalizationParitySealV1,
    postprocess_parity: RawPostprocessParitySealV1,
    progress: RawPostprocessProgressV1,
    _seal: RawPostprocessedPhysicalSealV1,
}

#[derive(Debug)]
struct RawPostprocessedPhysicalSealV1;

impl RawFinalizedPhysicalV1 {
    pub(in crate::mir) fn begin_postprocess(self) -> RawPostprocessPhysicalOwnerV1 {
        RawPostprocessPhysicalOwnerV1::from_finalized(self)
    }
}

impl RawPostprocessPhysicalOwnerV1 {
    fn from_finalized(physical: RawFinalizedPhysicalV1) -> Self {
        let (token, builder, module, witness, parity) = physical.into_postprocess_parts();
        Self {
            token,
            builder,
            loan: RawPostprocessModuleLoanV1 {
                module,
                progress: RawPostprocessProgressV1::Ready,
                _seal: RawPostprocessModuleLoanSealV1,
            },
            witness,
            finalization_parity: parity,
            _seal: RawPostprocessPhysicalOwnerSealV1,
        }
    }

    pub(in crate::mir) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.token.brand()
    }

    pub(in crate::mir) fn module_name(&self) -> &str {
        self.loan.module.name()
    }

    pub(in crate::mir) fn function_count(&self) -> usize {
        self.loan.module.function_count()
    }

    pub(in crate::mir) fn function_arity(&self, symbol: &str) -> Option<usize> {
        self.loan
            .module
            .function(symbol)
            .map(|function| function.signature.params.len())
    }

    pub(in crate::mir) fn function_signature_name(&self, symbol: &str) -> Option<&str> {
        self.loan
            .module
            .function(symbol)
            .map(|function| function.signature.name.as_str())
    }

    pub(in crate::mir) fn progress(&self) -> RawPostprocessProgressV1 {
        self.loan.progress
    }

    pub(in crate::mir) fn symbols(&self) -> impl Iterator<Item = &String> {
        self.loan.module.symbols()
    }

    pub(in crate::mir) fn refresh_rune_plans(&mut self) {
        self.loan.module.refresh_rune_plans();
        self.loan.progress = RawPostprocessProgressV1::RunePlansRefreshed;
    }

    pub(in crate::mir) fn optimize(&mut self) -> OptimizationStats {
        let stats = self.loan.module.optimize();
        self.loan.progress = RawPostprocessProgressV1::Optimized;
        stats
    }

    pub(in crate::mir) fn refresh_contracts(&mut self) -> Result<(), String> {
        let result = self.loan.module.refresh_contracts();
        if result.is_ok() {
            self.loan.progress = RawPostprocessProgressV1::ContractsRefreshed;
        }
        result
    }

    pub(in crate::mir) fn verify(
        &mut self,
        verifier: &mut MirVerifier,
    ) -> Result<(), Box<[VerificationError]>> {
        let result = self.loan.module.verify(verifier);
        self.loan.progress = RawPostprocessProgressV1::PreTransformObserved;
        result
    }

    pub(in crate::mir) fn insert_rc(&mut self) {
        self.loan.module.insert_rc();
        self.loan.progress = RawPostprocessProgressV1::RcInserted;
    }

    pub(in crate::mir) fn refresh_semantic_metadata(&mut self) {
        self.loan.module.refresh_semantic_metadata();
        self.loan.progress = RawPostprocessProgressV1::SemanticMetadataRefreshed;
    }

    pub(in crate::mir) fn canonicalize_callsites(&mut self) -> usize {
        let changed = self.loan.module.canonicalize_callsites();
        self.loan.progress = RawPostprocessProgressV1::CallsitesCanonicalized;
        changed
    }

    pub(in crate::mir) fn prepare_parity(
        &mut self,
        expected_module_name: &str,
    ) -> Result<RawPostprocessParitySealV1, RawPostprocessCarrierParityErrorV1> {
        if self.module_name() != expected_module_name {
            return Err(RawPostprocessCarrierParityErrorV1::ModuleNameMismatch {
                expected: expected_module_name.into(),
                actual: self.module_name().into(),
            });
        }
        let manifest = self.witness.manifest();
        let expected = manifest.rows().len();
        let actual = self.function_count();
        if expected != actual {
            return Err(RawPostprocessCarrierParityErrorV1::FunctionCountMismatch {
                expected,
                actual,
            });
        }
        for row in manifest.rows() {
            let Some(actual_arity) = self.function_arity(row.symbol()) else {
                return Err(RawPostprocessCarrierParityErrorV1::MissingFunction {
                    symbol: row.symbol().into(),
                });
            };
            if let Some(signature_name) = self.function_signature_name(row.symbol()) {
                if signature_name != row.symbol() {
                    return Err(RawPostprocessCarrierParityErrorV1::SignatureNameMismatch {
                        key: row.symbol().into(),
                        signature: signature_name.into(),
                    });
                }
            }
            if actual_arity != row.arity() {
                return Err(RawPostprocessCarrierParityErrorV1::FunctionArityMismatch {
                    symbol: row.symbol().into(),
                    expected: row.arity(),
                    actual: actual_arity,
                });
            }
        }
        for symbol in self.symbols() {
            if !manifest.rows().iter().any(|row| row.symbol() == symbol) {
                return Err(RawPostprocessCarrierParityErrorV1::SurplusFunction {
                    symbol: symbol.clone().into_boxed_str(),
                });
            }
        }
        self.loan.progress = RawPostprocessProgressV1::ParitySealed;
        Ok(RawPostprocessParitySealV1 {
            brand: self.brand(),
            function_count: actual,
            _seal: RawPostprocessParitySealInnerV1,
        })
    }

    pub(in crate::mir) fn finish(
        self,
        postprocess_parity: RawPostprocessParitySealV1,
    ) -> RawPostprocessedPhysicalV1 {
        let Self {
            token,
            builder,
            loan,
            witness,
            finalization_parity,
            _seal: _,
        } = self;
        let progress = loan.progress;
        RawPostprocessedPhysicalV1 {
            token,
            builder,
            module: RawPostprocessedModuleV1 {
                module: loan,
                _seal: RawPostprocessedModuleSealV1,
            },
            witness,
            finalization_parity,
            postprocess_parity,
            progress,
            _seal: RawPostprocessedPhysicalSealV1,
        }
    }
}

impl RawPostprocessedPhysicalV1 {
    pub(in crate::mir) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.token.brand()
    }

    pub(in crate::mir) fn progress(&self) -> RawPostprocessProgressV1 {
        self.progress
    }
}
