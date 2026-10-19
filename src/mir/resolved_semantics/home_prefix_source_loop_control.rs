//! Retain decisions made by the existing recursive source walk. No AST
//! classifier, route Recipe expansion, joined-state reconstruction or keys.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1;
use crate::mir::compiler::located::{LocatedBodyV1, SourceBodySiteV1};
use crate::mir::resolved_semantics::{ResolvedIfRegionBundleV1, ResolvedExitRecordV1, SourceStmtSiteV1};
use super::super::super::{HomeExpressionPlanV1, CheckedBoolConditionFrameV1};

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
    Expression(HomeExpressionPlanV1),
    OwnedArrayAppend(NamedArrayHomeObservationV1),
    If { bundle: ResolvedIfRegionBundleV1, condition: Rc<CheckedBoolConditionFrameV1>,
        then_body: SourceBodySiteV1, else_body: Option<SourceBodySiteV1>, then_terminated: bool, else_terminated: bool },
    Exit { record: ResolvedExitRecordV1, port: usize },
}
#[derive(Debug)]
pub(crate) struct CheckedSourceLoopBodyV1 {
    pub(crate) site: SourceBodySiteV1,
    pub(crate) statements: Box<[CheckedSourceLoopStatementV1]>,
    pub(crate) terminated: bool,
}
#[derive(Debug)]
pub(crate) enum CheckedSourceLoopStatementV1 {
    Expression { site: SourceStmtSiteV1, frame: HomeExpressionPlanV1 },
    OwnedArrayAppend { site: SourceStmtSiteV1, observation: NamedArrayHomeObservationV1 },
    If { site: SourceStmtSiteV1, bundle: ResolvedIfRegionBundleV1,
        condition: Rc<CheckedBoolConditionFrameV1>, then_body: CheckedSourceLoopBodyV1,
        else_body: Option<CheckedSourceLoopBodyV1> },
    Exit { site: SourceStmtSiteV1, record: ResolvedExitRecordV1, port: usize },
}
impl SourceLoopControlDraftV1 {
    fn fail(&mut self, reason: &'static str) { self.error.get_or_insert(reason); }
    fn action(&mut self, site: &SourceStmtSiteV1, action: Action) {
        if self.actions.insert(site.clone(), action).is_some() { self.fail("duplicate source control action"); }
    }
    pub(super) fn finish(mut self, root: &SourceBodySiteV1) -> Result<CheckedSourceLoopBodyV1, &'static str> {
        if let Some(error) = self.error { return Err(error); }
        let body = self.take_body(root)?;
        if !self.bodies.is_empty() || !self.actions.is_empty() { return Err("orphan source control body/action"); }
        Ok(body)
    }
    fn take_body(&mut self, site: &SourceBodySiteV1) -> Result<CheckedSourceLoopBodyV1, &'static str> {
        let index = self.bodies.iter().position(|body| &body.site == site).ok_or("source control body missing")?;
        let draft = self.bodies.remove(index);
        let terminated = draft.terminated.ok_or("source control body not closed")?;
        if draft.visited.len() > draft.source_length || (!terminated && draft.visited.len() != draft.source_length) {
            return Err("source control statement coverage");
        }
        let mut statements = Vec::new();
        let mut observed_termination = false;
        for site in draft.visited {
            if observed_termination { return Err("source control statement after exit"); }
            let statement = match self.actions.remove(&site).ok_or("source statement has no checked execution action")? {
                Action::Expression(frame) => CheckedSourceLoopStatementV1::Expression { site, frame },
                Action::OwnedArrayAppend(observation) => CheckedSourceLoopStatementV1::OwnedArrayAppend { site, observation },
                Action::Exit { record, port } => {
                    observed_termination = true;
                    CheckedSourceLoopStatementV1::Exit { site, record, port }
                }
                Action::If { bundle, condition, then_body, else_body, then_terminated, else_terminated } => {
                    let then_body = self.take_body(&then_body)?;
                    let else_body = else_body.as_ref().map(|body| self.take_body(body)).transpose()?;
                    if then_body.terminated != then_terminated
                        || else_body.as_ref().is_some_and(|body| body.terminated != else_terminated)
                        || (else_body.is_none() && else_terminated) {
                        return Err("source control branch outcome mismatch");
                    }
                    observed_termination = then_terminated && else_terminated;
                    CheckedSourceLoopStatementV1::If { site, bundle, condition, then_body, else_body }
                }
            };
            statements.push(statement);
        }
        if observed_termination != terminated { return Err("source control body outcome mismatch"); }
        Ok(CheckedSourceLoopBodyV1 { site: draft.site, statements: statements.into_boxed_slice(), terminated })
    }
}

