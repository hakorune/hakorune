//! Raw invocation consumer for the bounded source-bound static result row.
//!
//! This is kept separate from source transport so the transport module remains
//! a locator-only box.  The hook resolves one canonical target, consumes one
//! handoff, emits one physical receipt, and publishes its result type once.

use crate::mir::builder::calls::emit_static_global_value_terminal_with_receipt_v1;
use crate::mir::builder::calls::static_result_publication::PreparedStaticCallResultPublicationV1;
use crate::mir::builder::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1,
};
use crate::mir::builder::recursive_child_lowering::RawInvocationChildPortV1;
use crate::mir::builder::{MirBuilder, SameModuleCallableNamespaceV1, ValueId};

pub(super) fn try_emit_source_bound_static_call_result_v1(
    port: &mut RawInvocationChildPortV1<'_, '_>,
    builder: &mut MirBuilder,
    owner: &str,
    method: &str,
    checked_source_arity: u32,
    arguments: &[ValueId],
) -> Result<Option<ValueId>, String> {
    let Some(source) = port.active_source.as_ref() else {
        return Ok(None);
    };
    let RawInvocationSourceContextV1::Located {
        root: RawInvocationRootLineageV1::Cataloged(caller),
        site,
        ..
    } = source
    else {
        return Ok(None);
    };
    let Ok(declarations) = builder.comp_ctx.callable_declaration_catalog() else {
        return Ok(None);
    };
    let Some(target) = declarations
        .declaration_for(
            SameModuleCallableNamespaceV1::StaticBoxMethod,
            owner,
            method,
            checked_source_arity as usize,
        )
        .map(|declaration| declaration.key().clone())
    else {
        return Ok(None);
    };
    let source_site = crate::mir::resolved_semantics::SourceExprSiteV1::from_node(site.clone());
    let handoff = port
        .module_port
        .take_static_result_publication_handoff(declarations, caller, &source_site, &target)
        .map_err(|error| format!("[freeze:contract][static-result-owner/take] {error:?}"))?;
    let Some(handoff) = handoff else {
        return Ok(None);
    };
    let (demand, _required_i64_arguments) = handoff.consume();
    let emission = emit_static_global_value_terminal_with_receipt_v1(
        builder,
        port,
        owner,
        method,
        checked_source_arity,
        arguments.to_vec(),
    )
    .map_err(|error| format!("[freeze:contract][static-result-owner/emit] {error:?}"))?;
    let publication = PreparedStaticCallResultPublicationV1::prepare(demand, emission);
    let destination = publication.destination();
    publication.commit(builder)?;
    Ok(Some(destination))
}
