//! Passive original Return requirements, before result/Completion eligibility.
//! This immutable source loan has no numeric class or executable disposition.
use super::*;
use crate::mir::resolved_semantics::{BindingKindV1, BodyMeReceiverV1};
use crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1;

#[derive(Debug)]
struct OriginalReturnSourceV1 {
    owner: FunctionOwnerIdV1,
    batch_slot: u32,
    key: CanonicalSameModuleCallableKeyV1,
    receiver: Option<BindingRefV1>,
    body: OriginalReturnBodySourceV1,
}
#[derive(Debug)]
enum OriginalReturnBodySourceV1 {
    Value { plans: Box<[Rc<ExpressionSourcePlanV1>]>, field_values: Box<[SourceExprSiteV1]> },
    UnitCandidate {
        body: crate::mir::compiler::located::SourceBodySiteV1,
        body_end: u32,
        bare_returns: Box<[crate::mir::resolved_semantics::SourceStmtSiteV1]>,
    },
}
#[derive(Debug, Clone)]
pub(crate) struct OriginalReturnSourceLoanV1(Rc<OriginalReturnSourceV1>);
impl PartialEq for OriginalReturnSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for OriginalReturnSourceLoanV1 {}
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) type OriginalReturnsV1 =
    BTreeMap<FunctionOwnerIdV1, OriginalReturnCaptureV1>;

/// The old capture boundary is immutable declaration metadata, not a result
/// class. General capture errors may only be demanded by general Return users.
#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct OriginalReturnCaptureV1 {
    source: Result<Option<OriginalReturnSourceLoanV1>, String>,
    legacy: bool,
}
impl OriginalReturnCaptureV1 {
    pub(super) fn legacy_source(&self) -> Result<Option<&OriginalReturnSourceLoanV1>, String> {
        if !self.legacy { return Ok(None); }
        self.source.as_ref().map(|source| source.as_ref().filter(|loan|
            loan.is_unit_candidate_v1() || loan.legacy_field_receiver_v1().is_some())).map_err(Clone::clone)
    }
}

pub(super) fn exact_me_receiver_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: &SourceExprSiteV1,
    receiver: BindingRefV1,
) -> bool {
    input.function().variable_ref(site) == Some(ResolvedLexicalRefV1::Local(receiver))
        && matches!(input.body_shape().and_then(|shape| shape.expression_shape(site)),
            Some(BodyExpressionShapeV1::Me { receiver: BodyMeReceiverV1::Lexical(original), .. }) if *original == receiver)
}
/// Passive re-demand detects requests even after a retained proof is lost.
/// Fresh structural plans here are never substituted for the retained loan.
pub(super) fn field_return_values_v1(
    input: ResolvedFunctionLoweringInputV1<'_>, receiver: BindingRefV1,
) -> Result<Vec<SourceExprSiteV1>, String> {
    if input.function().declaration_sites().any(|site| matches!(site, SourceBindingSiteV1::Parameter { .. })) {
        return Ok(Vec::new());
    }
    if receiver.owner() != input.owner() || input.function().binding(receiver)
        .is_none_or(|binding| binding.kind() != BindingKindV1::Receiver)
    { return Err(freeze("borrowed-result/original-return-receiver")); }
    original_return_plans_v1(input, Some(receiver)).map(|(_, values)| values)
}