impl PrefixLocalFlow<'_> {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn record_source_loop_array_append_v1(
        &self, site: &SourceStmtSiteV1, observation: &NamedArrayHomeObservationV1,
    ) {
        if let Some(context) = &self.source_loop {
            context.control.borrow_mut().action(site, Action::OwnedArrayAppend(observation.clone()));
        }
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn begin_source_loop_body_v1(&self, body: &LocatedBodyV1<'_>) {
        let Some(context) = &self.source_loop else { return };
        let mut draft = context.control.borrow_mut();
        if body.site().owner() != self.input.owner()
            || !body.site().root().segments().starts_with(context.source.site().node().segments())
            || draft.bodies.iter().any(|existing| &existing.site == body.site()) {
            draft.fail("duplicate or foreign source control body"); return;
        }
        draft.bodies.push(BodyDraft { site: body.site().clone(), source_length: body.statements().len(),
            visited: Vec::new(), terminated: None });
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn visit_source_loop_statement_v1(
        &self, body: &LocatedBodyV1<'_>, site: &SourceStmtSiteV1, index: usize,
    ) {
        let Some(context) = &self.source_loop else { return };
        let mut draft = context.control.borrow_mut();
        let Some(record) = draft.bodies.iter_mut().find(|record| &record.site == body.site()) else {
            draft.fail("unopened source control body"); return;
        };
        if record.terminated.is_some() || record.visited.len() != index
            || self.input.source().body_stmt(body, index).ok().is_none_or(|statement| statement.site() != site) {
            draft.fail("source control statement order"); return;
        }
        record.visited.push(site.clone());
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn end_source_loop_body_v1(&self, body: &LocatedBodyV1<'_>, terminated: bool) {
        let Some(context) = &self.source_loop else { return };
        let mut draft = context.control.borrow_mut();
        match draft.bodies.iter_mut().find(|record| &record.site == body.site()) {
            Some(record) if record.terminated.is_none() => record.terminated = Some(terminated),
            _ => draft.fail("source control body closed twice or missing"),
        }
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn record_source_loop_expression_v1(&self, site: &SourceStmtSiteV1) {
        let Some(context) = &self.source_loop else { return };
        let plans = self.expression_plans();
        let mut matching = plans.into_iter().filter(|plan| plan.source().root().statement() == site
            && (plan.initializer_frame().is_some() || plan.rebind_frame().is_some() || plan.print_frame().is_some()));
        let Some(frame) = matching.next() else { context.control.borrow_mut().fail("source expression has no checked execution root"); return };
        if matching.next().is_some() { context.control.borrow_mut().fail("duplicate source expression execution root"); return; }
        context.control.borrow_mut().action(site, Action::Expression(frame));
    }
    #[allow(clippy::too_many_arguments)]
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn record_source_loop_if_v1(
        &self, site: &SourceStmtSiteV1, bundle: ResolvedIfRegionBundleV1,
        then_body: SourceBodySiteV1, else_body: Option<SourceBodySiteV1>,
        then_terminated: bool, else_terminated: bool, available: bool,
    ) {
        let Some(context) = &self.source_loop else { return };
        let mut draft = context.control.borrow_mut();
        if !available || self.input.function().if_region_bundle(site).ok() != Some(&bundle)
            || else_body.is_some() != bundle.else_pair().is_some() {
            draft.fail("source control If region/availability"); return;
        }
        let plans = self.expression_plans();
        let mut frames = plans.iter().filter_map(|plan| plan.condition_frame())
            .filter(|frame| frame.source().root().statement() == site);
        let Some(frame) = frames.next() else { draft.fail("source control If has no checked condition"); return };
        if frames.next().is_some() { draft.fail("source control If duplicate condition"); return; }
        draft.action(site, Action::If { bundle, condition: Rc::clone(frame), then_body, else_body, then_terminated, else_terminated });
    }
    pub(super) fn record_source_loop_control_exit_v1(&self, site: &SourceStmtSiteV1, record: ResolvedExitRecordV1, port: usize) {
        if let Some(context) = &self.source_loop {
            context.control.borrow_mut().action(site, Action::Exit { record, port });
        }
    }
}

impl ports::SourceLoopScanSummaryV1 {
    /// Co-seal the SAME source control, expression inventory and exit snapshots
    /// before Recipe assembly. Success alone still grants no Home After.
    pub(crate) fn validate_for_recipe_v1(
        &self, input: ResolvedFunctionLoweringInputV1<'_>,
    ) -> Result<(), &'static str> {
        use ports::SourceLoopPortKindV1 as Port;
        if self.refused || self.ports.iter().any(|port| port.unavailable.is_some()) {
            return Err("source Loop has an unavailable path");
        }
        if self.source.condition_source_v1(input).is_none_or(|source| !Rc::ptr_eq(source, self.condition.source())) {
            return Err("source Loop condition identity");
        }
        self.incoming.as_ref().map_err(|reason| *reason)?;
        let conditions = self.bool_expressions.as_ref().map_err(|_| "source Loop Bool projection refused")?;
        let body = self.control.as_ref().map_err(|reason| *reason)?;
        let mut roots = Vec::new();
        let mut frames = vec![&self.condition];
        let mut exits = BTreeSet::new();
        fn visit<'a>(body: &'a CheckedSourceLoopBodyV1, summary: &ports::SourceLoopScanSummaryV1,
            input: ResolvedFunctionLoweringInputV1<'_>,
            roots: &mut Vec<&'a Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>>,
            frames: &mut Vec<&'a Rc<CheckedBoolConditionFrameV1>>, exits: &mut BTreeSet<usize>,
        ) -> Result<(), &'static str> {
            for statement in body.statements.iter() {
                match statement {
                    CheckedSourceLoopStatementV1::OwnedArrayAppend { site, observation } => {
                        let NamedArrayHomeObservationV1::Append { source, call, candidate, availability, .. } = observation
                            else { return Err("source Loop append observation kind"); };
                        let ledger = input.forest().callable_source_ledger(input.owner())
                            .map_err(|_| "source Loop append ledger")?;
                        let original = ledger.method_call(call.site()).ok_or("source Loop append source missing")?;
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
                    }
                    CheckedSourceLoopStatementV1::Expression { site, frame } => {
                        if frame.source().owner() != input.owner() || frame.source().root().statement() != site {
                            return Err("source Loop execution frame identity");
                        }
                        roots.push(frame.source());
                    }
                    CheckedSourceLoopStatementV1::If { site, bundle, condition, then_body, else_body } => {
                        if input.function().if_region_bundle(site).ok() != Some(bundle)
                            || condition.source().root().statement() != site {
                            return Err("source Loop If identity");
                        }
                        roots.push(condition.source()); frames.push(condition);
                        visit(then_body, summary, input, roots, frames, exits)?;
                        if let Some(body) = else_body { visit(body, summary, input, roots, frames, exits)?; }
                    }
                    CheckedSourceLoopStatementV1::Exit { site, record, port } => {
                        let original = input.function().resolved_exit(&crate::mir::resolved_semantics::ResolvedExitSiteV1::Statement(site.clone()));
                        let snapshot = summary.ports.get(*port).ok_or("source Loop Exit port missing")?;
                        let target = input.function().loop_region_bundle(summary.source.site())
                            .map_err(|_| "source Loop Exit target missing")?.loop_pair().region();
                        if !matches!(record.transfer(),
                            crate::mir::resolved_semantics::ResolvedControlTransferV1::Break { target_loop }
                            | crate::mir::resolved_semantics::ResolvedControlTransferV1::Continue { target_loop } if target_loop == target) {
                            return Err("source Loop Exit target identity");
                        }
                        if original != Some(record) || !exits.insert(*port) || !matches!((&snapshot.kind, record.transfer()),
                            (Port::Break(original), crate::mir::resolved_semantics::ResolvedControlTransferV1::Break { .. })
                            | (Port::Continue(original), crate::mir::resolved_semantics::ResolvedControlTransferV1::Continue { .. }) if original == site) {
                            return Err("source Loop Exit port identity");
                        }
                    }
                }
            }
            Ok(())
        }
        visit(body, self, input, &mut roots, &mut frames, &mut exits)?;
        if roots.len() != self.body_expressions.len() || roots.iter().any(|source|
            self.body_expressions.iter().filter(|plan| Rc::ptr_eq(source, plan.source())).count() != 1)
            || self.body_expressions.iter().any(|plan| roots.iter().filter(|source| Rc::ptr_eq(source, plan.source())).count() != 1) {
            return Err("source Loop execution root census");
        }
        if frames.len() != conditions.len() || frames.iter().any(|frame|
            conditions.iter().filter(|condition| condition.matches_frame(frame)).count() != 1) {
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
        if backedges != usize::from(!body.terminated) { return Err("source Loop normal port census"); }
        Ok(())
    }
}
