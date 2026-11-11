//! Original argument-plan membership and exact binding-read projection.
//! This query grants neither execution nor ownership transfer.
use super::model::{ArgumentExpressionV1, BorrowedFormalActualSourceV1};
use super::*;
use std::collections::BTreeSet;

impl BorrowedFormalActualSourceV1 {
    pub(crate) fn transfers_home_at_entry_v1(&self) -> bool {
        self.taken_home_v1().is_some() || matches!(self, Self::FreshTakenHome { .. })
    }

    pub(crate) fn taken_home_v1(&self) -> Option<(BindingRefV1, BindingRefV1, &OwnedExprSiteV1)> {
        match self {
            Self::TypedHome {
                moved: Some(_),
                binding,
                root,
                acquisition,
                ..
            } => Some((*binding, *root, acquisition)),
            _ => None,
        }
    }

    pub(crate) fn binding_read_site_v1<'a>(
        &'a self,
        actual: &'a SourceExprSiteV1,
    ) -> Result<&'a SourceExprSiteV1, String> {
        match self {
            Self::TypedHome {
                moved: Some(moved), ..
            } => {
                if moved.site() != actual || !moved.is_move() {
                    return Err(freeze("borrowed-actual/argument-source-identity"));
                }
                Ok(moved.operand())
            }
            _ => Ok(actual),
        }
    }
}

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
        let read_site = match &candidate.value {
            BorrowedCallActualValueV1::Home {
                moved: Some(moved), ..
            } => {
                if moved.site() != &candidate.site
                    || input.function().expression_source().moved(&candidate.site) != Some(moved)
                {
                    return Err(reject());
                }
                moved.operand()
            }
            _ => &candidate.site,
        };
        let atom = crate::mir::resolved_semantics::home_new_prefix::borrowed_actual_source_atom_v1(
            input, read_site,
        );
        let same = match (&candidate.value, atom.as_ref()) {
            (BorrowedCallActualValueV1::FreshConstruction(construction), None) => {
                input.function().expression_source().construction(read_site) == Some(construction)
            }
            (
                BorrowedCallActualValueV1::DeclaredRead(read),
                Some(BorrowedCallActualValueV1::Binding(original)),
            ) => {
                read.binding() == *original
                    && read.read().site() == read_site
                    && read.corroborates_v1(input)
            }
            (
                BorrowedCallActualValueV1::Scalar(binding, _)
                | BorrowedCallActualValueV1::SelfRooted { binding, .. }
                | BorrowedCallActualValueV1::Home { binding, .. }
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
pub(super) fn validate_original_argument_plans_v1(
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

/// Where the common actual constructor borrows the original local New domain.
/// This chooses an issuer context; a failed source lookup never retries another.
#[derive(Clone, Copy)]
pub(super) enum ObjectActualHomeSourceV1<'a> {
    Candidates(&'a [super::super::super::candidate::OrdinaryNewCandidate]),
    Ledger(&'a crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1),
    SourceOnly,
}
impl ObjectActualHomeSourceV1<'_> {
    pub(super) fn home_class(
        self,
        call: &OwnedExprSiteV1,
        binding: BindingRefV1,
        root: BindingRefV1,
        acquisition: &OwnedExprSiteV1,
    ) -> Result<Option<Box<str>>, String> {
        if binding.owner() != call.owner()
            || root.owner() != call.owner()
            || acquisition.owner() != call.owner()
        {
            return Err(format!("{}: call={call:?}, binding={binding:?}, root={root:?}, acquisition={acquisition:?}",
                freeze("borrowed-actual/home-domain")));
        }
        match self {
            Self::Candidates(candidates) => {
                let mut matching = candidates
                    .iter()
                    .filter(|row| &row.site == acquisition && row.destination == root);
                let candidate = matching
                    .next()
                    .ok_or_else(|| freeze("borrowed-actual/home-source"))?;
                if matching.next().is_some() || candidate.construction.is_err() {
                    return Err(format!("{}: call={call:?}, root={root:?}, acquisition={acquisition:?}, class={}, construction={:?}",
                        freeze("borrowed-actual/home-domain"), candidate.class, candidate.construction.as_ref().err()));
                }
                Ok(Some(candidate.class.clone()))
            }
            Self::Ledger(ledger) => ledger.borrowed_local_new_actual_class_v1(acquisition, root),
            Self::SourceOnly => Err(freeze("borrowed-actual/home-source-unconnected")),
        }
    }
}

impl ObjectActualHomeSourceV1<'_> {
    pub(super) fn prepare_home_actual_v1(
        self,
        call: &OwnedExprSiteV1,
        formal: &crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractV1,
        actual: &BorrowedCallActualCandidateV1,
    ) -> Result<Option<BorrowedFormalActualSourceV1>, String> {
        let BorrowedCallActualValueV1::Home {
            binding,
            root,
            acquisition,
            moved,
        } = &actual.value
        else {
            return Err(freeze("borrowed-actual/source-identity"));
        };
        if formal.ordinal != actual.ordinal
            || (formal.demand == crate::mir::resolved_semantics::HomeDemandV1::Home)
                != moved.is_some()
            || moved
                .as_ref()
                .is_some_and(|unary| unary.site() != &actual.site || !unary.is_move())
        {
            return Err(freeze("borrowed-actual/source-identity"));
        }
        let Some(class) = self.home_class(call, *binding, *root, acquisition)? else {
            return Ok(None);
        };
        if matches!(&formal.kind, CallableParameterContractKindV1::DeclaredObject(expected) if expected != &class)
        {
            return Err(freeze("borrowed-actual/declared-object-class"));
        }
        Ok(Some(BorrowedFormalActualSourceV1::TypedHome {
            binding: *binding,
            root: *root,
            acquisition: acquisition.clone(),
            moved: moved.clone(),
            class,
        }))
    }
}

/// Project ownership demand from the same admitted argument observations. This
/// does not complete a pending input cohort or grant physical-call permission.
pub(in crate::mir::normal_callable_semantic_package) fn source_take_homes_v1(
    pending: &PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
    candidates: &[super::super::super::candidate::OrdinaryNewCandidate],
    contracts: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    prepared: &Result<PreparedBorrowedFormalIngressV1, String>,
    targets: &super::super::PreparedLexicalInstanceCallSourceTargetsV1,
) -> Result<
    (
        bool,
        bool,
        Box<[(BindingRefV1, BindingRefV1, OwnedExprSiteV1)]>,
        Box<[(u32, OwnedExprSiteV1)]>,
        Box<
            [Rc<
                crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1,
            >],
        >,
    ),
    String,
> {
    let (instance, taken, fresh) = source_take_homes_with_source_v1(
        pending,
        site,
        ObjectActualHomeSourceV1::Candidates(candidates),
        contracts,
    )?;
    let rows = pending
        .get(site)
        .map(|row| row.as_ref().map_err(Clone::clone))
        .transpose()?;
    let retained_contract = rows.is_some_and(|row| {
        row.source_static_snapshot_v1().is_some()
            || matches!(
                row.phase,
                BorrowedCallActualEvidencePhaseV1::SourceInstance(_)
                    | BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(_)
                    | BorrowedCallActualEvidencePhaseV1::SourceObject(_)
                    | BorrowedCallActualEvidencePhaseV1::ExecutableSourceObject(_)
            )
    });
    let proved =
        retained_contract || exact_no_entry_transfer_v1(input, prepared, targets, contracts, site)?;
    let mut children = Vec::new();
    if let Some(rows) = rows {
        let slots = if let Some(dynamic) = &rows.dynamic_invocation_arguments {
            Some(&dynamic.arguments)
        } else if let Some(snapshot) = rows.source_static_snapshot_v1() {
            match &snapshot.phase {
                BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) => {
                    Some(&identity.argument_sources)
                }
                _ => None,
            }
        } else {
            match &rows.phase {
                BorrowedCallActualEvidencePhaseV1::SourceInstance(identity)
                | BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(identity) => {
                    Some(&identity.argument_sources)
                }
                BorrowedCallActualEvidencePhaseV1::SourceObject(identity)
                | BorrowedCallActualEvidencePhaseV1::ExecutableSourceObject(identity) => {
                    Some(&identity.argument_sources)
                }
                _ => None,
            }
        };
        for slot in slots.into_iter().flat_map(|slots| slots.values()) {
            if let ArgumentExpressionV1::Qualified(frame) = slot {
                children.push(Rc::clone(frame));
            }
        }
        for actual in &rows.opaque_actuals {
            if let BorrowedFormalActualSourceV1::Expression(source) = &actual.source {
                if !children.iter().any(|old| Rc::ptr_eq(old, source.frame())) {
                    children.push(Rc::clone(source.frame()));
                }
            }
        }
        children.sort_by_key(|frame| match frame.source().root() {
            crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { ordinal, .. } => *ordinal,
            _ => u32::MAX,
        });
        let mut ordinals = BTreeSet::new();
        for frame in &children {
            let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call, ordinal, .. } = frame.source().root() else {
                return Err(freeze("borrowed-actual/argument-source-identity"));
            };
            if call.site() != site.site() || !ordinals.insert(*ordinal) {
                return Err(freeze("borrowed-actual/argument-source-identity"));
            }
        }
    }
    Ok((instance, proved, taken, fresh, children.into_boxed_slice()))
}

