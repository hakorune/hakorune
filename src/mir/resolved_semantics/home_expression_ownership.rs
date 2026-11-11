//! One source-owned responsibility transition for each expression effect.
//! Physical cleanup and the source prefix borrow this plan; neither replays moves.
use crate::mir::resolved_semantics::home_new_prefix::{
    CheckedArgumentExpressionFrameV1, HomesV1, LiveHomeV1, LocalCallObservationV1,
};
use crate::mir::resolved_semantics::{BindingRefV1, OwnedExprSiteV1};
use crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1;
use std::rc::Rc;

/// Immutable facts from the canonical parameter/actual source contract.
/// Fresh acquisitions are not fabricated bindings or already acquired values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpressionCallEntryFactsV1 {
    pub(crate) site: OwnedExprSiteV1,
    pub(crate) taken: Box<[(BindingRefV1, BindingRefV1, OwnedExprSiteV1)]>,
    pub(crate) fresh: Box<[(u32, OwnedExprSiteV1)]>,
    pub(crate) arguments: Box<[Rc<CheckedArgumentExpressionFrameV1>]>,
    pub(crate) nested: Box<[(u32, Rc<ExpressionCallEntryFactsV1>)]>,
    pub(crate) preserves_reads: bool,
}

/// Mutually exclusive source evidence for one original argument ordinal.
enum ArgumentEffectSourceV1<'a> {
    Evaluation(&'a CheckedArgumentExpressionFrameV1),
    Fresh(&'a OwnedExprSiteV1),
    Call(&'a Rc<ExpressionCallEntryFactsV1>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExpressionOwnershipPointV1 {
    site: OwnedExprSiteV1,
    argument_start: usize,
    argument_entry: Rc<HomesV1>,
    before_entry: Rc<HomesV1>,
    entered: Rc<HomesV1>,
    call: Option<Rc<ExpressionCallEntryFactsV1>>,
}

/// Only this source plan folds the transitions. Each physical user projects it.
/// The points borrow one immutable Home list until a transfer actually changes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpressionOwnershipV1 {
    initial: Rc<HomesV1>,
    points: Box<[ExpressionOwnershipPointV1]>,
    end: Rc<HomesV1>,
}
impl ExpressionOwnershipV1 {
    pub(crate) fn entry(homes: &[LiveHomeV1]) -> Self {
        let initial = Rc::new(HomesV1::snapshot(homes));
        Self {
            end: Rc::clone(&initial),
            initial,
            points: Box::new([]),
        }
    }

    pub(crate) fn prepare(
        source: &Rc<ExpressionSourcePlanV1>,
        order: &[usize],
        transferred: &std::collections::BTreeSet<usize>,
        calls: &[LocalCallObservationV1],
        homes: &[LiveHomeV1],
    ) -> Result<Self, &'static str> {
        let mut plan = Self::entry(homes);
        if calls.is_empty() {
            return Ok(plan);
        }
        let mut current = Rc::clone(&plan.initial);
        let mut points = Vec::new();
        for &index in order {
            if transferred.contains(&index) {
                continue;
            }
            let site = OwnedExprSiteV1::new(source.owner(), source.nodes()[index].site().clone());
            let facts = calls
                .iter()
                .find(|call| call.site() == &site)
                .map(|call| {
                    call.entry_ownership_v1()
                        .cloned()
                        .ok_or("expression/argument-owner-unavailable")
                })
                .transpose()?;
            if let Some(facts) = facts {
                Self::append_call(source, &facts, &mut current, &mut points)?;
            } else {
                points.push(ExpressionOwnershipPointV1 {
                    site,
                    argument_start: points.len(),
                    argument_entry: Rc::clone(&current),
                    before_entry: Rc::clone(&current),
                    entered: Rc::clone(&current),
                    call: None,
                });
            }
        }
        let mut sites = std::collections::BTreeSet::new();
        if points.iter().any(|point| !sites.insert(point.site.clone())) {
            return Err("expression/argument-source-drift");
        }
        plan.points = points.into_boxed_slice();
        plan.end = current;
        Ok(plan)
    }

    fn append_call(
        source: &ExpressionSourcePlanV1,
        facts: &Rc<ExpressionCallEntryFactsV1>,
        current: &mut Rc<HomesV1>,
        points: &mut Vec<ExpressionOwnershipPointV1>,
    ) -> Result<(), &'static str> {
        let argument_start = points.len();
        let argument_entry = Rc::clone(current);
        // Ordinals belong to the retained source call, never physical IDs.
        let mut arguments: Vec<_> = facts.arguments.iter().map(|child| {
            let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { ordinal, .. } = child.source().root()
                else { return Err("expression/argument-source-drift"); };
            Ok((*ordinal, ArgumentEffectSourceV1::Evaluation(child)))
        }).collect::<Result<_, _>>()?;
        arguments.extend(
            facts
                .fresh
                .iter()
                .map(|(ordinal, site)| (*ordinal, ArgumentEffectSourceV1::Fresh(site))),
        );
        arguments.extend(
            facts
                .nested
                .iter()
                .map(|(ordinal, child)| (*ordinal, ArgumentEffectSourceV1::Call(child))),
        );
        arguments.sort_by_key(|(ordinal, _)| *ordinal);
        let mut seen = std::collections::BTreeSet::new();
        for (ordinal, effect) in arguments {
            if !seen.insert(ordinal) {
                return Err("expression/argument-source-drift");
            }
            let offset = points.len();
            match effect {
                ArgumentEffectSourceV1::Fresh(site) => {
                    points.push(ExpressionOwnershipPointV1 {
                        site: site.clone(),
                        argument_start: offset,
                        argument_entry: Rc::clone(current),
                        before_entry: Rc::clone(current),
                        entered: Rc::clone(current),
                        call: None,
                    });
                }
                ArgumentEffectSourceV1::Call(nested) => {
                    Self::append_call(source, nested, current, points)?
                }
                ArgumentEffectSourceV1::Evaluation(child) => {
                    if child.evaluation().ownership_v1().points.is_empty() {
                        for &index in child.evaluation().ordered_nodes() {
                            if child.evaluation().transfers_node(index) {
                                continue;
                            }
                            points.push(ExpressionOwnershipPointV1 {
                                site: OwnedExprSiteV1::new(
                                    source.owner(),
                                    child.source().nodes()[index].site().clone(),
                                ),
                                argument_start: points.len(),
                                argument_entry: Rc::clone(current),
                                before_entry: Rc::clone(current),
                                entered: Rc::clone(current),
                                call: None,
                            });
                        }
                    }
                    let mut prefixes = vec![Rc::clone(current)];
                    for point in &child.evaluation().ownership_v1().points {
                        let argument_entry = Rc::clone(
                            prefixes
                                .get(point.argument_start)
                                .ok_or("expression/argument-source-drift")?,
                        );
                        let before = Rc::clone(current);
                        if let Some(entry) = &point.call {
                            Self::check_reach(source, entry)?;
                            *current = Self::enter(current, entry)?;
                        }
                        points.push(ExpressionOwnershipPointV1 {
                            site: point.site.clone(),
                            argument_start: offset + point.argument_start,
                            argument_entry,
                            before_entry: before,
                            entered: Rc::clone(current),
                            call: point.call.clone(),
                        });
                        prefixes.push(Rc::clone(current));
                    }
                }
            }
        }
        Self::check_reach(source, facts)?;
        let before_entry = Rc::clone(current);
        *current = Self::enter(current, facts)?;
        points.push(ExpressionOwnershipPointV1 {
            site: facts.site.clone(),
            argument_start,
            argument_entry,
            before_entry,
            entered: Rc::clone(current),
            call: Some(Rc::clone(facts)),
        });
        Ok(())
    }

    fn check_reach(
        source: &ExpressionSourcePlanV1,
        facts: &ExpressionCallEntryFactsV1,
    ) -> Result<(), &'static str> {
        use crate::mir::resolved_value_profile::expression_children::{
            ExpressionChildReachV1, ExpressionChildrenV1,
        };
        if facts.taken.is_empty() {
            return Ok(());
        }
        if matches!(source.root(), crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::LoopCondition { .. })
            || source.nodes().iter().any(|node| matches!(node.children(),
                ExpressionChildrenV1::Known(children) if children.iter().any(|child|
                    child.reach != ExpressionChildReachV1::Always
                        && facts.site.site().node().segments().starts_with(child.site.node().segments())))) {
            return Err("expression/take-successor-unavailable");
        }
        Ok(())
    }

    fn enter(
        current: &Rc<HomesV1>,
        facts: &ExpressionCallEntryFactsV1,
    ) -> Result<Rc<HomesV1>, &'static str> {
        if facts.taken.iter().any(|(binding, _, acquisition)| {
            binding.owner() != facts.site.owner() || acquisition.owner() != facts.site.owner()
        }) {
            return Err("expression/take-cleanup-unavailable");
        }
        let mut fresh = std::collections::BTreeSet::new();
        for (ordinal, acquisition) in &facts.fresh {
            if acquisition.owner() != facts.site.owner()
                || !fresh.insert(*ordinal)
                || !acquisition
                    .site()
                    .is_argument_of(facts.site.site(), *ordinal)
            {
                return Err("borrowed-actual/source-identity");
            }
        }
        if facts.taken.is_empty() {
            return Ok(Rc::clone(current));
        }
        HomesV1::after_call_entry_v1(
            current,
            facts
                .taken
                .iter()
                .map(|(binding, root, acquisition)| (*binding, *root, acquisition)),
        )
        .map(Rc::new)
        .ok_or("expression/take-cleanup-unavailable")
    }
    pub(crate) fn first_site(&self) -> Option<&OwnedExprSiteV1> {
        self.points.first().map(|point| &point.site)
    }
    pub(crate) fn last_site(&self) -> Option<&OwnedExprSiteV1> {
        self.points.last().map(|point| &point.site)
    }
    pub(crate) fn entries(&self) -> impl Iterator<Item = &ExpressionCallEntryFactsV1> {
        self.points.iter().filter_map(|point| point.call.as_deref())
    }
    pub(crate) fn before_call(&self, site: &OwnedExprSiteV1) -> Option<&[LiveHomeV1]> {
        self.points
            .iter()
            .find(|row| &row.site == site)
            .map(|row| &**row.argument_entry as &[LiveHomeV1])
    }
    pub(crate) fn before_entry_call(&self, site: &OwnedExprSiteV1) -> Option<&[LiveHomeV1]> {
        self.points
            .iter()
            .find(|row| &row.site == site)
            .map(|row| &**row.before_entry as &[LiveHomeV1])
    }
    pub(crate) fn entered_call(&self, site: &OwnedExprSiteV1) -> Option<&[LiveHomeV1]> {
        self.points
            .iter()
            .find(|row| &row.site == site)
            .map(|row| &**row.entered as &[LiveHomeV1])
    }
    pub(crate) fn before_site(&self, site: &OwnedExprSiteV1) -> Option<&[LiveHomeV1]> {
        if self.points.is_empty() {
            return Some(&self.initial);
        }
        self.before_call(site)
    }
    pub(crate) fn end(&self) -> &[LiveHomeV1] {
        &self.end
    }
    pub(crate) fn has_transfers(&self) -> bool {
        self.points.iter().any(|point| {
            point
                .call
                .as_ref()
                .is_some_and(|facts| !facts.taken.is_empty())
        })
    }
    pub(crate) fn contains_site(&self, site: &OwnedExprSiteV1) -> bool {
        self.points.iter().any(|point| &point.site == site)
    }
    pub(crate) fn after_site(&self, site: &OwnedExprSiteV1) -> Option<&[LiveHomeV1]> {
        if self.points.is_empty() {
            return Some(&self.end);
        }
        self.entered_call(site)
    }
}

