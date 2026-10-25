//! Lend the sole sealed result facts, never an executable call disposition.
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::BorrowedResultCohortV1;
use super::*;
use super::composition::{source_dependencies_v1, grounded_source_results_with_ingress_v1, SourceDependencyRoleV1};
use crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind;

#[derive(Debug, Clone)]
pub(crate) struct BorrowedScalarResultSourceLoanV1 {
    source: LexicalInstanceCallSourceTargetV1,
    facts: Rc<BorrowedResultSourceFactsV1>,
}

impl PartialEq for BorrowedScalarResultSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source && Rc::ptr_eq(&self.facts, &other.facts)
    }
}
impl Eq for BorrowedScalarResultSourceLoanV1 {}

impl BorrowedScalarResultSourceLoanV1 {
    pub(crate) fn source(&self) -> &LexicalInstanceCallSourceTargetV1 {
        &self.source
    }

    pub(crate) fn kind(&self) -> SourceScalarKind {
        // Issuance admits precisely these two classes. Facts are immutable once
        // sealed; readiness and executable corroboration live outside this Rc.
        match self.facts.class {
            BorrowedResultClassV1::I64 => SourceScalarKind::Integer,
            BorrowedResultClassV1::Bool => SourceScalarKind::Bool,
            _ => unreachable!("sealed scalar loan class"),
        }
    }
}

type ResultSources = BorrowedResultCohortV1;

fn exact_original_target(
    ingress: &PreparedBorrowedFormalIngressV1,
    target: &LexicalInstanceCallSourceTargetV1,
) -> Result<(), String> {
    let source_member = ingress.source_incoming.source_rows_with_legacy_v1()
        .any(|(row, legacy)| !legacy && &row.call == target.call_site());
    let calls = if source_member {
        ingress.source_incoming.project_source_v1(&BTreeSet::from([target.callee_owner()]))
            .map_err(|issue| format!("{}: {issue:?}", freeze("scalar-loan/original-identity")))?
    } else {
        ingress.source_incoming.exact_rows().filter(|row| &row.call == target.call_site())
            .cloned().collect::<Vec<_>>().into_boxed_slice()
    };
    let mut originals = calls.iter().filter(|row| &row.call == target.call_site());
    let original = originals.next().ok_or_else(|| freeze("scalar-loan/original-missing"))?;
    if originals.next().is_some() || original.callee != target.callee_owner()
        || original.source.require_instance()? != target {
        return Err(freeze("scalar-loan/original-identity"));
    }
    Ok(())
}

/// One immutable source scan; derived grounding never outlives its inputs.
/// Per-loan identity and dependency checks remain independent of this fold.
struct ScalarSourceValidationV1<'a> {
    source: &'a Result<PreparedBorrowedFormalIngressV1, String>,
    results: &'a ResultSources,
    grounded: std::cell::OnceCell<BTreeSet<FunctionOwnerIdV1>>,
}
impl<'a> ScalarSourceValidationV1<'a> {
    fn new(source: &'a Result<PreparedBorrowedFormalIngressV1, String>, results: &'a ResultSources) -> Self {
        Self { source, results, grounded: std::cell::OnceCell::new() }
    }
    fn demand(&self, target: &LexicalInstanceCallSourceTargetV1) -> Result<&'a BorrowedI64ResultSourceV1, String> {
        let ingress = self.source.as_ref().map_err(Clone::clone)?;
        let results = self.results;
        exact_original_target(ingress, target)?;
        let proof = results
            .get(&target.callee_owner())
            .ok_or_else(|| freeze("scalar-loan/result-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        proof.require_source_scalar_v1()?;
        if let super::super::LexicalCallSourceResultRequirementV1::ScalarReturnSource(original) = &target.result_requirement {
            if !original.matches(&super::super::source::CallTargetReferenceV1::from_target(target))
                || !proof.matches_original_return_v1(original)
            { return Err(freeze("scalar-loan/original-return-source")); }
        }
        if proof.returns.is_empty()
            || proof
                .returns
                .iter()
                .any(|site| site.owner() != target.callee_owner())
            || proof.returns.iter().collect::<BTreeSet<_>>().len() != proof.returns.len()
            || !self.grounded.get_or_init(|| grounded_source_results_with_ingress_v1(results, ingress))
                .contains(&target.callee_owner())
        {
            return Err(freeze("scalar-loan/result-coverage"));
        }
        source_dependencies_v1(proof, ingress)?;
        Ok(proof)
    }

    fn validate(&self, loan: &BorrowedScalarResultSourceLoanV1, target: &LexicalInstanceCallSourceTargetV1) -> Result<(), String> {
        if loan.source() != target { return Err(freeze("scalar-loan/target-drift")); }
        let proof = self.demand(target)?;
        if !Rc::ptr_eq(&loan.facts, &proof.source_facts) {
            return Err(freeze("scalar-loan/result-identity"));
        }
        Ok(())
    }
}

pub(in crate::mir::normal_callable_semantic_package) fn issue_source_scalar_result_loan_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    results: &ResultSources,
    target: &LexicalInstanceCallSourceTargetV1,
) -> Result<BorrowedScalarResultSourceLoanV1, String> {
    let validation = ScalarSourceValidationV1::new(source, results);
    let proof = validation.demand(target)?;
    Ok(BorrowedScalarResultSourceLoanV1 {
        source: target.clone(),
        facts: Rc::clone(&proof.source_facts),
    })
}

