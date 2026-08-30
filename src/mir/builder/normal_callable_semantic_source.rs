//! Atomic semantic source authority for one selected callable batch.

use crate::ast::ASTNode;
use crate::mir::compiler::callable_single_loop_recipe_coseal::VerifiedCallableSingleLoopRecipeProductV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{
    forest_has_unissued_direct_call_observation_v1, CallableSemanticSourceLedgerView,
    FunctionOwnerIdV1, FunctionSemanticResolverSessionV1, ResolveSelectedCallableForestsOutcomeV1,
    VerifiedSemanticOwnerForestV1,
};

use super::callable_declaration_catalog::{
    SameModuleCallableCatalogBrandV1, SelectedNormalCallableKeyV1,
    SelectedNormalCallableSourceSiteV1, VerifiedSelectedNormalCallableSourceInventoryV1,
};
use super::normal_callable_semantic_source_lookup::{function_at_site, view_for_key};

#[derive(Debug)]
struct VerifiedNormalCallableSemanticSourceRowV1 {
    key: SelectedNormalCallableKeyV1,
    site: SelectedNormalCallableSourceSiteV1,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
}

#[derive(Debug)]
pub(in crate::mir) struct VerifiedNormalCallableSemanticSourceV1<'source> {
    program: &'source ASTNode,
    catalog_brand: SameModuleCallableCatalogBrandV1,
    rows: Box<[VerifiedNormalCallableSemanticSourceRowV1]>,
}

#[cfg(test)]
#[path = "normal_callable_prepared_ingress_tests.rs"]
mod normal_callable_prepared_ingress_tests;

pub(in crate::mir) struct VerifiedNormalCallableSemanticLoanV1<'source, 'loan> {
    pub(super) lineage: super::raw_invocation_source_transport::RawInvocationRootLineageV1,
    pub(super) catalog_brand: SameModuleCallableCatalogBrandV1,
    _function: &'source ASTNode,
    pub(super) source_ingress: VerifiedNormalCallableSourceIngressReceiptV1<'loan>,
}

/// Exact source-only ingress carried by an already-issued callable loan.
///
/// This is a transport receipt over the resolver forest/projection owners. It
/// is intentionally not a Recipe, Prepared physicalization, or Builder state;
/// the future ingress may consume it once, while the current raw host simply
/// drops it after preserving its existing behavior.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedNormalCallableSourceIngressReceiptV1<'source> {
    input: ResolvedFunctionLoweringInputV1<'source>,
    ledger: CallableSemanticSourceLedgerView<'source>,
}

/// One-shot, Builder-free assembly of the exact callable source receipt and
/// the already-issued logical Loop product.
///
/// This is deliberately narrower than a physicalization request: it carries
/// no ABI, completion, CFG, SSA, PHI, ValueId, BasicBlockId, selector, or
/// publication state.  The source receipt and logical product are consumed
/// together so a later row cannot accidentally retain two independent owners.
#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedCallableLoopIngressV1<'source> {
    source: VerifiedNormalCallableSourceIngressReceiptV1<'source>,
    logical: VerifiedCallableSingleLoopRecipeProductV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum PreparedCallableLoopIngressRejectV1 {
    SourceLoopIdentityUnavailable,
    SourceOwnerMismatch,
    LogicalCoreOwnerMismatch,
    LogicalPreludeOwnerMismatch,
    LogicalTailOwnerMismatch,
    LogicalContinuationOwnerMismatch,
    LogicalContextOwnerMismatch,
    LogicalOriginMismatch,
    LogicalSourceKindMismatch,
    LogicalLoopSiteMismatch,
    LogicalFrameMismatch,
    LogicalScopeRegionMismatch,
}

