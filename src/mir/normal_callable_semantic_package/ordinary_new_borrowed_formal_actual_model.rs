//! Actual value representations and source/executable phase validation.
use super::*;
use std::rc::Rc;

/// A computed actual keeps its original argument-purpose frame. Equality is
/// identity of that qualification, never structural equality of a rebuilt plan.
#[derive(Debug, Clone)]
pub(crate) struct BorrowedExpressionActualSourceV1 {
    frame: std::rc::Rc<
        crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1,
    >,
}
impl PartialEq for BorrowedExpressionActualSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        std::rc::Rc::ptr_eq(&self.frame, &other.frame)
    }
}
impl Eq for BorrowedExpressionActualSourceV1 {}
impl BorrowedExpressionActualSourceV1 {
    pub(super) fn from_frame(
        frame: std::rc::Rc<
            crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1,
        >,
    ) -> Result<Self, String> {
        use crate::mir::resolved_value_profile::{
            expression_class::ExpressionValueClassV1 as Class,
            expression_projection::ExpressionLeafClassV1 as Leaf,
        };
        let result_class = if frame.is_i64() {
            Class::I64
        } else if frame.is_dynamic_carrier_append_v1() {
            Class::Dynamic
        } else {
            Class::String
        };
        if frame.evaluation().root_class() != Leaf::Value(result_class) {
            return Err(freeze("borrowed-expression/source-result"));
        }
        Ok(Self { frame })
    }
    pub(crate) fn frame(
        &self,
    ) -> &std::rc::Rc<
        crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1,
    > {
        &self.frame
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BorrowedFormalActualSourceV1 {
    /// The exact original Get; no local destination or independent Home.
    Read(crate::mir::normal_callable_semantic_package::ReadActualSourceLoanV1),
    /// Borrow the computed Normal value; its enclosing Call owns the temporary
    /// lifetime. This is not a forwarded formal, local alias or new Home.
    Expression(BorrowedExpressionActualSourceV1),
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
    /// Exact local object-handle read borrowed into the call. Its class comes
    /// from the same single-object source sum as the callee formal.
    LocalObjectBinding {
        binding: BindingRefV1,
        class: Box<str>,
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
    /// Retaining a qualified source is not an executable-actual phase change.
    /// The completed cohort must separately corroborate this exact frame.
    pub(in crate::mir::normal_callable_semantic_package) fn retain_expression_frame_v1(
        frame: std::rc::Rc<
            crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1,
        >,
    ) -> Result<Self, String> {
        BorrowedExpressionActualSourceV1::from_frame(frame).map(Self::Expression)
    }

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
    ExecutableStaticInput(static_input_finish::StaticInputFinishV1),
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
    pub(super) dynamic_invocation_arguments: Option<DynamicInvocationArgumentSourcesV1>,
}

impl PreparedBorrowedCallActualsV1 {
    /// Original source observations survive completion in their sole input
    /// proof. This immutable loan grants neither execution nor requalification.
    pub(super) fn source_static_snapshot_v1(&self) -> Option<&Self> {
        match &self.phase {
            BorrowedCallActualEvidencePhaseV1::SourceStatic(_) => Some(self),
            BorrowedCallActualEvidencePhaseV1::ExecutableStaticInput(finish) => {
                finish.source_snapshot_v1()
            }
            _ => None,
        }
    }
    /// Dispatch selection only. The source projection still validates the whole
    /// original tuple; this never grants executable actuals or Home completion.
    pub(in crate::mir::normal_callable_semantic_package) fn has_qualified_static_argument_source_v1(
        &self,
    ) -> bool {
        matches!(&self.phase, BorrowedCallActualEvidencePhaseV1::SourceStatic(identity)
            if identity.argument_sources.values().any(|slot|
                matches!(slot, model::ArgumentExpressionV1::Qualified(_))))
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
            | BorrowedCallActualEvidencePhaseV1::ExecutableStaticInput(_)
            | BorrowedCallActualEvidencePhaseV1::ExecutableStaticOne(_)
            | BorrowedCallActualEvidencePhaseV1::ExecutableStaticMixed(_) => Ok(()),
            BorrowedCallActualEvidencePhaseV1::ExecutableStaticTagged(_) => Ok(()),
            BorrowedCallActualEvidencePhaseV1::ExecutableStaticScalar(_) => Ok(()),
            BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(ref identity) => {
                let mut ordinals = std::collections::BTreeSet::new();
                for (ordinal, original) in &identity.array_loans {
                    let mut incoming = identity
                        .incoming_arguments
                        .iter()
                        .filter(|(index, _, _)| index == ordinal);
                    let (_, site, formal) = incoming
                        .next()
                        .ok_or_else(|| freeze("borrowed-instance/array-original-argument"))?;
                    let mut actuals = self
                        .opaque_actuals
                        .iter()
                        .filter(|actual| actual.ordinal == *ordinal);
                    let actual = actuals
                        .next()
                        .ok_or_else(|| freeze("borrowed-instance/array-actual-payload-missing"))?;
                    let BorrowedFormalActualSourceV1::Read(read) = &actual.source else {
                        return Err(freeze("borrowed-instance/array-actual-source-drift"));
                    };
                    if incoming.next().is_some()
                        || actuals.next().is_some()
                        || !ordinals.insert(*ordinal)
                        || &actual.site != site
                        || actual.formal != *formal
                        || !read.corroborates_array_input_v1(original)
                    {
                        return Err(freeze("borrowed-instance/array-actual-identity-drift"));
                    }
                }
                for (ordinal, original) in &identity.field_loans {
                    let mut incoming = identity
                        .incoming_arguments
                        .iter()
                        .filter(|(index, _, _)| index == ordinal);
                    let (_, site, formal) = incoming
                        .next()
                        .ok_or_else(|| freeze("borrowed-instance/field-original-argument"))?;
                    let mut actuals = self
                        .opaque_actuals
                        .iter()
                        .filter(|actual| actual.ordinal == *ordinal);
                    let actual = actuals
                        .next()
                        .ok_or_else(|| freeze("borrowed-instance/field-actual-payload-missing"))?;
                    let BorrowedFormalActualSourceV1::Read(read) = &actual.source else {
                        return Err(freeze("borrowed-instance/field-actual-source-drift"));
                    };
                    if incoming.next().is_some()
                        || actuals.next().is_some()
                        || !ordinals.insert(*ordinal)
                        || &actual.site != site
                        || actual.formal != *formal
                        || !read.corroborates_field_input_v1(
                            identity.source.call_site(),
                            *ordinal,
                            original,
                        )
                    {
                        return Err(freeze("borrowed-instance/field-actual-identity-drift"));
                    }
                }
                if self
                    .opaque_actuals
                    .iter()
                    .filter(|actual| {
                        matches!(&actual.source, BorrowedFormalActualSourceV1::Read(_))
                    })
                    .count()
                    != ordinals.len()
                {
                    return Err(freeze("borrowed-instance/array-actual-domain-drift"));
                }
                Ok(())
            }
            BorrowedCallActualEvidencePhaseV1::EntryOnlyStaticBool(_) => {
                Err(freeze("ordinary-new/borrowed-entry/bool-entry-not-call"))
            }
            BorrowedCallActualEvidencePhaseV1::SourceInstance(ref original) => Err(format!(
                "{}: call={:?}, callee={:?}, target={:?}",
                freeze("ordinary-new/borrowed-entry/source-only-instance-actuals"),
                original.source.call_site(),
                original.source.callee_owner(),
                original.source.target(),
            )),
            BorrowedCallActualEvidencePhaseV1::SourceObject(ref original) => Err(format!(
                "{}: call={:?}, callee={:?}, target={:?}",
                freeze("ordinary-new/borrowed-entry/source-only-object-actuals"),
                original.source.call_site(),
                original.source.callee_owner(),
                original.source.target(),
            )),
            BorrowedCallActualEvidencePhaseV1::SourceStatic(ref original) => Err(format!(
                "{}: call={:?}, callee={:?}, target={:?}, result={:?}",
                freeze("ordinary-new/borrowed-entry/source-only-static-actuals"),
                original.source.call_site(),
                original.source.callee_owner(),
                original.source.target(),
                original.source.source_result(),
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
        if self.dynamic_invocation_arguments.is_some() {
            return self.checked_dynamic_ordered_arguments_v1(call);
        }
        for (ordinal, argument) in self.ordered_arguments.iter().enumerate() {
            let opaque = call
                .arguments
                .iter()
                .find(|(index, _, _)| *index as usize == ordinal);
            match (opaque, argument) {
                    (Some((index, site, _)), crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::BorrowedActual { ordinal: actual, site: observed })
                        if actual == index && observed == site => {}
                    (Some((index, site, _)), crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Expression { ordinal: actual, site: observed })
                        if actual == index
                            && *actual as usize == ordinal
                            && observed == site
                            && call.source.argument_sites().get(ordinal) == Some(observed) => {}
                    (None, crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Expression { ordinal: actual, site: observed })
                        if *actual as usize == ordinal
                            && call.source.argument_sites().get(ordinal) == Some(observed) => {}
                    (None, crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Integer(_)) => {}
                    (None, crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Scalar(binding))
                        if binding.owner() == call.call.owner() => {}
                    _ => {
                        return Err(freeze("borrowed-entry/ordered-arguments-identity"));
                    }
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

    fn checked_dynamic_ordered_arguments_v1(
        &self,
        call: &super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    ) -> Result<&[LocalCallArgumentV1], String> {
        let reject = || freeze("borrowed-entry/dynamic-ordered-arguments-identity");
        let dynamic = self
            .dynamic_invocation_arguments
            .as_ref()
            .ok_or_else(reject)?;
        let original = call.source.require_instance()?;
        let target = dynamic.source.target();
        let sites = original.argument_sites();
        if !self.opaque_actuals.is_empty()
            || dynamic.source.call_site() != &call.call
            || target.owner() != call.call.owner()
            || target.call_site() != call.call.site()
            || target.result_site() != call.call.site()
            || target.receiver_site() != original.receiver_site()
            || target.receiver_binding() != original.receiver_binding()?
            || target.dispatch().selector() != original.target().name()
            || target.dispatch().arity() != original.target().arity()
            || target.dispatch().arity() as usize != sites.len()
            || dynamic.source.argument_sites() != sites
            || dynamic.candidates.len() != sites.len()
            || dynamic.arguments.len() != sites.len()
        {
            return Err(reject());
        }
        for (ordinal, ((candidate, source), argument)) in dynamic
            .candidates
            .iter()
            .zip(target.arguments())
            .zip(self.ordered_arguments.iter())
            .enumerate()
        {
            if candidate.ordinal as usize != ordinal
                || candidate.ordinal != source.ordinal()
                || candidate.site != *source.site()
                || candidate.site != sites[ordinal]
                || !matches!(argument, LocalCallArgumentV1::Expression {
                    ordinal: observed, site
                } if *observed as usize == ordinal && site == &candidate.site)
            {
                return Err(reject());
            }
            let slot = dynamic.arguments.get(&candidate.ordinal).ok_or_else(reject)?;
            let plan = match slot {
                ArgumentExpressionV1::Source(plan) => plan,
                ArgumentExpressionV1::Qualified(frame) => frame.source(),
            };
            let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
                call: expression_call,
                ordinal: expression_ordinal,
                ..
            } = plan.root()
            else {
                return Err(reject());
            };
            if plan.owner() != call.call.owner()
                || expression_call.owner() != call.call.owner()
                || expression_call.site() != call.call.site()
                || *expression_ordinal as usize != ordinal
                || expression_call.arity() != target.dispatch().arity()
                || expression_call.arguments().len() != sites.len()
                || expression_call.arguments().iter().zip(target.arguments()).any(
                    |(observed, expected)| {
                        observed.ordinal() != expected.ordinal()
                            || observed.site() != expected.site()
                    },
                )
                || plan
                    .nodes()
                    .first()
                    .is_none_or(|node| node.site() != &candidate.site)
            {
                return Err(reject());
            }
        }
        Ok(&self.ordered_arguments)
    }
}

/// Qualification refines the existing ordinal product; it never creates a
/// second plan/frame inventory or makes the original actual executable.
#[derive(Debug, Clone)]
pub(super) enum ArgumentExpressionV1 {
    Source(Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>),
    Qualified(
        Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>,
    ),
}

/// Dynamic invocation arguments keep their source plans and checked frames in
/// the same pending actual row used by the call observer. The target loan is
/// the argument-domain authority; this product adds no signature or ABI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DynamicInvocationArgumentSourcesV1 {
    pub(super) source: crate::mir::dynamic_invocation_contract::DynamicInvocationSourceLoanV1,
    pub(super) candidates: Box<[BorrowedCallActualCandidateV1]>,
    pub(super) arguments: std::collections::BTreeMap<u32, ArgumentExpressionV1>,
}

impl std::ops::Deref for ArgumentExpressionV1 {
    type Target = Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Source(plan) => plan,
            Self::Qualified(frame) => frame.source(),
        }
    }
}
impl From<Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>>
    for ArgumentExpressionV1
{
    fn from(
        plan: Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
    ) -> Self {
        Self::Source(plan)
    }
}
impl PartialEq for ArgumentExpressionV1 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Source(left), Self::Source(right)) => Rc::ptr_eq(left, right),
            (Self::Qualified(left), Self::Qualified(right)) => Rc::ptr_eq(left, right),
            _ => false,
        }
    }
}
impl Eq for ArgumentExpressionV1 {}

