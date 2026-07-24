//! FINAL0-PHYSICAL0: witness-to-module finalization readiness.
//!
//! The terminal consumes the opaque DRAIN0 physical product.  It never
//! reprojects source facts, mutates the candidate module, or returns a bare
//! module to the compiler.

use super::drain_terminal::{
    RawDrainWitnessV1, RawDrainedPhysicalV1, RawFinalizedModuleV1, RawUnfinalizedModuleV1,
};
use crate::mir::builder::module_invocation_identity::ModuleInvocationFamilyV1;
use crate::mir::builder::module_invocation_session::{
    BuilderCommitReadinessErrorV1, PreparedBuilderModuleSessionV1,
};
use crate::mir::raw_finalization_contract::{
    RawFinalizationRouteEvidenceV1, RawFinalizationRouteKindV1,
};
use crate::mir::raw_physical_drain::{
    RawPhysicalCallableMainDispositionV1, RawPhysicalDrainRoleV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawPhysicalFinalizationErrorV1 {
    NonRawFamily,
    ForeignBrand,
    ModuleNameMismatch {
        expected: Box<str>,
        actual: Box<str>,
    },
    ManifestRouteMismatch,
    HelperEvidenceMismatch {
        expected: usize,
        actual: usize,
    },
    CallableMainEvidenceMismatch,
    FunctionCountMismatch {
        expected: usize,
        actual: usize,
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
    RootWitnessMismatch,
    BuilderReadiness(BuilderCommitReadinessErrorV1),
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawPhysicalFinalizationV1 {
    owner: RawDrainedPhysicalV1,
    error: RawPhysicalFinalizationErrorV1,
    _seal: RejectedRawPhysicalFinalizationSealV1,
}

#[derive(Debug)]
struct RejectedRawPhysicalFinalizationSealV1;

#[derive(Debug)]
pub(in crate::mir) struct PreparedRawDrainedPhysicalFinalizationV1 {
    token: crate::mir::module_invocation_identity::ModuleInvocationTokenV1,
    builder: PreparedBuilderModuleSessionV1,
    candidate: RawUnfinalizedModuleV1,
    witness: RawDrainWitnessV1,
    parity: RawFinalizationParitySealV1,
    _seal: PreparedRawDrainedPhysicalFinalizationSealV1,
}

#[derive(Debug)]
struct PreparedRawDrainedPhysicalFinalizationSealV1;

#[derive(Debug)]
pub(in crate::mir) struct RawFinalizationParitySealV1 {
    brand: crate::mir::module_invocation_identity::ModuleInvocationBrandV1,
    function_count: usize,
    _seal: RawFinalizationParitySealSealV1,
}

impl RawFinalizationParitySealV1 {
    pub(in crate::mir) const fn brand(&self) -> crate::mir::module_invocation_identity::ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir) const fn function_count(&self) -> usize {
        self.function_count
    }
}

#[derive(Debug)]
struct RawFinalizationParitySealSealV1;

#[derive(Debug)]
pub(in crate::mir) struct RawFinalizedPhysicalV1 {
    token: crate::mir::module_invocation_identity::ModuleInvocationTokenV1,
    builder: PreparedBuilderModuleSessionV1,
    module: RawFinalizedModuleV1,
    witness: RawDrainWitnessV1,
    parity: RawFinalizationParitySealV1,
    _seal: RawFinalizedPhysicalSealV1,
}

#[derive(Debug)]
struct RawFinalizedPhysicalSealV1;

impl RawFinalizedPhysicalV1 {
    pub(in crate::mir::builder) fn into_postprocess_parts(
        self,
    ) -> (
        crate::mir::module_invocation_identity::ModuleInvocationTokenV1,
        PreparedBuilderModuleSessionV1,
        RawFinalizedModuleV1,
        RawDrainWitnessV1,
        RawFinalizationParitySealV1,
    ) {
        let Self {
            token,
            builder,
            module,
            witness,
            parity,
            _seal: _,
        } = self;
        (token, builder, module, witness, parity)
    }
}

impl RawDrainedPhysicalV1 {
    pub(in crate::mir) fn prepare_raw_finalization(
        self,
        route: RawFinalizationRouteEvidenceV1<'_>,
    ) -> Result<PreparedRawDrainedPhysicalFinalizationV1, RejectedRawPhysicalFinalizationV1> {
        let physical = match validate_route_and_module(self, route) {
            Ok(physical) => physical,
            Err(rejected) => return Err(rejected),
        };
        let expected = physical.token.brand();
        if physical.token.family() != ModuleInvocationFamilyV1::Raw
            || physical.session.family() != ModuleInvocationFamilyV1::Raw
            || physical.session.brand() != expected
            || physical.witness.manifest().brand() != expected
            || physical.witness.root().brand() != expected
        {
            return Err(reject(
                physical,
                RawPhysicalFinalizationErrorV1::ForeignBrand,
            ));
        }

        let RawDrainedPhysicalV1 {
            token,
            session,
            candidate,
            witness,
            _seal: _,
        } = physical;
        let builder = match session.prepare_module_session() {
            Ok(builder) => builder,
            Err(rejected) => {
                let (session, error) = rejected.into_parts();
                return Err(reject(
                    RawDrainedPhysicalV1 {
                        token,
                        session,
                        candidate,
                        witness,
                        _seal: super::drain_terminal::RawDrainedPhysicalSealV1,
                    },
                    RawPhysicalFinalizationErrorV1::BuilderReadiness(error),
                ));
            }
        };
        let parity = RawFinalizationParitySealV1 {
            brand: expected,
            function_count: candidate.function_count(),
            _seal: RawFinalizationParitySealSealV1,
        };
        Ok(PreparedRawDrainedPhysicalFinalizationV1 {
            token,
            builder,
            candidate,
            witness,
            parity,
            _seal: PreparedRawDrainedPhysicalFinalizationSealV1,
        })
    }
}

impl PreparedRawDrainedPhysicalFinalizationV1 {
    pub(in crate::mir) fn commit(self) -> RawFinalizedPhysicalV1 {
        let Self {
            token,
            builder,
            candidate,
            witness,
            parity,
            _seal: _,
        } = self;
        RawFinalizedPhysicalV1 {
            token,
            builder,
            module: candidate.finalize(),
            witness,
            parity,
            _seal: RawFinalizedPhysicalSealV1,
        }
    }
}

impl RejectedRawPhysicalFinalizationV1 {
    pub(in crate::mir) fn error(&self) -> &RawPhysicalFinalizationErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {}
}

fn reject(
    owner: RawDrainedPhysicalV1,
    error: RawPhysicalFinalizationErrorV1,
) -> RejectedRawPhysicalFinalizationV1 {
    RejectedRawPhysicalFinalizationV1 {
        owner,
        error,
        _seal: RejectedRawPhysicalFinalizationSealV1,
    }
}

fn validate_route_and_module(
    physical: RawDrainedPhysicalV1,
    route: RawFinalizationRouteEvidenceV1<'_>,
) -> Result<RawDrainedPhysicalV1, RejectedRawPhysicalFinalizationV1> {
    let error = validate_route_and_module_borrowed(&physical, route);
    match error {
        Ok(()) => Ok(physical),
        Err(error) => Err(reject(physical, error)),
    }
}

fn validate_route_and_module_borrowed(
    physical: &RawDrainedPhysicalV1,
    route: RawFinalizationRouteEvidenceV1<'_>,
) -> Result<(), RawPhysicalFinalizationErrorV1> {
    let manifest = physical.witness.manifest();
    if manifest.route() != route_kind(route) {
        return Err(RawPhysicalFinalizationErrorV1::ManifestRouteMismatch);
    }
    if physical.candidate.name() != route.name() {
        return Err(RawPhysicalFinalizationErrorV1::ModuleNameMismatch {
            expected: route.name().into(),
            actual: physical.candidate.name().into(),
        });
    }
    let helper_count = manifest
        .rows()
        .iter()
        .filter(|row| row.role() == RawPhysicalDrainRoleV1::StaticHelper)
        .count();
    if helper_count != route.helper_count() {
        return Err(RawPhysicalFinalizationErrorV1::HelperEvidenceMismatch {
            expected: helper_count,
            actual: route.helper_count(),
        });
    }
    let expected_callable = route.callable_main();
    let actual_callable = if manifest
        .rows()
        .iter()
        .any(|row| row.role() == RawPhysicalDrainRoleV1::CallableMainCompatibility)
    {
        RawPhysicalCallableMainDispositionV1::Selected
    } else {
        RawPhysicalCallableMainDispositionV1::NotSelected
    };
    if actual_callable != expected_callable || manifest.callable_main() != expected_callable {
        return Err(RawPhysicalFinalizationErrorV1::CallableMainEvidenceMismatch);
    }
    let expected = manifest.rows().len();
    let actual = physical.candidate.function_count();
    if actual != expected {
        return Err(RawPhysicalFinalizationErrorV1::FunctionCountMismatch { expected, actual });
    }
    for row in manifest.rows() {
        let Some(function) = physical.candidate.function(row.symbol()) else {
            return Err(RawPhysicalFinalizationErrorV1::MissingFunction {
                symbol: row.symbol().into(),
            });
        };
        if function.signature.name != row.symbol() {
            return Err(RawPhysicalFinalizationErrorV1::MissingFunction {
                symbol: row.symbol().into(),
            });
        }
        let actual_arity = function.signature.params.len();
        if actual_arity != row.arity() {
            return Err(RawPhysicalFinalizationErrorV1::FunctionArityMismatch {
                symbol: row.symbol().into(),
                expected: row.arity(),
                actual: actual_arity,
            });
        }
    }
    for symbol in physical.candidate.symbols() {
        if !manifest.rows().iter().any(|row| row.symbol() == symbol) {
            return Err(RawPhysicalFinalizationErrorV1::SurplusFunction {
                symbol: symbol.clone().into_boxed_str(),
            });
        }
    }
    Ok(())
}

fn route_kind(
    route: RawFinalizationRouteEvidenceV1<'_>,
) -> crate::mir::raw_physical_drain::RawPhysicalDrainRouteV1 {
    match route {
        RawFinalizationRouteEvidenceV1::Script { .. } => {
            crate::mir::raw_physical_drain::RawPhysicalDrainRouteV1::Script
        }
        RawFinalizationRouteEvidenceV1::App { .. } => {
            crate::mir::raw_physical_drain::RawPhysicalDrainRouteV1::App
        }
    }
}
