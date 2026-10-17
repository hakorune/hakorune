//! Home-effect verification on the SAME retained NoExitBody Recipe.
//! This does not recognize a Loop route, issue Recipe keys or lower effects.
use super::planner::PreissuedCallableLoopSourcePlanV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::*;
use crate::mir::resolved_semantics::home_new_prefix::LoopI64PreStateRequestV1;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

/// Original expression proved Integer by the same complete Home census.
/// No ValueId or physical representation enters this source product.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) struct VerifiedNoExitIntegerExpressionV1 {
    site: SourceExprSiteV1,
    kind: NoExitIntegerExpressionKindV1,
}
#[derive(Debug, PartialEq, Eq)]
enum NoExitIntegerExpressionKindV1 {
    Literal(i64),
    Binding(BindingRefV1),
    Add(Box<(VerifiedNoExitIntegerExpressionV1, VerifiedNoExitIntegerExpressionV1)>),
}
impl VerifiedNoExitIntegerExpressionV1 {
    pub(in crate::mir) fn site(&self) -> &SourceExprSiteV1 { &self.site }
    pub(in crate::mir) fn literal(&self) -> Option<i64> {
        match &self.kind { NoExitIntegerExpressionKindV1::Literal(value) => Some(*value), _ => None }
    }
    pub(in crate::mir) fn binding(&self) -> Option<BindingRefV1> {
        match &self.kind { NoExitIntegerExpressionKindV1::Binding(binding) => Some(*binding), _ => None }
    }
    pub(in crate::mir) fn operands(&self) -> Option<(&Self, &Self)> {
        match &self.kind { NoExitIntegerExpressionKindV1::Add(pair) => Some((&pair.0, &pair.1)), _ => None }
    }
}
#[derive(Debug)]
pub(in crate::mir) struct VerifiedNoExitArrayAppendV1 {
    requirement: Rc<NamedArrayFieldResidenceRequirementV1>,
    fault: crate::mir::resolved_control_flow::LoopCallFaultContinuationV1,
    argument: VerifiedNoExitIntegerExpressionV1,
}
impl VerifiedNoExitArrayAppendV1 {
    pub(in crate::mir) fn requirement(&self) -> &Rc<NamedArrayFieldResidenceRequirementV1> { &self.requirement }
    pub(in crate::mir) fn fault(&self) -> &crate::mir::resolved_control_flow::LoopCallFaultContinuationV1 { &self.fault }
    pub(in crate::mir) fn argument(&self) -> &VerifiedNoExitIntegerExpressionV1 { &self.argument }
}

#[derive(Debug)]
pub(in crate::mir) struct VerifiedNoExitArrayHomeAfterV1 {
    owner: FunctionOwnerIdV1,
    site: SourceStmtSiteV1,
    updated: Box<[BindingRefV1]>,
    arrays: Box<[(BindingRefV1, Rc<NamedArrayFieldResidenceRequirementV1>)]>,
    frame: LoopExecutionFrameKeyV1,
    prestate: LoopI64PreStateRequestV1,
    appends: Box<[VerifiedNoExitArrayAppendV1]>,
}
impl VerifiedNoExitArrayHomeAfterV1 {
    pub(in crate::mir) fn owner(&self) -> FunctionOwnerIdV1 { self.owner }
    pub(in crate::mir) fn site(&self) -> &SourceStmtSiteV1 { &self.site }
    pub(in crate::mir) fn updated(&self) -> &[BindingRefV1] { &self.updated }
    pub(in crate::mir) fn frame(&self) -> &LoopExecutionFrameKeyV1 { &self.frame }
    pub(in crate::mir) fn prior_homes(&self) -> &[BindingRefV1] { self.prestate.prior_homes() }
    pub(in crate::mir) fn appends(&self) -> &[VerifiedNoExitArrayAppendV1] { &self.appends }
    pub(in crate::mir) fn matches_prestate(&self, current: &LoopI64PreStateRequestV1) -> bool {
        self.prestate.same_snapshot(current)
    }
    /// The planner is not reissued after its affine take. Recheck original
    /// function/frame/call membership and SAME requirement loans instead.
    pub(in crate::mir) fn recheck_source<'a>(
        &self, input: ResolvedFunctionLoweringInputV1<'_>,
        contracts: impl IntoIterator<Item = &'a VerifiedResolverCoreMethodCallableContractV1>,
    ) -> bool {
        if input.owner() != self.owner { return false; }
        let Ok(ledger) = input.forest().callable_source_ledger(input.owner()) else { return false };
        let Ok(original) = ledger.resolved_loop_source(&self.site) else { return false };
        if !original.source().matches_identity(input.function().function_origin(), input.function().source_kind(), &self.site)
            || !original.source().frame_key().matches(&self.frame) { return false; }
        let contracts: Vec<_> = contracts.into_iter().filter(|row| row.owner() == self.owner && row.loop_site() == &self.site).collect();
        if contracts.len() != self.appends.len() { return false; }
        for append in &self.appends {
            let requirement = append.requirement();
            let fault = append.fault();
            let mut rows = contracts.iter().filter(|row| row.call_site() == requirement.call());
            let Some(contract) = rows.next() else { return false };
            let Some(call) = input.function().method_call(requirement.call()) else { return false };
            if rows.next().is_some() || !contract.frame().matches(&self.frame)
                || contract.placement() != ResolvedLoopPlacementV1::Body
                || contract.receiver() != call.receiver() || contract.receiver_site() != call.receiver_site()
                || contract.arguments() != call.arguments() || call.arguments().len() != 1
                || call.arguments()[0].site() != requirement.argument()
                || call.receiver() != ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(requirement.binding()))
                || ledger.variable_ref(call.receiver_site()) != Some(ResolvedLexicalRefV1::Local(requirement.binding()))
                || !matches!(contract.named_array_requirement(), Some(NamedArrayRequirementV1::FieldResidence(current)) if Rc::ptr_eq(current, requirement))
                || crate::mir::resolved_control_flow::issue_loop_call_fault_continuation_v1(input, &self.site, &self.frame, fault.call()).as_ref() != Ok(fault)
                || integer_expression(&ledger, &self.prestate, requirement.argument(), &mut BTreeSet::new()).as_ref() != Some(append.argument())
            { return false; }
        }
        true
    }
    pub(in crate::mir) fn arrays(&self) -> &[(BindingRefV1, Rc<NamedArrayFieldResidenceRequirementV1>)] { &self.arrays }
}