/// Capture each original Return tree once, then select passive field requests
/// from those SAME plans. Numeric/Completion success cannot select this family.
fn original_return_plans_v1(
    input: ResolvedFunctionLoweringInputV1<'_>, receiver: Option<BindingRefV1>,
) -> Result<(Vec<Rc<ExpressionSourcePlanV1>>, Vec<SourceExprSiteV1>), String> {
    let shape = input.body_shape().ok_or_else(|| freeze("borrowed-result/original-return-shape"))?;
    let ledger = input.forest().callable_source_ledger(input.owner())
        .map_err(|_| freeze("borrowed-result/original-return-ledger"))?;
    let mut plans = Vec::new();
    let mut field_values = Vec::new();
    for statement in shape.statements() {
        let crate::mir::resolved_semantics::BodyStatementShapeV1::Return { value: Some(value), .. } = statement
            else { continue; };
        let plan = crate::mir::resolved_value_profile::expression_plan::statement_expression_source_plan_v1(
            &ledger, shape, statement.site(),
        ).map_err(|_| freeze("borrowed-result/original-return-plan"))?
            .ok_or_else(|| freeze("borrowed-result/original-return-missing"))?;
        if plan.owner() != input.owner() { return Err(freeze("borrowed-result/original-return-owner")); }
        if plan.nodes().iter().any(|node| {
            let Some((root, fields)) = plan.field_path_v1(node.site()) else { return false; };
            let crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Shape(
                BodyExpressionShapeV1::FieldAccess { object, .. }) = plan.nodes()[fields[0]].source()
                else { return false; };
            receiver.is_some_and(|receiver| root == receiver && exact_me_receiver_v1(input, object, receiver))
        }) { field_values.push(value.clone()); }
        plans.push(plan);
    }
    Ok((plans, field_values))
}

impl OriginalReturnSourceLoanV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn matches(
        &self,
        reference: &super::super::source::CallTargetReferenceV1,
    ) -> bool {
        self.0.owner == reference.callee_owner
            && self.0.batch_slot == reference.target_batch_slot
            && self.0.key == reference.target
            && reference.argument_sites.len() == self.0.key.arity() as usize
    }
    pub(crate) fn plan_for_exit_v1(&self, statement: &crate::mir::resolved_semantics::SourceStmtSiteV1,
        value: &SourceExprSiteV1) -> Option<&Rc<ExpressionSourcePlanV1>> {
        self.plans().iter().find(|plan| matches!(plan.root(),
            crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::Statement {
                statement: original, role: crate::mir::resolved_value_profile::expression_root::StatementExpressionRoleV1::ReturnValue,
                value: root,
            } if original == statement && root == value)
            && plan.owner() == self.0.owner && plan.nodes().first().is_some_and(|node| node.site() == value))
    }
    pub(super) fn plan(&self, value: &SourceExprSiteV1) -> Option<&Rc<ExpressionSourcePlanV1>> {
        self.plans().iter().find(|plan| {
            plan.nodes()
                .first()
                .is_some_and(|node| node.site() == value)
        })
    }
    pub(super) fn field_plans(&self) -> impl Iterator<Item = &Rc<ExpressionSourcePlanV1>> {
        let values = match &self.0.body {
            OriginalReturnBodySourceV1::Value { field_values, .. } => field_values.as_ref(),
            OriginalReturnBodySourceV1::UnitCandidate { .. } => &[],
        };
        self.plans().iter().filter(move |plan| self.legacy_field_receiver_v1().is_some()
            && plan.nodes().first().is_some_and(|node| values.contains(node.site())))
    }
    fn plans(&self) -> &[Rc<ExpressionSourcePlanV1>] {
        match &self.0.body {
            OriginalReturnBodySourceV1::Value { plans, .. } => plans,
            OriginalReturnBodySourceV1::UnitCandidate { .. } => &[],
        }
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn is_unit_candidate_v1(&self) -> bool {
        matches!(self.0.body, OriginalReturnBodySourceV1::UnitCandidate { .. })
    }
    /// Corroboration against the existing canonical Completion, never issuance.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn matches_unit_completion_v1(
        &self, completion: &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
    ) -> bool {
        use crate::mir::resolved_control_flow::{DeclaredFunctionResultContractV1, SealedFunctionExitDispositionV1 as Exit};
        let OriginalReturnBodySourceV1::UnitCandidate { body, body_end, bare_returns } = &self.0.body else { return false; };
        if completion.owner() != self.0.owner || completion.returns_value()
            || completion.explicit_sites() != bare_returns.as_ref()
            || !matches!(completion.function_exit_contract().declared_result(), DeclaredFunctionResultContractV1::Unannotated)
            || completion.implicit_body_end().is_some_and(|(original, end)| original != body || end != *body_end)
        { return false; }
        matches!(completion.function_exit_contract().disposition(),
            Exit::ExplicitUnit { .. } | Exit::ExplicitUnitSet { .. }
            | Exit::ImplicitUnit { .. } | Exit::ExplicitUnitSetWithImplicitEnd { .. })
    }
    pub(super) fn legacy_field_receiver_v1(&self) -> Option<BindingRefV1> {
        if self.0.key.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod
            || self.0.key.arity() != 0
            || !matches!(&self.0.body, OriginalReturnBodySourceV1::Value { field_values, .. } if !field_values.is_empty())
        { return None; }
        self.0.receiver
    }
}

