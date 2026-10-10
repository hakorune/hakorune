//! Root Static i64 Call from the original source packet and Home exit.
use super::*;
use crate::mir::normal_callable_semantic_package::{
    CallPacketSourceLoanV1, CallPacketSourceV1, PreparedLexicalCallProjectionV1,
};

pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    exit: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    source: CallPacketSourceV1,
) -> Result<ValueId, String> {
    let (completion, terminal) = ledger
        .call_source_completion_for_owner_at(owner, exit)
        .ok_or_else(|| freeze("static-terminal/source-missing"))?;
    let (call_site, observation, original) = match source.loan() {
        CallPacketSourceLoanV1::Static { observation, original, .. } => {
            (source.loan().call_site(), observation, original)
        }
        _ => return Err(freeze("static-terminal/packet-kind")),
    };
    if completion.owner() != owner
        || terminal.owner() != owner
        || terminal.return_site() != exit
        || terminal.call_site() != call_site.site()
        || call_site.owner() != owner
        || observation.owner() != owner
        || observation.site() != call_site
        || !terminal.arguments().is_empty()
    {
        return Err(freeze("static-terminal/source-identity"));
    }
    source.loan().validate_static(ledger)?;
    ledger.claim_static_terminal_root_route_v1(original)?;
    let arguments = observation.arguments().to_vec();
    let mut bindings = Vec::new();
    let prepared: PreparedLexicalCallProjectionV1 = lexical_i64::prepare_arguments_for_source(
        builder,
        state,
        ledger,
        owner,
        None,
        &arguments,
        source.loan(),
        &mut bindings,
    )?;
    let call = prepared.materialize_using(owner, source.loan(), &arguments, Some(ledger))?;
    let value = builder.next_value_id();
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(value, result_type(InvokeCallResultKind::I64, None)?);
    emit_root_home_exit_payload(
        builder,
        state,
        ledger,
        owner,
        exit,
        Some(value),
        value,
        Some(RootExitIngress::Call(Emission {
            source: Source::Packet {
                row: source,
                prepared,
            },
            arguments: bindings,
            call,
            result: InvokeCallResultKind::I64,
        })),
    )
}
