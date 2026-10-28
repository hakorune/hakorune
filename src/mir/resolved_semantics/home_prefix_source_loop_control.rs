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

impl ports::SourceLoopScanSummaryV1 {
    /// Co-seal the SAME source control, expression inventory and exit snapshots
    /// before Recipe assembly. Success alone still grants no Home After.
    pub(crate) fn validate_for_recipe_v1(
        &self,
        input: ResolvedFunctionLoweringInputV1<'_>,
    ) -> Result<(), &'static str> {
        use ports::SourceLoopPortKindV1 as Port;
        if self.refused || self.ports.iter().any(|port| port.unavailable.is_some()) {
            return Err("source Loop has an unavailable path");
        }
        if self
            .source
            .condition_source_v1(input)
            .is_none_or(|source| !Rc::ptr_eq(source, self.condition.source()))
        {
            return Err("source Loop condition identity");
        }
        self.incoming.as_ref().map_err(|reason| *reason)?;
        let conditions = self
            .bool_expressions
            .as_ref()
            .map_err(|_| "source Loop Bool projection refused")?;
        let body = self.control.as_ref().map_err(|reason| *reason)?;
        let mut roots = Vec::new();
        let mut frames = vec![&self.condition];
        let mut exits = BTreeSet::new();
        let argument_ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .map_err(|_| "source Loop condition argument ledger")?;
        let condition_statement = self.condition.source().root().statement();
        let mut expected_condition_arguments = BTreeSet::new();
        for observation in self.condition.calls() {
            let original = argument_ledger
                .method_call_source_v1(observation.site().site())
                .ok_or("source Loop condition call source missing")?;
            if observation.owner() != input.owner()
                || original.owner() != input.owner()
                || !observation.is_expression_value()
                || !observation.result_source().is_some()
            {
                return Err("source Loop condition call identity");
            }
            for argument in observation.arguments() {
                let crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Expression {
                    ordinal, site,
                } = argument else { continue; };
                let key = (observation.site().site().clone(), *ordinal);
                if !expected_condition_arguments.insert(key.clone()) {
                    return Err("source Loop condition argument duplicate");
                }
                let mut matching = self.body_expressions.iter().filter_map(|plan| {
                    let frame = plan.argument_frame()?;
                    let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
                        call, ordinal: found_ordinal, statement,
                    } = frame.source().root() else { return None; };
                    (statement == condition_statement
                        && call.owner() == input.owner()
                        && call.site() == observation.site().site()
                        && *found_ordinal == *ordinal
                        && call.arguments().get(*found_ordinal as usize).is_some_and(|actual| {
                            actual.ordinal() == *found_ordinal && actual.site() == site
                        })
                        && frame.source().nodes().first().is_some_and(|root| root.site() == site)
                        && Rc::ptr_eq(frame.source_loan().plan(), frame.source())
                        && frame.evaluation().reference_coverage_is_exact()
                        && frame.prior_homes() == self.condition.prior_homes()
                        && argument_ledger.method_call_source_v1(call.site())
                            .is_some_and(|canonical| std::sync::Arc::ptr_eq(canonical, call)))
                    .then_some(frame)
                });
                let Some(frame) = matching.next() else {
                    return Err("source Loop condition argument frame missing");
                };
                if matching.next().is_some() {
                    return Err("source Loop condition argument duplicate");
                }
                roots.push(frame.source());
            }
        }
        let mut retained_condition_arguments = BTreeSet::new();
        for plan in self.body_expressions.iter().filter(|plan| plan.argument_frame().is_some()) {
            let frame = plan.argument_frame().ok_or("source Loop condition argument frame")?;
            let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
                call, ordinal, statement,
            } = frame.source().root() else { continue; };
            if statement != condition_statement
                || !self.condition.source().nodes().iter().any(|node| {
                    node.site() == call.site()
                        && matches!(node.source(), crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::MethodCall)
                })
            {
                continue;
            }
            let key = (call.site().clone(), *ordinal);
            if !expected_condition_arguments.contains(&key)
                || !retained_condition_arguments.insert(key)
            {
                return Err("source Loop condition argument identity");
            }
        }
        if retained_condition_arguments != expected_condition_arguments {
            return Err("source Loop condition argument coverage");
        }
        fn visit<'a>(
            body: &'a CheckedSourceLoopBodyV1,
            summary: &'a ports::SourceLoopScanSummaryV1,
            input: ResolvedFunctionLoweringInputV1<'_>,
            roots: &mut Vec<
                &'a Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
            >,
            frames: &mut Vec<&'a Rc<CheckedBoolConditionFrameV1>>,
            exits: &mut BTreeSet<usize>,
        ) -> Result<(), &'static str> {
            for statement in body.statements.iter() {
                match statement {
                    CheckedSourceLoopStatementV1::Return(source) => {
                        if !source.corroborates(input) {
                            return Err("source Loop Exit port identity");
                        }
                    }
                    CheckedSourceLoopStatementV1::OwnedArrayAppend { site, observation } => {
                        let NamedArrayHomeObservationV1::Append {
                            source,
                            call,
                            candidate,
                            availability,
                            prior_homes,
                            ..
                        } = observation
                        else {
                            return Err("source Loop append observation kind");
                        };
                        let ledger = input
                            .forest()
                            .callable_source_ledger(input.owner())
                            .map_err(|_| "source Loop append ledger")?;
                        let original = ledger
                            .method_call(call.site())
                            .ok_or("source Loop append source missing")?;
                        if call.owner() != input.owner() || call.site().node() != site.node()
                            || source.binding().owner() != input.owner() || availability.is_err()
                            || source.append_candidate(call).as_ref() != Some(candidate)
                            || original.receiver() != crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1::Lexical(
                                crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(source.binding()))
                            || !matches!(original.arguments(), [argument] if argument.site() == candidate.site())
                            || ledger.resolved_loop_placement(summary.source.site(), call.site()).ok()
                                != Some(Some(crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Body)) {
                            return Err("source Loop append identity");
                        }
                        // The append's argument frame owns the candidate result;
                        // frames rooted at the candidate call own its operands.
                        let mut ordinals = BTreeSet::new();
                        for plan in summary.body_expressions.iter().filter(|plan| {
                            plan.argument_frame().is_some()
                                && plan.source().root().statement() == site
                        }) {
                            let frame = plan
                                .argument_frame()
                                .ok_or("source Loop append argument frame")?;
                            let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
                                call: argument_call, ordinal, ..
                            } = plan.source().root() else { return Err("source Loop append argument purpose"); };
                            let argument_identity = argument_call.arguments().get(*ordinal as usize)
                                .is_some_and(|argument| argument.ordinal() == *ordinal
                                    && plan.source().nodes().first().is_some_and(|root|
                                        root.site() == argument.site()));
                            let frame_identity = Rc::ptr_eq(frame.source(), plan.source())
                                && Rc::ptr_eq(frame.source_loan().plan(), plan.source())
                                && frame.evaluation().reference_coverage_is_exact();
                            let homes_identity = frame.prior_homes() == prior_homes.as_ref();
                            let append_operand_frame = argument_call.owner() == input.owner()
                                && argument_call.site() == call.site()
                                && *ordinal == 0
                                && ledger.method_call_source_v1(call.site()).is_some_and(
                                    |original| std::sync::Arc::ptr_eq(original, argument_call),
                                )
                                && argument_identity
                                && frame.is_dynamic_carrier_append_v1()
                                && frame.source_loan().dynamic_array_carrier_append_v1()
                                    .is_some_and(|holder| holder == source);
                            let candidate_argument_frame = argument_call.owner() == input.owner()
                                && argument_call.site() == candidate.site()
                                && ledger.method_call_source_v1(candidate.site()).is_some_and(
                                    |original| std::sync::Arc::ptr_eq(original, argument_call),
                                )
                                && argument_identity;
                            let unique_ordinal = ordinals.insert((argument_call.site().clone(), *ordinal));
                            if !((append_operand_frame || candidate_argument_frame) && frame_identity
                                && homes_identity && unique_ordinal)
                            {
                                return Err("source Loop append argument identity");
                            }
                            roots.push(plan.source());
                        }
                    }
                    CheckedSourceLoopStatementV1::Call { site, observation } => {
                        use crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1 as ResultClass;
                        let ledger = input
                            .forest()
                            .callable_source_ledger(input.owner())
                            .map_err(|_| "source Loop call ledger")?;
                        let original = ledger
                            .method_call(observation.site().site())
                            .ok_or("source Loop call source missing")?;
                        if observation.owner() != input.owner()
                            || observation.statement() != site
                            || observation.is_expression_value()
                            || !matches!(
                                observation.result(),
                                ResultClass::I64 | ResultClass::Bool | ResultClass::Unit
                                    | ResultClass::Handle | ResultClass::Nullable
                            )
                            || (observation.result() == ResultClass::Unit
                                && observation.local_binding().is_some())
                            || (matches!(observation.result(), ResultClass::Handle | ResultClass::Nullable)
                                && observation.local_binding().is_none())
                            || original.owner() != input.owner()
                            || original.arguments().len() != observation.arguments().len()
                            || ledger
                                .resolved_loop_placement(
                                    summary.source.site(),
                                    observation.site().site(),
                                )
                                .ok()
                                != Some(Some(
                                    crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Body,
                                ))
                        {
                            return Err("source Loop call identity");
                        }
                        if let Some((declaration, binding)) = observation.local_binding() {
                            if input.function().declaration_binding(declaration) != Some(binding)
                                || !input
                                    .function()
                                    .expression_source()
                                    .initializers()
                                    .any(|row| {
                                        row.declaration_site() == declaration
                                            && row.binding() == binding
                                            && row.initializer_site()
                                                == Some(observation.site().site())
                                    })
                            {
                                return Err("source Loop call initializer identity");
                            }
                        } else if observation.site().site().node() != site.node() {
                            return Err("source Loop call statement identity");
                        }
                        let mut ordinals = BTreeSet::new();
                        for plan in summary.body_expressions.iter().filter(|plan| {
                            plan.argument_frame().is_some()
                                && plan.source().root().statement() == site
                        }) {
                            let frame = plan
                                .argument_frame()
                                .ok_or("source Loop call argument frame")?;
                            let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
                                call, ordinal, ..
                            } = plan.source().root() else { return Err("source Loop call argument purpose"); };
                            let owner_matches = call.owner() == input.owner();
                            let site_matches = call
                                .site()
                                .node()
                                .segments()
                                .starts_with(observation.site().site().node().segments());
                            let ledger_matches = ledger
                                .method_call_source_v1(call.site())
                                .is_some_and(|original| std::sync::Arc::ptr_eq(original, call));
                            let argument_matches = call.arguments().get(*ordinal as usize).is_some_and(|argument| {
                                argument.ordinal() == *ordinal
                                    && plan.source().nodes().first().is_some_and(|root| root.site() == argument.site())
                            });
                            let source_matches = Rc::ptr_eq(frame.source(), plan.source());
                            let loan_matches = Rc::ptr_eq(frame.source_loan().plan(), plan.source());
                            let coverage_exact = frame.evaluation().reference_coverage_is_exact();
                            let prior_homes_match = frame.prior_homes() == observation.prior_homes();
                            let unique_ordinal = ordinals.insert((
                                crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                                    input.owner(),
                                    call.site().clone(),
                                ),
                                *ordinal,
                            ));
                            if !(owner_matches && site_matches && ledger_matches && argument_matches
                                && source_matches && loan_matches && coverage_exact
                                && prior_homes_match && unique_ordinal)
                            {
                                return Err("source Loop call argument identity");
                            }
                            roots.push(plan.source());
                        }
                    }
                    CheckedSourceLoopStatementV1::Expression { site, frame } => {
                        if frame.source().owner() != input.owner()
                            || frame.source().root().statement() != site
                        {
                            return Err("source Loop execution frame identity");
                        }
                        roots.push(frame.source());
                        let argument_ledger = input
                            .forest()
                            .callable_source_ledger(input.owner())
                            .map_err(|_| "source Loop expression argument ledger")?;
                        let mut argument_sites = BTreeSet::new();
                        for plan in summary.body_expressions.iter().filter(|plan| {
                            plan.argument_frame().is_some()
                                && plan.source().root().statement() == site
                        }) {
                            let argument_frame = plan
                                .argument_frame()
                                .ok_or("source Loop expression argument frame")?;
                            let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
                                call,
                                ordinal,
                                ..
                            } = plan.source().root() else {
                                return Err("source Loop expression argument purpose");
                            };
                            let nested_call = frame
                                .source()
                                .nodes()
                                .iter()
                                .filter(|node| {
                                    node.site() == call.site()
                                        && matches!(node.source(),
                                            crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::MethodCall)
                                })
                                .count()
                                == 1;
                            let argument_identity = call
                                .arguments()
                                .get(*ordinal as usize)
                                .is_some_and(|argument| {
                                    argument.ordinal() == *ordinal
                                        && plan
                                            .source()
                                            .nodes()
                                            .first()
                                            .is_some_and(|root| root.site() == argument.site())
                                });
                            let owner_matches = call.owner() == input.owner();
                            let call_is_nested = call
                                .site()
                                .node()
                                .segments()
                                .starts_with(site.node().segments());
                            let ledger_matches = argument_ledger
                                .method_call_source_v1(call.site())
                                .is_some_and(|original| std::sync::Arc::ptr_eq(original, call));
                            let source_matches = Rc::ptr_eq(argument_frame.source(), plan.source());
                            let loan_matches = Rc::ptr_eq(argument_frame.source_loan().plan(), plan.source());
                            let coverage_exact = argument_frame.evaluation().reference_coverage_is_exact();
                            let unique_site = argument_sites.insert((
                                crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                                    input.owner(),
                                    call.site().clone(),
                                ),
                                *ordinal,
                            ));
                            if !owner_matches
                                || !call_is_nested
                                || !ledger_matches
                                || !nested_call
                                || !argument_identity
                                || !source_matches
                                || !loan_matches
                                || !coverage_exact
                                || !unique_site
                            {
                                return Err("source Loop expression argument identity");
                            }
                            roots.push(plan.source());
                        }
                    }
                    CheckedSourceLoopStatementV1::If {
                        site,
                        bundle,
                        condition,
                        then_body,
                        else_body,
                    } => {
                        if input.function().if_region_bundle(site).ok() != Some(bundle)
                            || condition.source().root().statement() != site
                        {
                            return Err("source Loop If identity");
                        }
                        roots.push(condition.source());
                        frames.push(condition);
                        let ledger = input
                            .forest()
                            .callable_source_ledger(input.owner())
                            .map_err(|_| "source Loop condition call ledger")?;
                        for observation in condition.calls() {
                            let original = ledger
                                .method_call(observation.site().site())
                                .ok_or("source Loop condition call source missing")?;
                            if observation.owner() != input.owner()
                                || observation.statement() != site
                                || !observation.is_expression_value()
                                || observation.result_source().is_none()
                                || !matches!(
                                    observation.result(),
                                    crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::I64
                                        | crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Bool
                                )
                                || original.owner() != input.owner()
                                || original.arguments().len() != observation.arguments().len()
                                || ledger
                                    .resolved_loop_placement(
                                        summary.source.site(),
                                        observation.site().site(),
                                    )
                                    .ok()
                                    != Some(Some(
                                        crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Body,
                                    ))
                            {
                                return Err("source Loop condition call identity");
                            }
                            for plan in summary.body_expressions.iter().filter(|plan| {
                                plan.argument_frame().is_some()
                                    && plan.source().root().statement() == site
                            }) {
                                let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
                                    call,
                                    ordinal,
                                    ..
                                } = plan.source().root() else {
                                    return Err("source Loop condition argument purpose");
                                };
                                if call.site() != observation.site().site() {
                                    continue;
                                }
                                let frame = plan
                                    .argument_frame()
                                    .ok_or("source Loop condition argument frame")?;
                                if call.owner() != input.owner()
                                    || ledger
                                        .method_call_source_v1(call.site())
                                        .is_none_or(|source| !std::sync::Arc::ptr_eq(source, call))
                                    || call.arguments().get(*ordinal as usize).is_none_or(
                                        |argument| {
                                            argument.ordinal() != *ordinal
                                                || plan
                                                    .source()
                                                    .nodes()
                                                    .first()
                                                    .is_none_or(|root| root.site() != argument.site())
                                        },
                                    )
                                    || !Rc::ptr_eq(frame.source(), plan.source())
                                    || !Rc::ptr_eq(frame.source_loan().plan(), plan.source())
                                    || !frame.evaluation().reference_coverage_is_exact()
                                    || frame.prior_homes() != observation.prior_homes()
                                {
                                    return Err("source Loop condition argument identity");
                                }
                                roots.push(plan.source());
                            }
                        }
                        let observed_calls: BTreeSet<_> = condition
                            .calls()
                            .iter()
                            .map(|call| call.site().site().clone())
                            .collect();
                        let mut condition_argument_sites = BTreeSet::new();
                        for plan in summary.body_expressions.iter().filter(|plan| {
                            plan.argument_frame().is_some()
                                && plan.source().root().statement() == site
                        }) {
                            let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
                                call,
                                ordinal,
                                ..
                            } = plan.source().root() else {
                                return Err("source Loop condition argument purpose");
                            };
                            if observed_calls.contains(call.site()) {
                                continue;
                            }
                            let argument_frame = plan
                                .argument_frame()
                                .ok_or("source Loop condition argument frame")?;
                            let nested_call = condition
                                .source()
                                .nodes()
                                .iter()
                                .filter(|node| {
                                    node.site() == call.site()
                                        && matches!(node.source(),
                                            crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::MethodCall)
                                })
                                .count()
                                == 1;
                            let argument_identity = call
                                .arguments()
                                .get(*ordinal as usize)
                                .is_some_and(|argument| {
                                    argument.ordinal() == *ordinal
                                        && plan
                                            .source()
                                            .nodes()
                                            .first()
                                            .is_some_and(|root| root.site() == argument.site())
                                });
                            if call.owner() != input.owner()
                                || !call
                                    .site()
                                    .node()
                                    .segments()
                                    .starts_with(site.node().segments())
                                || ledger
                                    .method_call_source_v1(call.site())
                                    .is_none_or(|original| !std::sync::Arc::ptr_eq(original, call))
                                || !nested_call
                                || !argument_identity
                                || !Rc::ptr_eq(argument_frame.source(), plan.source())
                                || !Rc::ptr_eq(argument_frame.source_loan().plan(), plan.source())
                                || !argument_frame.evaluation().reference_coverage_is_exact()
                                || argument_frame.prior_homes() != condition.prior_homes()
                                || !condition_argument_sites.insert((
                                    crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                                        input.owner(),
                                        call.site().clone(),
                                    ),
                                    *ordinal,
                                ))
                            {
                                return Err("source Loop condition nested argument identity");
                            }
                            roots.push(plan.source());
                        }
                        visit(then_body, summary, input, roots, frames, exits)?;
                        if let Some(body) = else_body {
                            visit(body, summary, input, roots, frames, exits)?;
                        }
                    }
                    CheckedSourceLoopStatementV1::Exit { site, record, port } => {
                        let original = input.function().resolved_exit(
                            &crate::mir::resolved_semantics::ResolvedExitSiteV1::Statement(
                                site.clone(),
                            ),
                        );
                        let snapshot = summary
                            .ports
                            .get(*port)
                            .ok_or("source Loop Exit port missing")?;
                        let target = input
                            .function()
                            .loop_region_bundle(summary.source.site())
                            .map_err(|_| "source Loop Exit target missing")?
                            .loop_pair()
                            .region();
                        if !matches!(record.transfer(),
                            crate::mir::resolved_semantics::ResolvedControlTransferV1::Break { target_loop }
                            | crate::mir::resolved_semantics::ResolvedControlTransferV1::Continue { target_loop } if target_loop == target)
                        {
                            return Err("source Loop Exit target identity");
                        }
                        if original != Some(record)
                            || !exits.insert(*port)
                            || !matches!((&snapshot.kind, record.transfer()),
                            (Port::Break(original), crate::mir::resolved_semantics::ResolvedControlTransferV1::Break { .. })
                            | (Port::Continue(original), crate::mir::resolved_semantics::ResolvedControlTransferV1::Continue { .. }) if original == site)
                        {
                            return Err("source Loop Exit port identity");
                        }
                    }
                }
            }
            Ok(())
        }
        visit(body, self, input, &mut roots, &mut frames, &mut exits)?;
        if roots.len() != self.body_expressions.len()
            || roots.iter().any(|source| {
                self.body_expressions
                    .iter()
                    .filter(|plan| Rc::ptr_eq(source, plan.source()))
                    .count()
                    != 1
            })
            || self.body_expressions.iter().any(|plan| {
                roots
                    .iter()
                    .filter(|source| Rc::ptr_eq(source, plan.source()))
                    .count()
                    != 1
            })
        {
            return Err("source Loop execution root census");
        }
        if frames.len() != conditions.len()
            || frames.iter().any(|frame| {
                conditions
                    .iter()
                    .filter(|condition| condition.matches_frame(frame))
                    .count()
                    != 1
            })
        {
            return Err("source Loop Bool frame census");
        }
        let mut backedges = 0;
        for (index, port) in self.ports.iter().enumerate() {
            match port.kind {
                Port::Backedge => backedges += 1,
                _ if exits.contains(&index) => (),
                _ => return Err("source Loop Exit census"),
            }
        }
        if backedges != usize::from(!body.terminated) {
            return Err("source Loop normal port census");
        }
        Ok(())
    }
}