/// Reuse the existing result owner map across raw capture and qualification.
/// No Completion query participates in affirmative source-family selection.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn prepare_original_returns_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
) -> OriginalReturnsV1 {
    contracts.iter().map(|contract| {
        let instance = contract.mode == crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod;
        let mut legacy = instance;
        let source = batch.with_lowering_input(contract.batch_slot, |input| {
            let key = selected.key_for_batch_slot(contract.batch_slot).and_then(|key| match key {
                crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key) => Some(key), _ => None,
            }).ok_or_else(|| freeze("borrowed-result/original-return-key"))?;
            if input.owner() != contract.owner || input.function().owner() != contract.owner
                || key.namespace() != if instance { hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod }
                    else { hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod }
                || key.arity() as usize != contract.parameters.len()
                || contracts.iter().filter(|row| row.owner == contract.owner).count() != 1
                || contract.parameters.iter().enumerate().any(|(ordinal, row)|
                    row.ordinal as usize != ordinal || row.binding.owner() != input.owner())
            { return Err(freeze("borrowed-result/original-return-owner")); }
            let mut receivers = input.function().bindings().filter(|(_, row)| row.kind() == BindingKindV1::Receiver);
            let receiver = receivers.next().map(|(binding, _)| binding);
            if receivers.next().is_some() || receiver.is_some() != instance {
                return Err(freeze("borrowed-result/original-return-receiver"));
            }
            // Preserve the old scalar/Unit capture boundary BEFORE general
            // source planning can fail. Only the general Return demand sees
            // failures introduced by the newly covered source family.
            if !contract.parameters.is_empty() && input.body_shape().is_none_or(|shape|
                shape.statements().iter().any(|row| matches!(row,
                    crate::mir::resolved_semantics::BodyStatementShapeV1::Return { value: Some(_), .. }))) {
                legacy = false;
            }
            let (plans, field_values) = original_return_plans_v1(input, receiver)?;
            let body = if !plans.is_empty() {
                OriginalReturnBodySourceV1::Value { plans: plans.into_boxed_slice(), field_values: field_values.into_boxed_slice() }
            } else {
                let shape = input.body_shape().ok_or_else(|| freeze("unit-source/body-shape"))?;
                if shape.statements().iter().any(|statement| matches!(statement,
                    crate::mir::resolved_semantics::BodyStatementShapeV1::Return { value: Some(_), .. })) { return Ok(None); }
                let original = input.source().root_body().map_err(|_| freeze("unit-source/body"))?;
                OriginalReturnBodySourceV1::UnitCandidate {
                    body: original.site().clone(),
                    body_end: u32::try_from(original.statements().len()).map_err(|_| freeze("unit-source/body-length"))?,
                    bare_returns: shape.statements().iter().filter_map(|statement| match statement {
                        crate::mir::resolved_semantics::BodyStatementShapeV1::Return { site, value: None } => Some(site.clone()),
                        _ => None,
                    }).collect(),
                }
            };
            Ok(Some(OriginalReturnSourceLoanV1(Rc::new(OriginalReturnSourceV1 {
                owner: input.owner(), batch_slot: contract.batch_slot, key: key.clone(), receiver, body,
            }))))
        }).map_err(|_| freeze("borrowed-result/original-return-input")).and_then(|result| result);
        (contract.owner, OriginalReturnCaptureV1 { source, legacy })
    }).collect()
}

