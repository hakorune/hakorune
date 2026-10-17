//! Actual value representations and source/executable phase validation.
use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BorrowedFormalActualSourceV1 {
    /// The exact original Get; no local destination or independent Home.
    Read(crate::mir::normal_callable_semantic_package::ReadActualSourceLoanV1),
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
    /// Original conditional result: Null/Fresh/Forward remain distinct at
    /// runtime. This class is source attribution, never an owned Home grant.
    ReceivedMixed {
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

impl BorrowedFormalActualSourceV1 {
    /// Original caller formal identity, not permission to select a physical
    /// borrowed entry. Declared formals must also join that caller's sealed
    /// entry values; EntryReceiver and TypedHome are different source roots.
    pub(crate) fn caller_formal_origin_v1(&self) -> Option<(BindingRefV1, BindingRefV1)> {
        match self {
            Self::Forwarded { binding, formal } => Some((*binding, *formal)),
            Self::DeclaredFormal { binding, root, .. } => Some((*binding, *root)),
            _ => None,
        }
    }
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
    /// Reconstructed original input protocol; source identity remains demandable.
    ExecutableSourceObject(object_source::ObjectSourceActualIdentityV1),
    ExecutableSourceInstance(instance_source::InstanceSourceActualIdentityV1),
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
    /// Dispatch selection only. The source projection still validates the whole
    /// original tuple; this never grants executable actuals or Home completion.
    pub(in crate::mir::normal_callable_semantic_package) fn has_qualified_static_argument_source_v1(&self) -> bool {
        matches!(&self.phase, BorrowedCallActualEvidencePhaseV1::SourceStatic(identity)
            if identity.argument_sources.values().any(|slot|
                matches!(slot, static_source::StaticArgumentExpressionV1::Qualified(_))))
    }

    /// Source-only attribution does not lend an executable argument plan.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn is_source_only_object_v1(
        &self,
    ) -> bool {
        matches!(
            self.phase,
            BorrowedCallActualEvidencePhaseV1::SourceObject(_)
        )
    }

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
            | BorrowedCallActualEvidencePhaseV1::ExecutableSourceObject(_)
            | BorrowedCallActualEvidencePhaseV1::ExecutableStaticZero(_)
            | BorrowedCallActualEvidencePhaseV1::ExecutableStaticOne(_)
            | BorrowedCallActualEvidencePhaseV1::ExecutableStaticMixed(_) => Ok(()),
            BorrowedCallActualEvidencePhaseV1::ExecutableStaticTagged(_) => Ok(()),
            BorrowedCallActualEvidencePhaseV1::ExecutableStaticScalar(_) => Ok(()),
            BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(ref identity) => {
                let mut ordinals = std::collections::BTreeSet::new();
                for (ordinal, original) in &identity.array_loans {
                    let mut incoming = identity.incoming_arguments.iter().filter(|(index, _, _)| index == ordinal);
                    let (_, site, formal) = incoming.next().ok_or_else(|| freeze("borrowed-instance/array-original-argument"))?;
                    let mut actuals = self.opaque_actuals.iter().filter(|actual| actual.ordinal == *ordinal);
                    let actual = actuals.next().ok_or_else(|| freeze("borrowed-instance/array-actual-payload-missing"))?;
                    let BorrowedFormalActualSourceV1::Read(read) = &actual.source else {
                        return Err(freeze("borrowed-instance/array-actual-source-drift"));
                    };
                    if incoming.next().is_some() || actuals.next().is_some() || !ordinals.insert(*ordinal)
                        || &actual.site != site || actual.formal != *formal || !read.corroborates_array_input_v1(original) {
                        return Err(freeze("borrowed-instance/array-actual-identity-drift"));
                    }
                }
                for (ordinal, original) in &identity.field_loans {
                    let mut incoming = identity.incoming_arguments.iter().filter(|(index, _, _)| index == ordinal);
                    let (_, site, formal) = incoming.next().ok_or_else(|| freeze("borrowed-instance/field-original-argument"))?;
                    let mut actuals = self.opaque_actuals.iter().filter(|actual| actual.ordinal == *ordinal);
                    let actual = actuals.next().ok_or_else(|| freeze("borrowed-instance/field-actual-payload-missing"))?;
                    let BorrowedFormalActualSourceV1::Read(read) = &actual.source else {
                        return Err(freeze("borrowed-instance/field-actual-source-drift"));
                    };
                    if incoming.next().is_some() || actuals.next().is_some() || !ordinals.insert(*ordinal)
                        || &actual.site != site || actual.formal != *formal
                        || !read.corroborates_field_input_v1(identity.source.call_site(), *ordinal, original) {
                        return Err(freeze("borrowed-instance/field-actual-identity-drift"));
                    }
                }
                if self.opaque_actuals.iter().filter(|actual| matches!(&actual.source,
                    BorrowedFormalActualSourceV1::Read(_))).count() != ordinals.len() {
                    return Err(freeze("borrowed-instance/array-actual-domain-drift"));
                }
                Ok(())
            }
            BorrowedCallActualEvidencePhaseV1::EntryOnlyStaticBool(_) => {
                Err(freeze("ordinary-new/borrowed-entry/bool-entry-not-call"))
            }
            BorrowedCallActualEvidencePhaseV1::SourceInstance(_) => Err(freeze(
                "ordinary-new/borrowed-entry/source-only-instance-actuals",
            )),
            BorrowedCallActualEvidencePhaseV1::SourceObject(ref original) => Err(format!(
                "{}: call={:?}, callee={:?}, target={:?}",
                freeze("ordinary-new/borrowed-entry/source-only-object-actuals"),
                original.source.call_site(),
                original.source.callee_owner(),
                original.source.target(),
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
