//! Same-array count guard for an original constant-index read. No MIR or ABI.
use super::*;
use crate::mir::resolved_semantics::{BodyStatementShapeV1, BodyEffectKindV1,
    ResolvedBinaryExpressionSourceV1, ResolvedBinaryOperatorV1 as Op,
    ResolvedLiteralSourceV1 as Literal, ResolvedExitSiteV1, ResolvedExitOriginV1,
    ResolvedControlTransferV1, SourcePathSegmentV1 as Path, SourcePathV1, SourceStmtSiteV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CheckedArrayGetBoundsSourceV1 {
    guard: SourceStmtSiteV1,
    comparison: ResolvedBinaryExpressionSourceV1,
    count: Rc<CheckedFieldArrayAccessSourceV1>,
}
impl CheckedArrayGetBoundsSourceV1 {
    pub(super) fn matches(&self, input: ResolvedFunctionLoweringInputV1<'_>, get: &CheckedFieldArrayAccessSourceV1) -> bool {
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
        let witness = CheckedArrayGetBoundsSourceV1 { guard: guard.clone(), comparison: comparison.clone(), count };
        if witness.matches(input, get) { return Ok(Some(witness)); }
    }
    Ok(None)
}
