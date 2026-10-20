//! The existing physical packet retains its original source and affine owner.
//! Static calls own the original publication handoff, never an instance row.
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use super::*;
use crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationHandoffV1;
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1;
use crate::mir::normal_callable_semantic_package::{
    OrdinaryNewClaimLedgerV1, VerifiedStaticLoopPacketSourceV1,
};
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
    SelectedStaticLoop {
        packet: VerifiedStaticLoopPacketSourceV1,
        observation: LocalCallObservationV1,
    },
}

#[derive(Debug)]
pub(in crate::mir) struct CallPacketSourceV1 {
    kind: CallPacketSourceKindV1,
}

/// Original Static packet checked against a borrowed publication row. The
/// collector's later exact batch take supplies that same owned row.
#[derive(Debug)]
pub(crate) struct PreparedStaticTerminalPacketV1 {
    original: Rc<StaticIncomingSourceV1>,
    observation: LocalCallObservationV1,
}

impl PreparedStaticTerminalPacketV1 {
    pub(crate) fn loan<'a>(
        &'a self,
        publication: &'a VerifiedStaticCallResultPublicationHandoffV1,
    ) -> CallPacketSourceLoanV1<'a> {
        CallPacketSourceLoanV1::Static {
            original: &self.original,
            observation: &self.observation,
            publication,
        }
    }

    pub(crate) fn original(&self) -> &Rc<StaticIncomingSourceV1> {
        &self.original
    }

    pub(crate) fn observation(&self) -> &LocalCallObservationV1 {
        &self.observation
    }

    pub(crate) fn prepare_zeroarg_projection(
        &self,
        publication: &VerifiedStaticCallResultPublicationHandoffV1,
    ) -> Result<PreparedLexicalCallProjectionV1, String> {
        if !self.original.argument_sites().is_empty() || !self.observation.arguments().is_empty() {
            return Err(freeze("static-terminal/zeroarg-projection-required"));
        }
        PreparedLexicalCallProjectionV1::for_source(
            CallPacketSourceLoanV1::Static {
                original: &self.original,
                observation: &self.observation,
                publication,
            },
            None,
            Vec::new(),
        )
    }

    pub(crate) fn into_packet(
        self,
        publication: VerifiedStaticCallResultPublicationHandoffV1,
    ) -> CallPacketSourceV1 {
        CallPacketSourceV1 {
            kind: CallPacketSourceKindV1::Static {
                original: self.original,
                observation: self.observation,
                publication,
            },
        }
    }
}

#[derive(Clone, Copy)]
pub(in crate::mir) enum CallPacketSourceLoanV1<'a> {
    Instance(&'a LexicalInstanceCallDispositionRowV1),
    Static {
        original: &'a Rc<StaticIncomingSourceV1>,
        observation: &'a LocalCallObservationV1,
        publication: &'a VerifiedStaticCallResultPublicationHandoffV1,
    },
    SelectedStaticLoop {
        packet: &'a VerifiedStaticLoopPacketSourceV1,
        observation: &'a LocalCallObservationV1,
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
            CallPacketSourceKindV1::SelectedStaticLoop {
                packet,
                observation,
            } => CallPacketSourceLoanV1::SelectedStaticLoop {
                packet,
                observation,
            },
        }
    }

    pub(in crate::mir) fn static_i64(
        original: Rc<StaticIncomingSourceV1>,
        publication: VerifiedStaticCallResultPublicationHandoffV1,
        ledger: &OrdinaryNewClaimLedgerV1,
    ) -> Result<Self, String> {
        Ok(Self::prepare_static_i64(original, &publication, ledger)?
            .into_packet(publication))
    }

    pub(crate) fn prepare_static_i64(
        original: Rc<StaticIncomingSourceV1>,
        publication: &VerifiedStaticCallResultPublicationHandoffV1,
        ledger: &OrdinaryNewClaimLedgerV1,
    ) -> Result<PreparedStaticTerminalPacketV1, String> {
        let observation = ledger
            .local_call_for_owner(original.call_site().owner(), original.call_site().site())
            .ok_or_else(|| freeze("static-packet/local-source-missing"))?
            .clone();
        CallPacketSourceLoanV1::Static {
            original: &original,
            observation: &observation,
            publication,
        }
        .validate_static(ledger)?;
        Ok(PreparedStaticTerminalPacketV1 {
            original,
            observation,
        })
    }

    pub(super) fn selected_static_loop(
        packet: VerifiedStaticLoopPacketSourceV1,
        ledger: &OrdinaryNewClaimLedgerV1,
    ) -> Result<Self, String> {
        let site = packet.local_call_site();
        let observation = ledger
            .local_call_for_owner(site.owner(), site.site())
            .ok_or_else(|| freeze("selected-static-packet/local-source-missing"))?
            .clone();
        if !packet.corroborates_local_call(&observation) {
            return Err(freeze("selected-static-packet/local-source-drift"));
        }
        Ok(Self {
            kind: CallPacketSourceKindV1::SelectedStaticLoop {
                packet,
                observation,
            },
        })
    }
}