impl PreissuedCallableLoopSourcePlanV1 {
    pub(in crate::mir) fn prove_array_home_after<'a>(
        &self, input: ResolvedFunctionLoweringInputV1<'_>, prestate: &LoopI64PreStateRequestV1,
        contracts: impl IntoIterator<Item = &'a VerifiedResolverCoreMethodCallableContractV1>,
    ) -> Option<VerifiedNoExitArrayHomeAfterV1> {
        if prestate.owner() != self.owner() || prestate.site() != self.site() { return None; }
        let statements = self.flat_no_exit_statement_sites(input)?;
        if statements.is_empty() { return None; }
        let contracts: Vec<_> = contracts.into_iter().collect();
        let ledger = input.forest().callable_source_ledger(input.owner()).ok()?;
        let shape = input.body_shape()?;
        let membership = ledger.resolved_loop_source(self.site()).ok()?;
        let source = input.source();
        let root = source.stmt_at(&membership).ok()?;
        let condition = source.child_expr_from_stmt(&root, ExprChildRoleV1::LoopCondition).ok()?;
        let mut expressions = BTreeSet::new();
        let binary = ledger.binary_source(condition.site())?;
        if !matches!(binary.operator(), ResolvedBinaryOperatorV1::Less | ResolvedBinaryOperatorV1::LessEqual
            | ResolvedBinaryOperatorV1::Greater | ResolvedBinaryOperatorV1::GreaterEqual
            | ResolvedBinaryOperatorV1::Equal | ResolvedBinaryOperatorV1::NotEqual) { return None; }
        expressions.insert(condition.site().clone());
        integer_expression(&ledger, prestate, binary.lhs(), &mut expressions)?;
        integer_expression(&ledger, prestate, binary.rhs(), &mut expressions)?;
        let mut effects = BTreeSet::new();
        let mut updated = BTreeSet::new();
        let mut arrays = BTreeMap::new();
        let mut appends = Vec::new();
        for statement in &statements {
            let call_site = SourceExprSiteV1::from_node(statement.node().clone());
            let mut calls = ledger.method_calls().filter(|(site, _)| *site == &call_site);
            if let Some((_, call)) = calls.next() {
                if calls.next().is_some() { return None; }
                let mut matching = contracts.iter().filter(|row| row.owner() == input.owner() && row.call_site() == &call_site);
                let contract = *matching.next()?;
                if matching.next().is_some() || contract.loop_site() != self.site()
                    || !contract.frame().matches(self.frame()) || contract.placement() != ResolvedLoopPlacementV1::Body
                    || contract.receiver() != call.receiver() || contract.receiver_site() != call.receiver_site()
                    || contract.arguments().len() != 1 || contract.arguments() != call.arguments() { return None; }
                let Some(NamedArrayRequirementV1::FieldResidence(requirement)) = contract.named_array_requirement() else { return None; };
                let ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding)) = call.receiver() else { return None; };
                let alias = prestate.borrowed_array(binding)?;
                if requirement.owner() != input.owner() || requirement.binding() != binding || requirement.call() != &call_site
                    || requirement.argument() != call.arguments()[0].site()
                    || contract.arguments()[0].site() != requirement.argument()
                    || ledger.variable_ref(call.receiver_site()) != Some(ResolvedLexicalRefV1::Local(binding))
                    || !same_residence(&alias.requirement, requirement)
                    || !contracts.iter().any(|row| matches!(row.named_array_requirement(), Some(NamedArrayRequirementV1::FieldResidence(original))
                        if Rc::ptr_eq(original, &alias.requirement))) { return None; }
                let fault = crate::mir::resolved_control_flow::issue_loop_call_fault_continuation_v1(
                    input, self.site(), self.frame(), &OwnedExprSiteV1::new(input.owner(), call_site.clone()),
                ).ok()?;
                let argument = integer_expression(&ledger, prestate, requirement.argument(), &mut expressions)?;
                appends.push(VerifiedNoExitArrayAppendV1 { requirement: Rc::clone(requirement), fault, argument });
                arrays.entry(binding).or_insert_with(|| Rc::clone(&alias.requirement));
                expressions.insert(call_site.clone()); expressions.insert(call.receiver_site().clone());
                effects.insert(BodyEffectShapeV1 { site: call_site, kind: BodyEffectKindV1::Call });
            } else {
                let mut rows = shape.assignment_sources().iter().filter(|row| row.statement_site() == statement);
                let assignment = rows.next()?;
                if rows.next().is_some() || assignment.form() != ResolvedAssignmentFormV1::Plain { return None; }
                let ResolvedAssignmentTargetV1::BindingRebind(binding) = ledger.assignment_target(assignment.target_site())? else { return None; };
                if !prestate.proves_integer(*binding) { return None; }
                let value = ledger.binary_source(assignment.value_site())?;
                if value.operator() != ResolvedBinaryOperatorV1::Add { return None; }
                integer_expression(&ledger, prestate, assignment.value_site(), &mut expressions)?;
                expressions.insert(assignment.target_site().clone()); updated.insert(*binding);
                effects.insert(BodyEffectShapeV1 { site: assignment.target_site().clone(), kind: BodyEffectKindV1::Write });
            }
        }
        let inside = |node: &SourceNodeSiteV1| node.segments().starts_with(self.site().node().segments());
        let original_expressions: BTreeSet<_> = ledger.source_site_inventory().expression_sites()
            .filter(|site| inside(site.node())).cloned().collect();
        let original_effects: BTreeSet<_> = shape.effects().iter().filter(|row| inside(row.site.node())).cloned().collect();
        let mut expected_statements: BTreeSet<_> = statements.into_iter().collect(); expected_statements.insert(self.site().clone());
        let original_statements: BTreeSet<_> = shape.statements().iter().filter(|row| inside(row.site().node())).map(|row| row.site().clone()).collect();
        if expressions != original_expressions || effects != original_effects || expected_statements != original_statements
            || ledger.resolved_exits().any(|(site, _)| inside(site.node())) || arrays.is_empty() { return None; }
        Some(VerifiedNoExitArrayHomeAfterV1 { owner: self.owner(), site: self.site().clone(),
            updated: updated.into_iter().collect(), arrays: arrays.into_iter().collect(),
            frame: self.frame().clone(), prestate: prestate.clone(), appends: appends.into_boxed_slice() })
    }
}