/// Bind original source structure only. Mode-specific class/loan requirements
/// stay with the original Static/Instance issuer, outside this common product.
pub(super) fn bind_original_argument_plans_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    source: super::super::borrowed_formal_uses::BorrowedCallSourceLoanV1<'_>,
    method: &std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
    candidates: &[BorrowedCallActualCandidateV1],
    previous: Option<&BTreeMap<u32, ArgumentExpressionV1>>,
) -> Result<BTreeMap<u32, ArgumentExpressionV1>, String> {
    let reject = || freeze("borrowed-actual/argument-source-identity");
    if input.owner() != source.call_site().owner()
        || !input
            .function()
            .method_call_source_v1(source.call_site().site())
            .is_some_and(|original| std::sync::Arc::ptr_eq(original, method))
        || previous.is_some_and(|old| old.len() != candidates.len())
    {
        return Err(reject());
    }
    let mut plans = BTreeMap::new();
    for candidate in candidates {
        let atom = crate::mir::resolved_semantics::home_new_prefix::borrowed_actual_source_atom_v1(
            input,
            &candidate.site,
        );
        let same = match (&candidate.value, atom.as_ref()) {
            (
                BorrowedCallActualValueV1::Scalar(binding, _)
                | BorrowedCallActualValueV1::SelfRooted { binding, .. }
                | BorrowedCallActualValueV1::ReceivedNullable(binding)
                | BorrowedCallActualValueV1::ReceivedMixed(binding),
                Some(BorrowedCallActualValueV1::Binding(original)),
            ) => binding == original,
            (
                BorrowedCallActualValueV1::ReceivedField { .. },
                Some(BorrowedCallActualValueV1::FieldAccess),
            ) => true,
            (
                BorrowedCallActualValueV1::MethodCall(method),
                Some(BorrowedCallActualValueV1::MethodCall(original)),
            ) => std::sync::Arc::ptr_eq(method, original),
            // Exact source binding identity only; value class is checked by its own source proof.
            (BorrowedCallActualValueV1::Unknown, Some(BorrowedCallActualValueV1::Binding(_))) => {
                true
            }
            (expected, Some(original)) => expected == original,
            _ => false,
        };
        if !same {
            return Err(reject());
        }
        let plan = if let Some(old) = previous {
            ArgumentExpressionV1::Source(Rc::clone(old.get(&candidate.ordinal).ok_or_else(reject)?))
        } else {
            crate::mir::resolved_value_profile::expression_plan::call_argument_expression_source_plan_v1(
                input, method, candidate.ordinal).map_err(|_| reject())?.into()
        };
        if !matches!(plan.root(), crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call, .. }
                if std::sync::Arc::ptr_eq(call, method))
            || input
                .function()
                .source_site_inventory()
                .containing_statement(source.call_site().site())
                != Some(plan.root().statement())
            || plans.insert(candidate.ordinal, plan).is_some()
        {
            return Err(reject());
        }
    }
    validate_original_argument_plans_v1(source, candidates, &plans)?;
    Ok(plans)
}

