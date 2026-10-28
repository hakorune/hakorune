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
    if crate::config::env::builder_debug_enabled() {
        eprintln!("[terminal-call] consulted owner={owner:?} site={stmt_site:?} return-expression={} root-instance-expected={}",
            ledger.return_expression_evaluation_v1(owner, &stmt_site)?.is_some(), ledger.root_instance_call_expected(owner));
    }
    if let Some(source) = ledger.return_expression_evaluation_v1(owner, &stmt_site)? {
        let consume = port.lexical_self_receiver_locator_consumer_v1()?;
        let state = std::rc::Rc::clone(port.callable_ledger.as_ref()
            .ok_or("[freeze:contract][terminal-call/state-missing]")?);
        let ledger = std::rc::Rc::clone(ledger);
        let port = std::cell::RefCell::new(port);
        let issue_static = |builder: &mut crate::mir::MirBuilder,
            original: &std::rc::Rc<crate::mir::normal_callable_semantic_package::StaticIncomingSourceV1>,
            frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
            observation: &crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1| {
            crate::mir::builder::static_result_publication_ingress::take_expression_static_packet_v1(
                &mut port.borrow_mut(), builder, original, frame, observation, &ledger)
        };
        return crate::mir::builder::ordinary_new_admission::selected::emit_return_expression_v1(
            builder, &mut state.borrow_mut(), &ledger, source,
            &consume, Some(&issue_static)).map(Some);
    }
    if let Some(row) = ledger.take_borrowed_lexical_call_for_return_v1(owner, &stmt_site)? {
        port.consume_lexical_self_receiver_locator_v1(&row)?;
        let state = port
            .callable_ledger
            .as_ref()
            .ok_or("[freeze:contract][terminal-call/state-missing]")?;
        return crate::mir::builder::ordinary_new_admission::selected::terminal_call::emit_borrowed_return(
            builder, &mut state.borrow_mut(), ledger, owner, &stmt_site, row,
        ).map(Some);
    }
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
        return Err(format!(
            "[freeze:contract][ordinary-new/local-commit/artifact-source-unavailable] owner={owner:?} site={stmt_site:?}"
        ));
    }
    if let Some((_, terminal)) = ledger.call_source_completion_for_owner_at(owner, &stmt_site) {
        let value_site = terminal.call_site().clone();
        let owned = crate::mir::resolved_semantics::OwnedExprSiteV1::new(owner, value_site);
        if let Some(original) = ledger.selected_static_local_source_v1(&owned)? {
            let ledger = std::rc::Rc::clone(ledger);
            use crate::mir::builder::static_result_publication_ingress::{
                take_terminal_static_publication_v1, StaticResultPublicationIngressV1,
            };
            let consume_receiver = port.lexical_self_receiver_locator_consumer_v1()?;
            let state = std::rc::Rc::clone(port.callable_ledger.as_ref()
                .ok_or("[freeze:contract][static-terminal/state-missing]")?);
            let port = std::cell::RefCell::new(port);
            let ingress = take_terminal_static_publication_v1(
                    &mut port.borrow_mut(),
                    builder.comp_ctx.callable_declaration_catalog().ok(),
                    original.caller(),
                    original.call_site().site(),
                    original.target(),
                )
                .map_err(|error| error.to_string())?;
            let StaticResultPublicationIngressV1::Selected(publication) = ingress else {
                return Err("[freeze:contract][static-terminal/publication-not-selected]".into());
            };
            let packet =
                crate::mir::normal_callable_semantic_package::CallPacketSourceV1::static_i64(
                    original,
                    publication,
                    &ledger,
                )?;
            let issue_static = |builder: &mut crate::mir::MirBuilder,
                nested: &std::rc::Rc<crate::mir::normal_callable_semantic_package::StaticIncomingSourceV1>,
                frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
                observation: &crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1| {
                crate::mir::builder::static_result_publication_ingress::take_expression_static_packet_v1(
                    &mut port.borrow_mut(), builder, nested, frame, observation, &ledger)
            };
            return crate::mir::builder::ordinary_new_admission::selected::terminal_call::emit_static_return(
                builder, &mut state.borrow_mut(), &ledger, owner, &stmt_site, packet,
                &consume_receiver, &issue_static,
            ).map(Some);
        }
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
