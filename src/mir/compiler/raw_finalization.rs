//! CUT0-I0-POST0-RAW-FINALIZER0: Raw finalization boundary.
//!
//! This module consumes the disconnected Raw physical input and turns it into
//! a route-specific compiler product.  It deliberately does not call the
//! legacy `MirBuilder::finalize_module`; POST0 will consume this product later.

use crate::mir::builder::{
    BuilderCommitReadinessErrorV1, ModuleBuilderInvocationSessionV1,
    PreparedBuilderModuleSessionV1, RawFinalizationInputV1 as RawPhysicalFinalizationInputV1,
    RawInvocationRootWitnessV1, SealedRawExpansionReceiptLedgerV1,
};
use crate::mir::module_invocation_identity::{ModuleInvocationFamilyV1, ModuleInvocationTokenV1};
use crate::mir::MirModule;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawFinalizationErrorV1 {
    NonRawFamily,
    ForeignBrand,
    BuilderReadiness(BuilderCommitReadinessErrorV1),
}

#[derive(Debug)]
pub(in crate::mir) struct RawFinalizationInputV1 {
    pub(in crate::mir) token: ModuleInvocationTokenV1,
    pub(in crate::mir) builder: PreparedBuilderModuleSessionV1,
    pub(in crate::mir) module: MirModule,
    pub(in crate::mir) ledger: SealedRawExpansionReceiptLedgerV1,
    pub(in crate::mir) root: RawInvocationRootWitnessV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RawFinalizedModuleInvocationV1 {
    pub(in crate::mir) input: RawFinalizationInputV1,
    _seal: RawFinalizedModuleInvocationSealV1,
}

#[derive(Debug)]
struct RawFinalizedModuleInvocationSealV1;

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawFinalizationV1 {
    pub(in crate::mir) owner: RawRejectedFinalizationOwnerV1,
    pub(in crate::mir) error: RawFinalizationErrorV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RawRejectedFinalizationOwnerV1 {
    pub(in crate::mir) token: ModuleInvocationTokenV1,
    pub(in crate::mir) session: ModuleBuilderInvocationSessionV1,
    pub(in crate::mir) module: MirModule,
    pub(in crate::mir) ledger: SealedRawExpansionReceiptLedgerV1,
    pub(in crate::mir) root: RawInvocationRootWitnessV1,
}

pub(in crate::mir) struct RawModuleFinalizerV1;

impl RawModuleFinalizerV1 {
    pub(in crate::mir) fn prepare(
        physical: RawPhysicalFinalizationInputV1,
    ) -> Result<RawFinalizationInputV1, RejectedRawFinalizationV1> {
        let RawPhysicalFinalizationInputV1 {
            token,
            session,
            module,
            ledger,
            root,
        } = physical;
        if token.family() != ModuleInvocationFamilyV1::Raw {
            return Err(RejectedRawFinalizationV1 {
                owner: RawRejectedFinalizationOwnerV1 {
                    token,
                    session,
                    module,
                    ledger,
                    root,
                },
                error: RawFinalizationErrorV1::NonRawFamily,
            });
        }
        let expected = token.brand();
        if session.brand() != expected
            || session.family() != ModuleInvocationFamilyV1::Raw
            || ledger.brand() != expected
            || root.brand() != expected
        {
            return Err(RejectedRawFinalizationV1 {
                owner: RawRejectedFinalizationOwnerV1 {
                    token,
                    session,
                    module,
                    ledger,
                    root,
                },
                error: RawFinalizationErrorV1::ForeignBrand,
            });
        }
        let builder = match session.prepare_module_session() {
            Ok(prepared) => prepared,
            Err(rejected) => {
                let error = rejected.error().clone();
                let session = rejected.into_parts().0;
                return Err(RejectedRawFinalizationV1 {
                    owner: RawRejectedFinalizationOwnerV1 {
                        token,
                        session,
                        module,
                        ledger,
                        root,
                    },
                    error: RawFinalizationErrorV1::BuilderReadiness(error),
                });
            }
        };
        if builder.brand() != expected || builder.family() != ModuleInvocationFamilyV1::Raw {
            let (_brand, _family, session) = builder.into_parts();
            return Err(RejectedRawFinalizationV1 {
                owner: RawRejectedFinalizationOwnerV1 {
                    token,
                    session,
                    module,
                    ledger,
                    root,
                },
                error: RawFinalizationErrorV1::ForeignBrand,
            });
        }
        Ok(RawFinalizationInputV1 {
            token,
            builder,
            module,
            ledger,
            root,
        })
    }

    pub(in crate::mir) fn finalize(
        input: RawFinalizationInputV1,
    ) -> RawFinalizedModuleInvocationV1 {
        RawFinalizedModuleInvocationV1 {
            input,
            _seal: RawFinalizedModuleInvocationSealV1,
        }
    }
}
