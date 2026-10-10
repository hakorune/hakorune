//! Sealed LoopCond after-PHI receipt from the selected source publication path.
use super::PlanLowerer;
use crate::mir::builder::control_flow::plan::lowering_context::PlanLoweringContext;
use crate::mir::builder::control_flow::plan::{CorePlan, LoweredRecipe};
use crate::mir::builder::normal_callable_loop_source_facts::SourceLoopCondPhysicalInputV1;
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceStmtSiteV1};
use crate::mir::{BasicBlockId, MirInstruction, ValueId};
use std::cell::RefCell;
use std::rc::Rc;

/// Only the selected LoopCond source path can obtain this role. Physical
/// capture reads it without reconstructing source authority from the MIR.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedLoopCondAfterPhysicalRoleV1 {
    owner: FunctionOwnerIdV1,
    site: SourceStmtSiteV1,
    block: BasicBlockId,
    phi_prefix: Box<[MirInstruction]>,
}

impl VerifiedLoopCondAfterPhysicalRoleV1 {
    pub(in crate::mir) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(in crate::mir) fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }
    pub(in crate::mir) const fn block(&self) -> BasicBlockId {
        self.block
    }
    pub(in crate::mir) fn phi_prefix(&self) -> &[MirInstruction] {
        &self.phi_prefix
    }
}

impl PlanLowerer {
    /// The ordinary Plan entry returns only a value. This selected source
    /// entry also returns the exact after-PHI role after emission and source
    /// publication both succeed; a failed lowering cannot issue a role.
    pub(in crate::mir::builder) fn lower_loop_cond_with_source_publication(
        builder: &mut MirBuilder,
        plan: LoweredRecipe,
        ctx: &dyn PlanLoweringContext,
        ledger: Rc<RefCell<CallableSemanticLoweringState>>,
        source: &SourceLoopCondPhysicalInputV1<'_, '_>,
    ) -> Result<(Option<ValueId>, Option<VerifiedLoopCondAfterPhysicalRoleV1>), String> {
        let CorePlan::Loop(loop_plan) = &plan else {
            return Err("[freeze:contract][loop-cond-after-role/non-loop-plan]".into());
        };
        let after = loop_plan.after_bb;
        let planned_dsts = loop_plan
            .phis
            .iter()
            .filter(|phi| phi.block == after)
            .map(|phi| phi.dst)
            .collect::<Vec<_>>();
        let result = Self::lower_with_source_publication(builder, plan, ctx, ledger)?;
        if builder.function_state.current_block != Some(after) {
            return Err("[freeze:contract][loop-cond-after-role/after-block-drift]".into());
        }
        let block = builder
            .function_state
            .current_function
            .as_ref()
            .and_then(|function| function.blocks.get(&after))
            .ok_or("[freeze:contract][loop-cond-after-role/after-block-missing]")?;
        let prefix = block
            .instructions
            .iter()
            .take_while(|row| matches!(row, MirInstruction::Phi { .. }))
            .cloned()
            .collect::<Vec<_>>();
        if prefix.len() != planned_dsts.len()
            || prefix
                .iter()
                .zip(planned_dsts)
                .any(|(row, planned)| row.dst_value() != Some(planned))
        {
            return Err("[freeze:contract][loop-cond-after-role/plan-phi-drift]".into());
        }
        let role = (!prefix.is_empty()).then(|| VerifiedLoopCondAfterPhysicalRoleV1 {
            owner: source.owner(),
            site: SourceStmtSiteV1::from_node(source.parent_site().clone()),
            block: after,
            phi_prefix: prefix.into_boxed_slice(),
        });
        Ok((result, role))
    }
}