pub(in crate::mir::normal_callable_semantic_package) fn validate_source_scalar_result_loan_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    results: &ResultSources,
    loan: &BorrowedScalarResultSourceLoanV1,
    target: &LexicalInstanceCallSourceTargetV1,
) -> Result<(), String> {
    ScalarSourceValidationV1::new(source, results).validate(loan, target)
}

/// Result agreement after co-seal, distinct from source issuance and execution.
/// Re-demand the retained Rc and every original dependency before consulting the
/// complete declaration/Completion row. This grants no input or ABI permission.
pub(in crate::mir::normal_callable_semantic_package) fn corroborate_source_scalar_result_loan_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    results: &ResultSources,
    loan: &BorrowedScalarResultSourceLoanV1,
    target: &LexicalInstanceCallSourceTargetV1,
    contracts: &crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1,
) -> Result<(), String> {
    let validation = ScalarSourceValidationV1::new(source, results);
    validation.validate(loan, target)?;
    let mut pending = vec![target];
    let mut seen = BTreeSet::new();
    while let Some(current) = pending.pop() {
        // Every edge keeps its original target even when two calls share a callee.
        let proof = validation.demand(current)?;
        if !seen.insert(current.callee_owner()) {
            continue;
        }
        if !proof.contract_corroborated
            || !source_result_contract_agrees_v1(proof, current, contracts, false)
        {
            return Err(freeze("scalar-loan/result-contract-mismatch"));
        }
        pending.extend(source_dependencies_v1(proof, source.as_ref().map_err(Clone::clone)?)?.into_iter().map(|(target, _)| target));
    }
    Ok(())
}

fn source_scalar_seed_target_v1(
    ingress: &PreparedBorrowedFormalIngressV1, target: &LexicalInstanceCallSourceTargetV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    results: &ResultSources, classes: &super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
) -> Result<bool, String> {
    if !target.uses_existing_borrowed_result_v1() { return Ok(false); }
    if let super::super::LexicalCallSourceResultRequirementV1::ScalarReturnSource(original) = &target.result_requirement {
        if !original.matches(&super::super::source::CallTargetReferenceV1::from_target(target)) {
            return Err(freeze("scalar-loan/original-return-target"));
        }
        return Ok(true);
    }
    // The original incoming census already retains zero-input scalar demands
    // without a borrowed-formal draft. Use that same source family here; a
    // missing/refused result must then fail demand, never deselect the call.
    let zero_input_source = target.target().arity() == 0 && target.argument_sites().is_empty();
    if zero_input_source { exact_original_target(ingress, target)?; }
    let existing_member = zero_input_source || target.stored_receiver().is_some()
        || ingress.source_only_definitions.contains_key(&target.call_site().owner());
    if !existing_member {
        // Affirmative other-result families keep their own issuer. Do not
        // select a generic ClaimLocal call merely because it returns scalar.
        if classes.outcomes(target.target()).is_some()
            || results.get(&target.callee_owner()).is_some_and(|proof|
                proof.as_ref().is_ok_and(|proof| proof.class == BorrowedResultClassV1::Nullable))
        { return Ok(false); }
        let array_input = ingress.original_array_argument_source_v1(target)?;
        let core_input = super::super::borrowed_formal_actuals::original_core_argument_input_request_v1(
            ingress, contracts, target.call_site())?;
        // Input membership is not a result class. The SAME passive Return
        // census selects this scalar demand even when scalar proof is missing;
        // Text/object results keep their independent result owner.
        let finite_input = if crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1::original_finite_instance_input_request_v1(
            ingress, contracts, target.callee_owner())? {
            match results.original_return_source_v1(&target.callee_owner())? {
                Some(original) if original.is_value_return_source_v1() => {
                    if !original.matches(&super::super::source::CallTargetReferenceV1::from_target(target)) {
                        return Err(freeze("scalar-loan/original-return-target"));
                    }
                    true
                }
                _ => false,
            }
        } else { false };
        if core_input || finite_input { exact_original_target(ingress, target)?; }
        if !array_input && !core_input && !finite_input { return Ok(false); }
    }
    // Affirmative original non-scalar facts select their own issuer before demand.
    // Absence/Err of selected scalar evidence is never an unselection predicate.
    if classes.outcomes(target.target()).is_some() {
        let roots = classes.checked_original_source_roots_v1(target.target(), target.callee_owner())
            .map_err(|issue| freeze(&format!("scalar-loan/non-scalar/{issue}")))?;
        if roots.is_empty() { return Err(freeze("scalar-loan/non-scalar/empty")); }
        return Ok(false);
    }
    if let Some(Ok(proof)) = results.get(&target.callee_owner()) {
        if proof.class == BorrowedResultClassV1::Nullable {
            proof.require_source_sealed_v1()?;
            return Ok(false);
        }
    }
    Ok(true)
}