/// Raw Return capture and optional numeric qualification share one owner row.
/// Numeric refusal/profile retirement never destroys the immutable source loan.
#[derive(Debug)]
struct OriginalResultRowV1 {
    original: OriginalReturnCaptureV1,
    scalar: Option<Result<BorrowedI64ResultSourceV1, String>>,
}
type ScalarResultV1 = Result<BorrowedI64ResultSourceV1, String>;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct BorrowedResultCohortV1 {
    rows: BTreeMap<FunctionOwnerIdV1, OriginalResultRowV1>,
    pub(super) field_candidates: Result<Rc<composition::FieldCandidatesV1>, String>,
    pub(super) field_component: Result<Option<Rc<composition::ClosedFieldComponentV1>>, String>,
}
impl Default for BorrowedResultCohortV1 {
    fn default() -> Self { Self::unavailable(freeze("borrowed-result/original-cohort-unprepared")) }
}
impl BorrowedResultCohortV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn unavailable(issue: String) -> Self {
        Self::unavailable_with_original_v1(issue, BTreeMap::new())
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn unavailable_with_original_v1(issue: String, originals: OriginalReturnsV1) -> Self {
        Self { rows: originals.into_iter().map(|(owner, original)|
            (owner, OriginalResultRowV1 { original, scalar: None })).collect(),
            field_candidates: Err(issue.clone()), field_component: Err(issue) }
    }
    pub(super) fn from_original_returns_v1(originals: OriginalReturnsV1) -> Self {
        Self::unavailable_with_original_v1(freeze("borrowed-result/original-cohort-unprepared"), originals)
    }
    #[cfg(test)]
    pub(super) fn prepared(rows: BTreeMap<FunctionOwnerIdV1, ScalarResultV1>,
        field_candidates: Rc<composition::FieldCandidatesV1>,
    ) -> Self {
        let mut cohort = Self::from_original_returns_v1(BTreeMap::new());
        cohort.field_candidates = Ok(field_candidates);
        cohort.field_component = Ok(None);
        for (owner, scalar) in rows { cohort.insert(owner, scalar); }
        cohort
    }
    pub(super) fn require_field_component_v1(&self, component: &Rc<composition::ClosedFieldComponentV1>) -> Result<(), String> {
        let original = self.field_candidates.as_ref().map_err(Clone::clone)?;
        let retained = self.field_component.as_ref().map_err(Clone::clone)?.as_ref()
            .ok_or_else(|| freeze("borrowed-result/field-component-unsealed"))?;
        if !Rc::ptr_eq(retained, component) || !component.belongs_to(original) {
            return Err(freeze("borrowed-result/field-component-identity"));
        }
        Ok(())
    }
    /// Source-only loan, independent of absence/error/sealing of a scalar row.
    pub(in crate::mir::normal_callable_semantic_package) fn original_return_source_v1(&self, owner: &FunctionOwnerIdV1)
        -> Result<Option<&OriginalReturnSourceLoanV1>, String> {
        self.rows.get(owner).map(|row| row.original.source.as_ref().map(Option::as_ref).map_err(Clone::clone))
            .unwrap_or(Ok(None))
    }
    /// Lend only the SAME retained Return tree at a canonical Home walk point.
    /// This source loan carries no result class, Completion or physical grant.
    pub(in crate::mir::normal_callable_semantic_package) fn lend_return_expression_source_v1(
        &self, batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
        input: ResolvedFunctionLoweringInputV1<'_>, statement: &crate::mir::resolved_semantics::SourceStmtSiteV1,
        value: &OwnedExprSiteV1,
    ) -> Result<Option<OriginalReturnSourceLoanV1>, String> {
        let mut rows = contracts.iter().filter(|row| row.owner == input.owner());
        let row = rows.next().ok_or_else(|| freeze("return-expression/contract-missing"))?;
        if rows.next().is_some() || value.owner() != input.owner()
            || !batch.with_lowering_input(row.batch_slot, |original|
                std::ptr::eq(original.function(), input.function())).unwrap_or(false) {
            return Err(freeze("return-expression/source-identity"));
        }
        let Some(source) = self.original_return_source_v1(&input.owner())? else { return Ok(None) };
        if source.0.owner != row.owner || source.0.batch_slot != row.batch_slot
            || !matches!(selected.key_for_batch_slot(row.batch_slot),
                Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) if key == &source.0.key)
            || source.plan_for_exit_v1(statement, value.site()).is_none() {
            return Err(freeze("return-expression/original-plan-identity"));
        }
        Ok(Some(source.clone()))
    }
    pub(super) fn legacy_original_return_source_v1(&self, owner: &FunctionOwnerIdV1)
        -> Result<Option<&OriginalReturnSourceLoanV1>, String> {
        self.rows.get(owner).map(|row| row.original.legacy_source()).unwrap_or(Ok(None))
    }
    // Existing scalar-map projection. No caller sees source-only rows as a
    // successful or unresolved numeric result; ordering is still canonical.
    pub(in crate::mir::normal_callable_semantic_package) fn get(&self, owner: &FunctionOwnerIdV1) -> Option<&ScalarResultV1> {
        self.rows.get(owner)?.scalar.as_ref()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn get_mut(&mut self, owner: &FunctionOwnerIdV1) -> Option<&mut ScalarResultV1> {
        self.rows.get_mut(owner)?.scalar.as_mut()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn iter(&self) -> impl Iterator<Item = (&FunctionOwnerIdV1, &ScalarResultV1)> {
        self.rows.iter().filter_map(|(owner, row)| row.scalar.as_ref().map(|scalar| (owner, scalar)))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn iter_mut(&mut self) -> impl Iterator<Item = (&FunctionOwnerIdV1, &mut ScalarResultV1)> {
        self.rows.iter_mut().filter_map(|(owner, row)| row.scalar.as_mut().map(|scalar| (owner, scalar)))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn values_mut(&mut self) -> impl Iterator<Item = &mut ScalarResultV1> {
        self.iter_mut().map(|(_, scalar)| scalar)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn insert(&mut self, owner: FunctionOwnerIdV1, scalar: ScalarResultV1) -> Option<ScalarResultV1> {
        self.rows.entry(owner).or_insert_with(|| OriginalResultRowV1 { original: OriginalReturnCaptureV1 { source: Ok(None), legacy: false }, scalar: None }).scalar.replace(scalar)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn retain(&mut self, mut keep: impl FnMut(&FunctionOwnerIdV1, &mut ScalarResultV1) -> bool) {
        for (owner, row) in &mut self.rows {
            if let Some(mut scalar) = row.scalar.take() {
                if keep(owner, &mut scalar) { row.scalar = Some(scalar); }
            }
        }
    }
    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn values(&self) -> impl Iterator<Item = &ScalarResultV1> { self.iter().map(|(_, scalar)| scalar) }
    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn keys(&self) -> impl Iterator<Item = &FunctionOwnerIdV1> { self.iter().map(|(owner, _)| owner) }
    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn len(&self) -> usize { self.iter().count() }
    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn is_empty(&self) -> bool { self.iter().next().is_none() }
    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn remove(&mut self, owner: &FunctionOwnerIdV1) -> Option<ScalarResultV1> { self.rows.get_mut(owner)?.scalar.take() }
    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn clear(&mut self) { for row in self.rows.values_mut() { row.scalar = None; } }
}
#[cfg(test)]
impl std::ops::Index<&FunctionOwnerIdV1> for BorrowedResultCohortV1 {
    type Output = ScalarResultV1;
    fn index(&self, owner: &FunctionOwnerIdV1) -> &Self::Output { self.get(owner).expect("existing scalar result row") }
}
