//! The existing physical packet retains its original source and affine owner.
//! Static calls own the original publication handoff, never an instance row.
use super::*;
use crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationHandoffV1;
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
use crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1;
use crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1;
use crate::mir::resolved_semantics::{OwnedExprSiteV1, SourceExprSiteV1};
use std::rc::Rc;

#[derive(Debug)]
enum CallPacketSourceKindV1 {
    Instance(LexicalInstanceCallDispositionRowV1),
    Static {
        original: Rc<StaticIncomingSourceV1>,
        observation: LocalCallObservationV1,
        publication: VerifiedStaticCallResultPublicationHandoffV1,
    },
}

#[derive(Debug)]
pub(in crate::mir) struct CallPacketSourceV1 {
    kind: CallPacketSourceKindV1,
}

#[derive(Clone, Copy)]
pub(in crate::mir) enum CallPacketSourceLoanV1<'a> {
    Instance(&'a LexicalInstanceCallDispositionRowV1),
    Static {
        original: &'a Rc<StaticIncomingSourceV1>,
        observation: &'a LocalCallObservationV1,
        publication: &'a VerifiedStaticCallResultPublicationHandoffV1,
    },
}

impl CallPacketSourceV1 {
    pub(in crate::mir) fn instance(row: LexicalInstanceCallDispositionRowV1) -> Self {
        Self {
            kind: CallPacketSourceKindV1::Instance(row),
        }
    }
    pub(in crate::mir) fn loan(&self) -> CallPacketSourceLoanV1<'_> {
        match &self.kind {
            CallPacketSourceKindV1::Instance(row) => CallPacketSourceLoanV1::Instance(row),
            CallPacketSourceKindV1::Static {
                original,
                observation,
                publication,
            } => CallPacketSourceLoanV1::Static {
                original,
                observation,
                publication,
            },
        }
    }

    pub(in crate::mir) fn static_i64(
        original: Rc<StaticIncomingSourceV1>,
        publication: VerifiedStaticCallResultPublicationHandoffV1,
        ledger: &OrdinaryNewClaimLedgerV1,
    ) -> Result<Self, String> {
        let observation = ledger
            .local_call_for_owner(original.call_site().owner(), original.call_site().site())
            .ok_or_else(|| freeze("static-packet/local-source-missing"))?
            .clone();
        let source = Self {
            kind: CallPacketSourceKindV1::Static {
                original,
                observation,
                publication,
            },
        };
        source.loan().validate_static(ledger)?;
        Ok(source)
    }
}

impl<'a> CallPacketSourceLoanV1<'a> {
    pub(in crate::mir) fn require_instance(
        self,
    ) -> Result<&'a LexicalInstanceCallDispositionRowV1, String> {
        match self {
            Self::Instance(row) => Ok(row),
            Self::Static { .. } => Err(freeze("static-packet/instance-source-required")),
        }
    }

    pub(in crate::mir) fn call_site(self) -> &'a OwnedExprSiteV1 {
        match self {
            Self::Instance(row) => row.call_site(),
            Self::Static { original, .. } => original.call_site(),
        }
    }

    pub(in crate::mir) fn target(self) -> &'a hakorune_mir_defs::CanonicalSameModuleCallableKeyV1 {
        match self {
            Self::Instance(row) => row.target(),
            Self::Static { original, .. } => original.target(),
        }
    }

    pub(in crate::mir) fn callee_owner(self) -> FunctionOwnerIdV1 {
        match self {
            Self::Instance(row) => row.callee_owner(),
            Self::Static { original, .. } => original.callee_owner(),
        }
    }

    pub(in crate::mir) fn argument_sites(self) -> &'a [SourceExprSiteV1] {
        match self {
            Self::Instance(row) => row.argument_sites(),
            Self::Static { original, .. } => original.argument_sites(),
        }
    }

    pub(in crate::mir) fn result(self) -> Option<InvokeCallResultKind> {
        match self {
            Self::Instance(row) => row.result(),
            Self::Static { .. } => Some(InvokeCallResultKind::I64),
        }
    }

    pub(in crate::mir) fn validate_static(
        self,
        ledger: &OrdinaryNewClaimLedgerV1,
    ) -> Result<(), String> {
        let Self::Static {
            original,
            observation,
            publication,
        } = self
        else {
            return Ok(());
        };
        ledger.verify_original_static_packet_source_v1(original)?;
        if !original.corroborates_publication_handoff(publication)
            || observation.owner() != original.call_site().owner()
            || observation.site() != original.call_site()
            || observation.result()
                != crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::I64
            || observation.arguments().len() != original.argument_sites().len()
            || observation.arguments().len() != original.target().arity() as usize
            || ledger.local_call_for_owner(observation.owner(), observation.site().site())
                != Some(observation)
        {
            return Err(freeze("static-packet/original-source-drift"));
        }
        // The selected Static packet uses its original complete input proof:
        // a checked zero-input cohort or the existing borrowed entry. An absent
        // borrowed kind never permits a scalar payload for Map/Text/Handle.
        ledger
            .borrowed_static_packet_actuals_v1(original)?
            .ok_or_else(|| freeze("static-packet/actuals-missing"))?;
        for (formal, argument) in original.parameters().iter().zip(observation.arguments()) {
            if formal.kind.is_ordinary_borrowed_handle() {
                if !matches!(argument, LocalCallArgumentV1::BorrowedActual { ordinal, site }
                    if *ordinal == formal.ordinal
                        && original.argument_sites().get(*ordinal as usize) == Some(site))
                {
                    return Err(freeze("static-packet/borrowed-input-source-required"));
                }
            } else if matches!(formal.kind,
                crate::mir::callable_parameter_contract::CallableParameterContractKindV1::ExactTrivial(abi)
                    if abi.is_i64())
            {
                if !matches!(argument, LocalCallArgumentV1::Integer(_))
                    && !matches!(argument, LocalCallArgumentV1::Scalar(binding)
                        if binding.owner() == observation.owner())
                {
                    return Err(freeze("static-packet/scalar-input-source-required"));
                }
            } else {
                return Err(freeze("static-packet/input-contract-unsupported"));
            }
        }
        Ok(())
    }

    pub(in crate::mir) fn borrowed_actuals<'ledger>(
        self,
        ledger: &'ledger OrdinaryNewClaimLedgerV1,
    ) -> Result<
        Option<&'ledger [crate::mir::normal_callable_semantic_package::PreparedBorrowedFormalActualV1]>,
        String,
    >{
        match self {
            Self::Instance(row) => ledger.borrowed_call_actuals_v1(row),
            Self::Static { original, .. } => {
                self.validate_static(ledger)?;
                ledger.borrowed_static_packet_actuals_v1(original)
            }
        }
    }
}