/// Source-plan membership, not execution/entry/ownership permission. Qualified
/// frames must retain their SAME source loan, not a rebuilt plan or frame.
fn validate_original_argument_plans_v1(
    source: super::super::borrowed_formal_uses::BorrowedCallSourceLoanV1<'_>,
    candidates: &[BorrowedCallActualCandidateV1],
    plans: &BTreeMap<u32, ArgumentExpressionV1>,
) -> Result<(), String> {
    use crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1;
    let reject = || freeze("borrowed-actual/argument-source-identity");
    if plans.len() != candidates.len() || candidates.len() != source.argument_sites().len() {
        return Err(reject());
    }
    let mut outer = None;
    for (index, candidate) in candidates.iter().enumerate() {
        let plan = plans.get(&(index as u32)).ok_or_else(reject)?;
        let ExpressionPlanRootV1::CallArgument { call, ordinal, .. } = plan.root() else {
            return Err(reject());
        };
        if candidate.ordinal as usize != index
            || *ordinal != candidate.ordinal
            || plan.owner() != source.call_site().owner()
            || call.owner() != plan.owner()
            || call.site() != source.call_site().site()
            || call.arity() != source.target().arity()
            || call
                .arguments()
                .iter()
                .map(|arg| arg.site())
                .ne(source.argument_sites().iter())
            || source.argument_sites().get(index) != Some(&candidate.site)
            || plan
                .nodes()
                .first()
                .is_none_or(|node| node.site() != &candidate.site)
            || outer.is_some_and(|previous| !std::sync::Arc::ptr_eq(previous, call))
        {
            return Err(reject());
        }
        outer = Some(call);
        if let ArgumentExpressionV1::Qualified(frame) = plan {
            if !Rc::ptr_eq(frame.source(), frame.source_loan().plan())
                || !frame.evaluation().reference_coverage_is_exact()
            {
                return Err(reject());
            }
        }
    }
    Ok(())
}

