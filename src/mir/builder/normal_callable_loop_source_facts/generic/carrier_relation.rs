//! Source binding -> Recipe carrier slots, co-sealed by the existing issuer.
//!
//! Slots are Recipe-local identities. Labels only retain the planner/emission
//! spelling; neither labels nor physical values resolve a source binding.

use std::collections::{BTreeMap, BTreeSet};

use super::*;
use crate::mir::builder::normal_callable_loop_handoff::{
    CallableLoopBindingRoleV1, CallableLoopReadyBindingClassV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, BodyChildRoleV1, ExprChildRoleV1, SourceBindingSiteV1, SourceStmtSiteV1,
};
use crate::mir::ValueId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) struct CallableLoopCarrierSlotV1(usize);

impl std::fmt::Display for CallableLoopCarrierSlotV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "slot:{}", self.0)
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) struct CallableLoopCarrierBindingV1 {
    slot: CallableLoopCarrierSlotV1,
    binding: BindingRefV1,
    label: String,
    targets: Box<[SourceNodeSiteV1]>,
}

impl CallableLoopCarrierBindingV1 {
    pub(in crate::mir::builder) fn slot(&self) -> CallableLoopCarrierSlotV1 {
        self.slot
    }
    pub(in crate::mir::builder) fn binding(&self) -> BindingRefV1 {
        self.binding
    }
    pub(in crate::mir::builder) fn label(&self) -> &str {
        &self.label
    }
    pub(in crate::mir::builder) fn targets(&self) -> &[SourceNodeSiteV1] {
        &self.targets
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) struct CallableLoopCarrierRelationV1 {
    induction: CallableLoopCarrierSlotV1,
    carriers: Box<[CallableLoopCarrierBindingV1]>,
    assignment_bindings: BTreeMap<SourceNodeSiteV1, BindingRefV1>,
}

impl CallableLoopCarrierRelationV1 {
    pub(in crate::mir::builder) fn induction(&self) -> CallableLoopCarrierSlotV1 {
        self.induction
    }

    pub(in crate::mir::builder) fn induction_binding(&self) -> Result<BindingRefV1, String> {
        self.carriers
            .iter()
            .find(|carrier| carrier.slot() == self.induction)
            .map(CallableLoopCarrierBindingV1::binding)
            .ok_or_else(|| "[freeze:contract][callable-loop/induction-carrier-missing]".to_owned())
    }
    pub(in crate::mir::builder) fn carriers(&self) -> &[CallableLoopCarrierBindingV1] {
        &self.carriers
    }
    pub(in crate::mir::builder) fn assignment_bindings(
        &self,
    ) -> &BTreeMap<SourceNodeSiteV1, BindingRefV1> {
        &self.assignment_bindings
    }

