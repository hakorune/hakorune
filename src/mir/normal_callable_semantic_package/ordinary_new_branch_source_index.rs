//! Query-local inventory of immutable original branch sources. Affine packet
//! movement does not change their identity; no physical proof is cached here.
use super::OrdinaryNewClaimLedgerV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    BranchNormalScalarSourceV1 as Branch, CheckedExpressionReferenceV1 as Reference,
    CurrentScalarProducerV1 as Producer, ExpressionScalarOperationLoanV1 as Scalar,
};
use crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1, SourceStmtSiteV1};
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

type Key = (usize, SourceValueRefV1, BindingRefV1, SourceStmtSiteV1);
#[derive(Debug, Default)]
pub(super) struct FunctionSourceInventory {
    seen: BTreeSet<Key>,
    by_site: BTreeMap<SourceStmtSiteV1, Vec<Rc<Branch>>>,
    scalars: Vec<Rc<Scalar>>,
    scalar_sites: BTreeMap<crate::mir::resolved_semantics::SourceExprSiteV1, usize>,
    scalar_error: Option<String>,
    visitor_error: Option<String>,
}
impl FunctionSourceInventory {
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
    fn retain_scalar(
        &mut self,
        source: &Rc<Scalar>,
        owner: FunctionOwnerIdV1,
    ) -> Result<(), String> {
        if source.owner() != owner {
            return Err(super::scalar_operations::reject("source-membership"));
        }
        if let Some(index) = self.scalar_sites.get(source.binary().site()) {
            if !Rc::ptr_eq(&self.scalars[*index], source) {
                return Err(super::scalar_operations::reject("source-membership"));
            }
        } else {
            self.scalar_sites
                .insert(source.binary().site().clone(), self.scalars.len());
            self.scalars.push(Rc::clone(source));
        }
        Ok(())
    }
    fn build(ledger: &OrdinaryNewClaimLedgerV1, owner: FunctionOwnerIdV1) -> Self {
        let mut index = Self::default();
        if let Some(flow) = ledger
            .completion_for_owner(owner)
            .and_then(|row| row.cleanup().root_flow())
        {
            for source in flow.scalar_operations() {
                if let Err(error) = index.retain_scalar(source, owner) {
                    index.scalar_error = Some(error);
                    break;
                }
            }
        }
        index.visitor_error = ledger
            .visit_expression_evaluations_v1(owner, &mut |evaluation| {
                for reference in evaluation.references().values() {
                    if let Reference::Scalar(read) = reference {
                        if let Some(read) = read.original_read() {
                            index.retain(read.producer(), owner);
                        }
                    }
                }
                if index.scalar_error.is_none() {
                    for operation in evaluation.operations() {
                        if let Some(source) = operation.scalar_source_v1() {
                            let result = if Rc::ptr_eq(source.plan(), evaluation.source()) {
                                index.retain_scalar(source, owner)
                            } else {
                                Err(super::scalar_operations::reject("source-membership"))
                            };
                            if let Err(error) = result {
                                index.scalar_error = Some(error);
                                break;
                            }
                        }
                    }
                }
                Ok(true)
            })
            .err();
        index
    }
    fn scalar_rows(&self) -> Result<&[Rc<Scalar>], String> {
        match self.scalar_error.as_ref().or(self.visitor_error.as_ref()) {
            Some(error) => Err(error.clone()),
            None => Ok(&self.scalars),
        }
    }
    fn at(&self, site: &SourceStmtSiteV1) -> Vec<Rc<Branch>> {
        self.by_site.get(site).cloned().unwrap_or_default()
    }
}
#[derive(Debug)]
pub(super) struct BranchSourceScope {
    owner: FunctionOwnerIdV1,
    index: Option<FunctionSourceInventory>,
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
    fn with_source_inventory_v1<R>(
        &self,
        owner: FunctionOwnerIdV1,
        query: impl FnOnce(&FunctionSourceInventory) -> R,
    ) -> R {
        let mut scope = self.branch_source_scope.borrow_mut();
        match scope.as_mut().filter(|scope| scope.owner == owner) {
            Some(scope) => query(
                scope
                    .index
                    .get_or_insert_with(|| FunctionSourceInventory::build(self, owner)),
            ),
            None => query(&FunctionSourceInventory::build(self, owner)),
        }
    }
    pub(super) fn scalar_operation_sources_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<Vec<Rc<Scalar>>, String> {
        self.with_source_inventory_v1(owner, |index| index.scalar_rows().map(<[_]>::to_vec))
    }
    pub(in crate::mir) fn has_scalar_operation_source_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<bool, String> {
        self.with_source_inventory_v1(owner, |index| {
            index.scalar_rows().map(|rows| !rows.is_empty())
        })
    }
    pub(in crate::mir) fn prepare_scalar_operation_source_v1(
        &self,
        owner: FunctionOwnerIdV1,
        site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
        operator: Option<crate::mir::BinaryOp>,
    ) -> Result<Option<Rc<Scalar>>, String> {
        use super::scalar_operations::{reject, source_operator};
        if site.owner() != owner {
            return Err(reject("owner"));
        }
        self.with_source_inventory_v1(owner, |index| {
            index.scalar_rows()?;
            let Some(position) = index.scalar_sites.get(site.site()) else {
                return Ok(None);
            };
            let row = &index.scalars[*position];
            if Some(source_operator(row)?) != operator {
                return Err(reject("operator"));
            }
            Ok(Some(Rc::clone(row)))
        })
    }
    pub(in crate::mir) fn branch_scalar_sources_v1(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
    ) -> Result<Vec<Rc<Branch>>, String> {
        let mut output =
            self.with_source_inventory_v1(owner, |index| match &index.visitor_error {
                Some(error) => Err(error.clone()),
                None => Ok(index.at(site)),
            })?;
        // Recipes outside Completion are a separate retained source inventory;
        // keep their exact site/prefix gate instead of inventing a Loop registry.
        let mut loops = FunctionSourceInventory::default();
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
