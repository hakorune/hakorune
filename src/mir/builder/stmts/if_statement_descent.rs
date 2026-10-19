//! Disconnected associated-input boundary for statement-position If control.
//!
//! This box owns only exact If syntax observation, one condition demand,
//! ordered branch-body carrier demand, and delegation to the existing IfForm
//! owner. CFG layout, variable snapshots, PHIs, branch termination, JoinIR
//! diagnostics, source location, and callable-result consumption remain
//! outside the driver core. This module also exposes one thin success-only
//! completion seam; the existing constant emitter remains the Void owner.

use crate::ast::ASTNode;
use crate::mir::builder::if_form::IfBranchKindV1;
use crate::mir::{MirBuilder, ValueId};

use super::super::recursive_child_lowering::{
    drive_legacy_body_v1, drive_legacy_expression_v1, drive_legacy_statement_v1,
    RecursiveChildLoweringPortV1,
};

/// Raw statement-If adapter preserving the retired facade's branch Program shell.
///
/// The shared raw child port lowers a body directly. Statement-position If
/// historically lowered each branch as an expression-position `Program` with
/// an unknown span. Keeping that shell here preserves its recursion boundary
/// and span publication without making the generic If driver own either law.
///
/// The adapter borrows the same raw child port through condition and branch
/// descent. It therefore preserves the unknown-span Program wrapper without
/// rebuilding a raw-recursion port at an If boundary.
struct RawStatementIfPortV1<'port, Port> {
    child: &'port mut Port,
}

impl<'port, Port> RawStatementIfPortV1<'port, Port> {
    const fn new(child: &'port mut Port) -> Self {
        Self { child }
    }
}

impl<Port> RecursiveChildLoweringPortV1 for RawStatementIfPortV1<'_, Port>
where
    Port: RecursiveChildLoweringPortV1<
        BodyInput = Vec<ASTNode>,
        StatementInput = ASTNode,
        ExpressionInput = ASTNode,
    >,
{
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

    fn prepare_borrowed_field_write_guard_observer_v1(&mut self)
        -> Result<Option<Box<dyn crate::mir::builder::if_form::IfFormPhysicalObserverV1>>, String> {
        self.child.prepare_borrowed_field_write_guard_observer_v1()
    }

    fn prepare_if_state_observer_v1(&mut self, builder: &MirBuilder)
        -> Result<Option<Box<dyn crate::mir::builder::if_form::IfFormPhysicalObserverV1>>, String> {
        self.child.prepare_if_state_observer_v1(builder)
    }

    fn prepare_checked_bool_condition_v1(&mut self) -> Result<Option<std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBoolConditionFrameV1>>, String> {
        self.child.prepare_checked_bool_condition_v1()
    }
    fn lower_checked_bool_condition_v1(&mut self, builder: &mut MirBuilder,
        frame: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedBoolConditionFrameV1>, condition: ASTNode,
    ) -> Result<crate::mir::builder::ordinary_new_admission::selected::bool_condition::LoweredBoolConditionV1, String> {
        self.child.lower_checked_bool_condition_v1(builder, frame, condition)
    }

    fn cleanup_exit_policy_v1(
        &self,
    ) -> crate::mir::builder::control_flow::cleanup::CleanupExitPolicyV1 {
        self.child.cleanup_exit_policy_v1()
    }

    fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        drive_legacy_expression_v1(
            builder,
            self.child,
            ASTNode::Program {
                statements: input,
                span: crate::ast::Span::unknown(),
            },
        )
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        drive_legacy_statement_v1(builder, self.child, input)
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        drive_legacy_expression_v1(builder, self.child, input)
    }
}

pub(in crate::mir::builder) struct RawLegacyIfStatementInputV1 {
    condition: ASTNode,
    then_body: Vec<ASTNode>,
    else_body: Option<Vec<ASTNode>>,
}

impl RawLegacyIfStatementInputV1 {
    pub(in crate::mir::builder) const fn new(
        condition: ASTNode,
        then_body: Vec<ASTNode>,
        else_body: Option<Vec<ASTNode>>,
    ) -> Self {
        Self {
            condition,
            then_body,
            else_body,
        }
    }
}

#[cfg(test)]
pub(super) fn raw_syntax_for_tests(
    input: &RawLegacyIfStatementInputV1,
) -> IfStatementSyntaxViewV1<'_> {
    IfStatementSyntaxViewV1::new(&input.condition, input.else_body.is_some())
}

#[cfg(test)]
pub(super) fn raw_condition_for_tests(input: &RawLegacyIfStatementInputV1) -> ASTNode {
    input.condition.clone()
}

#[cfg(test)]
pub(super) fn raw_then_body_for_tests(input: &RawLegacyIfStatementInputV1) -> Vec<ASTNode> {
    input.then_body.clone()
}