/// Follow only retained scalar Return dependencies to an original family seed.
/// No seedless cycle or unrelated call/discard makes a scalar family member.
fn selected_source_scalar_target_v1(
    ingress: &PreparedBorrowedFormalIngressV1, target: &LexicalInstanceCallSourceTargetV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    results: &ResultSources, classes: &super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
) -> Result<bool, String> {
    if source_scalar_seed_target_v1(ingress, target, contracts, results, classes)? {
        return Ok(true);
    }
    if !target.uses_existing_borrowed_result_v1()
        || classes.outcomes(target.target()).is_some()
        || results.get(&target.callee_owner()).is_some_and(|proof|
            proof.as_ref().is_ok_and(|proof| proof.class == BorrowedResultClassV1::Nullable))
    { return Ok(false); }
    let grounded = grounded_source_results_with_ingress_v1(results, ingress);
    let mut pending = vec![target.callee_owner()];
    let mut seen = BTreeSet::new();
    while let Some(owner) = pending.pop() {
        if !seen.insert(owner) { continue; }
        let Some(Ok(proof)) = results.get(&owner) else { continue; };
        if !matches!(proof.class, BorrowedResultClassV1::I64 | BorrowedResultClassV1::Bool) { continue; }
        if proof.returns.is_empty() || proof.returns.iter().any(|site| site.owner() != owner)
            || proof.returns.iter().collect::<BTreeSet<_>>().len() != proof.returns.len()
        { return Err(freeze("scalar-loan/result-coverage")); }
        let dependencies = source_dependencies_v1(proof, ingress)?;
        if !grounded.contains(&owner) { continue; }
        for (dependency, role) in dependencies {
            if role == SourceDependencyRoleV1::FieldWriter { continue; }
            // Full original target equality precedes owner traversal/dedup.
            if source_scalar_seed_target_v1(ingress, dependency, contracts, results, classes)? {
                return Ok(true);
            }
            if dependency.uses_existing_borrowed_result_v1()
                && classes.outcomes(dependency.target()).is_none()
            { pending.push(dependency.callee_owner()); }
        }
    }
    Ok(false)
}

