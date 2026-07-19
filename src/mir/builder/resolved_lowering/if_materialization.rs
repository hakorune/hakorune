//! Builder-disconnected canonical conditional CFG and final-PHI materializer.
//!
//! No syntax or resolved-flow authority enters this box. Callers must provide
//! a condition value, ordered join rows, and close each fallthrough branch at
//! its actual exit block. Publication is impossible until every final PHI has
//! been defined successfully.

use std::collections::BTreeSet;

use crate::mir::builder::emission::{branch, phi_lifecycle};
use crate::mir::resolved_semantics::BindingRefV1;
use crate::mir::verification::utils::compute_predecessors;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};

use super::branch_transaction::ResolvedJoinValueRowV1;
use super::if_cfg_ready_bridge::VerifiedResolvedIfCfgReadyJoinRowsV1;
use super::MirBuilder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ElseRouteV1 {
    ImplicitFalse,
    Explicit { entry: BasicBlockId },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct IfCfgLayoutV1 {
    header: BasicBlockId,
    then_entry: BasicBlockId,
    else_entry: Option<BasicBlockId>,
    merge: BasicBlockId,
}

impl IfCfgLayoutV1 {
    pub(super) const fn header(self) -> BasicBlockId {
        self.header
    }

    pub(super) const fn then_entry(self) -> BasicBlockId {
        self.then_entry
    }

    pub(super) const fn else_entry(self) -> Option<BasicBlockId> {
        self.else_entry
    }

    pub(super) const fn merge(self) -> BasicBlockId {
        self.merge
    }
}

#[derive(Debug)]
pub(super) struct IfCfgSessionV1 {
    layout: IfCfgLayoutV1,
    else_route: ElseRouteV1,
    then_exit: Option<BasicBlockId>,
    else_exit: Option<BasicBlockId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct VerifiedIfMergePredecessorsV1 {
    header: BasicBlockId,
    then_entry: BasicBlockId,
    else_entry: Option<BasicBlockId>,
    merge: BasicBlockId,
    then_pred: BasicBlockId,
    else_pred: BasicBlockId,
}

impl IfCfgSessionV1 {
    pub(super) fn open_implicit_false(
        builder: &mut MirBuilder,
        condition: ValueId,
    ) -> Result<Self, String> {
        Self::open(builder, condition, false)
    }

    pub(super) fn open_explicit_else(
        builder: &mut MirBuilder,
        condition: ValueId,
    ) -> Result<Self, String> {
        Self::open(builder, condition, true)
    }

    pub(super) const fn layout(&self) -> IfCfgLayoutV1 {
        self.layout
    }

    pub(super) fn enter_then(&self, builder: &mut MirBuilder) -> Result<(), String> {
        builder.start_new_block(self.layout.then_entry)
    }

    pub(super) fn close_then(&mut self, builder: &mut MirBuilder) -> Result<(), String> {
        if self.then_exit.is_some() {
            return Err("[freeze:contract][canonical_if/then_closed_twice]".to_string());
        }
        let exit = current_unterminated_block(builder, "then")?;
        branch::emit_jump(builder, self.layout.merge)?;
        self.then_exit = Some(exit);
        Ok(())
    }

    pub(super) fn enter_else(&self, builder: &mut MirBuilder) -> Result<(), String> {
        let ElseRouteV1::Explicit { entry } = self.else_route else {
            return Err("[freeze:contract][canonical_if/implicit_else_has_no_block]".to_string());
        };
        builder.start_new_block(entry)
    }

    pub(super) fn close_else(&mut self, builder: &mut MirBuilder) -> Result<(), String> {
        if self.else_exit.is_some() {
            return Err("[freeze:contract][canonical_if/else_closed_twice]".to_string());
        }
        if matches!(self.else_route, ElseRouteV1::ImplicitFalse) {
            return Err("[freeze:contract][canonical_if/implicit_else_close]".to_string());
        }
        let exit = current_unterminated_block(builder, "else")?;
        branch::emit_jump(builder, self.layout.merge)?;
        self.else_exit = Some(exit);
        Ok(())
    }

    pub(super) fn verify_actual_predecessors(
        &self,
        builder: &mut MirBuilder,
    ) -> Result<VerifiedIfMergePredecessorsV1, String> {
        let then_pred = self
            .then_exit
            .ok_or_else(|| "[freeze:contract][canonical_if/then_not_closed]".to_string())?;
        let else_pred = match self.else_route {
            ElseRouteV1::ImplicitFalse => self.layout.header,
            ElseRouteV1::Explicit { .. } => self.else_exit.ok_or_else(|| {
                "[freeze:contract][canonical_if/explicit_else_not_closed]".to_string()
            })?,
        };
        let verified = VerifiedIfMergePredecessorsV1 {
            header: self.layout.header,
            then_entry: self.layout.then_entry,
            else_entry: self.layout.else_entry,
            merge: self.layout.merge,
            then_pred,
            else_pred,
        };
        verified.reverify(builder)?;
        builder.start_new_block(self.layout.merge)?;
        Ok(verified)
    }

    /// Restores the post-condition dispatch block after an aborted If draft.
    ///
    /// The enclosing function transaction discards the partial CFG. Resetting
    /// the current block here closes only Builder-local cursor/cache state so
    /// nested cleanup can continue deterministically.
    pub(super) fn restore_header_after_error(
        &self,
        builder: &mut MirBuilder,
    ) -> Result<(), String> {
        builder.start_new_block(self.layout.header)
    }

    fn open(
        builder: &mut MirBuilder,
        condition: ValueId,
        explicit_else: bool,
    ) -> Result<Self, String> {
        let header = current_unterminated_block(builder, "header")?;
        let then_entry = builder.next_block_id();
        let else_entry = explicit_else.then(|| builder.next_block_id());
        let merge = builder.next_block_id();
        builder.ensure_block_exists(then_entry)?;
        if let Some(else_entry) = else_entry {
            builder.ensure_block_exists(else_entry)?;
        }
        builder.ensure_block_exists(merge)?;

        let false_target = else_entry.unwrap_or(merge);
        branch::emit_conditional(builder, condition, then_entry, false_target)?;
        Ok(Self {
            layout: IfCfgLayoutV1 {
                header,
                then_entry,
                else_entry,
                merge,
            },
            else_route: else_entry
                .map(|entry| ElseRouteV1::Explicit { entry })
                .unwrap_or(ElseRouteV1::ImplicitFalse),
            then_exit: None,
            else_exit: None,
        })
    }
}

impl VerifiedIfMergePredecessorsV1 {
    pub(super) fn reverify(self, builder: &MirBuilder) -> Result<(), String> {
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| {
                "[freeze:contract][canonical_if/predecessors_without_function]".to_string()
            })?;
        let false_target = self.else_entry.unwrap_or(self.merge);
        if !matches!(
            function
                .get_block(self.header)
                .and_then(|block| block.terminator.as_ref()),
            Some(MirInstruction::Branch {
                then_bb,
                else_bb,
                ..
            }) if *then_bb == self.then_entry && *else_bb == false_target
        ) {
            return Err("[freeze:contract][canonical_if/header_route_mismatch]".to_string());
        }
        if !reaches_exit_before_merge(function, self.then_entry, self.then_pred, self.merge) {
            return Err("[freeze:contract][canonical_if/then_exit_not_reachable]".to_string());
        }
        if let Some(else_entry) = self.else_entry {
            if !reaches_exit_before_merge(function, else_entry, self.else_pred, self.merge) {
                return Err("[freeze:contract][canonical_if/else_exit_not_reachable]".to_string());
            }
        } else if self.else_pred != self.header {
            return Err("[freeze:contract][canonical_if/implicit_false_not_header]".to_string());
        }
        let expected = [self.then_pred, self.else_pred]
            .into_iter()
            .collect::<BTreeSet<_>>();
        if expected.len() != 2 {
            return Err("[freeze:contract][canonical_if/duplicate_merge_predecessor]".to_string());
        }
        let actual = compute_predecessors(function)
            .remove(&self.merge)
            .unwrap_or_default()
            .into_iter()
            .collect::<BTreeSet<_>>();
        let cached = function
            .get_block(self.merge)
            .ok_or_else(|| "[freeze:contract][canonical_if/merge_block_missing]".to_string())?
            .predecessors
            .clone();
        if actual != expected || cached != actual {
            return Err(format!(
                "[freeze:contract][canonical_if/actual_predecessor_mismatch] expected={expected:?} actual={actual:?} cached={cached:?}"
            ));
        }
        Ok(())
    }

    pub(super) const fn merge(self) -> BasicBlockId {
        self.merge
    }

    pub(super) const fn then_predecessor(self) -> BasicBlockId {
        self.then_pred
    }

    pub(super) const fn else_predecessor(self) -> BasicBlockId {
        self.else_pred
    }
}

fn reaches_exit_before_merge(
    function: &MirFunction,
    entry: BasicBlockId,
    exit: BasicBlockId,
    merge: BasicBlockId,
) -> bool {
    let mut pending = vec![entry];
    let mut visited = BTreeSet::new();
    while let Some(block) = pending.pop() {
        if block == exit {
            return true;
        }
        if block == merge || !visited.insert(block) {
            continue;
        }
        if let Some(block) = function.get_block(block) {
            pending.extend(block.successors.iter().copied());
        }
    }
    false
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DefinedJoinValueV1 {
    binding: BindingRefV1,
    expected_entry: ValueId,
    value: ValueId,
}

#[derive(Debug)]
pub(super) struct DefinedIfJoinSetV1 {
    values: Vec<DefinedJoinValueV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct DefinedJoinPublishV1(DefinedJoinValueV1);

impl DefinedJoinPublishV1 {
    pub(super) const fn binding(self) -> BindingRefV1 {
        self.0.binding
    }

    pub(super) const fn value(self) -> ValueId {
        self.0.value
    }

    pub(super) const fn expected_entry(self) -> ValueId {
        self.0.expected_entry
    }
}

pub(super) trait DefinedJoinValueStoreV1 {
    fn defined_join_current_value(&self, binding: BindingRefV1) -> Result<ValueId, String>;

    fn publish_defined_join_batch(
        &mut self,
        publishes: Vec<DefinedJoinPublishV1>,
    ) -> Result<(), String>;
}

pub(super) fn define_join_phis(
    builder: &mut MirBuilder,
    predecessors: VerifiedIfMergePredecessorsV1,
    rows: &[ResolvedJoinValueRowV1],
) -> Result<DefinedIfJoinSetV1, String> {
    let cfg_ready_rows = VerifiedResolvedIfCfgReadyJoinRowsV1::verify(builder, predecessors, rows)?;

    let mut values = Vec::with_capacity(rows.len());
    for (row_index, row) in cfg_ready_rows.rows().iter().enumerate() {
        let dst = builder.next_value_id();
        let prepared_completion = crate::mir::builder::phi_completion::prepare_for_resolved_if(
            builder,
            &cfg_ready_rows,
            row_index,
            dst,
        )?;
        phi_lifecycle::define_final_from_prepared_completion(
            builder,
            prepared_completion,
            "canonical_if:final_join",
        )?;
        values.push(DefinedJoinValueV1 {
            binding: row.binding(),
            expected_entry: row.entry(),
            value: dst,
        });
    }
    Ok(DefinedIfJoinSetV1 { values })
}

impl DefinedIfJoinSetV1 {
    pub(super) fn publish_join_values<S: DefinedJoinValueStoreV1>(
        self,
        store: &mut S,
    ) -> Result<(), String> {
        for value in &self.values {
            let actual = store.defined_join_current_value(value.binding)?;
            if actual != value.expected_entry {
                return Err(format!(
                    "[freeze:contract][canonical_if/join_entry_changed_before_publish] binding={:?} expected=%{} actual=%{}",
                    value.binding, value.expected_entry.0, actual.0,
                ));
            }
        }
        store
            .publish_defined_join_batch(self.values.into_iter().map(DefinedJoinPublishV1).collect())
    }
}

fn current_unterminated_block(builder: &MirBuilder, role: &str) -> Result<BasicBlockId, String> {
    let block_id = builder
        .function_state
        .current_block
        .ok_or_else(|| format!("[freeze:contract][canonical_if/{role}_block_missing]"))?;
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| format!("[freeze:contract][canonical_if/{role}_function_missing]"))?;
    let block = function
        .get_block(block_id)
        .ok_or_else(|| format!("[freeze:contract][canonical_if/{role}_block_not_created]"))?;
    if block.is_terminated() {
        return Err(format!(
            "[freeze:contract][canonical_if/{role}_already_terminated] bb={block_id:?}"
        ));
    }
    Ok(block_id)
}
