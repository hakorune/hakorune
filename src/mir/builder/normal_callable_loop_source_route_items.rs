//! Source-item binding products for the callable LoopCond route token.
//!
//! These are the resolver-owned item/relation/probe products that the route
//! token co-seals. They copy only source sites and selector metadata; they do
//! not reopen AST dispatch, own entry order, or issue a Recipe.

use super::CallableLoopSourceRouteRejectV1;
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_result_representation::{
    VerifiedCallableResultRepresentationV1, VerifiedStaticCallResultPublicationHandoffV1,
};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceExprSiteV1, SourceStmtSiteV1};

#[path = "normal_callable_loop_source_route_checked_recipe.rs"]
mod checked_recipe;

/// Resolver-owned source item relation retained for the future LoopCond
/// physical port. This copies only source sites and selector metadata; it
/// does not reopen AST dispatch or issue a Recipe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct CallableLoopSourceItemBindingV1 {
    pub(in crate::mir::builder) call_site: SourceExprSiteV1,
    pub(in crate::mir::builder) receiver_site: SourceExprSiteV1,
    pub(in crate::mir::builder) argument_sites: Box<[SourceExprSiteV1]>,
    pub(in crate::mir::builder) result_site: SourceExprSiteV1,
    pub(in crate::mir::builder) selector: Box<str>,
    pub(in crate::mir::builder) arity: u32,
}

/// Exact Dynamic source target for one Loop item. The loop route borrows this
/// source disposition; the existing Local expression owner remains the sole
/// physical emitter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct CallableLoopDynamicInvocationSourceV1 {
    item: CallableLoopSourceItemBindingV1,
    loan: crate::mir::dynamic_invocation_contract::DynamicInvocationSourceLoanV1,
    observation: Option<
        crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1,
    >,
}

