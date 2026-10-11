//! Actual value representations and source/executable phase validation.
use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BorrowedFormalActualSourceV1 {
    Integer(i64),
    Bool(bool),
    /// The exact `null` literal — tag `0` on the wire, never a zero
    /// payload borrowed from an integer or bool producer.
    Null,
    /// A caller-owned received nullable (`NullableObject(C)` call
    /// result): the ABI owner selects tag `0` or tag `3` at runtime;
    /// the sealed claim class is the sole class authority.
    ReceivedNullable {
        binding: BindingRefV1,
        class: Box<str>,
    },
    Scalar {
        binding: BindingRefV1,
        kind: SourceScalarKind,
    },
    TypedHome {
        binding: BindingRefV1,
        root: BindingRefV1,
        acquisition: OwnedExprSiteV1,
        class: Box<str>,
    },
    EntryReceiver {
        binding: BindingRefV1,
        root: BindingRefV1,
        class: Box<str>,
    },
    /// The caller's own `DeclaredObject` formal passed through as an
    /// opaque actual — the caller's sealed parameter contract is the
    /// sole class authority; ordinary-borrowed `origins`/`forwards`
    /// drafts never cover declared formals.
    DeclaredFormal {
        binding: BindingRefV1,
        root: BindingRefV1,
        class: Box<str>,
    },
    Forwarded {
        binding: BindingRefV1,
        formal: BindingRefV1,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreparedBorrowedFormalActualV1 {
    pub(crate) ordinal: u32,
    pub(crate) site: SourceExprSiteV1,
    pub(crate) formal: BindingRefV1,
    pub(crate) source: BorrowedFormalActualSourceV1,
}

/// The same pending call owns both its opaque proofs and the full ordered
/// argument projection. No second site inventory or domain authority is minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum BorrowedCallActualEvidencePhaseV1 {
    Executable,
    ExecutableStaticZero(static_input_finish::StaticZeroInputFinishV1),
    ExecutableStaticOne(static_one_input_finish::StaticOneInputFinishV1),
    ExecutableStaticScalar(static_scalar_input_finish::StaticScalarInputFinishV1),
    ExecutableStaticMixed(static_mixed_input_finish::StaticMixedInputFinishV1),
    ExecutableStaticTagged(static_tagged_input_finish::StaticTaggedInputFinishV1),
    EntryOnlyStaticBool(static_bool_entry_finish::StaticBoolEntryFinishV1),
    SourceStatic(static_source::StaticSourceActualIdentityV1),
    SourceObject(object_source::ObjectSourceActualIdentityV1),
    SourceInstance(instance_source::InstanceSourceActualIdentityV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::normal_callable_semantic_package) struct PreparedBorrowedCallActualsV1 {
    pub(super) phase: BorrowedCallActualEvidencePhaseV1,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) opaque_actuals:
        Box<[PreparedBorrowedFormalActualV1]>,
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) ordered_arguments:
        Box<[LocalCallArgumentV1]>,
}

impl PreparedBorrowedCallActualsV1 {
    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn object_source_for_test(
        ingress: &PreparedBorrowedFormalIngressV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        call: &OwnedExprSiteV1,
        candidates: &[BorrowedCallActualCandidateV1],
    ) -> Self {
        object_source::prepare_object_source_actuals_v1(ingress, contracts, call, candidates)
            .unwrap()
            .unwrap()
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn require_executable_v1(
        &self,
    ) -> Result<(), String> {
        match self.phase {
            BorrowedCallActualEvidencePhaseV1::Executable
            | BorrowedCallActualEvidencePhaseV1::ExecutableStaticZero(_)
            | BorrowedCallActualEvidencePhaseV1::ExecutableStaticOne(_)
            | BorrowedCallActualEvidencePhaseV1::ExecutableStaticMixed(_) => Ok(()),
            BorrowedCallActualEvidencePhaseV1::ExecutableStaticTagged(_) => Ok(()),
            BorrowedCallActualEvidencePhaseV1::ExecutableStaticScalar(_) => Ok(()),
            BorrowedCallActualEvidencePhaseV1::EntryOnlyStaticBool(_) => {
                Err(freeze("ordinary-new/borrowed-entry/bool-entry-not-call"))
            }
            BorrowedCallActualEvidencePhaseV1::SourceInstance(_) => Err(freeze(
                "ordinary-new/borrowed-entry/source-only-instance-actuals",
            )),
            BorrowedCallActualEvidencePhaseV1::SourceObject(_) => Err(freeze(
                "ordinary-new/borrowed-entry/source-only-object-actuals",
            )),
            BorrowedCallActualEvidencePhaseV1::SourceStatic(_) => Err(freeze(
                "ordinary-new/borrowed-entry/source-only-static-actuals",
            )),
        }
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn ordered_arguments_for_v1(
        &self,
        call: &super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    ) -> Result<&[LocalCallArgumentV1], String> {
        self.require_executable_v1()?;
        self.checked_ordered_arguments_v1(call)
    }

    pub(super) fn checked_ordered_arguments_v1(
        &self,
        call: &super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    ) -> Result<&[LocalCallArgumentV1], String> {
        if self.ordered_arguments.len() != call.source.argument_sites().len() {
            return Err(freeze("borrowed-entry/ordered-arguments-cardinality"));
        }
        for (ordinal, argument) in self.ordered_arguments.iter().enumerate() {
            let opaque = call
                .arguments
                .iter()
                .find(|(index, _, _)| *index as usize == ordinal);
            match (opaque, argument) {
                    (Some((index, site, _)), crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::BorrowedActual { ordinal: actual, site: observed })
                        if actual == index && observed == site => {}
                    (None, crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Integer(_)) => {}
                    (None, crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Scalar(binding))
                        if binding.owner() == call.call.owner() => {}
                    _ => return Err(freeze("borrowed-entry/ordered-arguments-identity")),
                }
        }
        if self.opaque_actuals.len() != call.arguments.len() {
            return Err(freeze("borrowed-entry/actuals-cardinality"));
        }
        for (actual, (ordinal, site, formal)) in
            self.opaque_actuals.iter().zip(call.arguments.iter())
        {
            if actual.ordinal != *ordinal
                || &actual.site != site
                || actual.formal != *formal
                || formal.owner() != call.callee
            {
                return Err(freeze("borrowed-entry/actuals-identity"));
            }
        }
        Ok(&self.ordered_arguments)
    }
}