impl<'source> VerifiedNormalCallableSourceIngressReceiptV1<'source> {
    pub(in crate::mir) const fn input(&self) -> ResolvedFunctionLoweringInputV1<'_> {
        self.input
    }

    pub(in crate::mir) const fn ledger(&self) -> &CallableSemanticSourceLedgerView<'_> {
        &self.ledger
    }

    pub(in crate::mir) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.input.owner()
    }

    /// Test-only bridge for an already resolved callable-module input.  The
    /// physical canary must use the exact input/index/header owner pair; this
    /// helper does not resolve source or issue a second semantic owner.
    #[cfg(test)]
    pub(in crate::mir::builder) fn from_resolved_input_for_test(
        input: ResolvedFunctionLoweringInputV1<'source>,
    ) -> Result<Self, String> {
        let ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .map_err(|error| format!("callable source ledger: {error:?}"))?;
        Ok(Self { input, ledger })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum NormalCallableSemanticAdmissionRejectV1 {
    UnissuedDirectCallObservation,
}

#[derive(Debug)]
pub(in crate::mir) enum NormalCallableSemanticAdmissionV1<'source> {
    Complete(VerifiedNormalCallableSemanticSourceV1<'source>),
    Deferred,
    Rejected(NormalCallableSemanticAdmissionRejectV1),
}

impl<'source> VerifiedNormalCallableSemanticSourceV1<'source> {
    pub(in crate::mir) fn seal(
        program: &'source ASTNode,
        inventory: &VerifiedSelectedNormalCallableSourceInventoryV1,
        is_app_mode: bool,
        resolver: &mut FunctionSemanticResolverSessionV1,
    ) -> Result<NormalCallableSemanticAdmissionV1<'source>, String> {
        if !is_app_mode && !inventory.blockers().is_empty() {
            return Ok(NormalCallableSemanticAdmissionV1::Deferred);
        }
        let ASTNode::Program { statements, .. } = program else {
            return Err("[freeze:contract][mir/callable-semantic/program-required]".to_owned());
        };
        let mut candidates = Vec::with_capacity(inventory.len());
        for (key, site) in inventory.entries() {
            let function = function_at_site(statements, key, site)?;
            let view = view_for_key(function, key)?;
            candidates.push((key.clone(), site.clone(), function, view));
        }
        let views = candidates
            .iter()
            .map(|(_, _, _, view)| *view)
            .collect::<Vec<_>>();
        let forests = match resolver
            .resolve_selected_callable_forests(&views)
            .map_err(|error| format!("[freeze:contract][mir/callable-semantic/forest] {error:?}"))?
        {
            ResolveSelectedCallableForestsOutcomeV1::Complete(forests) => forests,
            ResolveSelectedCallableForestsOutcomeV1::Deferred => {
                return Ok(NormalCallableSemanticAdmissionV1::Deferred)
            }
        };
        if forests.len() != candidates.len() {
            return Err("[freeze:contract][mir/callable-semantic/cardinality]".to_owned());
        }
        if forests
            .iter()
            .any(forest_has_unissued_direct_call_observation_v1)
        {
            return Ok(NormalCallableSemanticAdmissionV1::Rejected(
                NormalCallableSemanticAdmissionRejectV1::UnissuedDirectCallObservation,
            ));
        }
        let mut rows = Vec::with_capacity(candidates.len());
        for ((key, site, function, view), forest) in candidates.into_iter().zip(forests) {
            let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
                function,
                &forest,
                view.root_profile(),
            )
            .map_err(|error| {
                format!("[freeze:contract][mir/callable-semantic/projection] {error}")
            })?;
            rows.push(VerifiedNormalCallableSemanticSourceRowV1 {
                key,
                site,
                forest,
                projection,
            });
        }
        Ok(NormalCallableSemanticAdmissionV1::Complete(Self {
            program,
            catalog_brand: inventory.brand().clone(),
            rows: rows.into_boxed_slice(),
        }))
    }

    pub(in crate::mir) fn loan<'loan>(
        &'loan self,
        key: &SelectedNormalCallableKeyV1,
    ) -> Result<VerifiedNormalCallableSemanticLoanV1<'source, 'loan>, String> {
        let row = self
            .rows
            .iter()
            .find(|row| &row.key == key)
            .ok_or_else(|| "[freeze:contract][mir/callable-semantic/missing-loan]".to_owned())?;
        let [root] = row.forest.roots() else {
            return Err("[freeze:contract][mir/callable-semantic/root-cardinality]".to_owned());
        };
        let ASTNode::Program { statements, .. } = self.program else {
            unreachable!("seal retained a Program")
        };
        let function = function_at_site(statements, &row.key, &row.site)?;
        let projected = row
            .projection
            .owner_root(function, *root)
            .map_err(|error| {
                format!("[freeze:contract][mir/callable-semantic/owner-root] {error}")
            })?;
        if !std::ptr::eq(projected, function) {
            return Err("[freeze:contract][mir/callable-semantic/root-identity]".to_owned());
        }
        let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
            function,
            &row.forest,
            &row.projection,
        )
        .map_err(|error| format!("[freeze:contract][mir/callable-semantic/input] {error:?}"))?;
        let ledger = row.forest.callable_source_ledger(*root).map_err(|error| {
            format!("[freeze:contract][mir/callable-semantic/ledger] {error:?}")
        })?;
        if input.owner() != ledger.owner() || !std::ptr::eq(input.forest(), &row.forest) {
            return Err("[freeze:contract][mir/callable-semantic/input-owner]".to_owned());
        }
        let source_ingress = VerifiedNormalCallableSourceIngressReceiptV1 { input, ledger };
        let lineage = match &row.key {
            SelectedNormalCallableKeyV1::TopLevel(key) => {
                super::raw_invocation_source_transport::RawInvocationRootLineageV1::TopLevel(
                    key.clone(),
                )
            }
            SelectedNormalCallableKeyV1::Cataloged(key) => {
                super::raw_invocation_source_transport::RawInvocationRootLineageV1::Cataloged(
                    key.clone(),
                )
            }
        };
        Ok(VerifiedNormalCallableSemanticLoanV1 {
            lineage,
            catalog_brand: self.catalog_brand.clone(),
            _function: function,
            source_ingress,
        })
    }

    /// Borrow one exact cataloged callable without exposing the mixed
    /// top-level/catalog selection key outside this authority.
    pub(in crate::mir) fn cataloged_loan<'loan>(
        &'loan self,
        key: &super::CanonicalSameModuleCallableKeyV1,
    ) -> Result<VerifiedNormalCallableSemanticLoanV1<'source, 'loan>, String> {
        self.loan(&SelectedNormalCallableKeyV1::Cataloged(key.clone()))
    }

    pub(in crate::mir) fn keys(&self) -> impl Iterator<Item = &SelectedNormalCallableKeyV1> {
        self.rows.iter().map(|row| &row.key)
    }
}

