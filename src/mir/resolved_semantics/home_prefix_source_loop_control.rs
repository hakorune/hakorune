//! Retain decisions made by the existing recursive source walk. No AST
//! classifier, route Recipe expansion, joined-state reconstruction or keys.
use super::super::super::{CheckedBoolConditionFrameV1, HomeExpressionPlanV1};
use super::*;
use crate::mir::compiler::located::{LocatedBodyV1, SourceBodySiteV1};
use crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1;
use crate::mir::resolved_semantics::{
    ResolvedExitRecordV1, ResolvedIfRegionBundleV1, SourceStmtSiteV1,
};

#[derive(Default)]
pub(super) struct SourceLoopControlDraftV1 {
    bodies: Vec<BodyDraft>,
    actions: BTreeMap<SourceStmtSiteV1, Action>,
    error: Option<&'static str>,
}
struct BodyDraft {
    site: SourceBodySiteV1,
    source_length: usize,
    visited: Vec<SourceStmtSiteV1>,
    terminated: Option<bool>,
}
enum Action {
    Return(Rc<SourceLoopReturnV1>),
    Expression(HomeExpressionPlanV1),
    OwnedArrayAppend(NamedArrayHomeObservationV1),
    Call(super::super::super::LocalCallObservationV1),
    If {
        bundle: ResolvedIfRegionBundleV1,
        condition: Rc<CheckedBoolConditionFrameV1>,
        then_body: SourceBodySiteV1,
        else_body: Option<SourceBodySiteV1>,
        then_terminated: bool,
        else_terminated: bool,
    },
    Exit {
        record: ResolvedExitRecordV1,
        port: usize,
    },
    Loop(Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>),
}
#[derive(Debug)]
pub(crate) struct CheckedSourceLoopBodyV1 {
    pub(crate) site: SourceBodySiteV1,
    pub(crate) statements: Box<[CheckedSourceLoopStatementV1]>,
    pub(crate) terminated: bool,
}
#[derive(Debug)]
pub(crate) enum CheckedSourceLoopStatementV1 {
    Return(Rc<SourceLoopReturnV1>),
    Expression {
        site: SourceStmtSiteV1,
        frame: HomeExpressionPlanV1,
    },
    Call {
        site: SourceStmtSiteV1,
        observation: super::super::super::LocalCallObservationV1,
    },
    OwnedArrayAppend {
        site: SourceStmtSiteV1,
        observation: NamedArrayHomeObservationV1,
    },
    If {
        site: SourceStmtSiteV1,
        bundle: ResolvedIfRegionBundleV1,
        condition: Rc<CheckedBoolConditionFrameV1>,
        then_body: CheckedSourceLoopBodyV1,
        else_body: Option<CheckedSourceLoopBodyV1>,
    },
    Exit {
        site: SourceStmtSiteV1,
        record: ResolvedExitRecordV1,
        port: usize,
    },
    /// A child source Loop. Its own verified product owns its condition, body,
    /// ports and Phis; this parent only composes its boundary.
    Loop {
        site: SourceStmtSiteV1,
        product: Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
    },
}