impl instance_source::InstanceSourceActualIdentityV1 {
    pub(super) fn same_original_source_v1(&self, other: &Self) -> bool {
        self.source == other.source
            && self.incoming_arguments == other.incoming_arguments
            && self.candidates == other.candidates
            && self.array_loans == other.array_loans
            && self.field_loans == other.field_loans
    }
}

/// Re-preparation checks the original semantic tuple; retain its already bound
/// refinement only after source membership checks. Empty is the existing direct
/// source-only prepare state and cannot grant an expression qualification.
pub(super) fn retain_instance_argument_sources_v1(
    prepared: &mut PreparedBorrowedCallActualsV1,
    original: &instance_source::InstanceSourceActualIdentityV1,
) -> Result<(), String> {
    let BorrowedCallActualEvidencePhaseV1::SourceInstance(identity) = &mut prepared.phase else {
        return Err(freeze("borrowed-instance/source-identity"));
    };
    if !identity.same_original_source_v1(original) || !identity.argument_sources.is_empty() {
        return Err(freeze("borrowed-instance/source-identity"));
    }
    if !original.argument_sources.is_empty() {
        validate_original_argument_plans_v1(
            super::super::borrowed_formal_uses::BorrowedCallSourceLoanV1::Instance(
                &original.source,
            ),
            &original.candidates,
            &original.argument_sources,
        )?;
    }
    identity.argument_sources = original.argument_sources.clone();
    Ok(())
}

