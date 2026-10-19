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
    receiver: BindingRefV1,
    body: OriginalReturnBodySourceV1,
}
#[derive(Debug)]
enum OriginalReturnBodySourceV1 {
    Fields { plans: Box<[Rc<ExpressionSourcePlanV1>]>, field_values: Box<[SourceExprSiteV1]> },
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
    BTreeMap<FunctionOwnerIdV1, Result<Option<OriginalReturnSourceLoanV1>, String>>;

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
    original_return_plans_v1(input, receiver).map(|(_, values)| values)
}

/// Capture each original Return tree once, then select passive field requests
/// from those SAME plans. Numeric/Completion success cannot select this family.
fn original_return_plans_v1(
    input: ResolvedFunctionLoweringInputV1<'_>, receiver: BindingRefV1,
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
            root == receiver && exact_me_receiver_v1(input, object, receiver)
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
    pub(super) fn plan(&self, value: &SourceExprSiteV1) -> Option<&Rc<ExpressionSourcePlanV1>> {
        self.plans().iter().find(|plan| {
            plan.nodes()
                .first()
                .is_some_and(|node| node.site() == value)
        })
    }
    pub(super) fn field_plans(&self) -> impl Iterator<Item = &Rc<ExpressionSourcePlanV1>> {
        let values = match &self.0.body {
            OriginalReturnBodySourceV1::Fields { field_values, .. } => field_values.as_ref(),
            OriginalReturnBodySourceV1::UnitCandidate { .. } => &[],
        };
        self.plans().iter().filter(move |plan| plan.nodes().first().is_some_and(|node| values.contains(node.site())))
    }
    fn plans(&self) -> &[Rc<ExpressionSourcePlanV1>] {
        match &self.0.body {
            OriginalReturnBodySourceV1::Fields { plans, .. } => plans,
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
    pub(super) fn receiver(&self) -> BindingRefV1 {
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
    contracts.iter().filter(|contract|
        contract.mode == crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod).map(|contract| {
        let source = batch.with_lowering_input(contract.batch_slot, |input| {
            let key = selected.key_for_batch_slot(contract.batch_slot).and_then(|key| match key {
                crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key) => Some(key), _ => None,
            }).ok_or_else(|| freeze("borrowed-result/original-return-key"))?;
            if input.owner() != contract.owner || input.function().owner() != contract.owner
                || key.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod || key.arity() as usize != contract.parameters.len()
                || contracts.iter().filter(|row| row.owner == contract.owner).count() != 1
                || contract.parameters.iter().enumerate().any(|(ordinal, row)|
                    row.ordinal as usize != ordinal || row.binding.owner() != input.owner())
            { return Err(freeze("borrowed-result/original-return-owner")); }
            let mut receivers = input.function().bindings().filter(|(_, row)| row.kind() == BindingKindV1::Receiver);
            let receiver = receivers.next().map(|(binding, _)| binding).ok_or_else(|| freeze("borrowed-result/original-return-receiver"))?;
            if receivers.next().is_some() { return Err(freeze("borrowed-result/original-return-receiver")); }
            // Arity expansion belongs only to Unit; the original scalar field
            // result owner retains its existing parameter-free boundary.
            if !contract.parameters.is_empty() && input.body_shape().is_none_or(|shape|
                shape.statements().iter().any(|row| matches!(row,
                    crate::mir::resolved_semantics::BodyStatementShapeV1::Return { value: Some(_), .. }))) {
                return Ok(None);
            }
            let (plans, field_values) = original_return_plans_v1(input, receiver)?;
            let body = if !field_values.is_empty() {
                OriginalReturnBodySourceV1::Fields { plans: plans.into_boxed_slice(), field_values: field_values.into_boxed_slice() }
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
        (contract.owner, source)
    }).collect()
}
