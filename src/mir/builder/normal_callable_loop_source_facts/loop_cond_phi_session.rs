//! One physical input owns early Header/After definitions through final patch.
//! The source session lends the SAME PhiTxn; it issues no semantic permission.
use crate::mir::builder::control_flow::plan::CoreLoopPlan;
use crate::mir::builder::emission::phi_lifecycle::PhiTxn;
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, ValueId};
use std::cell::RefCell;
use std::collections::BTreeSet;

#[derive(Debug, Default)]
struct State {
    scope: Option<(BasicBlockId, BasicBlockId)>,
    adopted: bool,
    finished: bool,
    transaction: Option<PhiTxn>,
}
#[derive(Debug, Default)]
pub(in crate::mir::builder) struct SourceLoopPhiSessionV1 {
    state: RefCell<State>,
}
impl SourceLoopPhiSessionV1 {
    /// Definitions precede both source-carrier publication points. A failed
    /// partial definition remains in this same txn for the enclosing abort.
    pub(in crate::mir::builder) fn prepare(
        &self, builder: &mut MirBuilder, header: BasicBlockId, after: BasicBlockId,
        headers: impl Iterator<Item = ValueId>, afters: impl Iterator<Item = ValueId>,
    ) -> Result<(), String> {
        let headers: Vec<_> = headers.collect();
        let afters: Vec<_> = afters.collect();
        let rows: Vec<_> = headers.iter().map(|value| (header, *value))
            .chain(afters.iter().map(|value| (after, *value))).collect();
        if header == after || headers.is_empty() || headers.len() != afters.len()
            || rows.iter().map(|(_, value)| *value).collect::<BTreeSet<_>>().len() != rows.len() {
            return Err(freeze("preparation-destinations"));
        }
        let mut state = self.state.borrow_mut();
        if state.scope.is_some() { return Err(freeze("preparation-repeat")); }
        state.scope = Some((header, after));
        state.transaction = Some(PhiTxn::begin("source-loop-header-after"));
        let txn = state.transaction.as_mut().expect("installed above");
        for (block, value) in rows {
            builder.ensure_block_exists(block)?;
            txn.define_provisional_phi(builder, block, value, "source-loop:before-expose")?;
        }
        Ok(())
    }

    /// Adopt every original Header/After token exactly once. Only remaining
    /// Step destinations are defined here, through the same transaction.
    pub(in crate::mir::builder) fn adopt_and_define(
        &self, builder: &mut MirBuilder, plan: &CoreLoopPlan,
    ) -> Result<Vec<(ValueId, BasicBlockId, String)>, String> {
        let mut state = self.state.borrow_mut();
        if state.scope != Some((plan.header_bb, plan.after_bb)) || state.adopted || state.finished {
            return Err(freeze("adoption-scope-or-repeat"));
        }
        let txn = state.transaction.as_mut().ok_or_else(|| freeze("transaction-missing"))?;
        let pending: BTreeSet<_> = txn.pending_tokens_v1().map(|(block, token)| (block, token.dst())).collect();
        let all: BTreeSet<_> = plan.phis.iter().map(|phi| (phi.block, phi.dst)).collect();
        let expected: BTreeSet<_> = all.iter().copied()
            .filter(|(block, _)| *block == plan.header_bb || *block == plan.after_bb).collect();
        if pending != expected || all.len() != plan.phis.len()
            || plan.phis.iter().map(|phi| phi.dst).collect::<BTreeSet<_>>().len() != plan.phis.len()
            || all.iter().any(|(block, _)| *block != plan.header_bb && *block != plan.after_bb && *block != plan.step_bb) {
            return Err(freeze("adoption-destination-census"));
        }
        // The source phase cannot patch or replace these provisional definitions.
        let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("function-missing"))?;
        for &(block, value) in &pending {
            let definitions: Vec<_> = function.blocks.iter().flat_map(|(id, body)| body.all_instructions()
                .filter(move |row| row.dst_value() == Some(value)).map(move |row| (*id, row))).collect();
            if !matches!(definitions.as_slice(), [(id, crate::mir::MirInstruction::Phi { dst, inputs, .. })]
                if *id == block && *dst == value && inputs.is_empty()) || function.params.contains(&value) {
                return Err(freeze("adoption-definition-drift"));
            }
        }
        for phi in &plan.phis {
            if !pending.contains(&(phi.block, phi.dst)) {
                builder.ensure_block_exists(phi.block)?;
                txn.define_provisional_phi(builder, phi.block, phi.dst, "source-loop:step-adoption")?;
            }
        }
        state.adopted = true;
        Ok(plan.phis.iter().map(|phi| (phi.dst, phi.block, phi.tag.clone())).collect())
    }

    /// Only linear patch operations borrow the txn; recursive source lowering
    /// never carries this RefMut across an expression or child callback.
    pub(in crate::mir::builder) fn with_transaction<R>(
        &self, action: impl FnOnce(&mut PhiTxn) -> Result<R, String>,
    ) -> Result<R, String> {
        let mut state = self.state.borrow_mut();
        if !state.adopted || state.finished { return Err(freeze("transaction-not-adopted")); }
        action(state.transaction.as_mut().ok_or_else(|| freeze("transaction-missing"))?)
    }
    pub(in crate::mir::builder) fn require_patched(&self) -> Result<(), String> {
        self.with_transaction(|txn| {
            if txn.pending_tokens_v1().next().is_some() { return Err(freeze("pending-after-patch")); }
            Ok(())
        })
    }
    pub(in crate::mir::builder) fn commit(&self, builder: &mut MirBuilder) -> Result<(), String> {
        self.require_patched()?;
        let mut state = self.state.borrow_mut();
        let txn = state.transaction.take().ok_or_else(|| freeze("transaction-missing"))?;
        txn.commit(builder).map_err(|error| error.to_string())?;
        state.finished = true;
        Ok(())
    }
    /// All enclosing error boundaries call the same owner. Once a txn has
    /// aborted, preserve the returned primary+cleanup error without wrapping it.
    pub(in crate::mir::builder) fn abort_error(&self, builder: &mut MirBuilder, error: String) -> String {
        let transaction = self.state.borrow_mut().transaction.take();
        match transaction { Some(txn) => txn.abort_on_err(builder, error).to_string(), None => error }
    }
}
fn freeze(reason: &str) -> String { format!("[freeze:contract][source-loop-phi/{reason}]") }
