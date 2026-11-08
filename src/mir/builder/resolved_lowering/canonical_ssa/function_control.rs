//! Function-owned physical CFG, reaching definitions and provisional PHIs.
//! Source packets supply verified definitions; this owner never infers source
//! semantics from the emitted graph. Child loops borrow it, child functions do not.
use super::super::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::builder::{
    emission::phi_lifecycle::{PhiToken, PhiTxn},
    ssa::binding::{BindingSsaBuilderV1, FunctionSsaKeyV1, MirBindingSsaAdapterV1},
    MirBuilder,
};
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};
use crate::mir::{BasicBlockId, ValueId};
use std::{cell::RefCell, rc::Rc};

pub(in crate::mir::builder) type SharedFunctionControlV1 = Rc<RefCell<FunctionControlV1>>;

#[derive(Debug, Default)]
pub(in crate::mir::builder) struct FunctionControlV1 {
    owner: Option<FunctionOwnerIdV1>,
    pub(in crate::mir::builder) cfg: CanonicalCfgSessionV1,
    bindings: Option<BindingSsaBuilderV1<PhiToken, FunctionSsaKeyV1>>,
    transaction: Option<PhiTxn>,
}
impl FunctionControlV1 {
    fn bind_owner(&mut self, owner: FunctionOwnerIdV1) -> Result<(), String> {
        if self.owner.is_some_and(|original| original != owner) {
            return Err("[freeze:contract][canonical-control/foreign-owner]".to_owned());
        }
        self.owner = Some(owner);
        self.cfg.bind_source_owner(owner)?;
        Ok(())
    }
    pub(in crate::mir::builder) fn is_empty_for_commit(&self) -> bool {
        self.owner.is_none()
            && self.bindings.is_none()
            && self.transaction.is_none()
            && self.cfg.is_empty_for_commit()
    }
    pub(in crate::mir::builder) fn is_active(&self) -> bool {
        self.transaction.is_some()
    }
    pub(in crate::mir::builder) fn with_transaction<T>(
        &mut self,
        action: impl FnOnce(&mut PhiTxn) -> Result<T, String>,
    ) -> Result<T, String> {
        action(
            self.transaction
                .get_or_insert_with(|| PhiTxn::begin("source-loop-header-after")),
        )
    }
    pub(in crate::mir::builder) fn reserve(
        &mut self,
        builder: &mut MirBuilder,
        binding: BindingRefV1,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String> {
        self.owner.ok_or_else(inactive)?;
        self.reserve_key(
            builder,
            FunctionSsaKeyV1::Source(binding),
            block,
            value,
            "source-loop:before-expose",
        )
    }
    fn reserve_key(
        &mut self,
        builder: &mut MirBuilder,
        key: FunctionSsaKeyV1,
        block: BasicBlockId,
        value: ValueId,
        tag: &str,
    ) -> Result<(), String> {
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(inactive)?;
        if function.get_block(block).is_none() {
            self.cfg
                .create_block(function, block)
                .map_err(|error| error.to_string())?;
        }
        let token = self
            .transaction
            .get_or_insert_with(|| PhiTxn::begin("source-loop-header-after"))
            .define_provisional_phi(builder, block, value, tag)?;
        self.bindings
            .get_or_insert_with(|| BindingSsaBuilderV1::with_keys(self.owner))
            .adopt_provisional_phi(key, block, value, token)
            .map_err(|error| error.to_string())
    }
    pub(in crate::mir::builder) fn reserve_loop_entries(
        &mut self,
        builder: &mut MirBuilder,
        header: BasicBlockId,
        after: BasicBlockId,
        headers: Vec<(FunctionSsaKeyV1, ValueId)>,
        afters: Vec<(FunctionSsaKeyV1, ValueId)>,
    ) -> Result<(), String> {
        use std::collections::BTreeSet;
        if header == after
            || headers.is_empty()
            || headers.len() != afters.len()
            || headers
                .iter()
                .chain(&afters)
                .map(|(_, value)| *value)
                .collect::<BTreeSet<_>>()
                .len()
                != headers.len() + afters.len()
        {
            return Err("[freeze:contract][source-loop-phi/preparation-destinations]".to_owned());
        }
        for (block, entries) in [(header, headers), (after, afters)] {
            for (key, value) in entries {
                self.reserve_key(builder, key, block, value, "source-loop:before-expose")?;
            }
        }
        Ok(())
    }
    pub(in crate::mir::builder) fn reserve_plan_phis(
        &mut self,
        builder: &mut MirBuilder,
        plan: &crate::mir::builder::control_flow::plan::CoreLoopPlan,
        adopt_early: bool,
    ) -> Result<(), String> {
        for phi in &plan.phis {
            let key = phi.ssa_key();
            let existing = self
                .bindings
                .as_ref()
                .and_then(|ssa| ssa.key_for_definition(phi.block, phi.dst));
            if adopt_early && existing == Some(key) {
                continue;
            }
            let tag = if phi.binding.is_some() {
                "source-loop:step-adoption".to_owned()
            } else {
                format!("loop_lowerer:step1.5:{}", phi.tag)
            };
            self.reserve_key(builder, key, phi.block, phi.dst, &tag)?;
        }
        Ok(())
    }
    pub(in crate::mir::builder) fn record_phi_edge(
        &mut self,
        source: BasicBlockId,
        target: BasicBlockId,
        destination: ValueId,
        value: ValueId,
    ) -> Result<(), String> {
        let ssa = self
            .bindings
            .get_or_insert_with(|| BindingSsaBuilderV1::with_keys(self.owner));
        let key =
            ssa.key_for_definition(target, destination)
                .unwrap_or(FunctionSsaKeyV1::PlannedPhi {
                    block: target,
                    destination,
                });
        ssa.define_edge(key, source, target, value)
            .map_err(|error| error.to_string())
    }
    pub(in crate::mir::builder) fn complete_plan_phis(
        &mut self,
        builder: &mut MirBuilder,
        plan: &mut crate::mir::builder::control_flow::plan::CoreLoopPlan,
    ) -> Result<(), String> {
        let witnesses = self
            .cfg
            .predecessor_witnesses(
                builder
                    .function_state
                    .current_function
                    .as_ref()
                    .ok_or_else(inactive)?,
            )
            .map_err(|error| error.to_string())?;
        if plan.phis.is_empty() {
            return Ok(());
        }
        // Plan seeds and original exit arguments are definitions on actual edges.
        // The original seed wins when both protocols name the same edge.
        let ssa = self.bindings.as_mut().ok_or_else(inactive)?;
        for phi in &plan.phis {
            let witness = witnesses
                .iter()
                .find(|row| row.block() == phi.block)
                .ok_or_else(inactive)?;
            for &(source, value) in &phi.inputs {
                if witness.predecessors().contains(&source) {
                    ssa.define_edge(phi.ssa_key(), source, phi.block, value)
                        .map_err(|error| error.to_string())?;
                }
            }
        }
        let unused: std::collections::BTreeSet<_> = plan
            .phis
            .iter()
            .filter(|phi| {
                phi.block == plan.step_bb
                    && phi.inputs.is_empty()
                    && !plan
                        .phis
                        .iter()
                        .any(|other| other.inputs.iter().any(|(_, value)| *value == phi.dst))
                    && witnesses
                        .iter()
                        .find(|row| row.block() == phi.block)
                        .is_some_and(|row| row.predecessors().is_empty())
            })
            .map(|phi| (phi.block, phi.dst))
            .collect();
        let mut adapter =
            MirBindingSsaAdapterV1::new(builder, self.transaction.as_mut().ok_or_else(inactive)?);
        for phi in &mut plan.phis {
            if unused.contains(&(phi.block, phi.dst)) {
                ssa.discard_adopted(&mut adapter, phi.ssa_key(), phi.block)
                    .map_err(|error| error.to_string())?;
            } else {
                let witness = witnesses
                    .iter()
                    .find(|row| row.block() == phi.block)
                    .ok_or_else(inactive)?;
                phi.inputs = ssa
                    .complete_adopted(&mut adapter, phi.ssa_key(), phi.block, witness)
                    .map_err(|error| error.to_string())?;
            }
        }
        Ok(())
    }
    pub(in crate::mir::builder) fn require_plan_patched(
        &mut self,
        plan: &crate::mir::builder::control_flow::plan::CoreLoopPlan,
    ) -> Result<(), String> {
        self.with_transaction(|txn| {
            if txn.pending_tokens_v1().any(|(block, token)| {
                plan.phis
                    .iter()
                    .any(|phi| phi.block == block && phi.dst == token.dst())
            }) {
                return Err("[freeze:contract][source-loop-phi/pending-after-patch]".to_owned());
            }
            Ok(())
        })
    }
    pub(in crate::mir::builder) fn define(
        &mut self,
        binding: BindingRefV1,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String> {
        self.bindings
            .as_mut()
            .ok_or_else(inactive)?
            .define(FunctionSsaKeyV1::Source(binding), block, value)
            .map_err(|error| error.to_string())
    }
    pub(in crate::mir::builder) fn complete_blocks(
        &mut self,
        builder: &mut MirBuilder,
        blocks: &[BasicBlockId],
        deferred_entry: BasicBlockId,
    ) -> Result<(), String> {
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(inactive)?;
        let witnesses = self
            .cfg
            .predecessor_witnesses(function)
            .map_err(|error| error.to_string())?;
        let mut adapter = MirBindingSsaAdapterV1::new_with_deferred_reachable(
            builder,
            self.transaction.as_mut().ok_or_else(inactive)?,
            deferred_entry,
        );
        for block in blocks {
            let witness = witnesses
                .iter()
                .find(|row| row.block() == *block)
                .ok_or_else(|| format!("[freeze:contract][canonical-control/predecessor-witness-missing] block={block}"))?;
            self.bindings
                .as_mut()
                .ok_or_else(inactive)?
                .seal(&mut adapter, *block, witness)
                .map_err(|error| error.to_string())?;
        }
        Ok(())
    }
    pub(in crate::mir::builder) fn abort(
        &mut self,
        builder: &mut MirBuilder,
        error: String,
    ) -> String {
        match self.transaction.take() {
            Some(txn) => txn.abort_on_err(builder, error).to_string(),
            None => error,
        }
    }
    fn finish(&mut self, builder: &mut MirBuilder) -> Result<(), String> {
        let result = (|| {
            let function = builder
                .function_state
                .current_function
                .as_mut()
                .ok_or_else(inactive)?;
            let witnesses = self
                .cfg
                .close_function_blocks(function)
                .map_err(|error| error.to_string())?;
            if let Some(bindings) = self.bindings.take() {
                bindings
                    .close(
                        &mut MirBindingSsaAdapterV1::new(
                            builder,
                            self.transaction.as_mut().ok_or_else(inactive)?,
                        ),
                        &witnesses,
                    )
                    .map_err(|error| error.to_string())?;
                MirBindingSsaAdapterV1::new(
                    builder,
                    self.transaction.as_mut().ok_or_else(inactive)?,
                )
                .verify_function_phi_inputs()?;
            }
            let function = builder
                .function_state
                .current_function
                .as_ref()
                .ok_or_else(inactive)?;
            std::mem::take(&mut self.cfg)
                .finish(function)
                .map_err(|error| error.to_string())?;
            match self.transaction.take() {
                Some(transaction) => transaction
                    .commit(builder)
                    .map_err(|error| error.to_string()),
                None => Ok(()),
            }
        })();
        result.map_err(|error| self.abort(builder, error))
    }
}
impl MirBuilder {
    pub(in crate::mir::builder) fn function_control_for_owner_v1(
        &mut self,
        owner: FunctionOwnerIdV1,
    ) -> Result<SharedFunctionControlV1, String> {
        let control = Rc::clone(&self.function_state.canonical_control);
        control.borrow_mut().bind_owner(owner)?;
        Ok(control)
    }
    pub(in crate::mir::builder) fn finish_function_control_v1(&mut self) -> Result<(), String> {
        let control = std::mem::take(&mut self.function_state.canonical_control);
        let result = control.borrow_mut().finish(self);
        result
    }
}
fn inactive() -> String {
    "[freeze:contract][canonical-control/inactive]".to_owned()
}

impl crate::mir::builder::recursive_child_lowering::RawInvocationChildPortV1<'_, '_> {
    pub(in crate::mir::builder) fn prepare_function_control_v1(
        &self,
        builder: &mut MirBuilder,
    ) -> Result<(), String> {
        if let Some(input) = self.source_input {
            builder.function_control_for_owner_v1(input.owner())?;
        }
        Ok(())
    }
}
