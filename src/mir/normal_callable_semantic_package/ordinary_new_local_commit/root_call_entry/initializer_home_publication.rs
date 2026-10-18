//! One prepared transaction publishes the original prefix and installed Text Home.
//! No source reclassification, second packet pool or fallible work after insertion.
use super::*;

pub(super) fn commit(prepared: PreparedLifecyclePrefixBindingGroupV1<'_>) -> Result<(), String> {
    let ledger = prepared.ledger;
    let expected = ledger.expected_local_call_binding_sources(prepared.owner)?;
    if expected.get(prepared.preceding_sites.len()).is_none_or(|source| !prepared.group.matches_source(source)) {
        return Err(freeze("local-call-binding-preflight-stale"));
    }
    let home = prepared.group.initializer_rc().filter(|packet| packet.frame().is_text_residence()).cloned();
    let mut homes = if home.is_some() { Some(ledger.local_commits.borrow_mut()) } else { None };
    if let (Some(packet), Some(rows)) = (&home, &homes) {
        if !packet.has_text_home() || packet.frame().source().owner() != prepared.owner
            || rows.contains_key(prepared.group.site())
            || rows.values().any(|row| row.binding() == Some(packet.frame().binding())) {
            return Err(freeze("initializer-expression/home-publication-conflict"));
        }
    }
    let mut rows = ledger.root_local_call_bindings.borrow_mut();
    let groups = rows.get(&prepared.owner).map(Vec::as_slice).unwrap_or(&[]);
    if groups.len() != prepared.preceding_sites.len()
        || groups.iter().map(|recorded| recorded.site()).ne(prepared.preceding_sites.iter())
        || groups.iter().any(|recorded| recorded.site() == prepared.group.site()) {
        return Err(freeze("local-call-binding-preflight-stale"));
    }
    // All checks precede both mutations, with both destination maps borrowed.
    if let (Some(packet), Some(homes)) = (home, &mut homes) {
        homes.insert(prepared.group.site().clone(), LocalCommitV1::TextResidence(packet));
    }
    rows.entry(prepared.owner).or_default().push(prepared.group);
    Ok(())
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn validate_text_initializer_home_v1(
        &self, packet: &EmittedInitializerExpressionProjectionV1, context: CallValidationContextV1<'_>,
    ) -> Result<(), String> {
        context.require_ledger(self)?;
        if !packet.frame().is_text_residence() { return Ok(()); }
        let site = OwnedExprSiteV1::new(packet.frame().source().owner(), packet.frame().source().nodes()[0].site().clone());
        let rows = self.local_commits.borrow();
        if !packet.has_text_home() || !matches!(rows.get(&site), Some(LocalCommitV1::TextResidence(original))
            if std::ptr::eq(original.as_ref(), packet))
            || rows.values().filter(|row| row.installs(packet.frame().binding())).count() != 1 {
            return Err(freeze("initializer-expression/published-home-identity"));
        }
        context.with_local_call_groups_v1(site.owner(), |groups| {
            if groups.iter().filter_map(|group| group.initializer())
                .filter(|original| std::ptr::eq(*original, packet)).count() != 1 {
                return Err(freeze("initializer-expression/published-prefix-identity"));
            }
            Ok(())
        })?;
        Ok(())
    }
}