/// Select the existing full Home walk from original scalar calls before Completion.
/// This lends no empty cleanup, input, signature or executable result permission.
pub(in crate::mir::normal_callable_semantic_package) fn required_scalar_source_walk_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>, results: &ResultSources,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    owner: FunctionOwnerIdV1, classes: &super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
) -> Result<bool, (OwnedExprSiteV1, String)> {
    let Ok(ingress) = source else { return Ok(false); };
    let mut pending = Vec::new();
    for original in ingress.source_incoming.exact_rows() {
        if let Some(target) = original.source.instance() {
            if selected_source_scalar_target_v1(ingress, target, contracts, results, classes)
                .map_err(|issue| (target.call_site().clone(), issue))?
                && match results.get(&target.callee_owner()) {
                    Some(Ok(proof)) => matches!(proof.class, BorrowedResultClassV1::I64 | BorrowedResultClassV1::Bool | BorrowedResultClassV1::UnresolvedScalar),
                    Some(Err(_)) | None => target.callee_owner() == owner,
                }
            { pending.push(target); }
        }
    }
    let mut seen = BTreeSet::new();
    let mut required = false;
    let validation = ScalarSourceValidationV1::new(source, results);
    while let Some(target) = pending.pop() {
        // Every edge is checked before deduplicating callee traversal.
        let proof = validation.demand(target).map_err(|issue| (target.call_site().clone(), issue))?;
        required |= target.callee_owner() == owner;
        if seen.insert(target.callee_owner()) {
            pending.extend(source_dependencies_v1(proof, ingress)
                .map_err(|issue| (target.call_site().clone(), issue))?.into_iter()
                .filter(|(_, role)| *role != SourceDependencyRoleV1::FieldWriter).map(|(target, _)| target));
        }
    }
    Ok(required)
}

/// Dedicated source demand, outside owning Object and nullable-LOCAL protocols.
/// Selected source absence/errors must never become an old-I64 retry.
pub(in crate::mir::normal_callable_semantic_package) fn source_scalar_result_arguments_v1(
    targets: &super::super::PreparedLexicalInstanceCallSourceTargetsV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    pending: &PendingBorrowedFormalActualsV1,
    results: &ResultSources,
    site: &OwnedExprSiteV1,
    classes: &super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
) -> Result<Option<crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1>, String>
{
    let mut matching = Vec::new();
    for row in targets.as_ref().map_err(Clone::clone)? {
        if let Some(target) = row.as_ref().map_err(Clone::clone)? {
            if target.call_site() == site {
                matching.push(target);
            }
        }
    }
    let ingress = match source {
        Ok(ingress) => ingress,
        Err(_) if matching.is_empty() => return Ok(None),
        Err(issue) => return Err(issue.clone()),
    };
    let original = crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1::original_instance_input_call_v1(ingress, contracts, site)?;
    let target = original.as_ref().and_then(|row| row.source.instance());
    let selected = target.map(|target| selected_source_scalar_target_v1(ingress, target, contracts, results, classes))
        .transpose()?.unwrap_or(false);
    let mut prepared_selected = false;
    for target in &matching {
        prepared_selected |= selected_source_scalar_target_v1(ingress, target, contracts, results, classes)?;
    }
    if !selected && !prepared_selected {
        if crate::config::env::builder_debug_enabled() && (target.is_some() || !matching.is_empty()) {
            eprintln!("[source-scalar] site={site:?} original={:?} prepared={} caller-source-only={} result={:?}",
                target, matching.len(), ingress.source_only_definitions.contains_key(&site.owner()),
                target.and_then(|target| results.get(&target.callee_owner())).map(|proof| proof.as_ref().map(|proof| &proof.class)));
        }
        return Ok(None);
    }
    let target = target.ok_or_else(|| freeze("scalar-loan/original-missing"))?;
    let [prepared] = matching.as_slice() else {
        return Err(freeze("scalar-loan/prepared-target-missing-or-duplicate"));
    };
    if *prepared != target {
        return Err(freeze("scalar-loan/target-drift"));
    }
    let loan = issue_source_scalar_result_loan_v1(source, results, target)?;
    let arguments =
        lend_source_scalar_result_arguments_v1(source, pending, contracts, results, &loan, None)?;
    Ok(Some(crate::mir::resolved_semantics::home_new_prefix::BorrowedCallArgumentsV1::SourceResult { loan: crate::mir::resolved_semantics::home_new_prefix::LocalCallResultSourceV1::Scalar(loan), arguments }))
}

