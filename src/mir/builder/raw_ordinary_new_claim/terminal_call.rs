//! Terminal i64 Call exit claim for the raw ordinary-new port.
//! The port's trait impl delegates here so the claim file stays under the
//! hard line boundary; the claim itself still runs on the port's own ledger,
//! direct-call loans, and exact statement site.
pub(super) fn emit_terminal_i64_call_exit(
    port: &mut super::super::RawInvocationChildPortV1<'_, '_>,
    builder: &mut crate::mir::MirBuilder,
) -> Result<Option<crate::mir::ValueId>, String> {
    let Some(ledger) = port.ordinary_new_claim_ledger.as_ref() else {
        return Ok(None);
    };
    let owner = port
        .callable_owner_v1()
        .ok_or("[freeze:contract][terminal-call/owner-missing]")?;
    let site = port
        .current_source_site_v1()
        .ok_or("[freeze:contract][terminal-call/site-missing]")?;
    let stmt_site = crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(site.clone());
    if let Some(row) = ledger
        .take_root_instance_call_for_return(owner, &site)
        .map_err(|error| format!("[freeze:contract][terminal-call/{error}]"))?
    {
        let state = port
            .callable_ledger
            .as_ref()
            .ok_or("[freeze:contract][terminal-call/state-missing]")?;
        let receiver = state
            .borrow_mut()
            .take_exact_lexical_value(owner, row.receiver_site().node(), row.receiver_binding())
            .map_err(|error| error.to_string())?;
        return crate::mir::builder::ordinary_new_admission::selected::terminal_call::emit_instance(
            builder,
            &mut state.borrow_mut(),
            ledger,
            owner,
            &stmt_site,
            row,
            receiver,
        )
        .map(Some);
    }
    if ledger.root_instance_call_expected(owner) {
        return Err(
            "[freeze:contract][ordinary-new/local-commit/artifact-source-unavailable]".to_owned(),
        );
    }
    if ledger
        .terminal_call_arguments_for_owner_at(owner, &stmt_site)
        .is_none()
    {
        return Ok(None);
    }
    let Some(loan) = port
        .direct_call_loans
        .as_deref_mut()
        .and_then(|loans| loans.get_mut(owner))
    else {
        // A source terminal Call without the exact direct or instance
        // disposition remains unavailable; do not coerce it into the
        // direct-call loan or synthesize a target.
        return Ok(None);
    };
    let Some(row) = loan
        .take_terminal(ledger, owner, &site)
        .map_err(|error| format!("[freeze:contract][terminal-call/{error:?}]"))?
    else {
        return Ok(None);
    };
    let state = port
        .callable_ledger
        .as_ref()
        .ok_or("[freeze:contract][terminal-call/state-missing]")?;
    crate::mir::builder::ordinary_new_admission::selected::terminal_call::emit(
        builder,
        &mut state.borrow_mut(),
        ledger,
        owner,
        &stmt_site,
        row,
    )
    .map(Some)
}
