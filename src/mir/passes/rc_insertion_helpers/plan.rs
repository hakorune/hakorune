#[cfg(feature = "rc-insertion-minimal")]
use std::collections::{HashMap, HashSet};

#[cfg(feature = "rc-insertion-minimal")]
use crate::mir::types::ConstValue;
#[cfg(feature = "rc-insertion-minimal")]
use crate::mir::{MirInstruction, ValueId};

#[cfg(feature = "rc-insertion-minimal")]
use super::types::{DropPoint, DropReason, DropSite, RcPlan};

#[cfg(feature = "rc-insertion-minimal")]
pub(super) fn plan_rc_insertion_for_block(
    insts: &[MirInstruction],
    terminator: Option<&MirInstruction>,
    initial_ptr_to_value: &HashMap<ValueId, ValueId>,
    initial_null_values: &HashSet<ValueId>, // P8: CFG越し null 伝播
) -> (RcPlan, HashMap<ValueId, ValueId>, HashSet<ValueId>) {
    let mut plan = RcPlan { drops: Vec::new() };

    let mut ptr_to_value: HashMap<ValueId, ValueId> = initial_ptr_to_value.clone();
    let mut null_values: HashSet<ValueId> = initial_null_values.clone(); // P8: 初期状態から開始

    for (idx, inst) in insts.iter().enumerate() {
        match inst {
            MirInstruction::Const {
                dst,
                value: ConstValue::Null,
            } => {
                null_values.insert(*dst);
            }
            MirInstruction::Const { dst, .. } => {
                null_values.remove(dst);
            }
            MirInstruction::Copy { dst, src } => {
                if null_values.contains(src) {
                    null_values.insert(*dst);
                } else {
                    null_values.remove(dst);
                }
            }
            _ => {}
        }

        if let MirInstruction::Store { value, ptr } = inst {
            if let Some(old_value) = ptr_to_value.get(ptr) {
                if old_value != value {
                    let reason = if null_values.contains(value) {
                        DropReason::ExplicitNull
                    } else {
                        DropReason::Overwrite
                    };
                    plan.drops.push(DropSite {
                        at: DropPoint::BeforeInstr(idx),
                        values: vec![*old_value],
                        reason,
                    });
                }
            }

            if null_values.contains(value) {
                ptr_to_value.remove(ptr);
            } else {
                ptr_to_value.insert(*ptr, *value);
            }
        }
    }

    if matches!(terminator, Some(MirInstruction::Return { .. })) && !ptr_to_value.is_empty() {
        // P7: HashSet 削除、sort+dedup は apply 側のヘルパーで処理される
        let release_values: Vec<ValueId> = ptr_to_value.values().copied().collect();
        if !release_values.is_empty() {
            plan.drops.push(DropSite {
                at: DropPoint::BeforeTerminator,
                values: release_values,
                reason: DropReason::ReturnCleanup,
            });
        }
    }

    (plan, ptr_to_value, null_values) // P8: end_null_values も返す
}