impl CallableLoopDynamicInvocationSourceV1 {
    fn from_checked_observation(
        owner: FunctionOwnerIdV1,
        items: &[CallableLoopSourceItemBindingV1],
        observation: &crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1,
    ) -> Result<Self, CallableLoopSourceRouteRejectV1> {
        use crate::mir::resolved_semantics::home_new_prefix::LocalCallResultSourceV1;
        let Some(LocalCallResultSourceV1::DynamicInvocation(loan)) = observation.result_source()
        else {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
        };
        let target = loan.target();
        let call_site = target.call_site();
        let mut matching_items = items.iter().filter(|item| item.call_site() == call_site);
        let Some(item) = matching_items.next() else {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
        };
        if matching_items.next().is_some()
            || observation.owner() != owner
            || observation.site()
                != &crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                    owner,
                    item.call_site().clone(),
                )
            || observation.result()
                != crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Dynamic
            || observation.dynamic_invocation_result_lifecycle_v1().is_none()
        {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
        }
        Self::from_target_and_observation(
            owner,
            item.clone(),
            loan.clone(),
            Some(observation.clone()),
        )
    }

    fn from_admitted_target(
        owner: FunctionOwnerIdV1,
        items: &[CallableLoopSourceItemBindingV1],
        target: &crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1,
    ) -> Result<Self, CallableLoopSourceRouteRejectV1> {
        if target.owner() != owner {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
        }
        let mut matching_items = items.iter().filter(|item| item.call_site() == target.call_site());
        let Some(item) = matching_items.next() else {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
        };
        if matching_items.next().is_some() {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
        }
        let expected = CallableLoopSourceItemBindingV1::from_dynamic_target(owner, target)?;
        if &expected != item {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
        }
        Self::from_target_and_observation(
            owner,
            item.clone(),
            crate::mir::dynamic_invocation_contract::DynamicInvocationSourceLoanV1::from_admitted_target_v1(target),
            None,
        )
    }

    fn from_target_and_observation(
        owner: FunctionOwnerIdV1,
        item: CallableLoopSourceItemBindingV1,
        loan: crate::mir::dynamic_invocation_contract::DynamicInvocationSourceLoanV1,
        observation: Option<crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1>,
    ) -> Result<Self, CallableLoopSourceRouteRejectV1> {
        let target = loan.target();
        if target.owner() != owner
            || target.call_site() != item.call_site()
            || target.result_site() != item.result_site()
            || target.receiver_site() != item.receiver_site()
            || target.dispatch().selector() != item.selector()
            || target.dispatch().arity() != item.arity()
            || target.arguments().len() != item.argument_sites().len()
            || loan.call_site()
                != &crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                    owner,
                    item.call_site().clone(),
                )
            || loan.argument_sites() != item.argument_sites()
            || target
                .arguments()
                .iter()
                .zip(item.argument_sites())
                .enumerate()
                .any(|(ordinal, (actual, expected))| {
                    actual.ordinal() as usize != ordinal || actual.site() != expected
                })
            || observation.as_ref().is_some_and(|call| {
                call.result_source().is_none_or(|source| {
                    !matches!(source,
                        crate::mir::resolved_semantics::home_new_prefix::LocalCallResultSourceV1::DynamicInvocation(observed)
                            if observed == &loan)
                })
            })
        {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
        }
        Ok(Self {
            item,
            loan,
            observation,
        })
    }

    fn merge_source(&mut self, other: Self) -> Result<(), CallableLoopSourceRouteRejectV1> {
        if self.item != other.item || self.loan != other.loan {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
        }
        match (&self.observation, other.observation) {
            (Some(existing), Some(incoming)) if existing != &incoming => {
                Err(CallableLoopSourceRouteRejectV1::SourceItemForeign)
            }
            (None, Some(incoming)) => {
                self.observation = Some(incoming);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub(in crate::mir::builder) fn item(&self) -> &CallableLoopSourceItemBindingV1 {
        &self.item
    }

    pub(in crate::mir::builder) fn loan(
        &self,
    ) -> &crate::mir::dynamic_invocation_contract::DynamicInvocationSourceLoanV1 {
        &self.loan
    }

    pub(in crate::mir::builder) fn observation(
        &self,
    ) -> Option<&crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1> {
        self.observation.as_ref()
    }

    pub(in crate::mir::builder) fn matches_checked_observation(
        &self,
        observation: &crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1,
    ) -> bool {
        use crate::mir::resolved_semantics::home_new_prefix::LocalCallResultSourceV1;
        observation.owner() == self.loan.target().owner()
            && observation.site() == self.loan.call_site()
            && observation.dynamic_invocation_result_lifecycle_v1().is_some()
            && matches!(observation.result_source(), Some(LocalCallResultSourceV1::DynamicInvocation(loan)) if loan == &self.loan)
            && self.observation.as_ref().is_none_or(|original| original == observation)
    }

    pub(in crate::mir::builder) fn matches_item(
        &self,
        owner: FunctionOwnerIdV1,
        item: &CallableLoopSourceItemBindingV1,
    ) -> bool {
        let target = self.loan.target();
        self.item == *item
            && target.owner() == owner
            && self.loan.call_site()
                == &crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                    owner,
                    item.call_site().clone(),
                )
            && self.loan.argument_sites() == item.argument_sites()
            && CallableLoopSourceItemBindingV1::from_dynamic_target(owner, target)
                .is_ok_and(|expected| expected == *item)
    }

    pub(in crate::mir::builder) fn call_site(&self) -> &SourceExprSiteV1 {
        self.item.call_site()
    }
}

impl CallableLoopSourceItemBindingV1 {
    fn from_dynamic_target(
        owner: FunctionOwnerIdV1,
        target: &crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1,
    ) -> Result<Self, CallableLoopSourceRouteRejectV1> {
        if target.owner() != owner
            || target.result_site() != target.call_site()
            || target.arguments().len() != target.dispatch().arity() as usize
            || target.arguments().iter().enumerate().any(|(ordinal, argument)| {
                argument.ordinal() as usize != ordinal
            })
        {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
        }
        Ok(Self {
            call_site: target.call_site().clone(),
            receiver_site: target.receiver_site().clone(),
            argument_sites: target
                .arguments()
                .iter()
                .map(|argument| argument.site().clone())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            result_site: target.result_site().clone(),
            selector: target.dispatch().selector().into(),
            arity: target.dispatch().arity(),
        })
    }

    pub(in crate::mir::builder) fn from_resolved(
        owner: FunctionOwnerIdV1,
        call: &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
    ) -> Result<Self, CallableLoopSourceRouteRejectV1> {
        if call.owner() != owner {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
        }
        let argument_sites = call
            .arguments()
            .iter()
            .map(|argument| argument.site().clone())
            .collect::<Vec<_>>()
            .into_boxed_slice();
        if argument_sites.len() != call.arity() as usize {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
        }
        Ok(Self {
            call_site: call.site().clone(),
            receiver_site: call.receiver_site().clone(),
            argument_sites,
            result_site: call.result_site().clone(),
            selector: call.selector().into(),
            arity: call.arity(),
        })
    }

    pub(in crate::mir::builder) const fn call_site(&self) -> &SourceExprSiteV1 {
        &self.call_site
    }

    pub(in crate::mir::builder) fn receiver_site(&self) -> &SourceExprSiteV1 {
        &self.receiver_site
    }

    pub(in crate::mir::builder) fn argument_sites(&self) -> &[SourceExprSiteV1] {
        &self.argument_sites
    }

    pub(in crate::mir::builder) const fn result_site(&self) -> &SourceExprSiteV1 {
        &self.result_site
    }

    pub(in crate::mir::builder) fn selector(&self) -> &str {
        &self.selector
    }

    pub(in crate::mir::builder) const fn arity(&self) -> u32 {
        self.arity
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn for_test(call_site: SourceExprSiteV1) -> Self {
        Self {
            receiver_site: call_site.clone(),
            argument_sites: Box::new([]),
            result_site: call_site.clone(),
            call_site,
            selector: "test".into(),
            arity: 0,
        }
    }
}

/// Selected static-result requirement evidence copied from one publication
/// row.  This is a read-only copy of the sealed requirement shape; the owned
/// handoff itself stays with the publication owner for its sole physical
/// consumer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct CallableLoopSourceTargetRequirementV1 {
    pub(in crate::mir::builder) representation: VerifiedCallableResultRepresentationV1,
    pub(in crate::mir::builder) required_callee_i64_arguments: Box<[u32]>,
}

impl CallableLoopSourceTargetRequirementV1 {
    pub(in crate::mir::builder) fn from_handoff(
        handoff: &VerifiedStaticCallResultPublicationHandoffV1,
    ) -> Self {
        Self {
            representation: handoff.representation().clone(),
            required_callee_i64_arguments: handoff
                .required_callee_i64_arguments()
                .to_vec()
                .into_boxed_slice(),
        }
    }

    pub(in crate::mir::builder) const fn representation(
        &self,
    ) -> &VerifiedCallableResultRepresentationV1 {
        &self.representation
    }

    pub(in crate::mir::builder) fn required_callee_i64_arguments(&self) -> &[u32] {
        &self.required_callee_i64_arguments
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn for_test(
        representation: VerifiedCallableResultRepresentationV1,
    ) -> Self {
        Self {
            representation,
            required_callee_i64_arguments: Box::new([]),
        }
    }
}

/// Exact target relation issued by the same invocation's source-target
/// authority. The route token never derives a target from selector text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct CallableLoopSourceTargetRelationV1 {
    call_site: SourceExprSiteV1,
    target: Option<CanonicalSameModuleCallableKeyV1>,
    requirement: Option<CallableLoopSourceTargetRequirementV1>,
    core_methods: Box<[CallableLoopSourceItemBindingV1]>,
    /// Items covered by an armed lexical instance-call disposition
    /// (claim-local or parameter receiver). They carry no static target and
    /// no publication requirement; the physical emitter resolves each row at
    /// lowering time.
    instance_methods: Box<[CallableLoopSourceItemBindingV1]>,
    /// Exact Dynamic invocation source observations retained by checked Loop
    /// expression frames; the Local owner remains their physical emitter.
    dynamic_invocations: Box<[CallableLoopDynamicInvocationSourceV1]>,
}

impl CallableLoopSourceTargetRelationV1 {
    pub(in crate::mir::builder) fn new(
        call_site: SourceExprSiteV1,
        target: CanonicalSameModuleCallableKeyV1,
        requirement: Option<CallableLoopSourceTargetRequirementV1>,
    ) -> Self {
        Self {
            call_site,
            target: Some(target),
            requirement,
            core_methods: Box::new([]),
            instance_methods: Box::new([]),
            dynamic_invocations: Box::new([]),
        }
    }

    pub(in crate::mir::builder) fn core_methods(
        methods: Box<[CallableLoopSourceItemBindingV1]>,
    ) -> Result<Self, CallableLoopSourceRouteRejectV1> {
        Self::covered_call_items(methods, Box::new([]), Box::new([]))
    }

    /// One relation for a loop whose call items are covered entirely by
    /// non-static evidence: resolver-issued CoreMethod rows plus armed
    /// lexical instance-call dispositions.
    pub(in crate::mir::builder) fn covered_call_items(
        core_methods: Box<[CallableLoopSourceItemBindingV1]>,
        instance_methods: Box<[CallableLoopSourceItemBindingV1]>,
        dynamic_invocations: Box<[CallableLoopDynamicInvocationSourceV1]>,
    ) -> Result<Self, CallableLoopSourceRouteRejectV1> {
        let Some(first) = core_methods
            .first()
            .or_else(|| instance_methods.first())
            .map(CallableLoopSourceItemBindingV1::call_site)
            .or_else(|| dynamic_invocations.first().map(CallableLoopDynamicInvocationSourceV1::call_site)) else {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemsMissing);
        };
        Ok(Self {
            call_site: first.clone(),
            target: None,
            requirement: None,
            core_methods,
            instance_methods,
            dynamic_invocations,
        })
    }

    /// Attach covered method items to a selected static relation for a
    /// mixed loop. The static obligation stays on `target`; the buckets
    /// carry the non-static coverage rows the physical validator unions
    /// against the exact source items.
    pub(in crate::mir::builder) fn with_covered_call_items(
        self,
        core_methods: Box<[CallableLoopSourceItemBindingV1]>,
        instance_methods: Box<[CallableLoopSourceItemBindingV1]>,
        dynamic_invocations: Box<[CallableLoopDynamicInvocationSourceV1]>,
    ) -> Self {
        Self {
            core_methods,
            instance_methods,
            dynamic_invocations,
            ..self
        }
    }

    pub(in crate::mir::builder) const fn call_site(&self) -> &SourceExprSiteV1 {
        &self.call_site
    }

    pub(in crate::mir::builder) fn target(&self) -> Option<&CanonicalSameModuleCallableKeyV1> {
        self.target.as_ref()
    }

    pub(in crate::mir::builder) fn requirement(
        &self,
    ) -> Option<&CallableLoopSourceTargetRequirementV1> {
        self.requirement.as_ref()
    }

    /// Whether the selected publication row carries the exact i64 result
    /// representation issued by the source-result owner.  The formal ordinal
    /// list is data carried by that owner; this route does not impose an ABI
    /// policy for a particular ordinal.
    pub(in crate::mir::builder) fn has_exact_i64_result(&self) -> bool {
        self.requirement.as_ref().is_some_and(|requirement| {
            requirement.representation() == &VerifiedCallableResultRepresentationV1::ExactI64
        })
    }

    pub(in crate::mir::builder) fn has_exact_scalar_result(&self) -> bool {
        self.requirement.as_ref().is_some_and(|requirement| {
            matches!(
                requirement.representation(),
                VerifiedCallableResultRepresentationV1::ExactI64
                    | VerifiedCallableResultRepresentationV1::ExactBool
            )
        })
    }

    pub(in crate::mir::builder) fn core_method_items(&self) -> &[CallableLoopSourceItemBindingV1] {
        &self.core_methods
    }

    pub(in crate::mir::builder) fn instance_method_items(
        &self,
    ) -> &[CallableLoopSourceItemBindingV1] {
        &self.instance_methods
    }

    pub(in crate::mir::builder) fn dynamic_invocation_items(
        &self,
    ) -> &[CallableLoopDynamicInvocationSourceV1] {
        &self.dynamic_invocations
    }
}