/// A missing pending row is not a no-transfer proof. Lend only the original
/// declaration contract of this exact retained Static/Instance source target.
fn exact_no_entry_transfer_v1(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    prepared: &Result<PreparedBorrowedFormalIngressV1, String>,
    targets: &super::super::PreparedLexicalInstanceCallSourceTargetsV1,
    contracts: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    site: &OwnedExprSiteV1,
) -> Result<bool, String> {
    let Some(method) = input.function().method_call_source_v1(site.site()) else {
        return Ok(false);
    };
    let target = match prepared
        .as_ref()
        .ok()
        .map(|source| super::static_source::original_static_source_v1(source, site))
        .transpose()?
        .flatten()
    {
        Some(source) => {
            if !std::sync::Arc::ptr_eq(source.method_source(), method) {
                return Err(freeze("borrowed-actual/source-identity"));
            }
            Some((
                source.callee_owner(),
                source.target_batch_slot(),
                source.argument_sites(),
            ))
        }
        None => match super::super::original_lexical_target_at_v1(targets, site)? {
            Some(target) => {
                if input.owner() != site.owner() || !target.matches_receiver(input, method) {
                    return Err(freeze("borrowed-actual/source-identity"));
                }
                Some((
                    target.callee_owner(),
                    target.target_batch_slot(),
                    target.argument_sites(),
                ))
            }
            None => None,
        },
    };
    let Some((owner, slot, arguments)) = target else {
        return Ok(false);
    };
    let mut matching = contracts.iter().filter(|row| row.owner == owner);
    let Some(contract) = matching.next() else {
        return Ok(false);
    };
    if matching.next().is_some()
        || contract.batch_slot != slot
        || arguments.len() != contract.parameters.len()
        || arguments.len() != method.arguments().len()
        || arguments
            .iter()
            .zip(method.arguments())
            .any(|(site, actual)| site != actual.site())
    {
        return Err(freeze("borrowed-actual/source-identity"));
    }
    Ok(contract
        .parameters
        .iter()
        .all(|formal| formal.demand != crate::mir::resolved_semantics::HomeDemandV1::Home))
}

