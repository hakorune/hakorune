//! One prepared transaction publishes the original prefix and installed Text Home.
//! No source reclassification, second packet pool or fallible work after insertion.
use super::*;

pub(super) fn commit(prepared: PreparedLifecyclePrefixBindingGroupV1<'_>) -> Result<(), String> {
    let ledger = prepared.ledger;
    let expected = prepared.destination.expected(ledger, prepared.owner)?;
    if expected.get(prepared.preceding_sites.len()).is_none_or(|source| !prepared.group.matches_source(source)) {
        return Err(freeze("local-call-binding-preflight-stale"));
    }
    let home = prepared.group.text_residence_v1();
    let mut homes = if home.is_some() { Some(ledger.local_commits.borrow_mut()) } else { None };
    if let (Some(packet), Some(rows)) = (&home, &homes) {
        if !packet.has_text_home() || packet.owner() != prepared.owner
            || rows.contains_key(prepared.group.site())
            || rows.values().any(|row| row.binding() == Some(packet.binding()) && !matches!(row, LocalCommitV1::TextResidence(_))) {
            return Err(freeze("initializer-expression/home-publication-conflict"));
        }
    }
    let mut rows = ledger.root_local_call_bindings.borrow_mut();
    let mut children = match &prepared.destination {
        source_loop_projection::LifecyclePrefixDestinationV2::Root => None,
        source_loop_projection::LifecyclePrefixDestinationV2::Loop(packet) => {
            if rows.get(&prepared.owner).map_or(0, |groups| groups.iter().filter_map(|group| group.source_loop_rc())
                .filter(|original| std::rc::Rc::ptr_eq(original, packet)).count()) != 1 {
                return Err(freeze("source-loop/publication-parent-stale"));
            }
            Some(packet.children.borrow_mut())
        }
    };
    let groups = match &children {
        Some(groups) => groups.as_slice(),
        None => rows.get(&prepared.owner).map(Vec::as_slice).unwrap_or(&[]),
    };
    let repeated_site = groups.iter().any(|recorded| recorded.site() == prepared.group.site());
    if groups.len() != prepared.preceding_sites.len()
        || groups.iter().map(|recorded| recorded.site()).ne(prepared.preceding_sites.iter())
        || (repeated_site && !prepared.destination.allows_same_site_neighbor_v2(
            &expected,
            groups,
            &prepared.group,
        )) {
        return Err(freeze("local-call-binding-preflight-stale"));
    }
    // All checks precede both mutations, with both destination maps borrowed.
    if let (Some(packet), Some(homes)) = (home, &mut homes) {
        homes.insert(prepared.group.site().clone(), LocalCommitV1::TextResidence(packet));
    }
    match &mut children {
        Some(groups) => groups.push(prepared.group),
        None => rows.entry(prepared.owner).or_default().push(prepared.group),
    }
    Ok(())
}

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn text_rebind_displaced_end_v1(&self, frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedBindingRebindFrameV1)
        -> Result<Option<InvokeOperation>, String> {
        let Some(previous) = frame.previous_text() else { return Ok(None) };
        let operations = self.prior_home_unwind_for(&[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1::Text(previous.clone())])?;
        if operations.len() != 1 || !matches!(operations[0], InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End { .. })) {
            return Err(freeze("rebind-expression/old-text-end"));
        }
        Ok(operations.into_iter().next())
    }
    pub(in crate::mir::normal_callable_semantic_package) fn validate_text_residence_home_v1(
        &self, packet: &TextResidencePacketV1, context: CallValidationContextV1<'_>,
    ) -> Result<(), String> {
        context.require_ledger(self)?;
        let site = packet.site();
        let rows = self.local_commits.borrow();
        if !packet.has_text_home() || !matches!(rows.get(&site), Some(LocalCommitV1::TextResidence(original))
            if original.same_packet(packet)) {
            return Err(freeze("text-residence/published-home-identity"));
        }
        context.with_local_call_group_tree_v2(site.owner(), |groups| {
            if groups.iter().filter_map(|group| group.text_residence_v1())
                .filter(|original| original.same_packet(packet)).count() != 1 {
                return Err(freeze("text-residence/published-prefix-identity"));
            }
            Ok(())
        })?;
        Ok(())
    }
}