/// Resolver-issued proof that one armed loop's complete source inventory is
/// call-free and stays inside the accepted scalar grammar.
///
/// The bridge issues this only after every expression site under the loop
/// resolves to an integer literal, a local `BindingRebind` target, a local
/// lexical read, or an `Add`/`Multiply`/`Less` binary, and every statement
/// under the loop is a flat `LoopBody(_)` plain binding rebind. It carries
/// the covered sites so the route token and physical input can verify the
/// sum and identities without re-reading source syntax.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct VerifiedCallableLoopCallFreeCoverageV1 {
    owner: FunctionOwnerIdV1,
    loop_site: SourceStmtSiteV1,
    condition_site: SourceExprSiteV1,
    statements: Box<[SourceStmtSiteV1]>,
    expressions: Box<[SourceExprSiteV1]>,
}

impl VerifiedCallableLoopCallFreeCoverageV1 {
    pub(in crate::mir::builder) fn issue(
        owner: FunctionOwnerIdV1,
        loop_site: SourceStmtSiteV1,
        condition_site: SourceExprSiteV1,
        statements: Box<[SourceStmtSiteV1]>,
        expressions: Box<[SourceExprSiteV1]>,
    ) -> Self {
        Self {
            owner,
            loop_site,
            condition_site,
            statements,
            expressions,
        }
    }

    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) const fn loop_site(&self) -> &SourceStmtSiteV1 {
        &self.loop_site
    }

    pub(in crate::mir::builder) const fn condition_site(&self) -> &SourceExprSiteV1 {
        &self.condition_site
    }

    pub(in crate::mir::builder) fn covered_statements(&self) -> &[SourceStmtSiteV1] {
        &self.statements
    }

    pub(in crate::mir::builder) fn covered_expressions(&self) -> &[SourceExprSiteV1] {
        &self.expressions
    }
}