impl<'source, 'loan> VerifiedNormalCallableSemanticLoanV1<'source, 'loan> {
    pub(super) fn into_source_ingress(self) -> VerifiedNormalCallableSourceIngressReceiptV1<'loan> {
        self.source_ingress
    }

    /// Consume this loan together with one already-issued logical product.
    /// No Builder/session effect occurs here; identity mismatches are rejected
    /// before any physical ingress can be opened.
    pub(super) fn prepare_loop_ingress(
        self,
        logical: VerifiedCallableSingleLoopRecipeProductV1,
    ) -> Result<PreparedCallableLoopIngressV1<'loan>, PreparedCallableLoopIngressRejectV1> {
        let source = self.source_ingress;
        let source_owner = source.owner();
        if source.input().owner() != source_owner || source.ledger().owner() != source_owner {
            return Err(PreparedCallableLoopIngressRejectV1::SourceOwnerMismatch);
        }

        let co_seal = logical.co_seal();
        if co_seal.core().owner() != source_owner {
            return Err(PreparedCallableLoopIngressRejectV1::LogicalCoreOwnerMismatch);
        }
        if logical.prelude().owner() != source_owner {
            return Err(PreparedCallableLoopIngressRejectV1::LogicalPreludeOwnerMismatch);
        }
        if logical.tail().owner() != source_owner {
            return Err(PreparedCallableLoopIngressRejectV1::LogicalTailOwnerMismatch);
        }
        if co_seal.continuation().owner() != source_owner {
            return Err(PreparedCallableLoopIngressRejectV1::LogicalContinuationOwnerMismatch);
        }

        let context = co_seal.context();
        if context.owner() != source_owner {
            return Err(PreparedCallableLoopIngressRejectV1::LogicalContextOwnerMismatch);
        }
        if context.origin() != source.ledger().function_origin() {
            return Err(PreparedCallableLoopIngressRejectV1::LogicalOriginMismatch);
        }
        if context.source_kind() != source.ledger().source_kind() {
            return Err(PreparedCallableLoopIngressRejectV1::LogicalSourceKindMismatch);
        }
        let membership = source
            .ledger()
            .only_loop_site()
            .map_err(|_| PreparedCallableLoopIngressRejectV1::SourceLoopIdentityUnavailable)?;
        if context.loop_site() != membership.source().site() {
            return Err(PreparedCallableLoopIngressRejectV1::LogicalLoopSiteMismatch);
        }
        if context.frame() != membership.frame() {
            return Err(PreparedCallableLoopIngressRejectV1::LogicalFrameMismatch);
        }
        if context.scope_region() != membership.scope_region() {
            return Err(PreparedCallableLoopIngressRejectV1::LogicalScopeRegionMismatch);
        }

        Ok(PreparedCallableLoopIngressV1 { source, logical })
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        super::raw_invocation_source_transport::RawInvocationRootLineageV1,
        VerifiedNormalCallableSourceIngressReceiptV1<'loan>,
    ) {
        (self.lineage, self.source_ingress)
    }
}

impl<'source> PreparedCallableLoopIngressV1<'source> {
    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.source.owner()
    }

    pub(super) fn source(&self) -> &VerifiedNormalCallableSourceIngressReceiptV1<'_> {
        &self.source
    }

    pub(super) fn logical(&self) -> &VerifiedCallableSingleLoopRecipeProductV1 {
        &self.logical
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        VerifiedNormalCallableSourceIngressReceiptV1<'source>,
        VerifiedCallableSingleLoopRecipeProductV1,
    ) {
        (self.source, self.logical)
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn from_source_for_test(
        source: VerifiedNormalCallableSourceIngressReceiptV1<'source>,
        logical: VerifiedCallableSingleLoopRecipeProductV1,
    ) -> Self {
        Self { source, logical }
    }
}

#[cfg(test)]
#[path = "normal_callable_semantic_source_tests.rs"]
mod tests;
