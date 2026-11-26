//! Atomic semantic source authority for one selected callable batch.

use crate::ast::ASTNode;
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

    pub(super) fn into_parts(
        self,
    ) -> (
        super::raw_invocation_source_transport::RawInvocationRootLineageV1,
        VerifiedNormalCallableSourceIngressReceiptV1<'loan>,
    ) {
        (self.lineage, self.source_ingress)
    }
}

#[cfg(test)]
#[path = "normal_callable_semantic_source_tests.rs"]
mod tests;