/// Source-proven call coverage for one armed LoopCond subtree.
///
/// `CallFree` carries the bounded grammar proof issued beside the bridge
/// inventory; `WithCalls` retains the existing selected-target relation.
/// The route token owns exactly one arm — a loop never presents both.
#[derive(Debug)]
pub(in crate::mir::builder) enum CallableLoopSourceCallCoverageV1 {
    CallFree(VerifiedCallableLoopCallFreeCoverageV1),
    WithCalls(CallableLoopSourceTargetRelationV1),
    CheckedRecipe(std::rc::Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>),
}

impl CallableLoopSourceCallCoverageV1 {


    pub(in crate::mir::builder) const fn as_with_calls(
        &self,
    ) -> Option<&CallableLoopSourceTargetRelationV1> {
        match self {
            Self::WithCalls(relation) => Some(relation),
            Self::CallFree(_) | Self::CheckedRecipe(_) => None,
        }
    }

    pub(in crate::mir::builder) const fn as_call_free(
        &self,
    ) -> Option<&VerifiedCallableLoopCallFreeCoverageV1> {
        match self {
            Self::CallFree(coverage) => Some(coverage),
            Self::WithCalls(_) | Self::CheckedRecipe(_) => None,
        }
    }
}

/// One source-order disposition for a resolver-issued method item.  Static
/// publication and bound CoreMethod rows share one batch, while their
/// authorities remain distinct and are validated by their existing owners.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableLoopSourceItemDispositionV1 {
    SelectedStatic(CallableLoopSourceTargetRelationV1),
    CoreMethod(CallableLoopSourceItemBindingV1),
    InstanceMethod(CallableLoopSourceItemBindingV1),
    DynamicInvocation(CallableLoopDynamicInvocationSourceV1),
}

impl CallableLoopSourceItemDispositionV1 {
    pub(in crate::mir::builder) fn call_site(&self) -> &SourceExprSiteV1 {
        match self {
            Self::SelectedStatic(relation) => relation.call_site(),
            Self::CoreMethod(item) | Self::InstanceMethod(item) => item.call_site(),
            Self::DynamicInvocation(source) => source.call_site(),
        }
    }
}