fn source_take_homes_with_source_v1(
    pending: &PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
    source: ObjectActualHomeSourceV1<'_>,
    contracts: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
) -> Result<
    (
        bool,
        Box<[(BindingRefV1, BindingRefV1, OwnedExprSiteV1)]>,
        Box<[(u32, OwnedExprSiteV1)]>,
    ),
    String,
> {
    let Some(rows) = pending.get(site) else {
        return Ok((false, Box::new([]), Box::new([])));
    };
    let rows = rows.as_ref().map_err(Clone::clone)?;
    let instance_input = matches!(
        &rows.phase,
        BorrowedCallActualEvidencePhaseV1::SourceInstance(_)
            | BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(_)
            | BorrowedCallActualEvidencePhaseV1::SourceObject(_)
            | BorrowedCallActualEvidencePhaseV1::ExecutableSourceObject(_)
    );
    let passive = if let Some(snapshot) = rows.source_static_snapshot_v1() {
        let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &snapshot.phase else {
            return Err(freeze("borrowed-actual/source-identity"));
        };
        if identity.source.call_site() != site {
            return Err(freeze("borrowed-actual/source-identity"));
        }
        Some((identity.source.parameters(), identity.candidates.as_ref()))
    } else {
        let instance = match &rows.phase {
            BorrowedCallActualEvidencePhaseV1::SourceInstance(identity)
            | BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(identity) => Some((
                &identity.source,
                identity.candidates.as_ref(),
                identity.incoming_arguments.as_ref(),
            )),
            BorrowedCallActualEvidencePhaseV1::SourceObject(identity)
            | BorrowedCallActualEvidencePhaseV1::ExecutableSourceObject(identity) => Some((
                &identity.source,
                identity.candidates.as_ref(),
                identity.incoming_arguments.as_ref(),
            )),
            _ => None,
        };
        if let Some((target, actuals, incoming)) = instance {
            let mut matching = contracts
                .iter()
                .filter(|row| row.owner == target.callee_owner());
            let contract = matching
                .next()
                .ok_or_else(|| freeze("borrowed-actual/formal-missing"))?;
            if matching.next().is_some()
                || target.call_site() != site
                || target.target_batch_slot() != contract.batch_slot
                || target.argument_sites().len() != contract.parameters.len()
                || actuals.len() != contract.parameters.len()
            {
                return Err(freeze("borrowed-actual/source-identity"));
            }
            for (index, (formal, actual)) in contract.parameters.iter().zip(actuals).enumerate() {
                if formal.ordinal as usize != index
                    || actual.ordinal != formal.ordinal
                    || formal.binding.owner() != contract.owner
                    || target.argument_sites().get(index) != Some(&actual.site)
                {
                    return Err(freeze("borrowed-actual/source-identity"));
                }
                if formal.demand == crate::mir::resolved_semantics::HomeDemandV1::Home
                    && incoming
                        .iter()
                        .filter(|(ordinal, at, binding)| {
                            *ordinal == formal.ordinal
                                && at == &actual.site
                                && *binding == formal.binding
                        })
                        .count()
                        != 1
                {
                    return Err(freeze("borrowed-actual/source-identity"));
                }
            }
            Some((contract.parameters.as_ref(), actuals))
        } else {
            None
        }
    };
    let mut homes = Vec::new();
    let mut fresh = Vec::new();
    if let Some((parameters, actuals)) = passive {
        if parameters.len() != actuals.len() {
            return Err(freeze("borrowed-actual/arity"));
        }
        for (formal, candidate) in parameters.iter().zip(actuals) {
            if formal.demand != crate::mir::resolved_semantics::HomeDemandV1::Home {
                continue;
            }
            if let BorrowedCallActualValueV1::FreshConstruction(construction) = &candidate.value {
                if formal.ordinal != candidate.ordinal
                    || construction.site() != &candidate.site
                    || !candidate.site.is_argument_of(site.site(), formal.ordinal)
                    || !formal.takes_fresh_construction_v1(construction)
                {
                    return Err(freeze("borrowed-actual/source-identity"));
                }
                fresh.push((
                    formal.ordinal,
                    OwnedExprSiteV1::new(site.owner(), construction.site().clone()),
                ));
                // Completion and pre-entry cleanup stay with the original New.
                continue;
            }
            let source = source
                .prepare_home_actual_v1(site, formal, candidate)?
                .ok_or_else(|| freeze("borrowed-actual/home-source"))?;
            let (binding, root, acquisition) = source
                .taken_home_v1()
                .ok_or_else(|| freeze("borrowed-actual/source-identity"))?;
            homes.push((binding, root, acquisition.clone()));
        }
    } else {
        for actual in &rows.opaque_actuals {
            if let BorrowedFormalActualSourceV1::FreshTakenHome { acquisition, .. } = &actual.source
            {
                if acquisition.owner() != site.owner()
                    || acquisition.site() != &actual.site
                    || !actual.site.is_argument_of(site.site(), actual.ordinal)
                {
                    return Err(freeze("borrowed-actual/source-identity"));
                }
                fresh.push((actual.ordinal, acquisition.clone()));
            }
        }
        homes.extend(
            rows.opaque_actuals
                .iter()
                .filter_map(|actual| actual.source.taken_home_v1())
                .map(|(binding, root, acquisition)| (binding, root, acquisition.clone())),
        );
    }
    Ok((
        instance_input,
        homes.into_boxed_slice(),
        fresh.into_boxed_slice(),
    ))
}

