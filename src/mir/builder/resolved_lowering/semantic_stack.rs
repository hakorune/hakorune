//! Sealed semantic RegionId/ScopeId stack for canonical lowering.
//!
//! Regions and lexical scopes are deliberately tracked independently. A
//! control-only region (such as statement `If`) must not fabricate a lexical
//! scope, while BlockExpr and branch bodies enter one exact sealed pair.

use std::collections::BTreeSet;

use crate::mir::resolved_semantics::{
    BindingRefV1, RegionId, RegionKindV1, ResolvedFunctionLoweringRootsV1,
    ResolvedScopeRegionPairV1, ScopeId, ScopeKindV1, VerifiedResolvedFunctionV1,
};

use super::identity::ResolvedIdentityStateV1;

#[derive(Debug)]
pub(super) struct ResolvedRegionSessionV1 {
    region: RegionId,
}

#[derive(Debug)]
pub(super) struct ResolvedScopeRegionSessionV1 {
    pair: ResolvedScopeRegionPairV1,
    declarations: Vec<BindingRefV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ResolvedSemanticExpectedCountsV1 {
    block_expr_pairs: usize,
    if_control_regions: usize,
    if_branch_pairs: usize,
}

impl ResolvedSemanticExpectedCountsV1 {
    pub(super) const fn new(
        block_expr_pairs: usize,
        if_control_regions: usize,
        if_branch_pairs: usize,
    ) -> Self {
        Self {
            block_expr_pairs,
            if_control_regions,
            if_branch_pairs,
        }
    }
}

#[derive(Debug)]
pub(super) struct ResolvedSemanticStackV1 {
    root_regions: [RegionId; 2],
    root_scopes: [ScopeId; 2],
    regions: Vec<RegionId>,
    scopes: Vec<ScopeId>,
    consumed_regions: BTreeSet<RegionId>,
    consumed_scopes: BTreeSet<ScopeId>,
    consumed_block_expr_pairs: usize,
    expected_block_expr_pairs: usize,
    consumed_if_control_regions: usize,
    expected_if_control_regions: usize,
    consumed_if_branch_pairs: usize,
    expected_if_branch_pairs: usize,
}

impl ResolvedSemanticStackV1 {
    pub(super) const fn function_region(&self) -> RegionId {
        self.root_regions[0]
    }

    pub(super) fn new(
        product: &VerifiedResolvedFunctionV1,
        roots: ResolvedFunctionLoweringRootsV1,
        expected_block_expr_pairs: usize,
    ) -> Result<Self, String> {
        Self::new_with_expectations(
            product,
            roots,
            ResolvedSemanticExpectedCountsV1::new(expected_block_expr_pairs, 0, 0),
        )
    }

    pub(super) fn new_with_expectations(
        product: &VerifiedResolvedFunctionV1,
        roots: ResolvedFunctionLoweringRootsV1,
        expected: ResolvedSemanticExpectedCountsV1,
    ) -> Result<Self, String> {
        let function = roots.function_pair();
        let body = roots.body_pair();
        verify_root_pair(
            product,
            function,
            ScopeKindV1::Function,
            RegionKindV1::Function,
            None,
            None,
        )?;
        verify_root_pair(
            product,
            body,
            ScopeKindV1::LexicalBlock,
            RegionKindV1::Sequence,
            Some(function.scope()),
            Some(function.region()),
        )?;

        let root_regions = [function.region(), body.region()];
        let root_scopes = [function.scope(), body.scope()];
        Ok(Self {
            root_regions,
            root_scopes,
            regions: root_regions.to_vec(),
            scopes: root_scopes.to_vec(),
            consumed_regions: BTreeSet::new(),
            consumed_scopes: BTreeSet::new(),
            consumed_block_expr_pairs: 0,
            expected_block_expr_pairs: expected.block_expr_pairs,
            consumed_if_control_regions: 0,
            expected_if_control_regions: expected.if_control_regions,
            consumed_if_branch_pairs: 0,
            expected_if_branch_pairs: expected.if_branch_pairs,
        })
    }

    pub(super) fn enter_block_expr(
        &mut self,
        product: &VerifiedResolvedFunctionV1,
        pair: ResolvedScopeRegionPairV1,
    ) -> Result<ResolvedScopeRegionSessionV1, String> {
        let session = self.enter_scope_region(
            product,
            pair,
            ScopeKindV1::BlockExpr,
            RegionKindV1::BlockExpr,
        )?;
        self.consumed_block_expr_pairs += 1;
        Ok(session)
    }

    /// Enters a semantic control region without inventing a lexical scope.
    ///
    /// I1a leaves this disconnected from production syntax. It exists so the
    /// later atomic If activation can consume the already-separated stack.
    pub(super) fn enter_region(
        &mut self,
        product: &VerifiedResolvedFunctionV1,
        region: RegionId,
        expected_kind: RegionKindV1,
    ) -> Result<ResolvedRegionSessionV1, String> {
        let record = product.region(region).ok_or_else(|| {
            "[freeze:contract][canonical_semantic_stack/missing_region]".to_string()
        })?;
        if record.kind() != expected_kind
            || record.parent() != self.regions.last().copied()
            || record.lexical_scope().is_some()
        {
            return Err(
                "[freeze:contract][canonical_semantic_stack/region_contract_mismatch]".to_string(),
            );
        }
        if !self.consumed_regions.insert(region) {
            return Err(
                "[freeze:contract][canonical_semantic_stack/region_reconsumed]".to_string(),
            );
        }
        self.regions.push(region);
        if expected_kind == RegionKindV1::If {
            self.consumed_if_control_regions += 1;
        }
        Ok(ResolvedRegionSessionV1 { region })
    }