/// Re-demand this loan and the SAME argument row without rebuilding a target list.
pub(in crate::mir::normal_callable_semantic_package) fn lend_source_scalar_result_arguments_v1(
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    pending: &PendingBorrowedFormalActualsV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    results: &ResultSources,
    loan: &BorrowedScalarResultSourceLoanV1,
    array_ledger: Option<&crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
) -> Result<Box<[crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1]>, String> {
    validate_source_scalar_result_loan_v1(source, results, loan, loan.source())?;
    let target = loan.source();
    if target.target().arity() == 0 && target.argument_sites().is_empty() {
        let ingress = source.as_ref().map_err(Clone::clone)?;
        exact_original_target(ingress, target)?;
        let mut matching = contracts
            .iter()
            .filter(|contract| contract.owner == target.callee_owner());
        let contract = matching
            .next()
            .ok_or_else(|| freeze("scalar-loan/zero-argument-contract-missing"))?;
        if matching.next().is_some()
            || contract.batch_slot != target.target_batch_slot()
            || !contract.parameters.is_empty()
        {
            return Err(freeze("scalar-loan/zero-argument-contract-identity"));
        }
        return Ok(Box::new([]));
    }
    super::super::borrowed_formal_actuals::project_pending_instance_arguments_by_phase_v1(
        source, pending, contracts, target, array_ledger,
    )
}