/// The existing terminal walk's immutable FunctionExit observation. This is a
/// source control node, never a Loop port or another cleanup permission.
#[derive(Debug)]
pub(crate) struct SourceLoopReturnV1 {
    pub(crate) site: SourceStmtSiteV1,
    pub(crate) record: ResolvedExitRecordV1,
    pub(crate) terminal: super::super::super::TerminalRelationV1,
    pub(crate) homes: super::super::super::RootHomeExitV1,
    pub(crate) read: Option<CurrentScalarReadV1>,
    pub(crate) source:
        Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
}
impl SourceLoopReturnV1 {
    /// An explicit `return null`: the original literal node under the Return root.
    pub(crate) fn is_null_literal_v1(&self) -> bool {
        use crate::mir::resolved_value_profile::{
            expression_plan::{ExpressionNodeSourceV1 as Node, ExpressionPlanRootV1},
            expression_root::StatementExpressionRoleV1,
        };
        let super::super::super::TerminalRelationV1::Value(terminal) = &self.terminal else { return false; };
        matches!(terminal.returned(), super::super::super::TerminalReturnedSourceV1::NullLiteral)
            && self.read.is_none()
            && self.source.nodes().len() == 1
            && terminal.return_site() == &self.site
            && matches!(self.source.root(), ExpressionPlanRootV1::Statement {
                statement, role: StatementExpressionRoleV1::ReturnValue, value
            } if statement == &self.site && value == terminal.value_site())
            && self.source.nodes()[0].site() == terminal.value_site()
            && matches!(self.source.nodes()[0].source(),
                Node::Literal(crate::mir::resolved_semantics::ResolvedLiteralSourceV1::Null))
    }
    pub(crate) fn integer_literal_value_v1(&self) -> Option<i64> {
        use crate::mir::resolved_value_profile::{
            expression_plan::{ExpressionNodeSourceV1 as Node, ExpressionPlanRootV1},
            expression_root::StatementExpressionRoleV1,
        };
        let value_site = match &self.terminal {
            crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::I64Scalar(terminal) => {
                terminal.value_site()
            }
            crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::IntegerLiteral(terminal) => {
                terminal.value_site()
            }
            _ => return None,
        };
        if self.read.is_some() || self.source.nodes().len() != 1 {
            return None;
        }
        let ExpressionPlanRootV1::Statement {
            statement,
            role: StatementExpressionRoleV1::ReturnValue,
            value,
        } = self.source.root()
        else {
            return None;
        };
        if statement != &self.site || value != value_site || self.source.nodes()[0].site() != value_site {
            return None;
        }
        let Node::Literal(crate::mir::resolved_semantics::ResolvedLiteralSourceV1::Integer(value)) =
            self.source.nodes()[0].source() else { return None; };
        match &self.terminal {
            crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::I64Scalar(_) => Some(*value),
            crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::IntegerLiteral(terminal)
                if terminal.value() == *value => Some(*value),
            _ => None,
        }
    }