    pub(super) fn close_region(&mut self, session: ResolvedRegionSessionV1) -> Result<(), String> {
        pop_exact(
            &mut self.regions,
            session.region,
            "region_leave_without_enter",
            "unbalanced_region_leave",
        )
    }

    pub(super) fn enter_scope_region(
        &mut self,
        product: &VerifiedResolvedFunctionV1,
        pair: ResolvedScopeRegionPairV1,
        expected_scope_kind: ScopeKindV1,
        expected_region_kind: RegionKindV1,
    ) -> Result<ResolvedScopeRegionSessionV1, String> {
        verify_root_pair(
            product,
            pair,
            expected_scope_kind,
            expected_region_kind,
            self.scopes.last().copied(),
            self.regions.last().copied(),
        )?;
        if self.consumed_scopes.contains(&pair.scope())
            || self.consumed_regions.contains(&pair.region())
        {
            return Err("[freeze:contract][canonical_semantic_stack/pair_reconsumed]".to_string());
        }
        self.consumed_scopes.insert(pair.scope());
        self.consumed_regions.insert(pair.region());
        self.scopes.push(pair.scope());
        self.regions.push(pair.region());
        if matches!(
            expected_scope_kind,
            ScopeKindV1::IfThen | ScopeKindV1::IfElse
        ) {
            self.consumed_if_branch_pairs += 1;
        }
        let declarations = product
            .scope(pair.scope())
            .expect("verified pair scope exists")
            .declarations()
            .to_vec();
        Ok(ResolvedScopeRegionSessionV1 { pair, declarations })
    }

    pub(super) fn close_scope_region_success(
        &mut self,
        session: ResolvedScopeRegionSessionV1,
        identity: &mut ResolvedIdentityStateV1<'_>,
    ) -> Result<(), String> {
        self.pop_pair(session.pair)?;
        identity.retire_scope_success(&session.declarations)
    }

    pub(super) fn close_scope_region_error(
        &mut self,
        session: ResolvedScopeRegionSessionV1,
        identity: &mut ResolvedIdentityStateV1<'_>,
    ) -> Result<(), String> {
        self.pop_pair(session.pair)?;
        identity.retire_scope_error(&session.declarations);
        Ok(())
    }

    pub(super) fn finish(&self) -> Result<(), String> {
        if self.regions.as_slice() != self.root_regions.as_slice()
            || self.scopes.as_slice() != self.root_scopes.as_slice()
            || self.consumed_block_expr_pairs != self.expected_block_expr_pairs
            || self.consumed_if_control_regions != self.expected_if_control_regions
            || self.consumed_if_branch_pairs != self.expected_if_branch_pairs
        {
            return Err(format!(
                "[freeze:contract][canonical_semantic_stack/finish_mismatch] region_depth={} scope_depth={} block_expr_pairs={}/{} if_controls={}/{} if_branches={}/{}",
                self.regions.len(),
                self.scopes.len(),
                self.consumed_block_expr_pairs,
                self.expected_block_expr_pairs,
                self.consumed_if_control_regions,
                self.expected_if_control_regions,
                self.consumed_if_branch_pairs,
                self.expected_if_branch_pairs,
            ));
        }
        Ok(())
    }

    #[cfg(test)]
    pub(super) fn depths(&self) -> (usize, usize) {
        (self.regions.len(), self.scopes.len())
    }

    fn pop_pair(&mut self, pair: ResolvedScopeRegionPairV1) -> Result<(), String> {
        if self.regions.last().copied() != Some(pair.region())
            || self.scopes.last().copied() != Some(pair.scope())
        {
            return Err(
                "[freeze:contract][canonical_semantic_stack/unbalanced_pair_leave]".to_string(),
            );
        }
        self.regions.pop();
        self.scopes.pop();
        Ok(())
    }
}

fn verify_root_pair(
    product: &VerifiedResolvedFunctionV1,
    pair: ResolvedScopeRegionPairV1,
    expected_scope_kind: ScopeKindV1,
    expected_region_kind: RegionKindV1,
    expected_scope_parent: Option<ScopeId>,
    expected_region_parent: Option<RegionId>,
) -> Result<(), String> {
    let scope = product
        .scope(pair.scope())
        .ok_or_else(|| "[freeze:contract][canonical_semantic_stack/missing_scope]".to_string())?;
    let region = product
        .region(pair.region())
        .ok_or_else(|| "[freeze:contract][canonical_semantic_stack/missing_region]".to_string())?;
    if scope.kind() != expected_scope_kind
        || region.kind() != expected_region_kind
        || scope.parent() != expected_scope_parent
        || region.parent() != expected_region_parent
        || scope.owner_region() != pair.region()
        || region.lexical_scope() != Some(pair.scope())
    {
        return Err(
            "[freeze:contract][canonical_semantic_stack/pair_contract_mismatch]".to_string(),
        );
    }
    Ok(())
}

fn pop_exact<T: Copy + PartialEq>(
    stack: &mut Vec<T>,
    expected: T,
    empty_tag: &str,
    mismatch_tag: &str,
) -> Result<(), String> {
    match stack.last().copied() {
        Some(actual) if actual == expected => {
            stack.pop();
            Ok(())
        }
        Some(_) => Err(format!(
            "[freeze:contract][canonical_semantic_stack/{mismatch_tag}]"
        )),
        None => Err(format!(
            "[freeze:contract][canonical_semantic_stack/{empty_tag}]"
        )),
    }
}
