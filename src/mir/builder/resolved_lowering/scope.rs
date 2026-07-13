//! Explicit resolved BlockExpr scope/region transaction.

use std::collections::BTreeSet;

use crate::mir::resolved_semantics::{
    BindingRefV1, RegionId, ResolvedScopeRegionPairV1, ScopeId, ScopeKindV1,
    VerifiedResolvedFunctionV1,
};

use super::identity::ResolvedIdentityStateV1;

#[derive(Debug)]
pub(super) struct ResolvedScopeSessionV1 {
    pair: ResolvedScopeRegionPairV1,
    declarations: Vec<BindingRefV1>,
}

#[derive(Debug)]
pub(super) struct ResolvedScopeStateV1 {
    active: Vec<ResolvedScopeRegionPairV1>,
    consumed: BTreeSet<(ScopeId, RegionId)>,
    expected_block_expr_pairs: usize,
}

impl ResolvedScopeStateV1 {
    pub(super) fn new(product: &VerifiedResolvedFunctionV1) -> Self {
        Self {
            active: Vec::new(),
            consumed: BTreeSet::new(),
            expected_block_expr_pairs: product
                .scopes()
                .filter(|(_, scope)| scope.kind() == ScopeKindV1::BlockExpr)
                .count(),
        }
    }

    pub(super) fn enter(
        &mut self,
        product: &VerifiedResolvedFunctionV1,
        pair: ResolvedScopeRegionPairV1,
    ) -> Result<ResolvedScopeSessionV1, String> {
        let scope = product
            .scope(pair.scope())
            .ok_or_else(|| "[freeze:contract][canonical_scope/missing_scope_record]".to_string())?;
        let region = product.region(pair.region()).ok_or_else(|| {
            "[freeze:contract][canonical_scope/missing_region_record]".to_string()
        })?;
        if scope.owner_region() != pair.region() || region.lexical_scope() != Some(pair.scope()) {
            return Err("[freeze:contract][canonical_scope/pair_mismatch]".to_string());
        }
        if let Some(parent) = self.active.last().copied() {
            if scope.parent() != Some(parent.scope()) || region.parent() != Some(parent.region()) {
                return Err("[freeze:contract][canonical_scope/non_lifo_parent]".to_string());
            }
        }
        if !self.consumed.insert((pair.scope(), pair.region())) {
            return Err("[freeze:contract][canonical_scope/pair_reconsumed]".to_string());
        }
        self.active.push(pair);
        Ok(ResolvedScopeSessionV1 {
            pair,
            declarations: scope.declarations().to_vec(),
        })
    }

    pub(super) fn close_success(
        &mut self,
        session: ResolvedScopeSessionV1,
        identity: &mut ResolvedIdentityStateV1<'_>,
    ) -> Result<(), String> {
        self.pop_exact(session.pair)?;
        identity.retire_scope_success(&session.declarations)?;
        Ok(())
    }

    pub(super) fn close_error(
        &mut self,
        session: ResolvedScopeSessionV1,
        identity: &mut ResolvedIdentityStateV1<'_>,
    ) -> Result<(), String> {
        self.pop_exact(session.pair)?;
        identity.retire_scope_error(&session.declarations);
        Ok(())
    }

    pub(super) fn finish(&self) -> Result<(), String> {
        if !self.active.is_empty() || self.consumed.len() != self.expected_block_expr_pairs {
            return Err(format!(
                "[freeze:contract][canonical_scope/finish_mismatch] active={} consumed={}/{}",
                self.active.len(),
                self.consumed.len(),
                self.expected_block_expr_pairs,
            ));
        }
        Ok(())
    }

    fn pop_exact(&mut self, expected: ResolvedScopeRegionPairV1) -> Result<(), String> {
        match self.active.pop() {
            Some(actual) if actual == expected => Ok(()),
            Some(actual) => {
                self.active.push(actual);
                Err("[freeze:contract][canonical_scope/unbalanced_leave]".to_string())
            }
            None => Err("[freeze:contract][canonical_scope/leave_without_enter]".to_string()),
        }
    }
}