    pub(in crate::mir::builder) fn physical_value_for_binding(
        &self,
        binding: BindingRefV1,
        physical_by_label: &BTreeMap<String, ValueId>,
    ) -> Result<Option<crate::mir::ValueId>, String> {
        let Some(carrier) = self.carriers.iter().find(|row| row.binding() == binding) else {
            return Ok(None);
        };
        physical_by_label
            .get(carrier.label())
            .copied()
            .map(Some)
            .ok_or_else(|| {
                format!(
                    "[freeze:contract][callable-loop/source-carrier-physical-missing] label={}",
                    carrier.label()
                )
            })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableLoopCarrierRelationRejectV1 {
    ForeignOwner,
    SourceLocation(String),
    MissingIncrement,
    IncrementNotVariableAssignment,
    DuplicateTarget,
    TargetCoverage,
    DeclarationCoverage,
    DuplicateDeclaration,
    InductionMismatch,
    BindingLabelMismatch,
}

/// Issue the carrier relation from the source owner before route admission.
///
/// This deliberately consumes only the pre-effect/source context and the
/// planner's GenericLoopV1 facts.  It must not require a selected route token:
/// source evidence is what authorizes the source-specific overlap admission.
/// The returned relation is retained inside the move-only source evidence
/// aggregate until the Recipe consumer observes it.
pub(super) fn issue_pre_route(
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    pre: &CallableSemanticLoopHandoffPreEffectReceiptV1,
    body_source: &RawInvocationSourceContextV1,
    generic: &GenericLoopV1Facts,
) -> Result<CallableLoopCarrierRelationV1, CallableLoopCarrierRelationRejectV1> {
    use CallableLoopCarrierRelationRejectV1 as Reject;
    if pre.owner() != owner {
        return Err(Reject::ForeignOwner);
    }
    let mut assignments = BTreeMap::new();
    for row in pre
        .rows()
        .iter()
        .flat_map(|row| row.receipts())
        .chain(pre.body_only_rows().iter().flat_map(|row| row.receipts()))
    {
        if row.binding().owner() != owner {
            return Err(Reject::ForeignOwner);
        }
        if row.role() == CallableLoopBindingRoleV1::BodyRebind
            && assignments
                .insert(row.site().clone(), row.binding())
                .is_some()
        {
            return Err(Reject::DuplicateTarget);
        }
    }
    let mut syntax = SourceTargets::default();
    syntax.collect(&generic.body.body, body_source)?;
    if assignments.keys().ne(syntax.assignments.keys()) {
        return Err(Reject::TargetCoverage);
    }
    let expected_declarations = pre
        .local_declarations()
        .values()
        .cloned()
        .collect::<BTreeSet<_>>();
    if pre
        .local_declarations()
        .keys()
        .any(|binding| binding.owner() != owner)
    {
        return Err(Reject::ForeignOwner);
    }
    if expected_declarations != syntax.declarations {
        return Err(Reject::DeclarationCoverage);
    }

    let increment_index = generic.increment_index.ok_or(Reject::MissingIncrement)?;
    let increment = generic
        .body
        .body
        .get(increment_index)
        .ok_or(Reject::MissingIncrement)?;
    let ASTNode::Assignment { target, .. } = increment else {
        return Err(Reject::IncrementNotVariableAssignment);
    };
    let ASTNode::Variable { name, .. } = target.as_ref() else {
        return Err(Reject::IncrementNotVariableAssignment);
    };
    let increment_source = body_source
        .body_statement_context(increment, increment_index)
        .and_then(|source| source.child_expression(increment, ExprChildRoleV1::AssignmentTarget))
        .map_err(Reject::SourceLocation)?;
    let increment_site = exact_site(&increment_source)?;
    let induction_binding = assignments
        .get(&increment_site)
        .copied()
        .ok_or(Reject::InductionMismatch)?;
    let source_carriers = pre
        .rows()
        .iter()
        .filter(|row| row.class() == CallableLoopReadyBindingClassV1::Carrier)
        .map(|row| row.binding())
        .collect::<Vec<_>>();
    if source_carriers != [induction_binding]
        || name != &generic.loop_var
        || pre.local_declarations().contains_key(&induction_binding)
    {
        return Err(Reject::InductionMismatch);
    }

    let mut grouped = BTreeMap::<BindingRefV1, (String, Vec<SourceNodeSiteV1>)>::new();
    for (site, binding) in &assignments {
        if pre.local_declarations().contains_key(binding) {
            continue;
        }
        let label = syntax.assignments.get(site).ok_or(Reject::TargetCoverage)?;
        let entry = grouped
            .entry(*binding)
            .or_insert_with(|| (label.clone(), Vec::new()));
        if &entry.0 != label {
            return Err(Reject::BindingLabelMismatch);
        }
        entry.1.push(site.clone());
    }
    let mut carriers = Vec::with_capacity(grouped.len());
    let mut induction = None;
    for (binding, (label, targets)) in grouped {
        let slot = CallableLoopCarrierSlotV1(carriers.len());
        if binding == induction_binding {
            induction = Some(slot);
        }
        carriers.push(CallableLoopCarrierBindingV1 {
            slot,
            binding,
            label,
            targets: targets.into_boxed_slice(),
        });
    }
    Ok(CallableLoopCarrierRelationV1 {
        induction: induction.ok_or(Reject::InductionMismatch)?,
        carriers: carriers.into_boxed_slice(),
        assignment_bindings: assignments,
    })
}

fn exact_site(
    source: &RawInvocationSourceContextV1,
) -> Result<SourceNodeSiteV1, CallableLoopCarrierRelationRejectV1> {
    source.site().cloned().ok_or_else(|| {
        CallableLoopCarrierRelationRejectV1::SourceLocation("unlocated target".into())
    })
}

#[derive(Default)]
struct SourceTargets {
    assignments: BTreeMap<SourceNodeSiteV1, String>,
    declarations: BTreeSet<SourceBindingSiteV1>,
}

impl SourceTargets {
    fn collect(
        &mut self,
        body: &[ASTNode],
        source: &RawInvocationSourceContextV1,
    ) -> Result<(), CallableLoopCarrierRelationRejectV1> {
        use CallableLoopCarrierRelationRejectV1 as Reject;
        for (index, statement) in body.iter().enumerate() {
            let context = source
                .body_statement_context(statement, index)
                .map_err(Reject::SourceLocation)?;
            match statement {
                ASTNode::Assignment { target, .. } => {
                    if let ASTNode::Variable { name, .. } = target.as_ref() {
                        let target = context
                            .child_expression(statement, ExprChildRoleV1::AssignmentTarget)
                            .map_err(Reject::SourceLocation)?;
                        if self
                            .assignments
                            .insert(exact_site(&target)?, name.clone())
                            .is_some()
                        {
                            return Err(Reject::DuplicateTarget);
                        }
                    }
                }
                ASTNode::Local { variables, .. } => {
                    let site = exact_site(&context)?;
                    for ordinal in 0..variables.len() {
                        let ordinal =
                            u32::try_from(ordinal).map_err(|_| Reject::DeclarationCoverage)?;
                        if !self.declarations.insert(SourceBindingSiteV1::Local {
                            statement: SourceStmtSiteV1::from_node(site.clone()),
                            ordinal,
                        }) {
                            return Err(Reject::DuplicateDeclaration);
                        }
                    }
                }
                ASTNode::If {
                    then_body,
                    else_body,
                    ..
                } => {
                    let then_source = context
                        .child_body(statement, BodyChildRoleV1::IfThen)
                        .map_err(Reject::SourceLocation)?;
                    self.collect(then_body, &then_source)?;
                    if let Some(else_body) = else_body {
                        let else_source = context
                            .child_body(statement, BodyChildRoleV1::IfElse)
                            .map_err(Reject::SourceLocation)?;
                        self.collect(else_body, &else_source)?;
                    }
                }
                _ => {} // Coverage comparison rejects any unvisited rebind/declaration.
            }
        }
        Ok(())
    }
}