    pub(crate) fn corroborates(&self, input: ResolvedFunctionLoweringInputV1<'_>) -> bool {
        use crate::mir::resolved_semantics::{
            ResolvedControlTransferV1 as Transfer, ResolvedExitOriginV1 as Origin,
        };
        use crate::mir::resolved_value_profile::{
            expression_plan::{ExpressionNodeSourceV1 as Node, ExpressionPlanRootV1},
            expression_root::StatementExpressionRoleV1,
        };
        let source_value = match (&self.terminal, &self.read) {
            (
                crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::I64Scalar(terminal),
                Some(read),
            ) => {
                terminal.owner() == input.owner()
                    && terminal.return_site() == &self.site
                    && terminal.value_site() == read.read()
                    && read.binding().owner() == input.owner()
                    && self.source.nodes().len() == 1
                    && matches!(self.source.root(), ExpressionPlanRootV1::Statement {
                        statement, role: StatementExpressionRoleV1::ReturnValue, value
                    } if statement == &self.site && value == read.read())
                    && matches!(self.source.nodes()[0].source(),
                        Node::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding))
                            if *binding == read.binding())
            }
            (
                crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::IntegerLiteral(terminal),
                None,
            ) => {
                terminal.owner() == input.owner()
                    && terminal.return_site() == &self.site
                    && self.integer_literal_value_v1() == Some(terminal.value())
            }
            (
                crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::I64Scalar(terminal),
                None,
            ) => {
                terminal.owner() == input.owner()
                    && terminal.return_site() == &self.site
                    && self.integer_literal_value_v1().is_some()
            }
            (crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::Value(terminal), None) => {
                terminal.owner() == input.owner() && self.is_null_literal_v1()
            }
            _ => false,
        };
        source_value
            && self.source.owner() == input.owner()
            && input.function().resolved_exit(
                &crate::mir::resolved_semantics::ResolvedExitSiteV1::Statement(self.site.clone()),
            ) == Some(&self.record)
            && matches!(
                (self.record.origin(), self.record.transfer()),
                (Origin::ExplicitReturn, Transfer::Return { .. })
            )
    }
}
impl SourceLoopControlDraftV1 {
    fn fail(&mut self, reason: &'static str) {
        self.error.get_or_insert(reason);
    }
    fn action(&mut self, site: &SourceStmtSiteV1, action: Action) {
        if self.actions.insert(site.clone(), action).is_some() {
            self.fail("duplicate source control action");
        }
    }
    pub(super) fn finish(
        mut self,
        root: &SourceBodySiteV1,
    ) -> Result<CheckedSourceLoopBodyV1, &'static str> {
        if let Some(error) = self.error {
            return Err(error);
        }
        let body = self.take_body(root)?;
        if !self.bodies.is_empty() || !self.actions.is_empty() {
            return Err("orphan source control body/action");
        }
        Ok(body)
    }
    fn take_body(
        &mut self,
        site: &SourceBodySiteV1,
    ) -> Result<CheckedSourceLoopBodyV1, &'static str> {
        let index = self
            .bodies
            .iter()
            .position(|body| &body.site == site)
            .ok_or("source control body missing")?;
        let draft = self.bodies.remove(index);
        let terminated = draft.terminated.ok_or("source control body not closed")?;
        if draft.visited.len() > draft.source_length
            || (!terminated && draft.visited.len() != draft.source_length)
        {
            return Err("source control statement coverage");
        }
        let mut statements = Vec::new();
        let mut observed_termination = false;
        for site in draft.visited {
            if observed_termination {
                return Err("source control statement after exit");
            }
            let statement = match self
                .actions
                .remove(&site)
                .ok_or_else(|| {
                    if crate::config::env::builder_debug_enabled() {
                        eprintln!("[loop-return] {site:?} has no checked execution action");
                    }
                    "source statement has no checked execution action"
                })?
            {
                Action::Return(source) => {
                    observed_termination = true;
                    CheckedSourceLoopStatementV1::Return(source)
                }
                Action::Call(observation) => {
                    CheckedSourceLoopStatementV1::Call { site, observation }
                }
                Action::Expression(frame) => {
                    CheckedSourceLoopStatementV1::Expression { site, frame }
                }
                Action::OwnedArrayAppend(observation) => {
                    CheckedSourceLoopStatementV1::OwnedArrayAppend { site, observation }
                }
                Action::Loop(product) => CheckedSourceLoopStatementV1::Loop { site, product },
                Action::Exit { record, port } => {
                    observed_termination = true;
                    CheckedSourceLoopStatementV1::Exit { site, record, port }
                }
                Action::If {
                    bundle,
                    condition,
                    then_body,
                    else_body,
                    then_terminated,
                    else_terminated,
                } => {
                    let then_body = self.take_body(&then_body)?;
                    let else_body = else_body
                        .as_ref()
                        .map(|body| self.take_body(body))
                        .transpose()?;
                    if then_body.terminated != then_terminated
                        || else_body
                            .as_ref()
                            .is_some_and(|body| body.terminated != else_terminated)
                        || (else_body.is_none() && else_terminated)
                    {
                        return Err("source control branch outcome mismatch");
                    }
                    observed_termination = then_terminated && else_terminated;
                    CheckedSourceLoopStatementV1::If {
                        site,
                        bundle,
                        condition,
                        then_body,
                        else_body,
                    }
                }
            };
            statements.push(statement);
        }
        if observed_termination != terminated {
            return Err("source control body outcome mismatch");
        }
        Ok(CheckedSourceLoopBodyV1 {
            site: draft.site,
            statements: statements.into_boxed_slice(),
            terminated,
        })
    }
}