/// Function-owned projection of the SAME source result, without a fabricated call.
/// The body token retains this loan; it grants no input, Home or executable ABI.
#[derive(Debug)]
pub(crate) struct CallableScalarResultSourceLoanV1 {
    owner: FunctionOwnerIdV1,
    pub(super) facts: Rc<BorrowedResultSourceFactsV1>,
    pub(super) completion: Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
}
impl CallableScalarResultSourceLoanV1 {
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 { self.owner }
    pub(crate) fn kind(&self) -> SourceScalarKind {
        match self.facts.class {
            BorrowedResultClassV1::I64 => SourceScalarKind::Integer,
            BorrowedResultClassV1::Bool => SourceScalarKind::Bool,
            _ => unreachable!("issued callable scalar source"),
        }
    }
    pub(crate) fn returns(&self) -> &[OwnedExprSiteV1] { &self.facts.returns }
    pub(crate) fn field_array_reads(&self) -> &[Rc<pending::field_array_read::CheckedFieldArrayReadSourceV1>] { &self.facts.field_array_reads }
    pub(crate) fn same_facts(&self, other: &Self) -> bool {
        self.owner == other.owner && Rc::ptr_eq(&self.facts, &other.facts)
            && Rc::ptr_eq(&self.completion, &other.completion)
    }
    pub(crate) fn completion(&self) -> &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1 {
        &self.completion
    }
}
impl OrdinaryNewClaimLedgerV1 {
    fn validate_callable_scalar_anchor_v1<'a>(
        &'a self,
        loan: &BorrowedScalarResultSourceLoanV1,
        site: &OwnedExprSiteV1,
        validation: &ScalarSourceValidationV1<'a>,
    ) -> Result<(), String> {
        let source = self.borrowed_formal_source.as_ref()
            .ok_or_else(|| freeze("callable-scalar/source-missing"))?;
        let ingress = source.as_ref().map_err(Clone::clone)?;
        let catalog = self.original_parameter_contracts.as_deref()
            .ok_or_else(|| freeze("object-input/original-catalog-missing"))?;
        let original = Self::original_instance_input_call_v1(ingress, catalog, site)?
            .ok_or_else(|| freeze("callable-scalar/anchor-missing"))?;
        let target = original.source.require_instance()?;
        validation.validate(loan, target)
    }

    /// Original scalar terminal/Discard membership selects this responsibility.
    /// Other result owners and their errors are not implicit scalar candidates.
    pub(in crate::mir::normal_callable_semantic_package) fn callable_scalar_result_participant_v1(&self, owner: FunctionOwnerIdV1) -> Result<bool, String> {
        self.callable_scalar_result_scope_v1(owner, false, None)
    }

    /// One original membership walk serves passive selection and source issuance.
    /// Only the issuer requests proof verification; entry selection grants no result.
    fn callable_scalar_result_scope_v1(&self, owner: FunctionOwnerIdV1, verify: bool,
        validation: Option<&ScalarSourceValidationV1<'_>>) -> Result<bool, String> {
        let mut anchors = Vec::new();
        let mut participants = BTreeSet::new();
        let mut pending = Vec::new();
        for relation in self.terminal_relation.values()
            .filter(|row| !self.terminal_relation_index.contains_key(&row.owner()))
            .chain(self.terminal_relation_index.values().flat_map(|index| index.values()))
        {
            if let crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::Call(terminal) = relation {
                if let Some(loan) = terminal.source_scalar() {
                    anchors.push((loan, OwnedExprSiteV1::new(terminal.owner(), terminal.call_site().clone())));
                    participants.insert(terminal.owner());
                    participants.insert(loan.source.callee_owner());
                    pending.push((loan.source.callee_owner(), loan.facts.as_ref()));
                    if let Some(Ok(proof)) = self.borrowed_i64_results.get(&terminal.owner()) {
                        pending.push((terminal.owner(), proof.source_facts.as_ref()));
                    }
                }
            }
        }
        for completion in self.completion_index.values().filter_map(|row| row.as_ref().ok())
            .chain(self.root_completion.as_ref().and_then(|row| row.as_ref().ok())
                .filter(|row| !self.completion_index.contains_key(&row.owner())))
        {
            if let Some(flow) = completion.cleanup().root_flow() {
                for call in flow.local_calls() {
                    if let Some(loan) = call.borrowed_scalar() {
                        anchors.push((loan, call.site().clone()));
                        participants.insert(loan.source.callee_owner());
                        pending.push((loan.source.callee_owner(), loan.facts.as_ref()));
                    }
                }
            }
        }
        if pending.is_empty() && !participants.contains(&owner) { return Ok(false); }
        let mut seen = BTreeSet::new();
        let mut selected_facts = Vec::new();
        while let Some((current, facts)) = pending.pop() {
            if !seen.insert(current) { continue; }
            // Membership follows the SAME retained forward/expression edges.
            // FieldWriter edges never selected participants in this owner.
            // Do not validate unrelated fields just to answer an entry query.
            for callee in facts.dependencies.iter().map(|call| call.callee_owner())
                .chain(facts.expressions.iter().flat_map(|expression|
                    expression.calls().map(|call| call.callee_owner))) {
                participants.insert(callee);
                if let Some(Ok(proof)) = self.borrowed_i64_results.get(&callee) {
                    pending.push((callee, proof.source_facts.as_ref()));
                }
            }
            selected_facts.push(facts);
        }
        if !participants.contains(&owner) { return Ok(false); }
        // Selection is not execution permission. Selected queries keep every
        // original anchor, grounding, field-writer and dependency check.
        if verify {
            let validation = validation.ok_or_else(|| freeze("callable-scalar/source-missing"))?;
            let ingress = validation.source.as_ref().map_err(Clone::clone)?;
            for (loan, site) in &anchors { self.validate_callable_scalar_anchor_v1(loan, site, validation)?; }
            for facts in selected_facts {
                // demand already checked these exact immutable anchor facts.
                if !anchors.iter().any(|(loan, _)| std::ptr::eq(loan.facts.as_ref(), facts)) {
                    source_dependencies_v1(facts, ingress)?;
                }
            }
        }
        let catalog = self.original_parameter_contracts.as_deref()
            .ok_or_else(|| freeze("callable-scalar/catalog-missing"))?;
        let mut contracts = catalog.iter().filter(|row| row.owner == owner);
        let contract = contracts.next().ok_or_else(|| freeze("callable-scalar/contract-missing"))?;
        if contracts.next().is_some() { return Err(freeze("callable-scalar/contract-duplicate")); }
        Ok(contract.mode == crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod)
    }

    pub(crate) fn callable_scalar_result_source_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<Option<CallableScalarResultSourceLoanV1>, String> {
        // One immutable borrow scopes grounding to this issuance only. A later
        // call, including after a source mutation, always starts a fresh scan.
        let validation = self.borrowed_formal_source.as_ref()
            .map(|source| ScalarSourceValidationV1::new(source, &self.borrowed_i64_results));
        if !self.callable_scalar_result_scope_v1(owner, true, validation.as_ref())? { return Ok(None); }
        let validation = validation.as_ref().ok_or_else(|| freeze("callable-scalar/source-missing"))?;
        self.checked_callable_scalar_result_source_v1(owner, validation).map(Some)
    }

    /// Field reads retain their own source responsibility even when no borrowed
    /// scalar Call anchor selects the enclosing method as a physical participant.
    pub(in crate::mir::normal_callable_semantic_package) fn field_array_return_reads_v1(
        &self, owner: FunctionOwnerIdV1,
    ) -> Result<Vec<Rc<pending::field_array_read::CheckedFieldArrayReadSourceV1>>, String> {
        let selected = self.borrowed_i64_results.get(&owner).and_then(|row| row.as_ref().ok())
            .is_some_and(|proof| !proof.field_array_reads.is_empty());
        if !selected { return Ok(Vec::new()); }
        let ingress = self.borrowed_formal_source.as_ref().ok_or_else(|| freeze("callable-scalar/source-missing"))?;
        let validation = ScalarSourceValidationV1::new(ingress, &self.borrowed_i64_results);
        let source = self.checked_callable_scalar_result_source_v1(owner, &validation)?;
        Ok(source.field_array_reads().to_vec())
    }

    fn checked_callable_scalar_result_source_v1(
        &self, owner: FunctionOwnerIdV1, validation: &ScalarSourceValidationV1<'_>,
    ) -> Result<CallableScalarResultSourceLoanV1, String> {
        let proof = self.borrowed_i64_results.get(&owner)
            .ok_or_else(|| freeze("callable-scalar/result-missing"))?
            .as_ref().map_err(Clone::clone)?;
        proof.require_source_scalar_v1()?;
        let ingress = self.borrowed_formal_source.as_ref()
            .ok_or_else(|| freeze("callable-scalar/source-missing"))?
            .as_ref().map_err(Clone::clone)?;
        let catalog = self.original_parameter_contracts.as_deref()
            .ok_or_else(|| freeze("callable-scalar/catalog-missing"))?;
        let grounded = validation.grounded.get_or_init(||
            grounded_source_results_with_ingress_v1(&self.borrowed_i64_results, ingress));
        let mut pending = vec![(owner, None)];
        let mut seen = BTreeSet::new();
        while let Some((current, expected)) = pending.pop() {
            let edge_proof = self.borrowed_i64_results.get(&current).and_then(|row| row.as_ref().ok())
                .ok_or_else(|| freeze("callable-scalar/dependency-missing"))?;
            if expected.is_some_and(|expected| edge_proof.class != expected) { return Err(freeze("callable-scalar/result-contract-mismatch")); }
            if !seen.insert(current) { continue; }
            let current_proof = self.borrowed_i64_results.get(&current)
                .ok_or_else(|| freeze("callable-scalar/dependency-missing"))?
                .as_ref().map_err(Clone::clone)?;
            current_proof.require_source_scalar_v1()?;
            let completion = self.completion_source_for_owner_v1(current)
                .ok_or_else(|| freeze("callable-scalar/completion-missing"))?;
            let mut contracts = catalog.iter().filter(|row| row.owner == current);
            let contract = contracts.next()
                .ok_or_else(|| freeze("callable-scalar/contract-missing"))?;
            if contracts.next().is_some() {
                return Err(freeze("callable-scalar/contract-duplicate"));
            }
            if let Some((original_owner, batch_slot, key)) = &current_proof.parameterless_instance {
                if *original_owner != current || *batch_slot != contract.batch_slot
                    || !contract.parameters.is_empty()
                    || contract.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod
                    || key.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
                    || key.arity() != 0
                { return Err(freeze("callable-scalar/parameterless-source-drift")); }
            }
            let result = crate::mir::normal_callable_semantic_package::result_contract::validate_result(
                completion, current, contract.batch_slot,
            ).map_err(|issue| format!("{}: {issue:?}", freeze("callable-scalar/declaration")))?;
            let agrees = match current_proof.class {
                BorrowedResultClassV1::I64 => crate::mir::normal_callable_semantic_package::result_contract::declared_i64_source_agrees_v1(result, completion),
                BorrowedResultClassV1::Bool => crate::mir::normal_callable_semantic_package::result_contract::declared_bool_source_agrees_v1(result, completion),
                _ => false,
            };
            pending::mul_result::verify_mul_returns(current, current_proof, ingress)?;
            for read in &current_proof.field_array_reads {
                read.recheck_census(current, &current_proof.returns, &self.array_i64_fields)?;
            }
            if !grounded.contains(&current)
                || !source_result_completion_agrees_v1(current_proof, current, completion) || !agrees
            {
                return Err(freeze("callable-scalar/result-contract-mismatch"));
            }
            for (dependency, role) in source_dependencies_v1(current_proof, ingress)? {
                pending.push((dependency.callee_owner(), (role == SourceDependencyRoleV1::ForwardReturn).then_some(current_proof.class)));
            }
        }
        Ok(CallableScalarResultSourceLoanV1 {
            owner, facts: Rc::clone(&proof.source_facts),
            completion: Rc::clone(self.completion_source_for_owner_v1(owner).unwrap()),
        })
    }
}