/// Immutable obligation/evidence classification for one armed loop's source
/// items, produced by the module-port probe before route selection.
///
/// `ModuleLoweringPortV1::target_for_source` reads the inventory-derived
/// exact-target map — the non-consumable "this site is an exact same-module
/// static call" requirement fact. `selected_static_result_handoff_for_source`
/// only peeks the consumable selected row. The probe classifies and never
/// decides fatality: `issue_with_source_relations` maps this product to the
/// named LoopCond terminals, and every other route ignores it.
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableLoopSourceTargetProbeV1 {
    /// Exact-target sites whose selected publication row is still present,
    /// in item order.
    selected: Box<[CallableLoopSourceTargetRelationV1]>,
    /// Exact-target sites whose selected publication row is absent —
    /// target-only, missing, or already consumed obligations.
    uncovered: Box<[SourceExprSiteV1]>,
    /// A published handoff disagreed with its exact target relation.
    requirement_mismatch: bool,
    /// Resolver-issued CoreMethod rows that cover the loop's method items.
    /// This is a distinct source family from static publication evidence.
    core_methods: Box<[CallableLoopSourceItemBindingV1]>,
    /// Items whose call site carries an armed lexical instance-call
    /// disposition in the ordinary-new claim ledger (claim-local or
    /// parameter receiver). This is route coverage evidence only; the
    /// canonical emitter consumes the row at lowering time.
    instance_methods: Box<[CallableLoopSourceItemBindingV1]>,
    /// Dynamic calls sourced from the package's exact resolver target catalog.
    dynamic_invocations: Box<[CallableLoopDynamicInvocationSourceV1]>,
}