#[cfg(test)]
pub(super) fn raw_else_body_for_tests(input: &RawLegacyIfStatementInputV1) -> Option<Vec<ASTNode>> {
    input.else_body.clone()
}

pub(in crate::mir::builder) struct IfStatementSyntaxViewV1<'input> {
    condition: &'input ASTNode,
    has_explicit_else: bool,
}

impl<'input> IfStatementSyntaxViewV1<'input> {
    pub(in crate::mir::builder) const fn new(
        condition: &'input ASTNode,
        has_explicit_else: bool,
    ) -> Self {
        Self {
            condition,
            has_explicit_else,
        }
    }

    pub(in crate::mir::builder) const fn condition(&self) -> &'input ASTNode {
        self.condition
    }

    pub(in crate::mir::builder) const fn has_explicit_else(&self) -> bool {
        self.has_explicit_else
    }
}

pub(in crate::mir::builder) trait IfStatementDescentPortV1:
    RecursiveChildLoweringPortV1
{
    type IfInput;

    fn if_syntax<'input>(
        &self,
        input: &'input Self::IfInput,
    ) -> Result<IfStatementSyntaxViewV1<'input>, String>;

    fn if_condition_expression_input(
        &self,
        input: &Self::IfInput,
    ) -> Result<Self::ExpressionInput, String>;

    fn if_then_body_input(&self, input: &Self::IfInput) -> Result<Self::BodyInput, String>;

    fn if_else_body_input(&self, input: &Self::IfInput) -> Result<Self::BodyInput, String>;
}

impl<Port> IfStatementDescentPortV1 for RawStatementIfPortV1<'_, Port>
where
    Port: RecursiveChildLoweringPortV1<
        BodyInput = Vec<ASTNode>,
        StatementInput = ASTNode,
        ExpressionInput = ASTNode,
    >,
{
    type IfInput = RawLegacyIfStatementInputV1;

    fn if_syntax<'input>(
        &self,
        input: &'input Self::IfInput,
    ) -> Result<IfStatementSyntaxViewV1<'input>, String> {
        Ok(IfStatementSyntaxViewV1::new(
            &input.condition,
            input.else_body.is_some(),
        ))
    }

    fn if_condition_expression_input(
        &self,
        input: &Self::IfInput,
    ) -> Result<Self::ExpressionInput, String> {
        Ok(input.condition.clone())
    }

    fn if_then_body_input(&self, input: &Self::IfInput) -> Result<Self::BodyInput, String> {
        Ok(input.then_body.clone())
    }

    fn if_else_body_input(&self, input: &Self::IfInput) -> Result<Self::BodyInput, String> {
        input
            .else_body
            .clone()
            .ok_or_else(|| "[if-statement-descent/raw-else-input-missing]".to_string())
    }
}

struct IfObserversV1<'a> {
    state: Option<Box<dyn crate::mir::builder::if_form::IfFormPhysicalObserverV1>>,
    condition: Option<&'a mut dyn crate::mir::builder::if_form::IfFormPhysicalObserverV1>,
}
impl crate::mir::builder::if_form::IfFormPhysicalObserverV1 for IfObserversV1<'_> {
    fn before_branches(&mut self, function: &crate::mir::MirFunction,
        pending: std::rc::Rc<crate::mir::builder::if_form::IfConditionMaterializationV1>,
    ) -> Result<(), String> {
        if let Some(state) = self.state.as_mut() { state.before_branches(function, std::rc::Rc::clone(&pending))?; }
        if let Some(condition) = self.condition.as_deref_mut() { condition.before_branches(function, pending)?; }
        Ok(())
    }
    fn emitted_branch(&mut self, function: &crate::mir::MirFunction,
        pending: &std::rc::Rc<crate::mir::builder::if_form::IfConditionMaterializationV1>,
        branch: (crate::mir::BasicBlockId, crate::mir::MirInstruction),
    ) -> Result<(), String> {
        if let Some(state) = self.state.as_mut() { state.emitted_branch(function, pending, branch.clone())?; }
        if let Some(condition) = self.condition.as_deref_mut() { condition.emitted_branch(function, pending, branch)?; }
        Ok(())
    }
    fn enter_branch(&mut self, branch: IfBranchKindV1) -> Result<(), String> {
        if let Some(state) = self.state.as_mut() { state.enter_branch(branch)?; }
        Ok(())
    }
    fn finished_branch(&mut self, branch: IfBranchKindV1) -> Result<(), String> {
        if let Some(state) = self.state.as_mut() { state.finished_branch(branch)?; }
        Ok(())
    }
    fn completed_merge(&mut self, builder: &mut MirBuilder,
        exits: [Option<crate::mir::BasicBlockId>; 2],
    ) -> Result<(), String> {
        if let Some(state) = self.state.as_mut() { state.completed_merge(builder, exits)?; }
        Ok(())
    }
}

