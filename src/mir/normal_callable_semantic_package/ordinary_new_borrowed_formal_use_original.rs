//! Canonical original read/use obligations, independent of transport classifiers.
//! Source identity only: no current read, execution, object or ABI authority.
use super::*;
use crate::mir::resolved_semantics::source_value_flow::VerifiedSourceValueFlowV1;
use crate::mir::resolved_semantics::ResolvedInitializerRelationV1;
use crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1;
use std::{collections::BTreeSet, sync::Arc};

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
struct OriginalFormalUseSourceV1
{
    owner: FunctionOwnerIdV1,
    batch_slot: u32,
    mode: crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1,
    parameters: Box<[(u32, BindingRefV1, CallableParameterContractKindV1)]>,
    roots: BTreeMap<BindingRefV1, BindingRefV1>,
    reads: Box<[(SourceExprSiteV1, ResolvedLexicalRefV1)]>,
    return_plans: Option<Box<[std::rc::Rc<ExpressionSourcePlanV1>]>>,
    flow: Option<Arc<VerifiedSourceValueFlowV1>>,
    initializers: Box<[ResolvedInitializerRelationV1]>,
    assignments: Box<[(SourceExprSiteV1, ResolvedAssignmentTargetV1)]>,
    captures: Box<[(OwnedExprSiteV1, BindingRefV1, bool)]>,
    scalar_expression_use_requested: bool,
}
impl OriginalFormalUseSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn issue(
        input: ResolvedFunctionLoweringInputV1<'_>,
        contract: &OwnedCallableParameterContractDeclarationV1,
    ) -> Result<Rc<Self>, BorrowedFormalUseDraftErrorV1> {
        let function = input.function();
        if input.owner() != contract.owner
            || function.owner() != contract.owner
            || function
                .declaration_sites()
                .filter(|site| matches!(site, SourceBindingSiteV1::Parameter { .. }))
                .count()
                != contract.parameters.len()
        {
            return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
        }
        let mut origins = BTreeMap::new();
        for (index, formal) in contract.parameters.iter().enumerate() {
            if formal.ordinal as usize != index
                || formal.binding.owner() != contract.owner
                || function.declaration_binding(&SourceBindingSiteV1::Parameter {
                    index: formal.ordinal,
                }) != Some(formal.binding)
                || !matches!(function.binding(formal.binding).map(|row| row.kind()),
                Some(BindingKindV1::Parameter { index }) if index == formal.ordinal)
            {
                return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
            }
            if formal.kind.is_ordinary_borrowed_handle() {
                if origins.insert(formal.binding, formal.binding).is_some() {
                    return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
                }
            }
        }

        let reads: Box<[_]> = function
            .variable_refs()
            .map(|(site, reference)| (site.clone(), *reference))
            .collect();
        let assignments: Box<[_]> = function
            .assignment_targets()
            .map(|(site, target)| (site.clone(), target.clone()))
            .collect();
        let return_plans = input.body_shape().and_then(|shape| {
            let ledger = input.forest().callable_source_ledger(input.owner()).ok()?;
            let mut plans = Vec::new();
            for statement in shape.statements() {
                if !matches!(statement,
                    crate::mir::resolved_semantics::BodyStatementShapeV1::Return { value: Some(_), .. }) {
                    continue;
                }
                plans.push(crate::mir::resolved_value_profile::expression_plan::statement_expression_source_plan_v1(
                    &ledger, shape, statement.site()).ok()??);
            }
            Some(plans.into_boxed_slice())
        });
        let mut captures = Vec::new();
        for (owner, descendant) in input.forest().semantic_owners() {
            let mut retain_assignment =
                |site: &SourceExprSiteV1, target: &ResolvedAssignmentTargetV1| {
                    if let ResolvedAssignmentTargetV1::UpvarRebind(upvar) = target {
                        if upvar.source().owner() == input.owner() {
                            captures.push((
                                OwnedExprSiteV1::new(owner, site.clone()),
                                upvar.source(),
                                true,
                            ));
                        }
                    }
                };
            if owner == input.owner() {
                for (site, target) in &assignments {
                    retain_assignment(site, target);
                }
            } else {
                for (site, target) in descendant.assignment_targets() {
                    retain_assignment(site, target);
                }
            }
            let mut retain = |site: &SourceExprSiteV1, reference: &ResolvedLexicalRefV1| {
                if let ResolvedLexicalRefV1::Upvar(upvar) = reference {
                    if upvar.source().owner() == input.owner() {
                        captures.push((
                            OwnedExprSiteV1::new(owner, site.clone()),
                            upvar.source(),
                            false,
                        ));
                    }
                }
            };
            if owner == input.owner() {
                for (site, reference) in &reads {
                    retain(site, reference);
                }
            } else {
                for (site, reference) in descendant.variable_refs() {
                    retain(site, reference);
                }
            }
        }
        let mut original = Self {
            owner: input.owner(),
            batch_slot: contract.batch_slot,
            mode: contract.mode,
            parameters: contract
                .parameters
                .iter()
                .map(|row| (row.ordinal, row.binding, row.kind.clone()))
                .collect(),
            roots: origins,
            reads,
            return_plans,
            flow: function.expression_source().value_flow().map(Arc::clone),
            initializers: function
                .expression_source()
                .initializers()
                .cloned()
                .collect(),
            assignments,
            captures: captures.into_boxed_slice(),
            scalar_expression_use_requested: false,
        };
        original.scalar_expression_use_requested = original.observe_scalar_expression_use_v1(input)?;
        Ok(Rc::new(original))
    }
    /// Original evaluation purposes select work before domain/use completion.
    /// A scalar SUM alone cannot activate a previously unsupported Return.
    fn observe_scalar_expression_use_v1(&self, input: ResolvedFunctionLoweringInputV1<'_>)
        -> Result<bool, BorrowedFormalUseDraftErrorV1> {
        use crate::mir::resolved_value_profile::expression_plan::{
            assignment_expression_source_plan_v1, condition_expression_source_plan_v1,
            local_expression_source_plan_v1, loop_condition_expression_source_plan_v1, ExpressionNodeSourceV1,
        };
        if self.roots.is_empty() { return Ok(false); }
        let reject = |_| BorrowedFormalUseDraftErrorV1::SourceIdentity;
        let Some(shape) = input.body_shape() else { return Ok(false) };
        let ledger = input.forest().callable_source_ledger(input.owner()).map_err(reject)?;
        let assignments = shape.assignment_sources().iter().filter(|assignment|
            matches!(input.function().assignment_target(assignment.target_site()),
                Some(ResolvedAssignmentTargetV1::FieldWrite { .. })))
            .map(|assignment| assignment_expression_source_plan_v1(&ledger, shape, assignment));
        let conditions = input.function().if_region_sites()
            .map(|site| condition_expression_source_plan_v1(input, site))
            .chain(input.function().loop_sites()
                .map(|site| loop_condition_expression_source_plan_v1(input, site)));
        let initializers = input.function().expression_source().initializers().filter(|row| row.initializer_site().is_some())
            .map(|row| local_expression_source_plan_v1(&ledger, Some(shape), row));
        for plan in assignments.chain(conditions).chain(initializers) {
            let plan = plan.map_err(|_| BorrowedFormalUseDraftErrorV1::SourceIdentity)?;
            for (_, node) in plan.nodes_outside_calls() {
                let ExpressionNodeSourceV1::Reference(ResolvedLexicalRefV1::Local(binding)) = node.source() else { continue };
                if self.reads.iter().filter(|(site, reference)| site == node.site()
                    && *reference == ResolvedLexicalRefV1::Local(*binding)).count() != 1 {
                    return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
                }
                if self.roots.get(binding) == Some(binding) { return Ok(true); }
                if self.flow.as_ref().and_then(|flow| flow.read(node.site(), *binding).and_then(|point|
                    flow.alias_entry_origin(point, |site| input.function().variable_ref(site))))
                    .is_some_and(|formal| self.roots.get(&formal) == Some(&formal)) { return Ok(true); }
            }
        }
        Ok(false)
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn scalar_expression_use_requested_v1(&self) -> bool {
        self.scalar_expression_use_requested
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn matches_contract(
        &self,
        contract: &OwnedCallableParameterContractDeclarationV1,
    ) -> bool {
        self.owner == contract.owner
            && self.batch_slot == contract.batch_slot
            && self.mode == contract.mode
            && self.parameters.len() == contract.parameters.len()
            && self.parameters.iter().zip(&contract.parameters).all(
                |((ordinal, binding, kind), row)| {
                    *ordinal == row.ordinal && *binding == row.binding && kind == &row.kind
                },
            )
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn roots(
        &self,
    ) -> &BTreeMap<BindingRefV1, BindingRefV1> {
        &self.roots
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn reads(
        &self,
    ) -> impl Iterator<Item = (&SourceExprSiteV1, &ResolvedLexicalRefV1)> {
        self.reads.iter().map(|(site, reference)| (site, reference))
    }
    /// A local read can satisfy the borrowed-input use census without a Home
    /// evaluation only when its complete resolver value graph has no path to
    /// any borrowed entry. Joins are followed with cycle protection; unknown
    /// nodes and non-local references remain unproved.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn corroborates_non_entry_source_read_v1(
        &self,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
        entry_origins: &BTreeMap<BindingRefV1, BindingRefV1>,
        consumed: &dyn Fn(&SourceExprSiteV1) -> bool,
    ) -> bool {
        use crate::mir::resolved_semantics::source_value_flow::{
            SourceValueOriginV1, SourceValueRefV1, VerifiedSourceValueFlowV1,
        };

        fn visit(
            value: SourceValueRefV1,
            flow: &VerifiedSourceValueFlowV1,
            reads: &[(SourceExprSiteV1, ResolvedLexicalRefV1)],
            entry_origins: &BTreeMap<BindingRefV1, BindingRefV1>,
            consumed: &dyn Fn(&SourceExprSiteV1) -> bool,
            active: &mut BTreeSet<SourceValueRefV1>,
            complete: &mut BTreeSet<SourceValueRefV1>,
        ) -> bool {
            if complete.contains(&value) {
                return true;
            }
            if !active.insert(value) {
                return true;
            }
            let result = match flow.origin(value) {
                Some(SourceValueOriginV1::Entry(binding)) => !entry_origins.contains_key(binding),
                Some(SourceValueOriginV1::Expression(root)) => {
                    let root_path = root.node().segments();
                    reads
                        .iter()
                        .filter(|(read_site, _)| {
                            read_site.node().segments().starts_with(root_path)
                        })
                        .all(|(read_site, reference)| {
                            // A read swallowed by a scalar-result Core call flows nowhere.
                            if consumed(read_site) { return true; }
                            let ResolvedLexicalRefV1::Local(read_binding) = reference else {
                                return false;
                            };
                            if read_binding.owner() != flow.owner()
                                || reads
                                    .iter()
                                    .filter(|(candidate, candidate_ref)| {
                                        candidate == read_site
                                            && *candidate_ref
                                                == ResolvedLexicalRefV1::Local(*read_binding)
                                    })
                                    .count()
                                    != 1
                            {
                                return false;
                            }
                            flow.read(read_site, *read_binding).is_some_and(|dependency| {
                                visit(
                                    dependency,
                                    flow,
                                    reads,
                                    entry_origins,
                                    consumed,
                                    active,
                                    complete,
                                )
                            })
                        })
                }
                Some(SourceValueOriginV1::Join { incoming, .. }) => {
                    !incoming.is_empty()
                        && incoming.iter().all(|(_, dependency)| {
                            visit(
                                *dependency,
                                flow,
                                reads,
                                entry_origins,
                                consumed,
                                active,
                                complete,
                            )
                        })
                }
                Some(SourceValueOriginV1::Unknown) | None => false,
            };
            active.remove(&value);
            if result {
                complete.insert(value);
            }
            result
        }

        if binding.owner() != self.owner
            || self
                .reads
                .iter()
                .filter(|(candidate, reference)| {
                    candidate == site
                        && *reference == ResolvedLexicalRefV1::Local(binding)
                })
                .count()
                != 1
        {
            return false;
        }
        let Some(flow) = self
            .flow
            .as_deref()
            .filter(|flow| flow.owner() == self.owner && flow.is_complete())
        else {
            return false;
        };
        let Some(value) = flow.read(site, binding) else {
            return false;
        };
        visit(
            value,
            flow,
            &self.reads,
            entry_origins,
            consumed,
            &mut BTreeSet::new(),
            &mut BTreeSet::new(),
        )
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
    fn cover_exact_bool_static_return_reads_v1(
        &self,
        owner: FunctionOwnerIdV1,
        batch_slot: u32,
        incoming: &[&super::BorrowedIncomingCallDraftV1],
        origins: &BTreeMap<BindingRefV1, BindingRefV1>,
        covered: &mut std::collections::BTreeSet<(SourceExprSiteV1, BindingRefV1)>,
    ) -> Result<Option<bool>, String> {
        let exact_bool_static = !incoming.is_empty() && incoming.iter().all(|call| match &call.source {
            super::BorrowedIncomingSourceV1::Static(source) => source.call_site() == &call.call
                && source.callee_owner() == owner && source.target_batch_slot() == batch_slot
                && matches!(source.source_result(),
                    crate::mir::callable_result_representation::VerifiedCallableResultDispositionV1::ExactBool),
            _ => false,
        });
        if !exact_bool_static { return Ok(None); }
        let Some(plans) = self.return_plans.as_deref() else { return Ok(Some(false)); };
        use crate::mir::resolved_value_profile::{
            expression_children::ExpressionChildrenV1,
            expression_plan::{ExpressionNodeSourceV1, ExpressionPlanRootV1},
            expression_root::StatementExpressionRoleV1,
        };
        for plan in plans {
            let ExpressionPlanRootV1::Statement { role: StatementExpressionRoleV1::ReturnValue, value, .. }
                = plan.root() else { return Err(super::super::freeze("static-input/qualified-use-identity")) };
            if plan.owner() != owner || plan.nodes().first().is_none_or(|node| node.site() != value)
                || plan.nodes().iter().any(|node| !matches!(node.children(), ExpressionChildrenV1::Known(_))) {
                return Err(super::super::freeze("static-input/qualified-use-identity"));
            }
            for (_, node) in plan.nodes_outside_calls() {
                let ExpressionNodeSourceV1::Reference(ResolvedLexicalRefV1::Local(binding)) = node.source()
                    else { continue; };
                if !origins.contains_key(binding) { continue; }
                let site = node.site().clone();
                if self.reads().filter(|(at, reference)| *at == &site
                    && **reference == ResolvedLexicalRefV1::Local(*binding)).count() != 1
                    || !covered.insert((site, *binding)) {
                    return Err(super::super::freeze("static-input/read-use-duplicate-or-foreign"));
                }
            }
        }
        Ok(Some(true))
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn initializers(
        &self,
    ) -> &[ResolvedInitializerRelationV1] {
        &self.initializers
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn assignments(
        &self,
    ) -> &[(SourceExprSiteV1, ResolvedAssignmentTargetV1)] {
        &self.assignments
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn captures(
        &self,
    ) -> &[(OwnedExprSiteV1, BindingRefV1, bool)] {
        &self.captures
    }
    /// Exact retained initializer and current source flow, not raw read membership.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn initializer_entry_alias_v1(
        &self, initializer: &ResolvedInitializerRelationV1, source_binding: Option<BindingRefV1>,
    ) -> Result<Option<BindingRefV1>, BorrowedFormalUseDraftErrorV1> {
        let reject = || BorrowedFormalUseDraftErrorV1::SourceIdentity;
        if initializer.binding().owner() != self.owner
            || self.initializers.iter().filter(|row| *row == initializer).count() != 1 { return Err(reject()); }
        let Some(source_binding) = source_binding else { return Ok(None) };
        let SourceBindingSiteV1::Local { statement, ordinal } = initializer.declaration_site() else { return Err(reject()) };
        let site = crate::mir::resolved_semantics::SourcePathV1::from_node(statement.node())
            .child(crate::mir::resolved_semantics::SourcePathSegmentV1::Initializer(*ordinal)).expr();
        if initializer.initializer_site() != Some(&site)
            || source_binding.owner() != self.owner
            || self.reads.iter().filter(|(at, reference)| *at == site
                && *reference == ResolvedLexicalRefV1::Local(source_binding)).count() != 1 { return Err(reject()); }
        let Some(flow) = &self.flow else { return Ok(None) };
        let Some(point) = flow.read(&site, source_binding) else { return Ok(None) };
        let Some(formal) = flow.alias_entry_origin(point, |site|
            self.reads.iter().find(|(at, _)| at == site).map(|(_, reference)| *reference)) else { return Ok(None) };
        if self.roots.get(&formal) != Some(&formal) { return Ok(None); }
        if initializer.declared_type_name().is_some() || formal == initializer.binding() { return Err(reject()); }
        Ok(Some(formal))
    }
    /// Account only for the original Return read of a fresh local construction.
    /// Constructor arguments remain separate uses; this grants no Home transfer.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn local_construction_return_read_v1(
        &self, root: &crate::mir::normal_callable_semantic_package::ResultOriginWitnessV1,
    ) -> Option<BindingRefV1> {
        use crate::mir::resolved_value_profile::{expression_plan::ExpressionPlanRootV1,
            expression_root::StatementExpressionRoleV1};
        if !matches!(root.step(), crate::mir::normal_callable_semantic_package::ResultWitnessStepV1::LocalConstruction(_)) {
            return None;
        }
        let acquisition = root.construction_site_v1()?;
        let mut plans = self.return_plans.as_deref()?.iter().filter(|plan|
            plan.owner() == self.owner && matches!(plan.root(), ExpressionPlanRootV1::Statement {
                role: StatementExpressionRoleV1::ReturnValue, value, .. } if value == root.site().site()));
        let plan = plans.next()?;
        let crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Reference(ResolvedLexicalRefV1::Local(binding))
            = plan.nodes().first()?.source() else { return None; };
        let mut initializers = self.initializers.iter().filter(|row| row.binding() == *binding);
        let initializer = initializers.next()?;
        if plans.next().is_some() || initializers.next().is_some() || self.roots.contains_key(binding)
            || binding.owner() != self.owner || plan.nodes().first()?.site() != root.site().site()
            || root.site().owner() != self.owner || acquisition.owner() != self.owner
            || initializer.initializer_site() != Some(acquisition.site())
            || self.reads().filter(|(site, reference)| *site == root.site().site()
                && **reference == ResolvedLexicalRefV1::Local(*binding)).count() != 1 {
            return None;
        }
        let flow = self.flow.as_ref()?;
        let value = flow.read(root.site().site(), *binding)?;
        matches!(flow.origin(value), Some(crate::mir::resolved_semantics::source_value_flow::SourceValueOriginV1::Expression(site))
            if site == acquisition.site()).then_some(*binding)
    }

    /// Corroborate a returned residence through the SAME original alias flow.
    /// Source membership alone cannot turn a replaced/joined value into this Home.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn corroborates_text_residence_return_v1(
        &self, returned: &crate::mir::resolved_semantics::home_new_prefix::TerminalValueReturnV1,
    ) -> bool {
        use crate::mir::resolved_semantics::home_new_prefix::TerminalReturnedSourceV1;
        use crate::mir::resolved_semantics::source_value_flow::SourceValueOriginV1;
        let TerminalReturnedSourceV1::TextResidence { binding, acquisition } = returned.returned()
            else { return false };
        let Some(root) = acquisition.source().nodes().first().map(|node| node.site()) else { return false };
        if returned.owner() != self.owner || binding.owner() != self.owner
            || acquisition.source().owner() != self.owner || !acquisition.owns_root()
            || !acquisition.is_text_residence()
            || self.reads.iter().filter(|(site, reference)| site == returned.value_site()
                && *reference == ResolvedLexicalRefV1::Local(*binding)).count() != 1 { return false; }
        let Some(flow) = &self.flow else { return false };
        flow.read(returned.value_site(), *binding).and_then(|point|
            flow.alias_origin_with_expression_boundary(point,
                |site| self.reads.iter().find(|(at, _)| at == site).map(|(_, reference)| *reference),
                |site| site == root))
            .and_then(|origin| flow.origin(origin))
            .is_some_and(|origin| matches!(origin, SourceValueOriginV1::Expression(site) if site == root))
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn lend_read(
        self: &Rc<Self>,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
        formal: BindingRefV1,
    ) -> Option<OriginalFormalReadLoanV1> {
        let index = self.reads.iter().position(|(original, reference)| {
            original == site && *reference == ResolvedLexicalRefV1::Local(binding)
        })?;
        if !self.roots.contains_key(&formal) {
            return None;
        }
        // Raw membership only. A Join/unknown point stays an obligation for the
        // existing source/current/Loop owner; this loan grants no alias view.
        let point = self.flow.as_ref().and_then(|flow| flow.read(site, binding));
        Some(OriginalFormalReadLoanV1 {
            original: Rc::clone(self),
            index,
            formal,
            point,
        })
    }
}

#[derive(Debug, Clone)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
struct OriginalFormalReadLoanV1
{
    original: Rc<OriginalFormalUseSourceV1>,
    index: usize,
    formal: BindingRefV1,
    point: Option<crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1>,
}
impl PartialEq for OriginalFormalReadLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.original, &other.original)
            && self.index == other.index
            && self.formal == other.formal
            && self.point == other.point
    }
}
impl Eq for OriginalFormalReadLoanV1 {}
impl OriginalFormalReadLoanV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn matches_contract(
        &self,
        contract: &OwnedCallableParameterContractDeclarationV1,
    ) -> bool {
        self.original.matches_contract(contract)
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn matches(
        &self,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
        formal: BindingRefV1,
    ) -> bool {
        self.formal == formal
            && self.original.reads.get(self.index)
                == Some(&(site.clone(), ResolvedLexicalRefV1::Local(binding)))
    }
}
