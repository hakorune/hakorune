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
) -> Result<(bool, Box<[(BindingRefV1, BindingRefV1, OwnedExprSiteV1)]>), String> {
    source_take_homes_with_source_v1(
        pending,
        site,
        ObjectActualHomeSourceV1::Candidates(candidates),
        contracts,
    )
}

fn source_take_homes_with_source_v1(
    pending: &PendingBorrowedFormalActualsV1,
    site: &OwnedExprSiteV1,
    source: ObjectActualHomeSourceV1<'_>,
    contracts: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
) -> Result<(bool, Box<[(BindingRefV1, BindingRefV1, OwnedExprSiteV1)]>), String> {
    let Some(rows) = pending.get(site) else {
        return Ok((false, Box::new([])));
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
                // This list moves existing bindings. The acquisition is owned by
                // the ordered argument and transfers through the call packet.
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
        homes.extend(
            rows.opaque_actuals
                .iter()
                .filter_map(|actual| actual.source.taken_home_v1())
                .map(|(binding, root, acquisition)| (binding, root, acquisition.clone())),
        );
    }
    Ok((instance_input, homes.into_boxed_slice()))
}

impl OrdinaryNewClaimLedgerV1 {
    /// Complete source cleanup snapshots must match every ordered entry transfer.
    pub(in crate::mir::normal_callable_semantic_package) fn validate_expression_entry_cleanup_v1(
        &self,
        contracts: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    ) -> Result<(), String> {
        use crate::mir::resolved_semantics::home_new_prefix::{
            TerminalRelationV1, TerminalReturnedSourceV1,
        };
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
            let ordinary = completion
                .cleanup()
                .root_flow()
                .into_iter()
                .flat_map(|flow| flow.expression_plans())
                .filter_map(|plan| plan.evaluation());
            let terminal = self
                .terminal_relations_for_owner(completion.owner())
                .into_iter()
                .filter_map(|row| match row {
                    TerminalRelationV1::Scalar(value) => {
                        value.evaluation().map(|frame| frame.as_ref())
                    }
                    TerminalRelationV1::Value(value) => match value.returned() {
                        TerminalReturnedSourceV1::TextExpression(frame) => Some(frame.as_ref()),
                        _ => None,
                    },
                    _ => None,
                });
            let evaluations: Vec<_> = ordinary.chain(terminal).collect();
            for evaluation in &evaluations {
                self.validate_evaluation_entry_cleanup_v1(evaluation, contracts)?;
            }
            if let Some(flow) = completion.cleanup().root_flow() {
                for frame in flow
                    .expression_plans()
                    .iter()
                    .filter_map(|plan| plan.argument_frame())
                {
                    let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call, ordinal, .. } = frame.source().root() else { continue; };
                    let at = OwnedExprSiteV1::new(completion.owner(), call.site().clone());
                    if evaluations
                        .iter()
                        .any(|evaluation| evaluation.calls().iter().any(|call| call.site() == &at))
                    {
                        continue;
                    }
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
                    for child in frame.evaluation().calls() {
                        if self.call_takes_declared_home_v1(child.site(), contracts)? {
                            return Err(freeze("expression/argument-owner-unavailable"));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn call_takes_declared_home_v1(
        &self,
        site: &OwnedExprSiteV1,
        contracts: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    ) -> Result<bool, String> {
        if let Some(pending) = self.borrowed_formal_actuals.get(site) {
            let pending = pending.as_ref().map_err(Clone::clone)?;
            if pending
                .opaque_actuals
                .iter()
                .any(|actual| actual.source.transfers_home_at_entry_v1())
            {
                return Ok(true);
            }
            if let Some(snapshot) = pending.source_static_snapshot_v1() {
                let BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) = &snapshot.phase
                else {
                    return Err(freeze("borrowed-actual/source-identity"));
                };
                if identity.source.call_site() != site {
                    return Err(freeze("borrowed-actual/source-identity"));
                }
                return Ok(identity
                    .source
                    .parameters()
                    .iter()
                    .any(|parameter| parameter.takes_declared_object_v1()));
            }
        }
        Ok(self
            .borrowed_formal_source
            .as_ref()
            .and_then(|row| row.as_ref().ok())
            .is_some_and(|prepared| {
                prepared
                    .source_incoming
                    .exact_rows()
                    .chain(prepared.source_incoming.source_rows_v1())
                    .filter(|row| &row.call == site)
                    .any(|row| {
                        contracts.iter().any(|contract| {
                            contract.owner == row.source.callee_owner()
                                && contract.batch_slot == row.source.target_batch_slot()
                                && contract
                                    .parameters
                                    .iter()
                                    .any(|parameter| parameter.takes_declared_object_v1())
                        })
                    })
            }))
    }

    fn validate_evaluation_entry_cleanup_v1(
        &self,
        evaluation: &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
        contracts: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    ) -> Result<(), String> {
        use crate::mir::resolved_value_profile::expression_plan::{
            ExpressionNodeSourceV1, ExpressionPlanRootV1,
        };
        use crate::mir::resolved_value_profile::expression_root::StatementExpressionRoleV1;
        let mut transferred = BTreeSet::new();
        let reject = || freeze("expression/take-cleanup-unavailable");
        let still_owed = |transferred: &BTreeSet<_>| {
            evaluation
                .prior_homes()
                .iter()
                .any(|home| transferred.contains(&home.binding()))
        };
        for &index in evaluation.ordered_nodes() {
            let node = &evaluation.source().nodes()[index];
            let site = OwnedExprSiteV1::new(evaluation.source().owner(), node.site().clone());
            let checked_field = self
                .local_field_reads
                .borrow()
                .get(&site)
                .map(|row| row.source.requires_checked_field_read_v1());
            if evaluation.node_requires_fault_frame_v1(index, checked_field)
                && still_owed(&transferred)
            {
                return Err(reject());
            }
            if !matches!(node.source(), ExpressionNodeSourceV1::MethodCall) {
                continue;
            }
            if self.call_takes_declared_home_v1(&site, contracts)? {
                let (_, taken) = source_take_homes_with_source_v1(
                    &self.borrowed_formal_actuals,
                    &site,
                    ObjectActualHomeSourceV1::Ledger(self),
                    contracts,
                )?;
                for (binding, root, _) in taken.iter().cloned() {
                    if evaluation
                        .prior_homes()
                        .iter()
                        .filter(|home| {
                            home.binding() == binding && home.stable_origin() == Some(root)
                        })
                        .count()
                        != 1
                        || !transferred.insert(binding)
                    {
                        return Err(reject());
                    }
                }
            }
        }
        if !transferred.is_empty()
            && matches!(
                evaluation.source().root(),
                ExpressionPlanRootV1::IfCondition { .. }
                    | ExpressionPlanRootV1::LoopCondition { .. }
            )
        {
            return Err(freeze("expression/take-successor-unavailable"));
        }
        let destination_may_fault = !evaluation.owned_result_nodes().is_empty()
            || matches!(
                evaluation.source().root(),
                ExpressionPlanRootV1::Statement {
                    role: StatementExpressionRoleV1::PrintValue,
                    ..
                }
            );
        if destination_may_fault && still_owed(&transferred) {
            return Err(reject());
        }
        Ok(())
    }
}
