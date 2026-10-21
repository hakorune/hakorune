//! Original Unit statement demand; no scalar result, LOCAL or executable ABI.
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use super::*;
pub(super) enum UnitDiscardDemandV1 {
    Unselected,
    Refused,
    Ready(LocalCallObservationV1),
}
pub(super) fn demand_v1<E>(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    site: &OwnedExprSiteV1,
    prior_homes: &[LiveHomeV1],
    locals: &PrefixLocalFlow<'_>,
    borrowed_arguments: &mut impl FnMut(&OwnedExprSiteV1, BorrowedCallActualRequestV1<'_>)
        -> Result<Option<BorrowedCallArgumentsV1>, E>,
) -> Result<UnitDiscardDemandV1, E> {
    let Some(response) = borrowed_arguments(site, BorrowedCallActualRequestV1::SourceUnitDiscardArguments)?
        else { return Ok(UnitDiscardDemandV1::Unselected); };
    let BorrowedCallArgumentsV1::SourceUnitDiscard { source, arguments } = response
        else { return Ok(UnitDiscardDemandV1::Refused); };
    if statement.owner() != input.owner() || !matches!(statement.node(), crate::ast::ASTNode::MethodCall { .. })
        || source.call_site() != site || !source.is_unit_discard_source_v1()
        || arguments.len() != source.argument_sites().len()
        || !input.function().method_call(site.site()).is_some_and(|call|
            call.owner() == input.owner() && call.arity() == source.target().arity()
                && call.arguments().iter().map(|argument| argument.site()).eq(source.argument_sites().iter())
                && call.selector() == source.target().name() && call.receiver_site() == source.receiver_site()) {
        return Ok(UnitDiscardDemandV1::Refused);
    }
    if source.borrowed_receiver_source_v1().is_some_and(|loan|
        !locals.borrowed_map_receiver_v1(source.receiver_site(), loan)) {
        return Ok(UnitDiscardDemandV1::Refused);
    }
    Ok(UnitDiscardDemandV1::Ready(LocalCallObservationV1 {
        owner: input.owner(), statement: statement.site().clone(), site: site.clone(),
        destination: LocalCallDestinationV1::Discard, prior_homes: prior_homes.into(),
        arguments, result: LocalCallResultClassV1::Unit,
        unit_source: Some(source), result_source: None, borrowed_nullable: None, mixed: None, field_alias: None,
    }))
}