pub(super) fn retain_object_argument_sources_v1(
    prepared: &mut PreparedBorrowedCallActualsV1,
    original: &object_source::ObjectSourceActualIdentityV1,
) -> Result<(), String> {
    let BorrowedCallActualEvidencePhaseV1::SourceObject(identity) = &mut prepared.phase else {
        return Err(freeze("borrowed-object/source-actual-identity"));
    };
    if !identity.same_original_source_v1(original) || !identity.argument_sources.is_empty() {
        return Err(freeze("borrowed-object/source-actual-identity"));
    }
    if !original.argument_sources.is_empty() {
        validate_original_argument_plans_v1(
            super::super::borrowed_formal_uses::BorrowedCallSourceLoanV1::Instance(
                &original.source,
            ),
            &original.candidates,
            &original.argument_sources,
        )?;
    }
    identity.argument_sources = original.argument_sources.clone();
    Ok(())
}

/// The shared actual constructor consumes real Read or Expression evidence.
/// These remain distinct source kinds with their own final membership checks.
pub(super) fn validate_computed_actual_source_v1(
    source: &BorrowedFormalActualSourceV1,
    actual: &BorrowedCallActualCandidateV1,
    call: &OwnedExprSiteV1,
    formal: &crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractV1,
) -> Result<(), String> {
    match source {
        BorrowedFormalActualSourceV1::Read(read) => {
            if read.site().owner() != call.owner() || read.site().site() != &actual.site {
                return Err(freeze("borrowed-actual/array-read-source-drift"));
            }
            if let BorrowedCallActualValueV1::MethodCall(method) = &actual.value {
                if !read.corroborates_method_source_v1(method) {
                    return Err(freeze("borrowed-actual/method-original-source-drift"));
                }
            }
        }
        BorrowedFormalActualSourceV1::Expression(expression) => {
            let frame = expression.frame();
            if frame.source().owner() != call.owner()
                || frame
                    .source()
                    .nodes()
                    .first()
                    .is_none_or(|node| node.site() != &actual.site)
                || !matches!(frame.source().root(), crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call: outer, ordinal, .. }
                    if outer.site() == call.site() && *ordinal == actual.ordinal)
                || formal.ordinal != actual.ordinal
                || !matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle)
            {
                return Err(freeze("borrowed-actual/expression-source-drift"));
            }
            if let BorrowedCallActualValueV1::String(value) = &actual.value {
                use crate::mir::resolved_value_profile::{
                    expression_class::ExpressionValueClassV1 as Class,
                    expression_plan::ExpressionNodeSourceV1 as Node,
                    expression_projection::ExpressionLeafClassV1 as Leaf,
                };
                if !matches!(frame.source().nodes()[0].source(),
                    Node::Literal(crate::mir::resolved_semantics::ResolvedLiteralSourceV1::String(original)) if original == value)
                    || frame.is_i64()
                    || frame.evaluation().root_class() != Leaf::Value(Class::String)
                {
                    return Err(freeze("borrowed-actual/expression-source-drift"));
                }
            }
        }
        BorrowedFormalActualSourceV1::Scalar {
            binding,
            kind: SourceScalarKind::Integer,
        } if matches!(actual.value, BorrowedCallActualValueV1::Unknown)
            && binding.owner() == call.owner()
            && formal.ordinal == actual.ordinal
            && formal.kind.is_ordinary_borrowed_handle() => {}
        BorrowedFormalActualSourceV1::LocalObjectBinding { binding, .. }
            if matches!(
                actual.value,
                BorrowedCallActualValueV1::Binding(actual_binding)
                    if actual_binding == *binding
            )
                && binding.owner() == call.owner()
                && formal.ordinal == actual.ordinal
                && formal.kind.is_ordinary_borrowed_handle() => {}
        _ => return Err(freeze("borrowed-actual/expression-or-read-required")),
    }
    Ok(())
}