impl OrdinaryNewClaimLedgerV1 {
    /// Check only unconnected use boundaries. Connected consumers project the
    /// evaluator's state; this gate never replays transfers from a snapshot.
    pub(in crate::mir::normal_callable_semantic_package) fn validate_expression_ownership_uses_v1(
        &self,
    ) -> Result<(), String> {
        use crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1 as Root;
        use crate::mir::resolved_value_profile::expression_root::StatementExpressionRoleV1 as Role;
        fn connected(
            plan: &crate::mir::resolved_semantics::home_new_prefix::HomeExpressionPlanV1,
            entries: &mut BTreeSet<OwnedExprSiteV1>,
        ) {
            if plan.initializer_frame().is_some()
                || plan.rebind_frame().is_some()
                || plan.print_frame().is_some()
                || plan.condition_frame().is_some()
            {
                if let Some(evaluation) = plan.evaluation() {
                    entries.extend(
                        evaluation
                            .ownership_v1()
                            .entries()
                            .map(|entry| entry.site.clone()),
                    );
                }
            }
            if let Some(product) = plan.loop_recipe_v2() {
                for body in &product.source.body_expressions {
                    connected(body, entries);
                }
            }
        }
        let completions = self
            .completion_index
            .values()
            .filter_map(|row| row.as_ref().ok())
            .chain(
                self.root_completion
                    .iter()
                    .filter_map(|row| row.as_ref().ok())
                    .filter(|row| !self.completion_index.contains_key(&row.owner())),
            );
        for completion in completions {
            let mut entries = BTreeSet::new();
            if let Some(flow) = completion.cleanup().root_flow() {
                for plan in flow.expression_plans() {
                    connected(plan, &mut entries);
                }
                for frame in flow
                    .expression_plans()
                    .iter()
                    .filter_map(|plan| plan.argument_frame())
                {
                    let Root::CallArgument { call, ordinal, .. } = frame.source().root() else {
                        continue;
                    };
                    let at = OwnedExprSiteV1::new(completion.owner(), call.site().clone());
                    let root = frame
                        .source()
                        .nodes()
                        .first()
                        .ok_or_else(|| freeze("expression/argument-owner-unavailable"))?;
                    let canonical =
                        self.qualified_argument_expression_frame_v1(&at, *ordinal, root.site())?;
                    if !Rc::ptr_eq(frame, &canonical) {
                        return Err(freeze("expression/argument-source-drift"));
                    }
                }
            }
            self.visit_expression_evaluations_v1(completion.owner(), &mut |evaluation| {
                if matches!(
                    evaluation.source().root(),
                    Root::Statement {
                        role: Role::ReturnValue,
                        ..
                    }
                ) {
                    entries.extend(
                        evaluation
                            .ownership_v1()
                            .entries()
                            .map(|entry| entry.site.clone()),
                    );
                }
                Ok(true)
            })?;
            self.visit_expression_evaluations_v1(completion.owner(), &mut |evaluation| {
                if !evaluation.ownership_v1().has_transfers() {
                    return Ok(true);
                }
                if matches!(evaluation.source().root(), Root::NewArgument { .. }) {
                    return Err(freeze("expression/argument-entry-transition-unavailable"));
                }
                if evaluation
                    .ownership_v1()
                    .entries()
                    .any(|entry| !entry.taken.is_empty() && !entries.contains(&entry.site))
                {
                    return Err(freeze("expression/argument-owner-unavailable"));
                }
                Ok(true)
            })?;
        }
        Ok(())
    }
}