impl CallableLoopSourceTargetProbeV1 {
    /// Empty probe: the loop is outside the selected family because the
    /// caller is not cataloged, the site is missing, or no item rows exist.
    pub(in crate::mir::builder) fn empty() -> Self {
        Self {
            selected: Box::new([]),
            uncovered: Box::new([]),
            requirement_mismatch: false,
            core_methods: Box::new([]),
            instance_methods: Box::new([]),
            dynamic_invocations: Box::new([]),
        }
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn from_parts(
        selected: Box<[CallableLoopSourceTargetRelationV1]>,
        uncovered: Box<[SourceExprSiteV1]>,
        requirement_mismatch: bool,
    ) -> Self {
        Self::from_parts_with_covered_methods(
            selected,
            uncovered,
            requirement_mismatch,
            Box::new([]),
            Box::new([]),
        )
    }

    pub(in crate::mir::builder) fn from_parts_with_core_methods(
        selected: Box<[CallableLoopSourceTargetRelationV1]>,
        uncovered: Box<[SourceExprSiteV1]>,
        requirement_mismatch: bool,
        core_methods: Box<[CallableLoopSourceItemBindingV1]>,
    ) -> Self {
        Self::from_parts_with_covered_methods(
            selected,
            uncovered,
            requirement_mismatch,
            core_methods,
            Box::new([]),
        )
    }

    pub(in crate::mir::builder) fn from_parts_with_covered_methods(
        selected: Box<[CallableLoopSourceTargetRelationV1]>,
        uncovered: Box<[SourceExprSiteV1]>,
        requirement_mismatch: bool,
        core_methods: Box<[CallableLoopSourceItemBindingV1]>,
        instance_methods: Box<[CallableLoopSourceItemBindingV1]>,
    ) -> Self {
        Self {
            selected,
            uncovered,
            requirement_mismatch,
            core_methods,
            instance_methods,
            dynamic_invocations: Box::new([]),
        }
    }

    pub(in crate::mir::builder) fn with_checked_recipe_call_dispositions_v2(
        mut self,
        items: &[CallableLoopSourceItemBindingV1],
        product: &crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2,
    ) -> Result<Self, CallableLoopSourceRouteRejectV1> {
        let owner = product.source.source.owner();
        let mut core_methods = self
            .core_methods
            .iter()
            .cloned()
            .map(|item| (item.call_site().clone(), item))
            .collect::<std::collections::BTreeMap<_, _>>();
        let mut dynamic_invocations =
            std::collections::BTreeMap::<SourceExprSiteV1, CallableLoopDynamicInvocationSourceV1>::new();
        for evaluation in product.checked_expression_evaluations_v2() {
            for (node, core) in evaluation.cores() {
                if !core.matches_plan(evaluation.source(), *node) {
                    return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
                }
                let item = CallableLoopSourceItemBindingV1::from_resolved(
                    owner,
                    core.source().source(),
                )?;
                if !items.contains(&item) {
                    return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
                }
                if let Some(original) = core_methods.get(item.call_site()) {
                    if original != &item {
                        return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
                    }
                } else {
                    core_methods.insert(item.call_site().clone(), item);
                }
            }
            for observation in evaluation.calls() {
                if observation.dynamic_invocation_source_v1().is_none() {
                    continue;
                }
                let source = CallableLoopDynamicInvocationSourceV1::from_checked_observation(
                    owner,
                    items,
                    observation,
                )?;
                if let Some(original) = dynamic_invocations.get(source.call_site()) {
                    let mut merged = original.clone();
                    merged.merge_source(source)?;
                    dynamic_invocations.insert(merged.call_site().clone(), merged);
                } else {
                    dynamic_invocations.insert(source.call_site().clone(), source);
                }
            }
        }
        // Array Append is an effect owned by its scheduled Append packet.
        // Ordinary CoreMethod/lexical-instance evidence can also classify
        // the same outer `push` call, but that is not a second disposition.
        // Transfer only the exact resolver item after checking the packet's
        // owner and candidate relation; conflicting owners remain refusals.
        for observation in product.schedule().blocks_array_appends_v2() {
            let crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1::Append {
                source,
                call,
                candidate,
                ..
            } = observation
            else {
                return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
            };
            let item = items
                .iter()
                .find(|item| item.call_site() == call.site())
                .ok_or(CallableLoopSourceRouteRejectV1::SourceItemForeign)?;
            let owner_matches = call.owner() == owner && source.binding().owner() == owner;
            let candidate_matches = source.append_candidate(call).as_ref() == Some(candidate);
            let uncovered = self.uncovered.iter().any(|site| site == call.site());
            let selected = self.selected.iter().any(|relation| relation.call_site() == call.site());
            let core_mismatch = core_methods
                .get(call.site())
                .is_some_and(|core_item| core_item != item);
            let dynamic = dynamic_invocations.contains_key(call.site());
            let mut matching_instance = self
                .instance_methods
                .iter()
                .filter(|instance| instance.call_site() == call.site());
            let instance_mismatch = matching_instance.next().is_some_and(|instance| instance != item)
                || matching_instance.next().is_some();
            if !owner_matches
                || !candidate_matches
                || uncovered
                || selected
                || core_mismatch
                || dynamic
                || instance_mismatch
            {
                return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
            }
            core_methods.remove(call.site());
            self.instance_methods = self
                .instance_methods
                .iter()
                .filter(|instance| instance.call_site() != call.site())
                .cloned()
                .collect();
        }
        // A checked CoreMethod frame is also a more specific owner than the
        // generic lexical-instance claim for the same resolver item.
        for core_item in core_methods.values() {
            let mut matching_instance = self
                .instance_methods
                .iter()
                .filter(|instance| instance.call_site() == core_item.call_site());
            if matching_instance
                .next()
                .is_some_and(|instance| instance != core_item)
                || matching_instance.next().is_some()
            {
                return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
            }
            self.instance_methods = self
                .instance_methods
                .iter()
                .filter(|instance| instance.call_site() != core_item.call_site())
                .cloned()
                .collect();
        }
        // A source-bound Dynamic invocation carries the richer result and
        // lifecycle owner for its exact call. Drop only an identical generic
        // lexical-instance classification so the source item has one owner.
        for dynamic in dynamic_invocations.values() {
            let item = dynamic.item();
            let mut matching_instance = self
                .instance_methods
                .iter()
                .filter(|instance| instance.call_site() == dynamic.call_site());
            if matching_instance.next().is_some_and(|instance| instance != item)
                || matching_instance.next().is_some()
            {
                return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
            }
            self.instance_methods = self
                .instance_methods
                .iter()
                .filter(|instance| instance.call_site() != dynamic.call_site())
                .cloned()
                .collect();
        }
        self.core_methods = core_methods.into_values().collect();
        self.dynamic_invocations = dynamic_invocations.into_values().collect();
        Ok(self)
    }

    /// Connect Dynamic calls from the package's exact source-target catalog.
    /// Checked Local observations, where available, remain supplemental
    /// evidence and must agree with the same admitted target.
    pub(in crate::mir::builder) fn with_admitted_dynamic_targets_v1(
        mut self,
        items: &[CallableLoopSourceItemBindingV1],
        owner: FunctionOwnerIdV1,
        targets: &[crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1],
    ) -> Result<Self, CallableLoopSourceRouteRejectV1> {
        let mut dynamic_invocations = self.dynamic_invocations.iter().cloned()
            .map(|source| (source.call_site().clone(), source))
            .collect::<std::collections::BTreeMap<_, _>>();
        for target in targets {
            let source = CallableLoopDynamicInvocationSourceV1::from_admitted_target(
                owner, items, target,
            )?;
            if let Some(original) = dynamic_invocations.get(source.call_site()) {
                let mut merged = original.clone();
                merged.merge_source(source)?;
                dynamic_invocations.insert(merged.call_site().clone(), merged);
            } else {
                dynamic_invocations.insert(source.call_site().clone(), source);
            }
        }
        self.dynamic_invocations = dynamic_invocations.into_values().collect();
        Ok(self)
    }



    /// Any classified evidence still carried by this probe. A CallFree loop
    /// must arrive with none — a selected, uncovered, mismatched, or
    /// CoreMethod row contradicts the call-free proof.
    pub(in crate::mir::builder) fn has_residual_evidence(&self) -> bool {
        !self.selected.is_empty()
            || !self.uncovered.is_empty()
            || self.requirement_mismatch
            || !self.core_methods.is_empty()
            || !self.instance_methods.is_empty()
            || !self.dynamic_invocations.is_empty()
    }

    /// Resolve the classification into the single co-sealed target relation.
    /// Evidence gaps are checked before selection arity so a dropped
    /// required row can never reclassify as out of scope.
    pub(in crate::mir::builder) fn into_selected_relation(
        self,
        source_items: &[CallableLoopSourceItemBindingV1],
    ) -> Result<CallableLoopSourceTargetRelationV1, CallableLoopSourceRouteRejectV1> {
        let Self {
            selected,
            uncovered,
            requirement_mismatch,
            core_methods,
            instance_methods,
            dynamic_invocations,
        } = self;
        if requirement_mismatch {
            return Err(CallableLoopSourceRouteRejectV1::SourceTargetRequirementMismatch);
        }
        if !uncovered.is_empty() {
            return Err(CallableLoopSourceRouteRejectV1::SourceTargetUnselected {
                call_sites: uncovered,
            });
        }
        if selected.len() > 1 {
            return Err(CallableLoopSourceRouteRejectV1::SourceTargetMultiple);
        }
        let mut selected = Vec::from(selected);
        if let Some(relation) = selected.pop() {
            if core_methods.is_empty()
                && instance_methods.is_empty()
                && dynamic_invocations.is_empty()
            {
                return Ok(relation);
            }
            // A selected static row beside covered method items is a mixed
            // loop: retain the static obligation and still verify every
            // source item exactly once — silently dropping the buckets
            // would seal WithCalls without coverage proof.
            let covered = std::iter::once(relation.call_site())
                .chain(core_methods.iter().map(|item| item.call_site()))
                .chain(instance_methods.iter().map(|item| item.call_site()))
                .chain(dynamic_invocations.iter().map(|item| item.call_site()))
                .collect::<std::collections::BTreeSet<_>>();
            let uncovered = source_items
                .iter()
                .filter(|item| !covered.contains(item.call_site()))
                .map(|item| item.call_site().clone())
                .collect::<Vec<_>>();
            if !uncovered.is_empty() {
                return Err(
                    CallableLoopSourceRouteRejectV1::SourceCallOutsideSelectedFamily {
                        call_sites: uncovered.into_boxed_slice(),
                    },
                );
            }
            return Ok(relation.with_covered_call_items(
                core_methods,
                instance_methods,
                dynamic_invocations,
            ));
        }
        if !core_methods.is_empty()
            || !instance_methods.is_empty()
            || !dynamic_invocations.is_empty()
        {
            let covered = core_methods
                .iter()
                .chain(instance_methods.iter())
                .map(|item| item.call_site())
                .chain(dynamic_invocations.iter().map(|item| item.call_site()))
                .collect::<std::collections::BTreeSet<_>>();
            let uncovered = source_items
                .iter()
                .filter(|item| !covered.contains(item.call_site()))
                .map(|item| item.call_site().clone())
                .collect::<Vec<_>>();
            if !uncovered.is_empty() {
                return Err(
                    CallableLoopSourceRouteRejectV1::SourceCallOutsideSelectedFamily {
                        call_sites: uncovered.into_boxed_slice(),
                    },
                );
            }
            return CallableLoopSourceTargetRelationV1::covered_call_items(
                core_methods,
                instance_methods,
                dynamic_invocations,
            );
        }
        Err(
            CallableLoopSourceRouteRejectV1::SourceCallOutsideSelectedFamily {
                call_sites: source_items
                    .iter()
                    .map(|item| item.call_site().clone())
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            },
        )
    }

    /// Consume the complete ordered selected set for a composite source
    /// Recipe.  The direct route keeps `into_selected_relation`'s singleton
    /// contract; this batch route only accepts one selected handoff for every
    /// resolver-issued source item, in the same order.
    #[cfg(test)]
    pub(in crate::mir::builder) fn into_selected_relations(
        self,
        source_items: &[CallableLoopSourceItemBindingV1],
    ) -> Result<Box<[CallableLoopSourceTargetRelationV1]>, CallableLoopSourceRouteRejectV1> {
        let Self {
            selected,
            uncovered,
            requirement_mismatch,
            core_methods,
            instance_methods,
            dynamic_invocations,
        } = self;
        if requirement_mismatch {
            return Err(CallableLoopSourceRouteRejectV1::SourceTargetRequirementMismatch);
        }
        if !uncovered.is_empty() {
            return Err(CallableLoopSourceRouteRejectV1::SourceTargetUnselected {
                call_sites: uncovered,
            });
        }
        if !core_methods.is_empty()
            || !instance_methods.is_empty()
            || !dynamic_invocations.is_empty()
        {
            return Err(
                CallableLoopSourceRouteRejectV1::SourceCallOutsideSelectedFamily {
                    call_sites: core_methods
                        .iter()
                        .chain(instance_methods.iter())
                        .map(|item| item.call_site().clone())
                        .chain(
                            dynamic_invocations
                                .iter()
                                .map(|item| item.call_site().clone()),
                        )
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                },
            );
        }
        if selected.len() != source_items.len() {
            return Err(CallableLoopSourceRouteRejectV1::SourceTargetCardinality {
                expected: source_items.len(),
                actual: selected.len(),
            });
        }
        for (item, relation) in source_items.iter().zip(selected.iter()) {
            if item.call_site() != relation.call_site() {
                return Err(CallableLoopSourceRouteRejectV1::SourceTargetOrderMismatch {
                    expected: item.call_site().clone(),
                    actual: relation.call_site().clone(),
                });
            }
            if !relation.has_exact_i64_result() {
                return Err(CallableLoopSourceRouteRejectV1::SourceTargetRequirementMismatch);
            }
        }
        Ok(selected)
    }

    /// Consume the complete source-order disposition batch for a structured
    /// source Recipe. Every resolver item is covered exactly once by either
    /// the selected publication owner or the existing CoreMethod owner.
    pub(in crate::mir::builder) fn into_item_dispositions(
        self,
        source_items: &[CallableLoopSourceItemBindingV1],
    ) -> Result<Box<[CallableLoopSourceItemDispositionV1]>, CallableLoopSourceRouteRejectV1> {
        use std::collections::BTreeMap;

        let Self {
            selected,
            uncovered,
            requirement_mismatch,
            core_methods,
            instance_methods,
            dynamic_invocations,
        } = self;
        if requirement_mismatch {
            return Err(CallableLoopSourceRouteRejectV1::SourceTargetRequirementMismatch);
        }
        if !uncovered.is_empty() {
            return Err(CallableLoopSourceRouteRejectV1::SourceTargetUnselected {
                call_sites: uncovered,
            });
        }

        let mut selected_by_site = BTreeMap::new();
        for relation in selected {
            let call_site = relation.call_site().clone();
            if selected_by_site
                .insert(call_site.clone(), relation)
                .is_some()
            {
                return Err(CallableLoopSourceRouteRejectV1::SourceItemDuplicate { call_site });
            }
        }
        let mut core_by_site = BTreeMap::new();
        for item in core_methods {
            let call_site = item.call_site().clone();
            if core_by_site.insert(call_site.clone(), item).is_some() {
                return Err(CallableLoopSourceRouteRejectV1::SourceItemDuplicate { call_site });
            }
        }
        let mut instance_by_site = BTreeMap::new();
        for item in instance_methods {
            let call_site = item.call_site().clone();
            if instance_by_site
                .insert(call_site.clone(), item)
                .is_some()
            {
                return Err(CallableLoopSourceRouteRejectV1::SourceItemDuplicate { call_site });
            }
        }
        let mut dynamic_by_site = BTreeMap::new();
        for source in dynamic_invocations {
            let call_site = source.call_site().clone();
            if dynamic_by_site.insert(call_site.clone(), source).is_some() {
                return Err(CallableLoopSourceRouteRejectV1::SourceItemDuplicate { call_site });
            }
        }

        let mut seen_items = BTreeMap::new();
        let mut dispositions = Vec::with_capacity(source_items.len());
        for item in source_items {
            if seen_items.insert(item.call_site().clone(), ()).is_some() {
                return Err(CallableLoopSourceRouteRejectV1::SourceItemDuplicate {
                    call_site: item.call_site().clone(),
                });
            }
            if let Some(relation) = selected_by_site.remove(item.call_site()) {
                if !relation.has_exact_scalar_result() {
                    return Err(CallableLoopSourceRouteRejectV1::SourceTargetRequirementMismatch);
                }
                dispositions.push(CallableLoopSourceItemDispositionV1::SelectedStatic(
                    relation,
                ));
            } else if let Some(core_method) = core_by_site.remove(item.call_site()) {
                dispositions.push(CallableLoopSourceItemDispositionV1::CoreMethod(core_method));
            } else if let Some(instance_method) = instance_by_site.remove(item.call_site()) {
                dispositions.push(CallableLoopSourceItemDispositionV1::InstanceMethod(
                    instance_method,
                ));
            } else if let Some(dynamic_invocation) = dynamic_by_site.remove(item.call_site()) {
                dispositions.push(CallableLoopSourceItemDispositionV1::DynamicInvocation(
                    dynamic_invocation,
                ));
            } else {
                return Err(
                    CallableLoopSourceRouteRejectV1::SourceItemDispositionMissing {
                        call_site: item.call_site().clone(),
                    },
                );
            }
        }

        let mut residual = selected_by_site
            .into_keys()
            .chain(core_by_site.into_keys())
            .chain(instance_by_site.into_keys())
            .chain(dynamic_by_site.into_keys())
            .collect::<Vec<_>>();
        if !residual.is_empty() {
            residual.sort();
            return Err(
                CallableLoopSourceRouteRejectV1::SourceItemDispositionResidual {
                    call_sites: residual.into_boxed_slice(),
                },
            );
        }
        Ok(dispositions.into_boxed_slice())
    }
}