fn same_residence(left: &NamedArrayFieldResidenceRequirementV1, right: &NamedArrayFieldResidenceRequirementV1) -> bool {
    left.owner() == right.owner() && left.binding() == right.binding() && left.field() == right.field()
        && left.field_site() == right.field_site() && left.field_name() == right.field_name()
        && left.provider_caller() == right.provider_caller() && left.provider_site() == right.provider_site()
}

/// Class proof is the real pre-Loop Home state; syntax alone grants no Integer.
fn integer_expression(ledger: &CallableSemanticSourceLedgerView<'_>, prestate: &LoopI64PreStateRequestV1,
    site: &SourceExprSiteV1, expressions: &mut BTreeSet<SourceExprSiteV1>,
) -> Option<VerifiedNoExitIntegerExpressionV1> {
    if !expressions.insert(site.clone()) { return None; }
    let kind = if let Some(ResolvedLiteralSourceV1::Integer(value)) = ledger.literal_source(site) {
        NoExitIntegerExpressionKindV1::Literal(*value)
    } else if let Some(ResolvedLexicalRefV1::Local(binding)) = ledger.variable_ref(site) {
        if !prestate.proves_integer(binding) { return None; }
        NoExitIntegerExpressionKindV1::Binding(binding)
    } else {
        let binary = ledger.binary_source(site)?;
        if binary.operator() != ResolvedBinaryOperatorV1::Add { return None; }
        let lhs = integer_expression(ledger, prestate, binary.lhs(), expressions)?;
        let rhs = integer_expression(ledger, prestate, binary.rhs(), expressions)?;
        NoExitIntegerExpressionKindV1::Add(Box::new((lhs, rhs)))
    };
    Some(VerifiedNoExitIntegerExpressionV1 { site: site.clone(), kind })
}