pub(in crate::mir::builder) fn drive_if_statement_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: &Port::IfInput,
) -> Result<(), String>
where
    Port: IfStatementDescentPortV1,
{
    let (condition_syntax, has_explicit_else) = {
        let syntax = port.if_syntax(input)?;
        (syntax.condition().clone(), syntax.has_explicit_else())
    };
    let checked = port.prepare_checked_bool_condition_v1()?;
    let condition_input = port.if_condition_expression_input(input)?;
    let (condition_value, mut checked) = match checked {
        Some(frame) => {
            let lowered = port.lower_checked_bool_condition_v1(builder, frame, condition_input)?;
            (lowered.value(), Some(lowered))
        },
        None => (drive_legacy_expression_v1(builder, port, condition_input)?, None),
    };
    let mut write_guard = port.prepare_borrowed_field_write_guard_observer_v1()?;
    if checked.is_some() && write_guard.is_some() {
        return Err("[freeze:contract][borrowed-write-guard/condition-owner-overlap]".into());
    }
    let condition_debug =
        prepare_if_statement_condition_value_v1(builder, condition_value, &condition_syntax)?;

    // Branch carrier demand stays at the existing IfForm execution points.
    // In particular, an else carrier is not requested unless the then branch
    // was requested and lowered successfully.
    let mut then_demanded = false;
    let mut else_demanded = false;

    let mut observers = IfObserversV1 {
        state: port.prepare_if_state_observer_v1(builder)?,
        condition: match write_guard.as_mut() {
            Some(observer) => Some(observer.as_mut()),
            None => checked.as_mut().map(|lowered| lowered.observer()),
        },
    };
    let armed = observers.state.is_some() || observers.condition.is_some();
    builder.lower_if_form_with_physical_observer_v1(
        condition_value,
        condition_debug,
        has_explicit_else,
        |builder, branch| {
            let body = match branch {
                IfBranchKindV1::Then => {
                    if std::mem::replace(&mut then_demanded, true) {
                        return Err("[if-statement-descent/then-demanded-twice]".to_string());
                    }
                    port.if_then_body_input(input)?
                }
                IfBranchKindV1::Else => {
                    if std::mem::replace(&mut else_demanded, true) {
                        return Err("[if-statement-descent/else-demanded-twice]".to_string());
                    }
                    port.if_else_body_input(input)?
                }
            };
            drive_legacy_body_v1(builder, port, body)
        },
        if armed { Some(&mut observers) } else { None },
    )?;
    Ok(())
}

/// Lower one raw statement-position If through an existing raw child port.
///
/// This owns no control-flow policy: it only preserves the historical Program
/// branch shell while threading the caller's recursive descent capability.
pub(in crate::mir::builder) fn drive_raw_if_statement_with_port_v1<Port>(
    builder: &mut MirBuilder,
    child: &mut Port,
    condition: ASTNode,
    then_body: Vec<ASTNode>,
    else_body: Option<Vec<ASTNode>>,
) -> Result<(), String>
where
    Port: RecursiveChildLoweringPortV1<
        BodyInput = Vec<ASTNode>,
        StatementInput = ASTNode,
        ExpressionInput = ASTNode,
    >,
{
    let input = RawLegacyIfStatementInputV1::new(condition, then_body, else_body);
    let mut port = RawStatementIfPortV1::new(child);
    drive_if_statement_v1(builder, &mut port, &input)
}

/// Publish the statement-position Void only after If control lowering succeeds.
///
/// Raw and located carriers share this completion so neither adapter owns a
/// second success/failure policy.
pub(in crate::mir::builder) fn complete_if_statement_v1(
    builder: &mut MirBuilder,
    lowering: Result<(), String>,
) -> Result<ValueId, String> {
    lowering?;
    crate::mir::builder::emission::constant::emit_void(builder)
}

fn prepare_if_statement_condition_value_v1(
    builder: &mut MirBuilder,
    condition_value: ValueId,
    condition: &ASTNode,
) -> Result<Option<ASTNode>, String> {
    let Some(region) = builder.current_fastmem_region() else {
        return Ok(None);
    };
    crate::mir::builder::fastmem::branch::ensure_fastmem_owner_eq_condition(
        builder,
        region,
        condition_value,
    )?;
    builder.add_fastmem_branch_condition_fact(
        region,
        condition_value,
        crate::mir::function::FastMemBranchConditionProofKind::SourceAssumeOwnerEq,
        true,
    )?;
    Ok(Some(condition.clone()))
}
