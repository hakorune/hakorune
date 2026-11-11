//! Query-local inventory of immutable original branch sources. Affine packet
//! movement does not change their identity; no physical proof is cached here.
use super::OrdinaryNewClaimLedgerV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    BranchNormalScalarSourceV1 as Branch, CheckedExpressionReferenceV1 as Reference,
    CurrentScalarProducerV1 as Producer,
};
use crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1, SourceStmtSiteV1};
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

type Key = (usize, SourceValueRefV1, BindingRefV1, SourceStmtSiteV1);
#[derive(Debug, Default)]
pub(super) struct BranchSourceIndex {
    seen: BTreeSet<Key>,
    by_site: BTreeMap<SourceStmtSiteV1, Vec<Rc<Branch>>>,
}
impl BranchSourceIndex {
    fn retain(&mut self, source: &Producer, owner: FunctionOwnerIdV1) {
        let mut pending = vec![source];
        while let Some(source) = pending.pop() {
            let Producer::BranchNormal(source) = source else {
                continue;
            };
            if !self.seen.insert(source.identity_v1()) {
                continue;
            }
            if source.binding().owner() == owner {
                self.by_site
                    .entry(source.site().clone())
                    .or_default()
                    .push(Rc::clone(source));
            }
            pending.extend(source.incoming().iter().rev().map(|(_, _, source)| source));
        }
    }
    fn build(ledger: &OrdinaryNewClaimLedgerV1, owner: FunctionOwnerIdV1) -> Result<Self, String> {
        let mut index = Self::default();
        ledger.visit_expression_evaluations_v1(owner, &mut |evaluation| {
            for reference in evaluation.references().values() {
                if let Reference::Scalar(read) = reference {
                    if let Some(read) = read.original_read() {
                        index.retain(read.producer(), owner);
                    }
                }
            }
            Ok(true)
        })?;
        Ok(index)
    }
    fn at(&self, site: &SourceStmtSiteV1) -> Vec<Rc<Branch>> {
        self.by_site.get(site).cloned().unwrap_or_default()
    }
}
#[derive(Debug)]
pub(super) struct BranchSourceScope {
    owner: FunctionOwnerIdV1,
    index: Option<Result<BranchSourceIndex, String>>,
}
/// Lexically restores the previous owner, including every Err/early return.
/// Holding the guard does not authorize any physical MIR correspondence.
pub(crate) struct BranchSourceScopeGuard<'a> {
    ledger: &'a OrdinaryNewClaimLedgerV1,
    previous: Option<BranchSourceScope>,
}
impl Drop for BranchSourceScopeGuard<'_> {
    fn drop(&mut self) {
        self.ledger
            .branch_source_scope
            .replace(self.previous.take());
    }
}
impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn branch_source_scope_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> BranchSourceScopeGuard<'_> {
        BranchSourceScopeGuard {
            ledger: self,
            previous: self
                .branch_source_scope
                .replace(Some(BranchSourceScope { owner, index: None })),
        }
    }
    pub(in crate::mir) fn branch_scalar_sources_v1(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
    ) -> Result<Vec<Rc<Branch>>, String> {
        let mut scope = self.branch_source_scope.borrow_mut();
        let mut output = match scope.as_mut().filter(|scope| scope.owner == owner) {
            Some(scope) => {
                let index = scope
                    .index
                    .get_or_insert_with(|| BranchSourceIndex::build(self, owner));
                index.as_ref().map_err(Clone::clone)?.at(site)
            }
            None => BranchSourceIndex::build(self, owner)?.at(site),
        };
        // Recipes outside Completion are a separate retained source inventory;
        // keep their exact site/prefix gate instead of inventing a Loop registry.
        let mut loops = BranchSourceIndex::default();
        for source in self.selected_source_loop_branch_joins_v2(owner, site) {
            loops.retain(&Producer::BranchNormal(source), owner);
        }
        for source in loops.at(site) {
            if !output.iter().any(|old| old == &source) {
                output.push(source);
            }
        }
        Ok(output)
    }
}
