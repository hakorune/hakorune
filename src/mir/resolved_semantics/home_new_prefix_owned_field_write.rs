//! Owned LOCAL assignment qualification, separate from formal-borrow permission.
//! Preserve the exact source frame and update readiness only after complete staging.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use std::rc::Rc;

pub(super) fn observe<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    locals: &mut PrefixLocalFlow<'_>,
    homes: &mut Vec<LiveHomeV1>,
    prefix_known: bool,
    local_calls: &[LocalCallObservationV1],
    source_consult: &mut impl HomeFieldSourceConsultV1<E>,
    fields: &mut impl FnMut(
        &[LocalFieldReadRequestV1],
        LocalFieldReadDemandV1,
    ) -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
) -> Result<Option<bool>, E> {
    let Some(shape) = input.body_shape() else {
        return Ok(None);
    };
    let Some(assignment) = shape
        .assignment_sources()
        .iter()
        .find(|row| row.statement_site() == statement.site())
    else {
        return Ok(None);
    };
    let Some(ResolvedAssignmentTargetV1::FieldWrite { receiver }) =
        input.function().assignment_target(assignment.target_site())
    else {
        return Ok(None);
    };
    let Some(ResolvedLexicalRefV1::Local(root)) = input.function().variable_ref(receiver) else {
        return Ok(None);
    };
    let Some((acquisition, construction)) = locals.owned_allocation_source(root) else {
        return Ok(None);
    };
    let Some(local) = source_consult.owned_local_write(
        input,
        &acquisition,
        root,
        &construction,
        assignment.target_site(),
    )?
    else {
        return Ok(Some(false));
    };
    let Some(frame) = CheckedFieldWriteFrameV1::for_owned_local(local, homes) else {
        return Ok(Some(false));
    };
    if let Some(array) = locals.available_named_array_holder(assignment.value_site()) {
        if !prefix_known {
            return Ok(Some(false));
        }
        let binding = array.binding();
        let Some(plan) =
            super::super::expression_plan::HomeExpressionPlanV1::owned_array_field_write(
                frame, array,
            )
        else {
            return Ok(Some(false));
        };
        let writer = plan
            .field_write_plan()
            .expect("owned Array transfer plan")
            .clone();
        if !locals.retain_expression_plan(plan) || !locals.record_normal_field_write(writer) {
            return Ok(Some(false));
        }
        // This is the source Normal successor. The exact frame retains the
        // precommit Fault Homes; physical commit/End correspondence is still owed.
        locals.consume_home(binding);
        homes.retain(|home| home.binding() != binding);
        return Ok(Some(true));
    }
    let source = Rc::clone(frame.source());
    let signed_reads = super::signed_reads(&source, locals, local_calls, source_consult);
    let finite_reads = super::finite_reads(input, &source, locals, source_consult)?;
    let mut installation = locals.expression_installation();
    let plan = super::super::expression_plan::qualify_field_write_expression_v1(
        source,
        true,
        Some(Rc::clone(&frame)),
        &mut |site| match locals.observe(site) {
            Some(OrdinaryObservation::Integer(_)) => Some(SourceScalarKind::Integer),
            Some(OrdinaryObservation::Bool(_)) => Some(SourceScalarKind::Bool),
            Some(OrdinaryObservation::TrivialLocal(_, kind)) => kind,
            _ => None,
        },
        &mut |site| signed_reads.get(site).cloned(),
        &mut |site| finite_reads.get(site).cloned(),
        &mut |site| super::super::field_read::field_read_request(input, site, locals),
        &mut |plan| installation.approve(plan),
        &mut |requests, demand| Ok(fields(requests, demand)?.is_some()),
    )?;
    let Some(writer) = plan
        .as_ref()
        .and_then(|plan| plan.field_write_plan())
        .cloned()
    else {
        return Ok(Some(false));
    };
    installation.commit();
    Ok(Some(locals.record_normal_field_write(writer)))
}