/// The observation carries immutable source facts, never a mutable Home set.
pub(crate) fn attach_expression_entry_facts_v1<E>(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    observation: &mut LocalCallObservationV1,
    port: &mut impl FnMut(
        &OwnedExprSiteV1,
        crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1<'_>,
    ) -> Result<
        Option<crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1>,
        E,
    >,
) -> Result<(), E> {
    let borrowed_input = matches!(observation.result_source(),
        Some(crate::mir::resolved_semantics::home_new_prefix::LocalCallResultSourceV1::DynamicInvocation(loan))
            if loan.envelope().input_home() == crate::mir::dynamic_invocation_contract::DynamicInvocationInputHomeV1::BorrowedNoEscapeForInvocation);
    observation.entry_ownership = expression_entry_facts_at_v1(
        input,
        observation.site(),
        observation.arguments(),
        borrowed_input,
        port,
    )?;
    Ok(())
}

/// Both ordinary observations and retained raw argument calls borrow the same
/// exact site contract. No result type grants an empty transfer contract.
pub(crate) fn expression_entry_facts_at_v1<E>(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    site: &OwnedExprSiteV1,
    actuals: &[crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1],
    borrowed_input: bool,
    port: &mut impl FnMut(
        &OwnedExprSiteV1,
        crate::mir::resolved_semantics::home_new_prefix::BorrowedCallActualRequestV1<'_>,
    ) -> Result<
        Option<crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1>,
        E,
    >,
) -> Result<Option<Rc<ExpressionCallEntryFactsV1>>, E> {
    use crate::mir::resolved_semantics::home_new_prefix::{
        BorrowedCallActualRequestV1 as Request, BorrowedCallArgumentsV1 as Reply,
        LocalCallArgumentV1,
    };
    let Some(Reply::CallEntryHomes {
        taken,
        fresh,
        arguments,
        contract_proved,
        preserves_reads,
        ..
    }) = port(site, Request::CallEntryHomes(input))?
    else {
        return Ok(None);
    };
    if !contract_proved && !(borrowed_input && taken.is_empty() && fresh.is_empty()) {
        return Ok(None);
    }
    let mut nested = Vec::new();
    for (ordinal, argument) in actuals.iter().enumerate() {
        let LocalCallArgumentV1::CallResult(child) = argument else {
            continue;
        };
        if !child
            .site()
            .site()
            .is_argument_of(site.site(), ordinal as u32)
        {
            return Ok(None);
        }
        if arguments.iter().any(|frame| matches!(frame.source().root(),
            crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { ordinal: candidate, .. }
                if *candidate == ordinal as u32)) { continue; }
        let Some(facts) = child.entry_ownership_v1() else {
            return Ok(None);
        };
        if facts.site != *child.site() {
            return Ok(None);
        }
        nested.push((ordinal as u32, Rc::clone(facts)));
    }
    Ok(Some(Rc::new(ExpressionCallEntryFactsV1 {
        site: site.clone(),
        taken,
        fresh,
        arguments,
        nested: nested.into_boxed_slice(),
        preserves_reads,
    })))
}

impl LocalCallObservationV1 {
    pub(crate) fn entry_ownership_v1(&self) -> Option<&Rc<ExpressionCallEntryFactsV1>> {
        self.entry_ownership.as_ref()
    }
}
impl crate::mir::resolved_semantics::home_new_prefix::ArgumentCallObservationV1 {
    pub(crate) fn entry_ownership_v1(&self) -> Option<&Rc<ExpressionCallEntryFactsV1>> {
        self.entry_ownership.as_ref()
    }
}
