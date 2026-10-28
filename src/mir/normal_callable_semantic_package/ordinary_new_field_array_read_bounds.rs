//! Same-array count guard for an original constant-index read. No MIR or ABI.
use super::*;
use crate::mir::resolved_semantics::{BodyStatementShapeV1, BodyEffectKindV1,
    ResolvedBinaryExpressionSourceV1, ResolvedBinaryOperatorV1 as Op,
    ResolvedLiteralSourceV1 as Literal, ResolvedExitSiteV1, ResolvedExitOriginV1,
    ResolvedControlTransferV1, ResolvedAssignmentFormV1, SourcePathSegmentV1 as Path, SourcePathV1,
    SourceStmtSiteV1};

/// Proof that one dynamic `get` index is in range. Each variant is re-checked against
/// the original source; the closed-world census that rules out shrinking or escaping
/// array uses is owned by the consumer that removes the out-of-bounds atom.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum CheckedArrayGetBoundsSourceV1 {
    Guard(GuardBoundsV1),
    Loop(LoopBoundsV1),
}
impl CheckedArrayGetBoundsSourceV1 {
    pub(super) fn matches(&self, input: ResolvedFunctionLoweringInputV1<'_>, get: &CheckedFieldArrayAccessSourceV1) -> bool {
        match self {
            Self::Guard(proof) => proof.matches(input, get),
            Self::Loop(proof) => proof.matches(input, get),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct GuardBoundsV1 {
    guard: SourceStmtSiteV1,
    comparison: ResolvedBinaryExpressionSourceV1,
    count: Rc<CheckedFieldArrayAccessSourceV1>,
}
impl GuardBoundsV1 {
    fn matches(&self, input: ResolvedFunctionLoweringInputV1<'_>, get: &CheckedFieldArrayAccessSourceV1) -> bool {
        let function = input.function();
        let Some(shape) = input.body_shape() else { return false; };
        let Some(call) = function.method_call_source_v1(get.call.site()) else { return false; };
        let Some(index) = call.arguments().first().and_then(|arg| function.expression_source().literal(arg.site()))
            else { return false; };
        let Literal::Integer(index) = index else { return false; };
        let limit = function.expression_source().literal(self.comparison.rhs());
        if *index < 0 || limit != Some(&Literal::Integer(*index))
            || !(self.comparison.operator() == Op::LessEqual
                || self.comparison.operator() == Op::Equal && *index == 0)
            || function.expression_source().binary(self.comparison.site()) != Some(&self.comparison)
            || self.comparison.lhs() != self.count.call.site()
            || !self.count.is_count() || !self.count.matches_original_call_v1(input)
            || self.count.residence != get.residence || self.count.receiver_root != get.receiver_root
        { return false; }
        let guard = self.guard.node();
        let get_site = get.call.site().node();
        let parent_len = guard.segments().len().saturating_sub(1);
        // The guard itself must be a preceding sibling, not a conditional
        // descendant of some earlier sibling that might never execute.
        if guard.segments().is_empty() || !guard.precedes_in_sequence(get_site)
            || get_site.segments().get(..parent_len) != Some(&guard.segments()[..parent_len])
            // A later loop/lambda/container could revisit the get without
            // re-evaluating this guard. Only a direct sibling expression is
            // covered; a guard inside that container may prove its own read.
            || get_site.segments().get(parent_len + 1..).is_none_or(|tail| tail.iter().any(|step|
                !matches!(step, Path::Value | Path::Initializer(_) | Path::Receiver | Path::Argument(_)
                    | Path::Lhs | Path::Rhs | Path::Operand | Path::Element(_) | Path::EntryValue(_)
                    | Path::FieldValue(_) | Path::UpdateValue(_) | Path::Base | Path::Target | Path::CheckItem(_))))
            || SourcePathV1::from_node(guard).child(Path::IfCondition).expr() != *self.comparison.site()
        { return false; }
        let Ok(bundle) = function.if_region_bundle(&self.guard) else { return false; };
        if bundle.else_pair().is_some() { return false; }
        let returned = SourcePathV1::from_node(guard).child(Path::IfThen(0)).stmt();
        let mut then_rows = shape.statements().iter().filter(|row|
            row.site().node().segments().starts_with(guard.segments())
                && matches!(row.site().node().segments().get(guard.segments().len()), Some(Path::IfThen(_))));
        if !matches!(then_rows.next(), Some(BodyStatementShapeV1::Return { site, .. }) if site == &returned)
            || then_rows.next().is_some()
            || function.resolved_exit(&ResolvedExitSiteV1::Statement(returned)).is_none_or(|exit|
                exit.origin() != ResolvedExitOriginV1::ExplicitReturn
                    || exit.source_region() != bundle.then_pair().region()
                    || exit.transfer() != (ResolvedControlTransferV1::Return { target_function: function.function_region() }))
        { return false; }
        shape.effects().iter().all(|effect| {
            let site = effect.site.node();
            if !guard.precedes_in_sequence(site) || get_site.precedes_in_sequence(site) { return true; }
            if effect.kind == BodyEffectKindV1::Call && &effect.site == get.call.site() { return true; }
            // A Return enclosing this read transfers control after evaluation.
            effect.kind == BodyEffectKindV1::NonLocalControl
                && get_site.segments().starts_with(site.segments())
                && shape.statements().iter().any(|row| matches!(row,
                    BodyStatementShapeV1::Return { site: returned, .. } if returned.node() == site))
        })
    }
}

pub(super) fn capture(
    input: ResolvedFunctionLoweringInputV1<'_>, get: &CheckedFieldArrayAccessSourceV1,
    mut count_source: impl FnMut(&SourceExprSiteV1) -> Result<Option<Rc<CheckedFieldArrayAccessSourceV1>>, String>,
) -> Result<Option<CheckedArrayGetBoundsSourceV1>, String> {
    if !get.is_dynamic_get() { return Ok(None); }
    for guard in input.function().if_region_sites() {
        let condition = SourcePathV1::from_node(guard.node()).child(Path::IfCondition).expr();
        let Some(comparison) = input.function().expression_source().binary(&condition) else { continue; };
        if !matches!(comparison.operator(), Op::Equal | Op::LessEqual) { continue; }
        let Some(count) = count_source(comparison.lhs())? else { continue; };
        let witness = GuardBoundsV1 { guard: guard.clone(), comparison: comparison.clone(), count };
        if witness.matches(input, get) { return Ok(Some(CheckedArrayGetBoundsSourceV1::Guard(witness))); }
    }
    Ok(capture_loop(input, get, &mut count_source)?.map(CheckedArrayGetBoundsSourceV1::Loop))
}

/// `loop(i < n) { ... ids.get(i) ... i = i + 1 }` with `n` the never-rebound
/// `ids.length()` of the same array, read before the loop.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct LoopBoundsV1 {
    loop_stmt: SourceStmtSiteV1,
    comparison: ResolvedBinaryExpressionSourceV1,
    index: BindingRefV1,
    limit: BindingRefV1,
    count: Rc<CheckedFieldArrayAccessSourceV1>,
}

fn local_ref(input: ResolvedFunctionLoweringInputV1<'_>, site: &SourceExprSiteV1) -> Option<BindingRefV1> {
    match input.function().variable_ref(site)? { ResolvedLexicalRefV1::Local(binding) => Some(binding), _ => None }
}

/// The nearest enclosing loop of `get`, as the loop statement and the body segment index.
fn enclosing_loop(get: &CheckedFieldArrayAccessSourceV1) -> Option<(SourceStmtSiteV1, usize)> {
    let segments = get.call.site().node().segments();
    let at = segments.iter().rposition(|segment| matches!(segment, Path::LoopBody(_)))?;
    Some((SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(segments[..at].to_vec())), at))
}

fn capture_loop(
    input: ResolvedFunctionLoweringInputV1<'_>, get: &CheckedFieldArrayAccessSourceV1,
    count_source: &mut impl FnMut(&SourceExprSiteV1) -> Result<Option<Rc<CheckedFieldArrayAccessSourceV1>>, String>,
) -> Result<Option<LoopBoundsV1>, String> {
    let Some((loop_stmt, _)) = enclosing_loop(get) else { return Ok(None); };
    let condition = SourcePathV1::from_node(loop_stmt.node()).child(Path::LoopCondition).expr();
    let Some(comparison) = input.function().expression_source().binary(&condition) else { return Ok(None); };
    let (Some(index), Some(limit)) = (local_ref(input, comparison.lhs()), local_ref(input, comparison.rhs())) else { return Ok(None); };
    let Some((_, initializer)) = local_initializer(input, limit) else { return Ok(None); };
    let Some(count) = count_source(&initializer)? else { return Ok(None); };
    let witness = LoopBoundsV1 { loop_stmt, comparison: comparison.clone(), index, limit, count };
    Ok(witness.matches(input, get).then_some(witness))
}

impl LoopBoundsV1 {
    fn matches(&self, input: ResolvedFunctionLoweringInputV1<'_>, get: &CheckedFieldArrayAccessSourceV1) -> bool {
        let function = input.function();
        let Some(shape) = input.body_shape() else { return false; };
        let Some(call) = function.method_call_source_v1(get.call.site()) else { return false; };
        let loop_node = self.loop_stmt.node();
        let loop_len = loop_node.segments().len();
        let get_node = get.call.site().node();
        // The read indexes with the loop variable, directly inside this loop's body.
        if call.arguments().first().and_then(|argument| local_ref(input, argument.site())) != Some(self.index)
            || enclosing_loop(get).as_ref().map(|(stmt, _)| stmt) != Some(&self.loop_stmt)
            || get_node.segments().get(loop_len + 1..).is_none_or(|tail| tail.iter().any(|step| !matches!(step,
                Path::IfThen(_) | Path::IfElse(_) | Path::IfCondition | Path::Value | Path::Initializer(_)
                    | Path::Receiver | Path::Argument(_) | Path::Lhs | Path::Rhs | Path::Operand | Path::Element(_)
                    | Path::EntryValue(_) | Path::FieldValue(_) | Path::UpdateValue(_) | Path::Base | Path::Target
                    | Path::CheckItem(_))))
        { return false; }
        // The header is `index < limit`.
        let condition = SourcePathV1::from_node(loop_node).child(Path::LoopCondition).expr();
        if function.expression_source().binary(&condition) != Some(&self.comparison)
            || self.comparison.operator() != Op::Less
            || local_ref(input, self.comparison.lhs()) != Some(self.index)
            || local_ref(input, self.comparison.rhs()) != Some(self.limit)
        { return false; }
        // The limit is the never-rebound same-array count, read before the loop.
        if local_initializer(input, self.limit).map(|(_, site)| site).as_ref() != Some(self.count.call.site())
            || !self.count.is_count() || !self.count.matches_original_call_v1(input)
            || self.count.residence != get.residence || self.count.receiver_root != get.receiver_root
            || !self.count.call.site().node().precedes_in_sequence(&loop_node)
        { return false; }
        // The index starts at a non-negative literal declared before the loop.
        let mut initializers = function.expression_source().initializers().filter(|row| row.binding() == self.index);
        let Some(row) = initializers.next() else { return false; };
        if initializers.next().is_some() || function.declaration_binding(row.declaration_site()) != Some(self.index) { return false; }
        let Some(start) = row.initializer_site() else { return false; };
        if !matches!(function.expression_source().literal(start), Some(Literal::Integer(value)) if *value >= 0)
            || !start.node().precedes_in_sequence(&loop_node)
        { return false; }
        // Only `index = index + 1`, after the read, in this loop's body; never from a closure.
        if input.forest().semantic_owners().any(|(_, product)| product.assignment_targets().any(|(_, target)|
            matches!(target, ResolvedAssignmentTargetV1::UpvarRebind(upvar) if upvar.source() == self.index))) { return false; }
        function.assignment_targets().all(|(site, target)| {
            if !matches!(target, ResolvedAssignmentTargetV1::BindingRebind(binding) if *binding == self.index) { return true; }
            let Some(assignment) = shape.assignment_sources().iter().find(|row| row.target_site() == site) else { return false; };
            let Some(step) = function.expression_source().binary(assignment.value_site()) else { return false; };
            let statement = assignment.statement_site().node();
            assignment.form() == ResolvedAssignmentFormV1::Plain
                && step.operator() == Op::Add
                && local_ref(input, step.lhs()) == Some(self.index)
                && function.expression_source().literal(step.rhs()) == Some(&Literal::Integer(1))
                && statement.segments().starts_with(loop_node.segments())
                && matches!(statement.segments().get(loop_len), Some(Path::LoopBody(_)))
                && get_node.precedes_in_sequence(statement)
        })
    }
}