impl PrefixLocalFlow<'_> {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn record_source_loop_return_v1(
        &self,
        site: &SourceStmtSiteV1,
        terminal: Option<&super::super::super::TerminalRelationV1>,
        homes: Option<
            &Result<
                super::super::super::RootHomeExitV1,
                super::super::super::HomePrefixUnavailableV1,
            >,
        >,
        calls: &[super::super::super::LocalCallObservationV1],
    ) {
        let Some(context) = &self.source_loop else {
            return;
        };
        let source = (|| {
            let terminal = terminal?;
            let read = match terminal {
                super::super::super::TerminalRelationV1::I64Scalar(terminal) => {
                    self.current_scalar_read_v1(terminal.value_site(), calls)
                }
                super::super::super::TerminalRelationV1::IntegerLiteral(_) => None,
                super::super::super::TerminalRelationV1::Value(_) => None,
                _ => return None,
            };
            let homes = homes?.as_ref().ok()?;
            let ledger = self
                .input
                .forest()
                .callable_source_ledger(self.input.owner())
                .ok()?;
            let source = crate::mir::resolved_value_profile::expression_plan::statement_expression_source_plan_v1(
                &ledger, self.input.body_shape()?, site).ok()??;
            let record = *self.input.function().resolved_exit(
                &crate::mir::resolved_semantics::ResolvedExitSiteV1::Statement(site.clone()),
            )?;
            let observed = Rc::new(SourceLoopReturnV1 {
                site: site.clone(),
                record,
                terminal: terminal.clone(),
                homes: homes.clone(),
                read,
                source,
            });
            let corroborated = observed.corroborates(self.input);
            if !corroborated && crate::config::env::builder_debug_enabled() {
                eprintln!("[loop-return] {site:?} terminal does not corroborate: null={} read={}",
                    observed.is_null_literal_v1(), observed.read.is_some());
            }
            corroborated.then_some(observed)
        })();
        if source.is_none() && crate::config::env::builder_debug_enabled() {
            eprintln!("[loop-return] {site:?} not recorded: terminal={:?} homes={}",
                terminal.map(std::mem::discriminant), homes.is_some_and(|homes| homes.is_ok()));
        }
        match source {
            Some(source) => context
                .control
                .borrow_mut()
                .action(site, Action::Return(source)),
            None => context
                .control
                .borrow_mut()
                .fail("source statement has no checked execution action"),
        }
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn record_source_loop_call_v1(
        &self,
        site: &SourceStmtSiteV1,
        observation: &super::super::super::LocalCallObservationV1,
    ) {
        let Some(context) = &self.source_loop else {
            return;
        };
        context
            .control
            .borrow_mut()
            .action(site, Action::Call(observation.clone()));
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn record_source_loop_child_v1(
        &self,
        site: &SourceStmtSiteV1,
        product: &Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
    ) {
        if let Some(context) = &self.source_loop {
            context.control.borrow_mut().action(site, Action::Loop(Rc::clone(product)));
        }
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn record_source_loop_array_append_v1(
        &self,
        site: &SourceStmtSiteV1,
        observation: &NamedArrayHomeObservationV1,
    ) {
        if let Some(context) = &self.source_loop {
            context
                .control
                .borrow_mut()
                .action(site, Action::OwnedArrayAppend(observation.clone()));
        }
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn begin_source_loop_body_v1(
        &self,
        body: &LocatedBodyV1<'_>,
    ) {
        let Some(context) = &self.source_loop else {
            return;
        };
        let mut draft = context.control.borrow_mut();
        if body.site().owner() != self.input.owner()
            || !body
                .site()
                .root()
                .segments()
                .starts_with(context.source.site().node().segments())
            || draft
                .bodies
                .iter()
                .any(|existing| &existing.site == body.site())
        {
            draft.fail("duplicate or foreign source control body");
            return;
        }
        draft.bodies.push(BodyDraft {
            site: body.site().clone(),
            source_length: body.statements().len(),
            visited: Vec::new(),
            terminated: None,
        });
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn visit_source_loop_statement_v1(
        &self,
        body: &LocatedBodyV1<'_>,
        site: &SourceStmtSiteV1,
        index: usize,
    ) {
        let Some(context) = &self.source_loop else {
            return;
        };
        let mut draft = context.control.borrow_mut();
        let Some(record) = draft
            .bodies
            .iter_mut()
            .find(|record| &record.site == body.site())
        else {
            draft.fail("unopened source control body");
            return;
        };
        if record.terminated.is_some()
            || record.visited.len() != index
            || self
                .input
                .source()
                .body_stmt(body, index)
                .ok()
                .is_none_or(|statement| statement.site() != site)
        {
            draft.fail("source control statement order");
            return;
        }
        record.visited.push(site.clone());
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn end_source_loop_body_v1(
        &self,
        body: &LocatedBodyV1<'_>,
        terminated: bool,
    ) {
        let Some(context) = &self.source_loop else {
            return;
        };
        let mut draft = context.control.borrow_mut();
        match draft
            .bodies
            .iter_mut()
            .find(|record| &record.site == body.site())
        {
            Some(record) if record.terminated.is_none() => record.terminated = Some(terminated),
            _ => draft.fail("source control body closed twice or missing"),
        }
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn record_source_loop_expression_v1(
        &self,
        site: &SourceStmtSiteV1,
    ) {
        let Some(context) = &self.source_loop else {
            return;
        };
        let plans = self.expression_plans();
        let mut matching = plans.into_iter().filter(|plan| {
            plan.source().root().statement() == site
                && (plan.initializer_frame().is_some()
                    || plan.rebind_frame().is_some()
                    || plan.print_frame().is_some())
        });
        let Some(frame) = matching.next() else {
            context
                .control
                .borrow_mut()
                .fail("source expression has no checked execution root");
            return;
        };
        if matching.next().is_some() {
            context
                .control
                .borrow_mut()
                .fail("duplicate source expression execution root");
            return;
        }
        context
            .control
            .borrow_mut()
            .action(site, Action::Expression(frame));
    }
    #[allow(clippy::too_many_arguments)]
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn record_source_loop_if_v1(
        &self,
        site: &SourceStmtSiteV1,
        bundle: ResolvedIfRegionBundleV1,
        then_body: SourceBodySiteV1,
        else_body: Option<SourceBodySiteV1>,
        then_terminated: bool,
        else_terminated: bool,
        available: bool,
    ) {
        let Some(context) = &self.source_loop else {
            return;
        };
        let mut draft = context.control.borrow_mut();
        if !available
            || self.input.function().if_region_bundle(site).ok() != Some(&bundle)
            || else_body.is_some() != bundle.else_pair().is_some()
        {
            draft.fail("source control If region/availability");
            return;
        }
        let plans = self.expression_plans();
        let mut frames = plans
            .iter()
            .filter_map(|plan| plan.condition_frame())
            .filter(|frame| frame.source().root().statement() == site);
        let Some(frame) = frames.next() else {
            draft.fail("source control If has no checked condition");
            return;
        };
        if frames.next().is_some() {
            draft.fail("source control If duplicate condition");
            return;
        }
        draft.action(
            site,
            Action::If {
                bundle,
                condition: Rc::clone(frame),
                then_body,
                else_body,
                then_terminated,
                else_terminated,
            },
        );
    }
    pub(super) fn record_source_loop_control_exit_v1(
        &self,
        site: &SourceStmtSiteV1,
        record: ResolvedExitRecordV1,
        port: usize,
    ) {
        if let Some(context) = &self.source_loop {
            context
                .control
                .borrow_mut()
                .action(site, Action::Exit { record, port });
        }
    }
}

#[path = "home_prefix_source_loop_verify.rs"]
mod verify;
