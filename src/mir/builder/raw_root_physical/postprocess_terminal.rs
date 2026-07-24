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

#[derive(Debug)]
pub(in crate::mir) struct RawPostprocessModuleLoanV1 {
    module: RawFinalizedModuleV1,
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
    pub(in crate::mir) token: ModuleInvocationTokenV1,
    pub(in crate::mir) builder: PreparedBuilderModuleSessionV1,
    loan: RawPostprocessModuleLoanV1,
    pub(in crate::mir) witness: RawDrainWitnessV1,
    pub(in crate::mir) finalization_parity: RawFinalizationParitySealV1,
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
    pub(in crate::mir) token: ModuleInvocationTokenV1,
    pub(in crate::mir) builder: PreparedBuilderModuleSessionV1,
    pub(in crate::mir) module: RawPostprocessedModuleV1,
    pub(in crate::mir) witness: RawDrainWitnessV1,
    pub(in crate::mir) finalization_parity: RawFinalizationParitySealV1,
    pub(in crate::mir) postprocess_parity: RawPostprocessParitySealV1,
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

    pub(in crate::mir) fn symbols(&self) -> impl Iterator<Item = &String> {
        self.loan.module.symbols()
    }

    pub(in crate::mir) fn refresh_rune_plans(&mut self) {
        self.loan.module.refresh_rune_plans();
    }

    pub(in crate::mir) fn optimize(&mut self) -> OptimizationStats {
        self.loan.module.optimize()
    }

    pub(in crate::mir) fn refresh_contracts(&mut self) -> Result<(), String> {
        self.loan.module.refresh_contracts()
    }

    pub(in crate::mir) fn verify(
        &mut self,
        verifier: &mut MirVerifier,
    ) -> Result<(), Box<[VerificationError]>> {
        self.loan.module.verify(verifier)
    }

    pub(in crate::mir) fn insert_rc(&mut self) {
        self.loan.module.insert_rc();
    }

    pub(in crate::mir) fn refresh_semantic_metadata(&mut self) {
        self.loan.module.refresh_semantic_metadata();
    }

    pub(in crate::mir) fn canonicalize_callsites(&mut self) -> usize {
        self.loan.module.canonicalize_callsites()
    }

    pub(in crate::mir) fn prepare_parity(
        &self,
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
            _seal: RawPostprocessedPhysicalSealV1,
        }
    }
}

impl RawPostprocessedPhysicalV1 {
    pub(in crate::mir) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.token.brand()
    }
}