impl<'a> CallPacketSourceLoanV1<'a> {
    pub(in crate::mir) fn require_instance(
        self,
    ) -> Result<&'a LexicalInstanceCallDispositionRowV1, String> {
        match self {
            Self::Instance(row) => Ok(row),
            Self::Static { .. } | Self::SelectedStaticLoop { .. } => {
                Err(freeze("static-packet/instance-source-required"))
            }
        }
    }

    pub(in crate::mir) fn call_site(self) -> &'a OwnedExprSiteV1 {
        match self {
            Self::Instance(row) => row.call_site(),
            Self::Static { original, .. } => original.call_site(),
            Self::SelectedStaticLoop { packet, .. } => packet.local_call_site(),
        }
    }

    pub(in crate::mir) fn target(self) -> &'a hakorune_mir_defs::CanonicalSameModuleCallableKeyV1 {
        match self {
            Self::Instance(row) => row.target(),
            Self::Static { original, .. } => original.target(),
            Self::SelectedStaticLoop { packet, .. } => packet.original_source().target(),
        }
    }

    pub(in crate::mir) fn callee_owner(self) -> FunctionOwnerIdV1 {
        match self {
            Self::Instance(row) => row.callee_owner(),
            Self::Static { original, .. } => original.callee_owner(),
            Self::SelectedStaticLoop { packet, .. } => packet.original_source().callee_owner(),
        }
    }

    pub(in crate::mir) fn argument_sites(self) -> &'a [SourceExprSiteV1] {
        match self {
            Self::Instance(row) => row.argument_sites(),
            Self::Static { original, .. } => original.argument_sites(),
            Self::SelectedStaticLoop { packet, .. } => packet.original_source().argument_sites(),
        }
    }

    pub(in crate::mir) fn result(self) -> Option<InvokeCallResultKind> {
        match self {
            Self::Instance(row) => row.result(),
            Self::Static { original, observation, publication } => {
                use crate::mir::resolved_semantics::home_new_prefix::{LocalCallResultClassV1 as Class, LocalCallResultSourceV1};
                match observation.result() {
                    Class::I64 => Some(InvokeCallResultKind::I64),
                    Class::Text if matches!(original.source_result(), crate::mir::callable_result_representation::VerifiedCallableResultDispositionV1::ExactString)
                        && *publication.representation() == crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1::ExactString
                        && matches!(observation.result_source(), Some(LocalCallResultSourceV1::OwnedText(source)) if Rc::ptr_eq(source, original)) => Some(InvokeCallResultKind::OwnedString),
                    _ => None,
                }
            }
            Self::SelectedStaticLoop { .. } => Some(InvokeCallResultKind::I64),
        }
    }

    pub(in crate::mir) fn validate_static(
        self,
        ledger: &OrdinaryNewClaimLedgerV1,
    ) -> Result<(), String> {
        if matches!(self, Self::SelectedStaticLoop { .. }) {
            return Err(freeze("selected-static-packet/canonical-entry-required"));
        }
        let Self::Static {
            original,
            observation,
            publication,
        } = self
        else {
            return Ok(());
        };
        ledger.verify_original_static_packet_source_v1(original)?;
        let handoff_agrees = original.corroborates_publication_handoff(publication)
            || (ledger
                .borrowed_static_packet_actuals_v1(original)?
                .is_some()
                && original.corroborates_selected_loop_publication_handoff(publication));
        if !handoff_agrees
            || observation.owner() != original.call_site().owner()
            || observation.site() != original.call_site()
            || self.result().is_none()
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
        self.borrowed_actuals_in_context(CallValidationContextV1::Lowering(ledger))
    }

    pub(in crate::mir) fn borrowed_actuals_in_context<'ledger>(
        self, context: CallValidationContextV1<'ledger>,
    ) -> Result<Option<&'ledger [crate::mir::normal_callable_semantic_package::PreparedBorrowedFormalActualV1]>, String> {
        let ledger = context.ledger();
        match self {
            Self::Instance(row) => ledger.borrowed_call_actuals_using_v1(row, context),
            Self::Static { original, .. } => {
                self.validate_static(ledger)?;
                ledger.borrowed_static_packet_actuals_v1(original)
            }
            Self::SelectedStaticLoop { .. } => {
                Err(freeze("selected-static-packet/canonical-entry-required"))
            }
        }
    }
}

impl EmittedLexicalCallProjectionV1 {
    /// Structural loan only. Lookup must not recursively re-enter result
    /// validation while that result is rechecking its borrowed support.
    pub(in crate::mir::normal_callable_semantic_package) fn retained_node_at_v1(
        &self, site: &OwnedExprSiteV1,
    ) -> Result<Option<&Self>, String> {
        let mut found = (self.call_site() == site).then_some(self);
        for argument in &self.prepared.arguments {
            let nested = match argument {
                LexicalCallArgumentProjectionV1::CallResult(inner) => Some(inner.as_ref()),
                _ => None,
            };
            for inner in nested.into_iter().chain(argument.expression_calls_v1()) {
                if let Some(packet) = inner.retained_node_at_v1(site)? {
                    if found.replace(packet).is_some() {
                        return Err(freeze("call-validation/duplicate-source-node"));
                    }
                }
            }
        }
        Ok(found)
    }

}
