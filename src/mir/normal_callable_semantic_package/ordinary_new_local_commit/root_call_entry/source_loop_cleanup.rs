//! Original Loop port obligations borrow installed Home End plans once.
//! No root Return identity, execution permission or second Home registry.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1;
use std::rc::Rc;
use crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2 as Product;
use crate::mir::instruction::InvokeOperation;

#[derive(Debug)]
pub(in crate::mir) struct SourceLoopCleanupPlanV2 {
    product: Rc<Product>,
    port: usize,
    full: Vec<(LiveHomeV1, super::super::RootHomeReleaseSubjectV1, InvokeOperation)>,
    normal: Vec<usize>,
    residuals: Vec<Vec<usize>>,
}
impl SourceLoopCleanupPlanV2 {
    pub(in crate::mir) fn product(&self) -> &Rc<Product> { &self.product }
    pub(in crate::mir) fn port(&self) -> usize { self.port }
    pub(in crate::mir) fn normal_operations(&self) -> Vec<InvokeOperation> {
        self.normal.iter().map(|index| self.full[*index].2.clone()).collect()
    }
    pub(in crate::mir) fn fault_operations(&self) -> Vec<InvokeOperation> {
        self.full.iter().map(|row| row.2.clone()).collect()
    }
    pub(in crate::mir) fn residuals(&self) -> &[Vec<usize>] { &self.residuals }
    pub(in crate::mir) fn normal_is_empty(&self) -> bool { self.normal.is_empty() }
    /// Borrow the same root path algorithm with an explicit Loop stop. The
    /// caller must also corroborate all retained Invoke loans and exact census.
    pub(in crate::mir) fn validate_paths_v2(&self, function: &MirFunction,
        entry: BasicBlockId, continuation: BasicBlockId, frame: Option<ValueId>,
        resolve: &mut impl FnMut(BasicBlockId, &InvokeOperation) -> Result<InvokeOperation, String>,
    ) -> Result<(), String> {
        let residuals = self.residuals.iter().map(|indices| indices.iter()
            .map(|index| self.full[*index].2.clone()).collect()).collect::<Vec<Vec<_>>>();
        super::super::super::root_cleanup_graph::ordered_paths::validate_to_continuation_v2(
            function, entry, continuation, &self.normal_operations(), &residuals, frame, resolve)
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn prepare_source_loop_port_cleanup_v2(&self,
        product: &Rc<Product>, port: usize,
    ) -> Result<SourceLoopCleanupPlanV2, String> {
        CallValidationContextV1::Lowering(self).require_original_source_loop_v2(product)?;
        let original = self.retained_source_loop_recipe_v2(product.source.source.owner(), product.source.source.site())?
            .ok_or_else(|| freeze("source-loop/cleanup-product-missing"))?;
        if !Rc::ptr_eq(original, product) { return Err(freeze("source-loop/cleanup-product-drift")); }
        let source = product.source.ports.get(port).ok_or_else(|| freeze("source-loop/cleanup-port"))?;
        let homes: Vec<_> = source.homes().iter().rev().cloned().collect();
        let (full, normal) = super::super::source_home_end_selection_v2(
            &self.local_commits.borrow(), &homes, source.normal_ends())?
            .ok_or_else(|| freeze("source-loop/cleanup-end-unavailable"))?;
        let residuals = (0..normal.len()).map(|step|
            super::super::fault_indices_after_normal_step_v2(full.len(), &normal, step))
            .collect::<Result<_, _>>()?;
        Ok(SourceLoopCleanupPlanV2 { product: Rc::clone(product), port, full, normal, residuals })
    }
}

#[path = "source_loop_cleanup_validation.rs"]
mod validation;
